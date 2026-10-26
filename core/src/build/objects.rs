//! Buildings of the module "Welt & Bau" (ADR-042): single objects with a
//! footprint, the type table the cells and objects refer to, and the
//! building commands `build`, `buildArea` and `demolish`.
//!
//! Floors and walls are values of cells (floor covering, wall type, flags);
//! single objects (furniture) are records in a [`Pool`] and occupy cells.
//! A command checks everything first and changes nothing if it is refused.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::Build;
use super::grid::{AreaKind, Cell, KIND_MASK, KIND_SHIFT, WALL, Zone, floor_index};
use crate::geometry::{CellPos, CellRect, MapSize, Rotation};
use crate::hash::StateHasher;
use crate::internal::{InternalEvent, InternalEvents, Stage};
use crate::pool::{Kind, Pool};

/// Most single objects a world holds (S-5, wirksam seit ADR-042).
pub const MAX_OBJECTS: usize = 1_000_000;
/// Most distinct object types one world refers to; type numbers are `u16`
/// and 0 means "none".
pub const MAX_TYPES: usize = 65_534;

/// Why a building command was refused. Nothing changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildRefusal {
    /// The footprint or area lies outside the map.
    OutOfMap,
    /// Only the ground floor can be built on (floors above and below
    /// follow in step 5.2).
    FloorLocked,
    /// A cell is occupied by an object or a wall.
    Blocked,
    /// A cell of an object has no floor.
    NeedsFloor,
    /// A quantity above its limit: [`MAX_OBJECTS`] objects, [`MAX_TYPES`]
    /// types or the counter of handles.
    LimitExceeded {
        /// The limit.
        max: usize,
    },
}

/// A single object in the world: furniture and other things that stand on
/// floor cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacedObject {
    /// Serial number, assigned in ascending order and never reused: the
    /// handle of the object. Records are stored in the order of their
    /// handles, so a handle is found by binary search and a stale handle
    /// finds nothing.
    pub handle: u32,
    /// Type number (≥ 1) in the type table of the module.
    pub ty: u16,
    /// Cell with the smallest coordinates of the footprint.
    pub x: u16,
    /// See `x`.
    pub y: u16,
    /// Floor; ground floor (0) until step 5.2.
    pub floor: i8,
    /// Width of the footprint in cells after the rotation.
    pub width: u16,
    /// Depth of the footprint in cells after the rotation.
    pub depth: u16,
    /// Rotation 0 … 3 (multiples of 90°).
    pub rotation: u8,
    /// `true` if the object keeps persons from walking over its cells.
    pub blocks: bool,
}

impl PlacedObject {
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
}

/// What a loaded object is checked against.
pub struct ObjectContext {
    /// Size of the map.
    pub map: MapSize,
    /// Number of entries of the type table.
    pub types: usize,
}

impl Kind for PlacedObject {
    const MAX: usize = MAX_OBJECTS;
    const LIMIT: &'static str = "objects";
    const CHECK: &'static str = "objects";
    type Context = ObjectContext;

    fn is_valid(&self, context: &ObjectContext) -> bool {
        let fits = |start: u16, size: u16, edge: u16| {
            size >= 1 && start.checked_add(size).is_some_and(|end| end <= edge)
        };
        self.ty >= 1
            && usize::from(self.ty) <= context.types
            && self.floor == 0
            && self.rotation <= 3
            && fits(self.x, self.width, context.map.width)
            && fits(self.y, self.depth, context.map.height)
    }

    fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(self.handle);
        hasher.u32(u32::from(self.ty));
        hasher.u32(u32::from(self.x));
        hasher.u32(u32::from(self.y));
        hasher.u32(u32::from(self.width));
        hasher.u32(u32::from(self.depth));
        hasher.bytes(&[
            self.floor.to_le_bytes()[0],
            self.rotation,
            u8::from(self.blocks),
        ]);
    }
}

