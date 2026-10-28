//! Module "Welt & Bau" (ADR-039): grid, targets and path networks, with
//! their part of the save game, the checks of a loaded state and their
//! contribution to the state hash.
//!
//! The module reads no other domain module. Changes that other modules
//! react to are laid down as internal events (`internal`).

pub mod access;
pub mod grid;
pub mod network;
pub mod objects;
pub mod orders;
pub mod target;

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::geometry::{CellPos, CellRect, MapSize};
use crate::hash::StateHasher;
use crate::internal::{InternalEvent, InternalEvents, Stage};
use crate::pool::Pool;
use crate::savegame::{invalid, limit};
use crate::text::Reason;
use access::AccessProfile;
use grid::{BLOCK_CELLS, Block, Cell, Grid, GridError, GridSnapshot, WALL, floor_index};
use network::{Network, NetworkKind, NodeId};
use objects::{MAX_OBJECTS, MAX_TYPES, ObjectContext, Occupancy, PlacedObject, TypeTable};
use orders::{DeliveryPoint, MAX_DELIVERY_POINTS, MAX_ORDERS, Order, OrderContext};
use target::{LoadKind, MAX_TARGETS, Target};

/// Why a target could not be added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetError {
    /// The world has [`MAX_TARGETS`] targets already.
    TooMany,
    /// The target has no cell.
    NoCells,
}

/// State of the module: grid, targets and path networks.
pub struct Build {
    grid: Grid,
    targets: Vec<Target>,
    networks: [Network; 3],
    /// Single objects (ADR-042), in the order of their handles.
    objects: Pool<PlacedObject>,
    /// Which objects touch which block; derived from `objects`.
    occupancy: Occupancy,
    /// Object types that cells and objects refer to.
    types: TypeTable,
    /// Handle of the next object placed.
    next_handle: u32,
    /// Open construction orders (ADR-045), in the order of their handles.
    orders: Pool<Order>,
    /// Which orders touch which block; derived from `orders`.
    order_occupancy: Occupancy,
    /// Handle of the next order placed.
    next_order: u32,
    /// Delivery points of construction companies (ADR-045), in the order of
    /// their object handles.
    delivery_points: Pool<DeliveryPoint>,
    /// Changes of the orders for the display; derived, not saved.
    orders_version: u32,
}

impl Build {
    /// Empty: nothing is walkable, there are no targets and no networks.
    #[must_use]
    pub fn new(map: MapSize) -> Self {
        Self {
            grid: Grid::new(map),
            targets: Vec::new(),
            networks: NetworkKind::ALL.map(Network::new),
            objects: Pool::default(),
            occupancy: Occupancy::default(),
            types: TypeTable::default(),
            next_handle: 0,
            orders: Pool::default(),
            order_occupancy: Occupancy::default(),
            next_order: 0,
            delivery_points: Pool::default(),
            orders_version: 0,
        }
    }

    /// Size of the map.
    #[must_use]
    pub fn map(&self) -> MapSize {
        self.grid.map()
    }

    /// The grid.
    #[must_use]
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// The grid for building. Changed derived data are recomputed on request
    /// (`Simulation::request_recompute` or the internal event
    /// [`InternalEvent::GridChanged`]), not automatically.
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    /// Targets; the index is the field number.
    #[must_use]
    pub fn targets(&self) -> &[Target] {
        &self.targets
    }

    /// Adds a target and returns its field number. The field exists after
    /// the next recomputation.
    ///
    /// # Errors
    ///
    /// Fails if the target has no cell or the world has too many targets.
    pub fn add_target(&mut self, target: Target) -> Result<u8, TargetError> {
        if target.cells.is_empty() {
            return Err(TargetError::NoCells);
        }
        if self.targets.len() >= MAX_TARGETS {
            return Err(TargetError::TooMany);
        }
        self.targets.push(target);
        Ok(u8::try_from(self.targets.len() - 1).unwrap_or(u8::MAX))
    }

