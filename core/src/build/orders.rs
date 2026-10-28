//! Construction orders and delivery points of the module "Welt & Bau"
//! (ADR-045).
//!
//! The commands `build` and `buildArea` place an order instead of building
//! at once. An order occupies its cells (no other command builds there),
//! is visible from the tick it was placed in and changes nothing about who
//! may walk where. Construction workers of the module "Personen" report
//! the work they did as an internal event; when the work of an order is
//! done, the order takes effect like a building of step 3.1a and is gone.
//!
//! A delivery point is a single object that stands at once, also on
//! terrain; its construction workers start there and fetch material there.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::Build;
use super::grid::{AreaKind, Cell, MAX_COST, floor_index};
use super::objects::{BuildRefusal, Occupancy, cells, intersect, is_wall};
use crate::geometry::{CellPos, CellRect, MapSize, Rotation};
use crate::hash::StateHasher;
use crate::internal::{InternalEvent, InternalEvents, Stage};
use crate::pool::Kind;

/// Most open orders of a world (ADR-045; estimate, re-measured in 4.1).
pub const MAX_ORDERS: usize = 65_536;
/// Most delivery points of a world (ADR-045: one flow field each).
pub const MAX_DELIVERY_POINTS: usize = 16;
/// Ticks of one game minute (a tick is a third of a game second, ADR-020):
/// the unit of work of an order is one tick of one construction worker.
pub const TICKS_PER_MINUTE: u32 = 180;
/// Most construction workers of one delivery point (S-4 `workers`).
pub const MAX_WORKERS: u8 = 64;

/// What an order builds when its work is done.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderWork {
    /// A floor with its area kind and cost factor (effect `floor`).
    Floor {
        /// What the floor is used for.
        area_kind: AreaKind,
        /// Cost factor in tenths.
        cost: u8,
    },
    /// A wall (effect `wall`).
    Wall,
    /// A single object.
    Object {
        /// Rotation 0 … 3; the footprint of the order is already turned.
        rotation: u8,
        /// The object keeps persons from walking over its cells.
        blocks: bool,
    },
}

/// State of an order in the state buffer S-1 (ADR-045).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OrderState {
    /// No material has arrived yet.
    Planned = 1,
    /// Work has started.
    Building = 2,
}

/// An open construction order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    /// Serial number of the order, ascending and never reused (orders are
    /// stored in this order).
    pub handle: u32,
    /// Type number (≥ 1) in the type table of the module.
    pub ty: u16,
    /// What the order builds.
    pub work_kind: OrderWork,
    /// Cell with the smallest coordinates of the footprint.
    pub x: u16,
    /// See `x`.
    pub y: u16,
    /// Floor; ground floor (0) until step 5.2.
    pub floor: i8,
    /// Width of the footprint in cells.
    pub width: u16,
    /// Depth of the footprint in cells.
    pub depth: u16,
    /// Work the order needs, in ticks of one construction worker.
    pub work: u32,
    /// Work done so far, below `work`.
    pub done: u32,
}

impl Order {
    /// Cells of the footprint.
    #[must_use]
    pub fn rect(&self) -> CellRect {
        CellRect {
            floor: self.floor,
            x0: self.x,
            y0: self.y,
            x1: self.x.saturating_add(self.width),
            y1: self.y.saturating_add(self.depth),
        }
    }

    /// State for the display: planned until work arrives.
    #[must_use]
    pub fn state(&self) -> OrderState {
        if self.done == 0 {
            OrderState::Planned
        } else {
            OrderState::Building
        }
    }

    /// `true` if a construction worker on `pos` works on this order: `pos`
    /// is outside the footprint and touches it (also diagonally).
    #[must_use]
    pub fn is_workplace(&self, pos: CellPos) -> bool {
        let r = self.rect();
        let inside = |v: u16, lo: u16, hi: u16| v >= lo && v < hi;
        let near = |v: u16, lo: u16, hi: u16| v.saturating_add(1) >= lo && v <= hi;
        pos.floor == r.floor
            && near(pos.x, r.x0, r.x1)
            && near(pos.y, r.y0, r.y1)
            && !(inside(pos.x, r.x0, r.x1) && inside(pos.y, r.y0, r.y1))
    }
}

/// What a loaded order is checked against.
pub struct OrderContext {
    /// Size of the map.
    pub map: MapSize,
    /// Number of entries of the type table.
    pub types: usize,
}