/// Full ids of the object types that cells and objects of this world refer
/// to (a type gets its number when it is first built). The save game stores
/// the table, so that a cell keeps its meaning when the catalog of a later
/// session is ordered differently.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypeTable {
    ids: Vec<String>,
    numbers: HashMap<String, u16>,
}

impl TypeTable {
    /// Table of a save game; `None` if an id is empty, too long or twice in
    /// the table, or there are too many.
    pub(crate) fn from_ids(ids: Vec<String>) -> Option<Self> {
        if ids.len() > MAX_TYPES {
            return None;
        }
        let mut numbers = HashMap::with_capacity(ids.len());
        for (i, id) in ids.iter().enumerate() {
            let n = u16::try_from(i + 1).ok()?;
            let chars = id.chars().count();
            if chars == 0 || chars > crate::command::MAX_CONTENT_ID_LEN {
                return None;
            }
            if numbers.insert(id.clone(), n).is_some() {
                return None;
            }
        }
        Some(Self { ids, numbers })
    }

    /// The full ids in the order of their numbers.
    #[must_use]
    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    /// Number (≥ 1) of the type with the full id `id`, if it has one.
    #[must_use]
    pub fn number(&self, id: &str) -> Option<u16> {
        self.numbers.get(id).copied()
    }

    /// Full id of the type with number `n`.
    #[must_use]
    pub fn id(&self, n: u16) -> Option<&str> {
        usize::from(n)
            .checked_sub(1)
            .and_then(|i| self.ids.get(i))
            .map(String::as_str)
    }

    fn intern(&mut self, id: &str) -> Result<u16, BuildRefusal> {
        if let Some(n) = self.number(id) {
            return Ok(n);
        }
        if self.ids.len() >= MAX_TYPES {
            return Err(BuildRefusal::LimitExceeded { max: MAX_TYPES });
        }
        self.ids.push(id.to_owned());
        let n = u16::try_from(self.ids.len()).unwrap_or(u16::MAX);
        self.numbers.insert(id.to_owned(), n);
        Ok(n)
    }

    pub(crate) fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(u32::try_from(self.ids.len()).unwrap_or(u32::MAX));
        for id in &self.ids {
            hasher.u32(u32::try_from(id.len()).unwrap_or(u32::MAX));
            hasher.bytes(id.as_bytes());
        }
    }
}

/// Which objects touch which block (32 × 32 cells) of a floor, for the
/// checks of building and demolishing without a search over all objects.
/// Derived data, not stored; rebuilt when a save game is loaded.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Occupancy(BTreeMap<(i8, u16, u16), Vec<u32>>);

impl Occupancy {
    fn blocks_of(rect: CellRect) -> impl Iterator<Item = (i8, u16, u16)> {
        let edge = u16::try_from(super::grid::BLOCK).unwrap_or(32);
        let (bx0, bx1) = (rect.x0 / edge, rect.x1.saturating_sub(1) / edge);
        let (by0, by1) = (rect.y0 / edge, rect.y1.saturating_sub(1) / edge);
        (by0..=by1).flat_map(move |by| (bx0..=bx1).map(move |bx| (rect.floor, bx, by)))
    }

    fn add(&mut self, rect: CellRect, handle: u32) {
        for key in Self::blocks_of(rect) {
            let list = self.0.entry(key).or_default();
            // Handles rise, so a new one belongs at the end; a loaded save
            // game is added in storage order.
            list.push(handle);
        }
    }

    fn remove(&mut self, rect: CellRect, gone: &BTreeSet<u32>) {
        for key in Self::blocks_of(rect) {
            if let Some(list) = self.0.get_mut(&key) {
                list.retain(|h| !gone.contains(h));
                if list.is_empty() {
                    self.0.remove(&key);
                }
            }
        }
    }

    /// Handles of the objects that touch a block the rectangle touches,
    /// ascending, each once.
    fn candidates(&self, rect: CellRect) -> BTreeSet<u32> {
        Self::blocks_of(rect)
            .filter_map(|key| self.0.get(&key))
            .flatten()
            .copied()
            .collect()
    }
}