    /// Network of the given kind.
    #[must_use]
    pub fn network(&self, kind: NetworkKind) -> &Network {
        &self.networks[kind.index()]
    }

    /// Network of the given kind for editing.
    pub fn network_mut(&mut self, kind: NetworkKind) -> &mut Network {
        &mut self.networks[kind.index()]
    }

    /// Sets (`wall`) or clears the wall flag of every cell in `rect`, keeping
    /// zone and cost (debug command for building without data packs), and
    /// lays down [`InternalEvent::GridChanged`] for the stage `from`.
    ///
    /// # Errors
    ///
    /// Fails, without changing anything, if the rectangle is empty, not on
    /// the map or on an invalid floor.
    pub fn set_walls(
        &mut self,
        rect: CellRect,
        wall: bool,
        events: &mut InternalEvents,
        from: Stage,
    ) -> Result<(), GridError> {
        if floor_index(rect.floor).is_none() {
            return Err(GridError::InvalidFloor);
        }
        if !rect.is_non_empty() || !self.map().contains_rect(rect) {
            return Err(GridError::OutOfMap);
        }
        for y in rect.y0..rect.y1 {
            for x in rect.x0..rect.x1 {
                let pos = CellPos {
                    x,
                    y,
                    floor: rect.floor,
                };
                let mut cell = self.grid.cell(pos).unwrap_or(Cell::VOID);
                if wall {
                    cell.flags |= WALL;
                } else {
                    cell.flags &= !WALL;
                }
                self.grid.set(pos, cell)?;
            }
        }
        events.push(from, InternalEvent::GridChanged);
        Ok(())
    }