impl Kind for Order {
    const MAX: usize = MAX_ORDERS;
    const LIMIT: &'static str = "orders";
    const CHECK: &'static str = "orders";
    type Context = OrderContext;

    fn is_valid(&self, context: &OrderContext) -> bool {
        let fits = |start: u16, size: u16, edge: u16| {
            size >= 1 && start.checked_add(size).is_some_and(|end| end <= edge)
        };
        let kind_ok = match self.work_kind {
            OrderWork::Floor { cost, .. } => (1..=MAX_COST).contains(&cost),
            OrderWork::Wall => true,
            OrderWork::Object { rotation, .. } => rotation <= 3,
        };
        self.ty >= 1
            && usize::from(self.ty) <= context.types
            && self.floor == 0
            && kind_ok
            && self.done < self.work
            && fits(self.x, self.width, context.map.width)
            && fits(self.y, self.depth, context.map.height)
    }

    fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(self.handle);
        hasher.u32(u32::from(self.ty));
        let (kind, a, b) = match self.work_kind {
            OrderWork::Floor { area_kind, cost } => (0, area_kind as u8, cost),
            OrderWork::Wall => (1, 0, 0),
            OrderWork::Object { rotation, blocks } => (2, rotation, u8::from(blocks)),
        };
        hasher.bytes(&[kind, a, b, self.floor.to_le_bytes()[0]]);
        for v in [self.x, self.y, self.width, self.depth] {
            hasher.u32(u32::from(v));
        }
        hasher.u32(self.work);
        hasher.u32(self.done);
    }
}

/// A delivery point of a construction company: the single object with the
/// handle `handle` and its construction workers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryPoint {
    /// Handle of the object.
    pub handle: u32,
    /// Construction workers of the company (1 … [`MAX_WORKERS`]).
    pub workers: u8,
}

impl Kind for DeliveryPoint {
    const MAX: usize = MAX_DELIVERY_POINTS;
    const LIMIT: &'static str = "deliveryPoints";
    const CHECK: &'static str = "deliveryPoints";
    type Context = ();

    fn is_valid(&self, (): &()) -> bool {
        (1..=MAX_WORKERS).contains(&self.workers)
    }

    fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(self.handle);
        hasher.bytes(&[self.workers]);
    }
}

impl Build {
    /// The open orders, in the order of their handles.
    #[must_use]
    pub fn orders(&self) -> &crate::pool::Pool<Order> {
        &self.orders
    }

    /// The open order with `handle`; `None` once it is done or cancelled.
    #[must_use]
    pub fn order(&self, handle: u32) -> Option<&Order> {
        let slice = self.orders.as_slice();
        slice
            .binary_search_by_key(&handle, |o| o.handle)
            .ok()
            .map(|i| &slice[i])
    }

    /// The delivery points, in the order of their handles.
    #[must_use]
    pub fn delivery_points(&self) -> &crate::pool::Pool<DeliveryPoint> {
        &self.delivery_points
    }

    /// The delivery point with the object handle `handle`.
    #[must_use]
    pub fn delivery_point(&self, handle: u32) -> Option<&DeliveryPoint> {
        self.delivery_points
            .as_slice()
            .iter()
            .find(|d| d.handle == handle)
    }

    /// Counter of the changes of the orders that the display shows (new,
    /// started, done, cancelled); derived, neither saved nor hashed.
    #[must_use]
    pub fn orders_version(&self) -> u32 {
        self.orders_version
    }

    /// The open order a construction worker on `pos` works on: among the
    /// orders whose footprint `pos` touches from outside the one with the
    /// smallest handle (ADR-045).
    #[must_use]
    pub fn workplace_order(&self, pos: CellPos) -> Option<&Order> {
        let around = CellRect {
            floor: pos.floor,
            x0: pos.x.saturating_sub(1),
            y0: pos.y.saturating_sub(1),
            x1: pos.x.saturating_add(2),
            y1: pos.y.saturating_add(2),
        };
        self.order_occupancy
            .candidates(around)
            .into_iter()
            .filter_map(|h| self.order(h))
            .find(|o| o.is_workplace(pos))
    }