fn intersect(a: CellRect, b: CellRect) -> Option<CellRect> {
    let r = CellRect {
        floor: a.floor,
        x0: a.x0.max(b.x0),
        y0: a.y0.max(b.y0),
        x1: a.x1.min(b.x1),
        y1: a.y1.min(b.y1),
    };
    (a.floor == b.floor && r.is_non_empty()).then_some(r)
}

fn cells(rect: CellRect) -> impl Iterator<Item = CellPos> {
    (rect.y0..rect.y1).flat_map(move |y| {
        (rect.x0..rect.x1).map(move |x| CellPos {
            x,
            y,
            floor: rect.floor,
        })
    })
}

/// A wall as opposed to void or a cell blocked by an object: it has a type.
fn is_wall(cell: Cell) -> bool {
    cell.flags & WALL != 0 && cell.wall_kind != 0
}

impl Build {
    /// The single objects, in the order of their handles.
    #[must_use]
    pub fn objects(&self) -> &Pool<PlacedObject> {
        &self.objects
    }

    /// The object with `handle`; `None` for a handle that was never issued
    /// or whose object is gone.
    #[must_use]
    pub fn object(&self, handle: u32) -> Option<&PlacedObject> {
        let slice = self.objects.as_slice();
        slice
            .binary_search_by_key(&handle, |o| o.handle)
            .ok()
            .map(|i| &slice[i])
    }

    /// The object types that cells and objects refer to.
    #[must_use]
    pub fn types(&self) -> &TypeTable {
        &self.types
    }

    /// Handle of the next object placed. Together with the number of
    /// objects it identifies the set of objects: a build raises the handle,
    /// a demolition lowers the number, neither goes back (S-1 sender).
    #[must_use]
    pub fn next_handle(&self) -> u32 {
        self.next_handle
    }

    /// The object whose footprint covers `pos`, if any.
    #[must_use]
    pub fn object_at(&self, pos: CellPos) -> Option<&PlacedObject> {
        let rect = CellRect {
            floor: pos.floor,
            x0: pos.x,
            y0: pos.y,
            x1: pos.x.saturating_add(1),
            y1: pos.y.saturating_add(1),
        };
        self.occupancy
            .candidates(rect)
            .into_iter()
            .filter_map(|h| self.object(h))
            .find(|o| intersect(o.rect(), rect).is_some())
    }

    /// Objects that cover at least one cell of `rect`, in handle order.
    fn objects_touching(&self, rect: CellRect) -> Vec<PlacedObject> {
        self.occupancy
            .candidates(rect)
            .into_iter()
            .filter_map(|h| self.object(h).copied())
            .filter(|o| intersect(o.rect(), rect).is_some())
            .collect()
    }