    /// Feeds the state of the module into `hasher`: map, grid, targets,
    /// networks.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(u32::from(self.map().width));
        hasher.u32(u32::from(self.map().height));
        self.grid.hash_into(hasher);
        for target in &self.targets {
            hasher.u32(target.profile as u32);
            hasher.u32(u32::try_from(target.cells.len()).unwrap_or(u32::MAX));
            for c in &target.cells {
                hasher.u32(u32::from(c.x));
                hasher.u32(u32::from(c.y));
                hasher.bytes(&c.floor.to_le_bytes());
            }
        }
        for network in &self.networks {
            network.hash_into(hasher);
        }
        self.types.hash_into(hasher);
        hasher.u32(self.next_handle);
        self.objects.hash_into(hasher);
        // Orders and delivery points (ADR-045) add nothing while there are
        // none, so the hash of a world without them stays as it was.
        if !self.orders.is_empty() || self.next_order != 0 || !self.delivery_points.is_empty() {
            hasher.u32(self.next_order);
            self.orders.hash_into(hasher);
            self.delivery_points.hash_into(hasher);
        }
    }

    /// State of the module for a save game, taken in one tick; cheap
    /// (blocks are shared).
    #[must_use]
    pub fn snapshot(&self) -> BuildSnapshot {
        BuildSnapshot {
            grid: self.grid.snapshot(),
            grid_version: self.grid.version(),
            targets: self.targets.clone(),
            networks: self.networks.to_vec(),
            types: self.types.ids().to_vec(),
            objects: self.objects.saved(),
            next_handle: self.next_handle,
            orders: self.orders.saved(),
            next_order: self.next_order,
            delivery_points: self.delivery_points.saved(),
        }
    }

    /// State of a loaded save game; the grid tables refer to `blocks`.
    ///
    /// # Errors
    ///
    /// `invalidState` naming the broken invariant: `blockIndex`, `table`,
    /// `targetCells`, `networks`, `edges`, `networkNodes`, `types`,
    /// `blockKind`, `objects`, `objectHandles`, `objectCells`, `orders`,
    /// `orderCells` or `deliveryPoints`; `limitExceeded` for too many
    /// targets, types, objects, orders or delivery points.
    pub(crate) fn restore(
        saved: SavedBuild,
        map: MapSize,
        blocks: &Blocks,
    ) -> Result<Self, Reason> {
        let grid = Grid::from_snapshot(blocks.snapshot(map, &saved.grid)?, saved.grid_version);
        let types = restore_types(saved.types)?;
        if blocks.highest_kind() > types.ids().len() {
            return Err(invalid("blockKind"));
        }
        let (objects, occupancy) = restore_objects(saved.objects, saved.next_handle, map, &types)?;
        if objects
            .as_slice()
            .iter()
            .filter(|o| o.blocks)
            .any(|o| !blocked_under(&grid, o))
        {
            return Err(invalid("objectCells"));
        }
        let (orders, order_occupancy) =
            restore_orders(saved.orders, saved.next_order, map, &types)?;
        let delivery_points = restore_delivery_points(saved.delivery_points, &objects)?;
        let targets = restore_targets(saved.targets)?;
        let on_map = |p: CellPos| map.contains(p) && floor_index(p.floor).is_some();
        if targets
            .iter()
            .any(|t| t.cells.is_empty() || !t.cells.iter().all(|&c| on_map(c)))
        {
            return Err(invalid("targetCells"));
        }
        if saved.networks.len() != NetworkKind::ALL.len() {
            return Err(invalid("networks"));
        }
        let mut networks = Vec::with_capacity(NetworkKind::ALL.len());
        for (kind, saved) in NetworkKind::ALL.into_iter().zip(saved.networks) {
            let edges = saved
                .edges
                .into_iter()
                .map(|(a, b)| (NodeId(a), NodeId(b)))
                .collect();
            networks.push(
                Network::from_parts(kind, saved.nodes, edges).ok_or_else(|| invalid("edges"))?,
            );
        }
        if networks
            .iter()
            .any(|n| !n.nodes().iter().all(|&c| on_map(c)))
        {
            return Err(invalid("networkNodes"));
        }
        let networks: [Network; 3] = networks.try_into().map_err(|_| invalid("networks"))?;
        Ok(Self {
            grid,
            targets,
            networks,
            objects,
            occupancy,
            types,
            next_handle: saved.next_handle,
            orders,
            order_occupancy,
            next_order: saved.next_order,
            delivery_points,
            orders_version: 0,
        })
    }
}

/// The orders of a save game with the occupancy derived from them: handles
/// ascending and below the counter, no two orders on the same cell.
fn restore_orders(
    saved: Vec<Order>,
    next_order: u32,
    map: MapSize,
    types: &TypeTable,
) -> Result<(Pool<Order>, Occupancy), Reason> {
    if !Pool::<Order>::fits(saved.len()) {
        return Err(limit("orders", MAX_ORDERS as f64));
    }
    let context = OrderContext {
        map,
        types: types.ids().len(),
    };
    let pool = Pool::restore(saved, &context).map_err(invalid)?;
    let orders = pool.as_slice();
    if !orders.windows(2).all(|w| w[0].handle < w[1].handle)
        || orders.last().is_some_and(|o| o.handle >= next_order)
    {
        return Err(invalid("orders"));
    }
    let occupancy = Build::order_occupancy_of(orders).ok_or_else(|| invalid("orderCells"))?;
    Ok((pool, occupancy))
}

/// The delivery points of a save game: each one an object of the world,
/// in the order of the handles.
fn restore_delivery_points(
    saved: Vec<DeliveryPoint>,
    objects: &Pool<PlacedObject>,
) -> Result<Pool<DeliveryPoint>, Reason> {
    if !Pool::<DeliveryPoint>::fits(saved.len()) {
        return Err(limit("deliveryPoints", MAX_DELIVERY_POINTS as f64));
    }
    let pool = Pool::restore(saved, &()).map_err(invalid)?;
    let points = pool.as_slice();
    let known = |h: u32| {
        objects
            .as_slice()
            .binary_search_by_key(&h, |o| o.handle)
            .is_ok()
    };
    if !points.windows(2).all(|w| w[0].handle < w[1].handle)
        || !points.iter().all(|d| known(d.handle))
    {
        return Err(invalid("deliveryPoints"));
    }
    Ok(pool)
}