    /// The cells construction workers work from: the cells around the
    /// footprints of all open orders that are outside every footprint and
    /// walkable for construction workers, ascending and each once (start
    /// cells of the field "all construction sites", ADR-045).
    #[must_use]
    pub fn site_cells(&self) -> Vec<CellPos> {
        let rules = super::access::AccessProfile::Worker.rules();
        let map = self.map();
        let mut out = BTreeSet::new();
        for order in self.orders.as_slice() {
            let r = order.rect();
            let x0 = r.x0.saturating_sub(1);
            let y0 = r.y0.saturating_sub(1);
            let x1 = r.x1.saturating_add(1).min(map.width);
            let y1 = r.y1.saturating_add(1).min(map.height);
            for y in y0..y1 {
                for x in x0..x1 {
                    let on_edge = x < r.x0 || x >= r.x1 || y < r.y0 || y >= r.y1;
                    if !on_edge {
                        continue;
                    }
                    let pos = CellPos {
                        x,
                        y,
                        floor: r.floor,
                    };
                    let free = self.grid.cell(pos).is_some_and(|c| rules.walkable(c.flags));
                    if free && !self.inside_order(pos) {
                        out.insert((y, x));
                    }
                }
            }
        }
        out.into_iter()
            .map(|(y, x)| CellPos { x, y, floor: 0 })
            .collect()
    }

    fn inside_order(&self, pos: CellPos) -> bool {
        let cell = CellRect {
            floor: pos.floor,
            x0: pos.x,
            y0: pos.y,
            x1: pos.x.saturating_add(1),
            y1: pos.y.saturating_add(1),
        };
        self.order_touches(cell)
    }

    /// `true` if an open order covers a cell of `rect`.
    pub(super) fn order_touches(&self, rect: CellRect) -> bool {
        self.order_occupancy
            .candidates(rect)
            .into_iter()
            .filter_map(|h| self.order(h))
            .any(|o| intersect(o.rect(), rect).is_some())
    }