    /// Places a single object (`build`, ADR-042): the whole footprint must
    /// be floor without a wall and without another object. `size` is the
    /// footprint in cells at rotation 0; rotations 1 and 3 swap width and
    /// depth. `at` is the cell with the smallest coordinates. With `blocks`
    /// the cells become impassable for persons (and the flow fields are
    /// recomputed).
    ///
    /// # Errors
    ///
    /// [`BuildRefusal::FloorLocked`] above or below the ground floor,
    /// [`BuildRefusal::OutOfMap`] if the footprint leaves the map,
    /// [`BuildRefusal::Blocked`] for a cell with a wall or an object,
    /// [`BuildRefusal::NeedsFloor`] for a cell without floor,
    /// [`BuildRefusal::LimitExceeded`] for too many objects or types.
    #[allow(clippy::too_many_arguments)] // one command, one call; a struct would only repeat the fields
    pub fn build_object(
        &mut self,
        type_id: &str,
        size: (u16, u16),
        blocks: bool,
        at: CellPos,
        rotation: Rotation,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<u32, BuildRefusal> {
        if at.floor != 0 {
            return Err(BuildRefusal::FloorLocked);
        }
        let (width, depth) = if rotation.0 % 2 == 1 {
            (size.1, size.0)
        } else {
            size
        };
        let rect = CellRect {
            floor: at.floor,
            x0: at.x,
            y0: at.y,
            x1: at.x.saturating_add(width),
            y1: at.y.saturating_add(depth),
        };
        if width == 0
            || depth == 0
            || at.x.checked_add(width).is_none()
            || at.y.checked_add(depth).is_none()
            || !self.map().contains_rect(rect)
        {
            return Err(BuildRefusal::OutOfMap);
        }
        if self.objects.room() == 0 {
            return Err(BuildRefusal::LimitExceeded { max: MAX_OBJECTS });
        }
        let following = self
            .next_handle
            .checked_add(1)
            .ok_or(BuildRefusal::LimitExceeded {
                max: u32::MAX as usize,
            })?;
        if !self.objects_touching(rect).is_empty() {
            return Err(BuildRefusal::Blocked);
        }
        let mut needs_floor = false;
        for pos in cells(rect) {
            let cell = self.grid.cell(pos).unwrap_or(Cell::VOID);
            if is_wall(cell) {
                return Err(BuildRefusal::Blocked);
            }
            needs_floor |= !cell.walkable();
        }
        if needs_floor {
            return Err(BuildRefusal::NeedsFloor);
        }
        let ty = self.types.intern(type_id)?;
        let object = PlacedObject {
            handle: self.next_handle,
            ty,
            x: at.x,
            y: at.y,
            floor: at.floor,
            width,
            depth,
            rotation: rotation.0,
            blocks,
        };
        self.next_handle = following;
        let pushed = self.objects.push(object);
        debug_assert!(pushed, "room was checked");
        self.occupancy.add(rect, object.handle);
        if blocks {
            for pos in cells(rect) {
                let cell = self.grid.cell(pos).unwrap_or(Cell::VOID);
                self.grid
                    .set(
                        pos,
                        Cell {
                            flags: cell.flags | WALL,
                            ..cell
                        },
                    )
                    .expect("the cell lies on the map and keeps its valid fields");
            }
            events.push(from, InternalEvent::GridChanged);
        }
        Ok(object.handle)
    }

    /// Draws a floor over `rect` (`buildArea` with the effect `floor`,
    /// ADR-042): floor covering, area kind and cost factor of every cell
    /// become those of the type; zone and staff zone stay. Wall cells stay
    /// walls and are not touched; a cell under a blocking object keeps its
    /// object and stays impassable.
    ///
    /// # Errors
    ///
    /// [`BuildRefusal::FloorLocked`] above or below the ground floor,
    /// [`BuildRefusal::LimitExceeded`] for too many types.
    pub fn build_floor(
        &mut self,
        type_id: &str,
        area_kind: AreaKind,
        cost: u8,
        rect: CellRect,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<(), BuildRefusal> {
        self.check_area(rect)?;
        let ty = self.types.intern(type_id)?;
        let under_object = self.blocked_cells(rect);
        let before = self.grid.version();
        for pos in cells(rect) {
            let cell = self.grid.cell(pos).unwrap_or(Cell::VOID);
            if is_wall(cell) {
                continue;
            }
            let mut flags = (cell.flags & !(KIND_MASK | WALL)) | ((area_kind as u8) << KIND_SHIFT);
            if under_object.contains(&(pos.x, pos.y)) {
                flags |= WALL;
            }
            let zone = if cell == Cell::VOID {
                Zone::Landside as u8
            } else {
                cell.zone
            };
            self.grid
                .set(
                    pos,
                    Cell {
                        flags,
                        zone,
                        cost,
                        floor_kind: ty,
                        wall_kind: 0,
                    },
                )
                .expect("the cell lies on the map and the cost was checked by the loader");
        }
        if self.grid.version() != before {
            events.push(from, InternalEvent::GridChanged);
        }
        Ok(())
    }

    /// Draws a wall over `rect` (`buildArea` with the effect `wall`,
    /// ADR-042): every cell becomes impassable and shows the wall type. A
    /// single object in the area refuses the whole command.
    ///
    /// # Errors
    ///
    /// [`BuildRefusal::FloorLocked`] above or below the ground floor,
    /// [`BuildRefusal::Blocked`] if an object stands in the area,
    /// [`BuildRefusal::LimitExceeded`] for too many types.
    pub fn build_wall(
        &mut self,
        type_id: &str,
        rect: CellRect,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<(), BuildRefusal> {
        self.check_area(rect)?;
        if !self.objects_touching(rect).is_empty() {
            return Err(BuildRefusal::Blocked);
        }
        let ty = self.types.intern(type_id)?;
        let before = self.grid.version();
        for pos in cells(rect) {
            let cell = self.grid.cell(pos).unwrap_or(Cell::VOID);
            self.grid
                .set(
                    pos,
                    Cell {
                        flags: cell.flags | WALL,
                        wall_kind: ty,
                        ..cell
                    },
                )
                .expect("the cell lies on the map and keeps its valid fields");
        }
        if self.grid.version() != before {
            events.push(from, InternalEvent::GridChanged);
        }
        Ok(())
    }

    /// Demolishes everything in `rect` (`demolish`, ADR-042): every object
    /// whose footprint touches the area is removed as a whole, floors and
    /// walls inside the area become void. The cells of a removed object
    /// outside the area keep their floor and become passable again.
    ///
    /// # Errors
    ///
    /// [`BuildRefusal::FloorLocked`] above or below the ground floor.
    pub fn demolish(
        &mut self,
        rect: CellRect,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<(), BuildRefusal> {
        self.check_area(rect)?;
        let before = self.grid.version();
        let removed = self.objects_touching(rect);
        if !removed.is_empty() {
            let gone: BTreeSet<u32> = removed.iter().map(|o| o.handle).collect();
            self.objects
                .items_mut()
                .retain(|o| !gone.contains(&o.handle));
            for object in &removed {
                self.occupancy.remove(object.rect(), &gone);
                if !object.blocks {
                    continue;
                }
                for pos in cells(object.rect()) {
                    let cell = self.grid.cell(pos).unwrap_or(Cell::VOID);
                    self.grid
                        .set(
                            pos,
                            Cell {
                                flags: cell.flags & !WALL,
                                ..cell
                            },
                        )
                        .expect("the cell lies on the map and keeps its valid fields");
                }
            }
        }
        for pos in cells(rect) {
            self.grid
                .set(pos, Cell::VOID)
                .expect("the cell lies on the map");
        }
        if self.grid.version() != before {
            events.push(from, InternalEvent::GridChanged);
        }
        Ok(())
    }

    fn check_area(&self, rect: CellRect) -> Result<(), BuildRefusal> {
        if rect.floor != 0 {
            return Err(BuildRefusal::FloorLocked);
        }
        if floor_index(rect.floor).is_none()
            || !rect.is_non_empty()
            || !self.map().contains_rect(rect)
        {
            return Err(BuildRefusal::OutOfMap);
        }
        Ok(())
    }

    /// Cells of `rect` under a blocking object.
    fn blocked_cells(&self, rect: CellRect) -> HashSet<(u16, u16)> {
        let mut set = HashSet::new();
        for object in self.objects_touching(rect).iter().filter(|o| o.blocks) {
            if let Some(inner) = intersect(object.rect(), rect) {
                set.extend(cells(inner).map(|p| (p.x, p.y)));
            }
        }
        set
    }

    /// Occupancy directory of a loaded list of objects; `None` if two
    /// objects overlap.
    pub(crate) fn occupancy_of(objects: &[PlacedObject]) -> Option<Occupancy> {
        let mut occupancy = Occupancy::default();
        for object in objects {
            let rect = object.rect();
            let clash = occupancy.candidates(rect).into_iter().any(|h| {
                objects
                    .binary_search_by_key(&h, |o| o.handle)
                    .ok()
                    .is_some_and(|i| intersect(objects[i].rect(), rect).is_some())
            });
            if clash {
                return None;
            }
            occupancy.add(rect, object.handle);
        }
        Some(occupancy)
    }
}