/// The type table of a save game.
fn restore_types(saved: Vec<String>) -> Result<TypeTable, Reason> {
    if saved.len() > MAX_TYPES {
        return Err(limit("types", MAX_TYPES as f64));
    }
    TypeTable::from_ids(saved).ok_or_else(|| invalid("types"))
}

/// The objects of a save game with the occupancy derived from them.
fn restore_objects(
    saved: Vec<PlacedObject>,
    next_handle: u32,
    map: MapSize,
    types: &TypeTable,
) -> Result<(Pool<PlacedObject>, Occupancy), Reason> {
    if !Pool::<PlacedObject>::fits(saved.len()) {
        return Err(limit("objects", MAX_OBJECTS as f64));
    }
    let context = ObjectContext {
        map,
        types: types.ids().len(),
    };
    let pool = Pool::restore(saved, &context).map_err(invalid)?;
    let objects = pool.as_slice();
    if !objects.windows(2).all(|w| w[0].handle < w[1].handle)
        || objects.last().is_some_and(|o| o.handle >= next_handle)
    {
        return Err(invalid("objectHandles"));
    }
    let occupancy = Build::occupancy_of(objects).ok_or_else(|| invalid("objectCells"))?;
    Ok((pool, occupancy))
}

/// A blocking object has impassable cells under its whole footprint.
fn blocked_under(grid: &Grid, object: &PlacedObject) -> bool {
    let rect = object.rect();
    (rect.y0..rect.y1).all(|y| {
        (rect.x0..rect.x1).all(|x| {
            grid.cell(CellPos {
                x,
                y,
                floor: rect.floor,
            })
            .is_some_and(|c| !c.walkable())
        })
    })
}

/// State of the module taken for a save game.
#[derive(Clone)]
pub struct BuildSnapshot {
    grid: GridSnapshot,
    grid_version: u32,
    targets: Vec<Target>,
    networks: Vec<Network>,
    types: Vec<String>,
    objects: Vec<PlacedObject>,
    next_handle: u32,
    orders: Vec<Order>,
    next_order: u32,
    delivery_points: Vec<DeliveryPoint>,
}

impl BuildSnapshot {
    /// Size of the map.
    #[must_use]
    pub fn map(&self) -> MapSize {
        self.grid.map()
    }

    pub(crate) fn to_saved(&self, blocks: &mut BlockPool) -> SavedBuild {
        SavedBuild {
            grid: blocks.table(&self.grid),
            grid_version: self.grid_version,
            targets: saved_targets(&self.targets),
            networks: self
                .networks
                .iter()
                .map(|n| SavedNetwork {
                    nodes: n.nodes().to_vec(),
                    edges: n.edges().iter().map(|(a, b)| (a.0, b.0)).collect(),
                })
                .collect(),
            types: self.types.clone(),
            objects: self.objects.clone(),
            next_handle: self.next_handle,
            orders: self.orders.clone(),
            next_order: self.next_order,
            delivery_points: self.delivery_points.clone(),
        }
    }
}