    /// Places an order for a floor over `rect` (`buildArea` with the effect
    /// `floor`, ADR-045) with `minutes` of work per cell.
    ///
    /// # Errors
    ///
    /// [`BuildRefusal::FloorLocked`] above or below the ground floor,
    /// [`BuildRefusal::OutOfMap`] outside the map, [`BuildRefusal::Blocked`]
    /// if an open order covers a cell, [`BuildRefusal::LimitExceeded`] for
    /// too many orders or types.
    #[allow(clippy::too_many_arguments)] // one command, one call
    pub fn order_floor(
        &mut self,
        type_id: &str,
        area_kind: AreaKind,
        cost: u8,
        minutes: u16,
        rect: CellRect,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<u32, BuildRefusal> {
        self.check_area(rect)?;
        self.check_order(rect)?;
        let ty = self.types.intern(type_id)?;
        Ok(self.add_order(
            ty,
            OrderWork::Floor { area_kind, cost },
            rect,
            minutes,
            events,
            from,
        ))
    }

    /// Places an order for a wall over `rect` (`buildArea` with the effect
    /// `wall`, ADR-045) with `minutes` of work per cell.
    ///
    /// # Errors
    ///
    /// As [`Build::order_floor`]; also [`BuildRefusal::Blocked`] if an
    /// object stands in the area.
    pub fn order_wall(
        &mut self,
        type_id: &str,
        minutes: u16,
        rect: CellRect,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<u32, BuildRefusal> {
        self.check_area(rect)?;
        if !self.objects_touching(rect).is_empty() {
            return Err(BuildRefusal::Blocked);
        }
        self.check_order(rect)?;
        let ty = self.types.intern(type_id)?;
        Ok(self.add_order(ty, OrderWork::Wall, rect, minutes, events, from))
    }

    /// Places an order for a single object (`build`, ADR-045) with
    /// `minutes` of work; the footprint follows [`Build::build_object`].
    ///
    /// # Errors
    ///
    /// As [`Build::build_object`]; also [`BuildRefusal::Blocked`] if an open
    /// order covers a cell and [`BuildRefusal::LimitExceeded`] for too many
    /// orders.
    #[allow(clippy::too_many_arguments)] // one command, one call
    pub fn order_object(
        &mut self,
        type_id: &str,
        size: (u16, u16),
        blocks: bool,
        minutes: u16,
        at: CellPos,
        rotation: Rotation,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<u32, BuildRefusal> {
        let rect = self.footprint(size, at, rotation)?;
        // Every order may become an object: room for all of them.
        if self.objects.room() <= self.orders.len() {
            return Err(BuildRefusal::LimitExceeded {
                max: super::objects::MAX_OBJECTS,
            });
        }
        self.check_room_for_object()?;
        self.check_object_cells(rect, true)?;
        self.check_order(rect)?;
        let ty = self.types.intern(type_id)?;
        Ok(self.add_order(
            ty,
            OrderWork::Object {
                rotation: rotation.0,
                blocks,
            },
            rect,
            minutes,
            events,
            from,
        ))
    }

    /// Builds a delivery point at once (`build` with the effect
    /// `deliveryPoint`, ADR-045): it may stand on terrain or floor, never
    /// blocks and brings `workers` construction workers.
    ///
    /// # Errors
    ///
    /// As [`Build::build_object`], except that terrain counts as ground;
    /// [`BuildRefusal::LimitExceeded`] for more than
    /// [`MAX_DELIVERY_POINTS`] delivery points.
    #[allow(clippy::too_many_arguments)] // one command, one call
    pub fn build_delivery_point(
        &mut self,
        type_id: &str,
        size: (u16, u16),
        workers: u8,
        at: CellPos,
        rotation: Rotation,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<u32, BuildRefusal> {
        let rect = self.footprint(size, at, rotation)?;
        if self.delivery_points.room() == 0 {
            return Err(BuildRefusal::LimitExceeded {
                max: MAX_DELIVERY_POINTS,
            });
        }
        self.check_room_for_object()?;
        self.check_object_cells(rect, false)?;
        let ty = self.types.intern(type_id)?;
        let handle = self.place_object(ty, rect, rotation.0, false, events, from)?;
        let pushed = self.delivery_points.push(DeliveryPoint {
            handle,
            workers: workers.clamp(1, MAX_WORKERS),
        });
        debug_assert!(pushed, "room was checked");
        events.push(from, InternalEvent::DeliveryPointBuilt { handle });
        Ok(handle)
    }

    fn check_order(&self, rect: CellRect) -> Result<(), BuildRefusal> {
        if self.order_touches(rect) {
            return Err(BuildRefusal::Blocked);
        }
        if self.orders.room() == 0 {
            return Err(BuildRefusal::LimitExceeded { max: MAX_ORDERS });
        }
        if self.next_order == u32::MAX {
            return Err(BuildRefusal::LimitExceeded {
                max: u32::MAX as usize,
            });
        }
        Ok(())
    }

    fn add_order(
        &mut self,
        ty: u16,
        work_kind: OrderWork,
        rect: CellRect,
        minutes: u16,
        events: &mut InternalEvents,
        from: Stage,
    ) -> u32 {
        let units = match work_kind {
            OrderWork::Object { .. } => 1,
            _ => u32::from(rect.x1 - rect.x0) * u32::from(rect.y1 - rect.y0),
        };
        let order = Order {
            handle: self.next_order,
            ty,
            work_kind,
            x: rect.x0,
            y: rect.y0,
            floor: rect.floor,
            width: rect.x1 - rect.x0,
            depth: rect.y1 - rect.y0,
            work: (units * u32::from(minutes.max(1)) * TICKS_PER_MINUTE).max(1),
            done: 0,
        };
        self.next_order += 1;
        let pushed = self.orders.push(order);
        debug_assert!(pushed, "room was checked");
        self.order_occupancy.add(rect, order.handle);
        self.orders_version = self.orders_version.wrapping_add(1);
        events.push(from, InternalEvent::SitesChanged);
        order.handle
    }

    /// Cancels the open orders whose footprint touches `rect` (part of
    /// `demolish`, ADR-045).
    pub(super) fn cancel_orders(
        &mut self,
        rect: CellRect,
        events: &mut InternalEvents,
        from: Stage,
    ) {
        let gone: BTreeSet<u32> = self
            .order_occupancy
            .candidates(rect)
            .into_iter()
            .filter(|&h| {
                self.order(h)
                    .is_some_and(|o| intersect(o.rect(), rect).is_some())
            })
            .collect();
        self.remove_orders(&gone, events, from);
    }

    fn remove_orders(&mut self, gone: &BTreeSet<u32>, events: &mut InternalEvents, from: Stage) {
        if gone.is_empty() {
            return;
        }
        for handle in gone {
            if let Some(order) = self.order(*handle).copied() {
                self.order_occupancy.remove(order.rect(), gone);
            }
        }
        self.orders
            .items_mut()
            .retain(|o| !gone.contains(&o.handle));
        self.orders_version = self.orders_version.wrapping_add(1);
        events.push(from, InternalEvent::SitesChanged);
    }

    /// Forgets the delivery points among the removed objects `gone`; their
    /// construction workers leave (ADR-045).
    pub(super) fn forget_delivery_points(
        &mut self,
        gone: &BTreeSet<u32>,
        events: &mut InternalEvents,
        from: Stage,
    ) {
        let removed: Vec<u32> = self
            .delivery_points
            .as_slice()
            .iter()
            .map(|d| d.handle)
            .filter(|h| gone.contains(h))
            .collect();
        if removed.is_empty() {
            return;
        }
        self.delivery_points
            .items_mut()
            .retain(|d| !gone.contains(&d.handle));
        for handle in removed {
            events.push(from, InternalEvent::DeliveryPointRemoved { handle });
        }
    }

    /// Stage of the module in a step (ADR-039, ADR-045): adds the work the
    /// construction workers reported in the previous step to their orders,
    /// in the order of the handles, and lets every order whose work is done
    /// take effect.
    pub(crate) fn step(&mut self, events: &mut InternalEvents) {
        let mut work: BTreeMap<u32, u32> = BTreeMap::new();
        for event in events.for_stage(Stage::Build) {
            if let InternalEvent::WorkDone { order, ticks } = event {
                let total = work.entry(order).or_default();
                *total = total.saturating_add(ticks);
            }
        }
        let mut finished = Vec::new();
        for (handle, ticks) in work {
            let slice = self.orders.items_mut();
            let Ok(i) = slice.binary_search_by_key(&handle, |o| o.handle) else {
                continue;
            };
            let order = &mut slice[i];
            let started = order.done == 0;
            order.done = order.done.saturating_add(ticks);
            if order.done >= order.work {
                finished.push(*order);
            } else if started {
                self.orders_version = self.orders_version.wrapping_add(1);
            }
        }
        for order in finished {
            self.finish(order, events, Stage::Build);
        }
    }

    /// Lets every open order take effect now, in the order of the handles
    /// (scenarios and tests that build without construction workers).
    pub fn complete_orders(&mut self, events: &mut InternalEvents, from: Stage) {
        for order in self.orders.saved() {
            self.finish(order, events, from);
        }
    }

    /// The order takes effect and is gone. If the building is not possible
    /// any more (a limit was reached meanwhile), the order is cancelled.
    fn finish(&mut self, order: Order, events: &mut InternalEvents, from: Stage) {
        self.remove_orders(&BTreeSet::from([order.handle]), events, from);
        let rect = order.rect();
        match order.work_kind {
            OrderWork::Floor { area_kind, cost } => {
                self.paint_floor(order.ty, area_kind, cost, rect, events, from);
            }
            OrderWork::Wall => {
                if self.objects_touching(rect).is_empty() {
                    self.paint_wall(order.ty, rect, events, from);
                }
            }
            OrderWork::Object { rotation, blocks } => {
                if self.check_object_cells(rect, true).is_ok() {
                    let _ = self.place_object(order.ty, rect, rotation, blocks, events, from);
                }
            }
        }
    }

    /// The display value of the cell `pos` for an open order on it: the
    /// order's state and the type it shows as floor covering or wall type.
    /// `None` for a cell without an order or a wall cell under a floor
    /// order (a floor leaves walls alone).
    #[must_use]
    pub fn planned_cell(&self, pos: CellPos) -> Option<PlannedCell> {
        let cell = CellRect {
            floor: pos.floor,
            x0: pos.x,
            y0: pos.y,
            x1: pos.x.saturating_add(1),
            y1: pos.y.saturating_add(1),
        };
        let order = self
            .order_occupancy
            .candidates(cell)
            .into_iter()
            .filter_map(|h| self.order(h))
            .find(|o| intersect(o.rect(), cell).is_some())?;
        let current = self.grid.cell(pos).unwrap_or(Cell::VOID);
        match order.work_kind {
            OrderWork::Floor { .. } if is_wall(current) => None,
            OrderWork::Floor { .. } => Some(PlannedCell {
                state: order.state(),
                floor_kind: order.ty,
                wall_kind: current.wall_kind,
            }),
            OrderWork::Wall => Some(PlannedCell {
                state: order.state(),
                floor_kind: current.floor_kind,
                wall_kind: order.ty,
            }),
            OrderWork::Object { .. } => None,
        }
    }

    /// Orders of floors and walls, for the display of a block.
    pub fn area_orders(&self) -> impl Iterator<Item = &Order> {
        self.orders
            .as_slice()
            .iter()
            .filter(|o| !matches!(o.work_kind, OrderWork::Object { .. }))
    }

    /// Orders of single objects, for the display.
    pub fn object_orders(&self) -> impl Iterator<Item = &Order> {
        self.orders
            .as_slice()
            .iter()
            .filter(|o| matches!(o.work_kind, OrderWork::Object { .. }))
    }

    /// Occupancy of loaded orders; `None` if two orders overlap.
    pub(super) fn order_occupancy_of(orders: &[Order]) -> Option<Occupancy> {
        let mut occupancy = Occupancy::default();
        for order in orders {
            let rect = order.rect();
            floor_index(rect.floor)?;
            let clash = occupancy.candidates(rect).into_iter().any(|h| {
                orders
                    .binary_search_by_key(&h, |o| o.handle)
                    .ok()
                    .is_some_and(|i| intersect(orders[i].rect(), rect).is_some())
            });
            if clash {
                return None;
            }
            occupancy.add(rect, order.handle);
        }
        Some(occupancy)
    }

    /// Cells of the order `order` (for tests and the display).
    pub fn order_cells(order: &Order) -> impl Iterator<Item = CellPos> {
        cells(order.rect())
    }
}

/// What the display shows on a cell with an open order of a floor or wall.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedCell {
    /// State of the order.
    pub state: OrderState,
    /// Floor covering shown (type number of the module, 0 for none).
    pub floor_kind: u16,
    /// Wall type shown (type number of the module, 0 for none).
    pub wall_kind: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order(x: u16, y: u16, width: u16, depth: u16) -> Order {
        Order {
            handle: 0,
            ty: 1,
            work_kind: OrderWork::Wall,
            x,
            y,
            floor: 0,
            width,
            depth,
            work: 10,
            done: 0,
        }
    }

    #[test]
    fn a_workplace_touches_the_footprint_from_outside() {
        let o = order(5, 5, 2, 1);
        let at = |x, y| o.is_workplace(CellPos { x, y, floor: 0 });
        assert!(at(4, 4) && at(7, 6) && at(6, 4) && at(4, 5));
        assert!(!at(5, 5) && !at(6, 5), "inside the footprint");
        assert!(!at(3, 5) && !at(8, 5) && !at(5, 7), "too far");
        assert!(!o.is_workplace(CellPos {
            x: 4,
            y: 4,
            floor: 1
        }));
        let corner = order(0, 0, 1, 1);
        assert!(corner.is_workplace(CellPos {
            x: 1,
            y: 1,
            floor: 0
        }));
        assert_eq!(corner.state(), OrderState::Planned);
        assert_eq!(Order { done: 1, ..corner }.state(), OrderState::Building);
    }

    #[test]
    fn loaded_orders_and_delivery_points_are_checked() {
        let context = OrderContext {
            map: MapSize::new(64, 64).unwrap(),
            types: 1,
        };
        assert!(order(0, 0, 2, 2).is_valid(&context));
        for broken in [
            Order {
                ty: 0,
                ..order(0, 0, 1, 1)
            },
            Order {
                ty: 2,
                ..order(0, 0, 1, 1)
            },
            Order {
                floor: 1,
                ..order(0, 0, 1, 1)
            },
            Order {
                done: 10,
                ..order(0, 0, 1, 1)
            },
            order(63, 0, 2, 1),
            order(0, 0, 0, 1),
            Order {
                work_kind: OrderWork::Floor {
                    area_kind: AreaKind::PublicWalk,
                    cost: 0,
                },
                ..order(0, 0, 1, 1)
            },
            Order {
                work_kind: OrderWork::Object {
                    rotation: 4,
                    blocks: true,
                },
                ..order(0, 0, 1, 1)
            },
        ] {
            assert!(!broken.is_valid(&context), "{broken:?}");
        }
        assert!(
            DeliveryPoint {
                handle: 0,
                workers: 1
            }
            .is_valid(&())
        );
        assert!(
            !DeliveryPoint {
                handle: 0,
                workers: 0
            }
            .is_valid(&())
        );
        assert!(
            !DeliveryPoint {
                handle: 0,
                workers: 65
            }
            .is_valid(&())
        );
    }
}