/// Part of the module in a save game (S-5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct SavedBuild {
    /// Block table of the grid (see [`Blocks`]).
    pub grid: Vec<u32>,
    pub grid_version: u32,
    pub targets: Vec<SavedTarget>,
    pub networks: Vec<SavedNetwork>,
    /// Full ids of the object types that cells and objects refer to; a
    /// type number n is entry n − 1.
    pub types: Vec<String>,
    pub objects: Vec<PlacedObject>,
    pub next_handle: u32,
    /// Open construction orders (format 4, ADR-045).
    pub orders: Vec<Order>,
    pub next_order: u32,
    /// Delivery points (format 4, ADR-045).
    pub delivery_points: Vec<DeliveryPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SavedTarget {
    pub cells: Vec<CellPos>,
    pub load: LoadKind,
    pub profile: AccessProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SavedNetwork {
    pub nodes: Vec<CellPos>,
    pub edges: Vec<(u32, u32)>,
}

/// Targets as stored.
pub(crate) fn saved_targets(targets: &[Target]) -> Vec<SavedTarget> {
    targets
        .iter()
        .map(|t| SavedTarget {
            cells: t.cells.clone(),
            load: t.load,
            profile: t.profile,
        })
        .collect()
}

/// Targets of a save game.
///
/// # Errors
///
/// `limitExceeded` for more than [`MAX_TARGETS`] targets.
pub(crate) fn restore_targets(saved: Vec<SavedTarget>) -> Result<Vec<Target>, Reason> {
    if saved.len() > MAX_TARGETS {
        return Err(limit("targets", MAX_TARGETS as f64));
    }
    Ok(saved
        .into_iter()
        .map(|t| Target {
            cells: t.cells,
            load: t.load,
            profile: t.profile,
        })
        .collect())
}

/// One block as stored: the cell fields in row order, 1,024 values each.
/// `floor_kind` and `wall_kind` refer to the type table of the module
/// ([`SavedBuild::types`], 0 for none).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SavedBlock {
    pub flags: Vec<u8>,
    pub zone: Vec<u8>,
    pub cost: Vec<u8>,
    pub floor_kind: Vec<u16>,
    pub wall_kind: Vec<u16>,
}

/// Collects the distinct blocks of several grid snapshots (the grid, the
/// inputs of the flow fields), identified by their shared allocation, so
/// that a save game stores each block once. A table entry 0 is no block,
/// n is block n − 1.
#[derive(Default)]
pub(crate) struct BlockPool {
    blocks: Vec<SavedBlock>,
    index: HashMap<*const Block, u32>,
}

impl BlockPool {
    /// Block table of `snapshot`, adding its blocks that are new.
    pub fn table(&mut self, snapshot: &GridSnapshot) -> Vec<u32> {
        snapshot
            .entries()
            .iter()
            .map(|entry| {
                entry.as_ref().map_or(0, |block| {
                    let key = Arc::as_ptr(block);
                    *self.index.entry(key).or_insert_with(|| {
                        self.blocks.push(SavedBlock {
                            flags: block.flags().to_vec(),
                            zone: block.zones().to_vec(),
                            cost: block.costs().to_vec(),
                            floor_kind: block.floor_kinds().to_vec(),
                            wall_kind: block.wall_kinds().to_vec(),
                        });
                        u32::try_from(self.blocks.len()).unwrap_or(u32::MAX)
                    })
                })
            })
            .collect()
    }

    /// The collected blocks, in table order.
    pub fn into_blocks(self) -> Vec<SavedBlock> {
        self.blocks
    }
}

/// The blocks of a loaded save game, checked.
pub(crate) struct Blocks(Vec<Arc<Block>>);

impl Blocks {
    /// Checks the stored blocks.
    ///
    /// # Errors
    ///
    /// `invalidState` with `block` for a block of the wrong size or with
    /// values out of range.
    pub fn restore(saved: Vec<SavedBlock>) -> Result<Self, Reason> {
        saved
            .into_iter()
            .map(block)
            .collect::<Option<Vec<_>>>()
            .map(Self)
            .ok_or_else(|| invalid("block"))
    }

    /// Highest floor covering or wall type number in any block.
    fn highest_kind(&self) -> usize {
        self.0
            .iter()
            .flat_map(|b| b.floor_kinds().iter().chain(b.wall_kinds()))
            .copied()
            .max()
            .map_or(0, usize::from)
    }

    /// Grid snapshot of a block table.
    ///
    /// # Errors
    ///
    /// `invalidState` with `blockIndex` for an unknown block, `table` for a
    /// table that does not fit the map.
    pub fn snapshot(&self, map: MapSize, table: &[u32]) -> Result<GridSnapshot, Reason> {
        let entries = table
            .iter()
            .map(|&n| match n {
                0 => Some(None),
                n => self.0.get(n as usize - 1).map(|b| Some(Arc::clone(b))),
            })
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| invalid("blockIndex"))?;
        GridSnapshot::from_entries(map, entries).ok_or_else(|| invalid("table"))
    }
}

fn block(saved: SavedBlock) -> Option<Arc<Block>> {
    let flags: [u8; BLOCK_CELLS] = saved.flags.try_into().ok()?;
    let zone: [u8; BLOCK_CELLS] = saved.zone.try_into().ok()?;
    let cost: [u8; BLOCK_CELLS] = saved.cost.try_into().ok()?;
    let floor_kind: [u16; BLOCK_CELLS] = saved.floor_kind.try_into().ok()?;
    let wall_kind: [u16; BLOCK_CELLS] = saved.wall_kind.try_into().ok()?;
    Block::from_fields(flags, zone, cost, floor_kind, wall_kind).map(Arc::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::Stage;

    fn pos(x: u16, y: u16) -> CellPos {
        CellPos { x, y, floor: 0 }
    }

    fn hash(build: &Build) -> u64 {
        let mut h = StateHasher::new();
        build.hash_into(&mut h);
        h.finish()
    }

    #[test]
    fn targets_are_validated() {
        let mut build = Build::new(MapSize::new(64, 8).unwrap());
        assert_eq!(
            build.add_target(Target::passengers(Vec::new())),
            Err(TargetError::NoCells)
        );
        for i in 0..MAX_TARGETS {
            assert_eq!(
                build.add_target(Target::passengers(vec![pos(0, 0)])),
                Ok(u8::try_from(i).unwrap())
            );
        }
        assert_eq!(
            build.add_target(Target::passengers(vec![pos(0, 0)])),
            Err(TargetError::TooMany)
        );
        assert_eq!(build.targets().len(), MAX_TARGETS);
    }

    #[test]
    fn networks_belong_to_the_module_and_are_part_of_its_state() {
        let mut build = Build::new(MapSize::new(16, 16).unwrap());
        let before = hash(&build);
        let net = build.network_mut(NetworkKind::Taxiway);
        let a = net.add_node(pos(1, 1));
        let b = net.add_node(pos(5, 1));
        net.add_edge(a, b).unwrap();
        assert_ne!(hash(&build), before);
        assert_eq!(build.network(NetworkKind::Taxiway).edge_count(), 1);
        assert_eq!(build.network(NetworkKind::Road).edge_count(), 0);
    }

    #[test]
    fn walls_change_the_grid_and_lay_down_an_event() {
        let map = MapSize::new(32, 32).unwrap();
        let mut build = Build::new(map);
        let mut events = InternalEvents::default();
        let rect = CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: 32,
            y1: 32,
        };
        build
            .grid_mut()
            .fill(rect, Cell::floor(grid::Zone::Landside))
            .unwrap();
        build
            .set_walls(rect, true, &mut events, Stage::Commands)
            .unwrap();
        assert!(!build.grid().walkable(pos(5, 5)));
        assert_eq!(
            events.for_stage(Stage::Wayfinding).collect::<Vec<_>>(),
            vec![InternalEvent::GridChanged]
        );
        let bad = CellRect { floor: 4, ..rect };
        let mut none = InternalEvents::default();
        assert_eq!(
            build.set_walls(bad, true, &mut none, Stage::Commands),
            Err(GridError::InvalidFloor)
        );
        let outside = CellRect { x1: 40, ..rect };
        assert_eq!(
            build.set_walls(outside, false, &mut none, Stage::Commands),
            Err(GridError::OutOfMap)
        );
        assert_eq!(none, InternalEvents::default(), "no event without change");
    }
}
