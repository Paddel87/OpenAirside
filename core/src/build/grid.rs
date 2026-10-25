//! World grid of the module "Welt & Bau": five floors of 2 m × 2 m cells,
//! stored in blocks of 32 × 32 cells (ADR-020).
//!
//! The ground floor always has all its blocks; the other floors only have
//! blocks where something was built. A cell that is not stored is void and
//! cannot be walked on. Blocks are shared (`Arc`), so a snapshot for the
//! background computation of flow fields costs one pointer per block.

use std::sync::Arc;

use crate::geometry::{CellPos, CellRect, MAX_FLOOR, MIN_FLOOR, MapSize, is_valid_floor};
use crate::hash::StateHasher;

/// Edge length of a block in cells.
pub const BLOCK: usize = 32;
/// Cells per block.
pub const BLOCK_CELLS: usize = BLOCK * BLOCK;
/// Number of floors (−2 … +2).
pub const FLOORS: usize = (MAX_FLOOR - MIN_FLOOR + 1) as usize;
/// Index of the ground floor in the floor tables.
pub const GROUND_INDEX: usize = (-MIN_FLOOR) as usize;

/// Cell flag: wall, cannot be walked on.
pub const WALL: u8 = 1;
/// Cell flag: stairs or escalator; connects to the floor above and below
/// where that cell has this flag as well.
pub const STAIRS: u8 = 2;
/// Position of the [`AreaKind`] in the flag byte (3 bits, ADR-037).
pub const KIND_SHIFT: u8 = 2;
/// Bits of the flag byte that hold the [`AreaKind`].
pub const KIND_MASK: u8 = 0b111 << KIND_SHIFT;
/// Cell flag: staff zone (ZON-02); passengers and visitors may only leave
/// it, never enter (ADR-037).
pub const STAFF_ZONE: u8 = 1 << 5;
/// All flag bits in use; the others must be 0.
pub const FLAG_BITS: u8 = WALL | STAIRS | KIND_MASK | STAFF_ZONE;

/// Cost factor of a cell in tenths: 10 = normal floor.
pub const COST_NORMAL: u8 = 10;
/// Cost factor of the floor marking "preferred route".
pub const COST_MARKED: u8 = 7;
/// Cost factor of the floor marking "avoid".
pub const COST_AVOID: u8 = 20;
/// Highest cost factor; keeps the largest step cost below the bucket count
/// of the flow field computation (14 · 4.0 = 56 < 64).
pub const MAX_COST: u8 = 40;

/// Zone of a cell. Allowed steps: within a zone, landside → security →
/// airside and airside → exit → landside (one-way passages as at an airport).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Zone {
    /// Public area before security.
    Landside = 0,
    /// Area behind security.
    Airside = 1,
    /// Security lane.
    Security = 2,
    /// One-way passage from airside to landside.
    Exit = 3,
}

impl Zone {
    /// Returns `true` if a person may step from a cell in zone `from` into a
    /// cell in zone `to`.
    #[must_use]
    pub fn step_allowed(from: u8, to: u8) -> bool {
        const LANDSIDE: u8 = Zone::Landside as u8;
        const AIRSIDE: u8 = Zone::Airside as u8;
        const SECURITY: u8 = Zone::Security as u8;
        const EXIT: u8 = Zone::Exit as u8;
        from == to
            || matches!(
                (from, to),
                (LANDSIDE, SECURITY) | (SECURITY, AIRSIDE) | (AIRSIDE, EXIT) | (EXIT, LANDSIDE)
            )
    }
}

/// What a cell is used for, which decides who may walk on it (ADR-037). Part
/// of the flag byte; 0 (public walkway) is what older cells and floors
/// without a kind mean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AreaKind {
    /// Public walkway: halls, corridors, plazas.
    PublicWalk = 0,
    /// Walkway across the apron (walk-out, apron staff).
    ApronWalkway = 1,
    /// Crossing of a walkway with a service road or taxiway (zebra crossing).
    Crossing = 2,
    /// Aircraft stand area.
    StandArea = 3,
    /// Apron and taxiway.
    Apron = 4,
    /// Runway.
    Runway = 5,
    /// Road for vehicles.
    Road = 6,
    /// Terrain without use.
    Terrain = 7,
}

impl AreaKind {
    /// All kinds in the order of their numbers.
    pub const ALL: [Self; 8] = [
        Self::PublicWalk,
        Self::ApronWalkway,
        Self::Crossing,
        Self::StandArea,
        Self::Apron,
        Self::Runway,
        Self::Road,
        Self::Terrain,
    ];

    /// The kind stored in `flags`.
    #[must_use]
    pub fn of_flags(flags: u8) -> Self {
        Self::ALL[usize::from((flags & KIND_MASK) >> KIND_SHIFT)]
    }

    /// The kind with number `n` (0 … 7), `None` above.
    #[must_use]
    pub fn from_number(n: u8) -> Option<Self> {
        Self::ALL.get(usize::from(n)).copied()
    }
}

/// Content of one cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// [`WALL`], [`STAIRS`], the [`AreaKind`] in [`KIND_MASK`] and
    /// [`STAFF_ZONE`].
    pub flags: u8,
    /// A [`Zone`] as number.
    pub zone: u8,
    /// Cost factor in tenths, 1 … [`MAX_COST`].
    pub cost: u8,
}

impl Cell {
    /// Cell that is not stored: void, not walkable.
    pub const VOID: Self = Self {
        flags: WALL,
        zone: Zone::Landside as u8,
        cost: COST_NORMAL,
    };

    /// Walkable floor in the given zone with the normal cost.
    #[must_use]
    pub fn floor(zone: Zone) -> Self {
        Self {
            flags: 0,
            zone: zone as u8,
            cost: COST_NORMAL,
        }
    }

    /// The cell with the given kind.
    #[must_use]
    pub fn with_kind(mut self, kind: AreaKind) -> Self {
        self.flags = (self.flags & !KIND_MASK) | ((kind as u8) << KIND_SHIFT);
        self
    }

    /// The cell inside or outside a staff zone.
    #[must_use]
    pub fn with_staff_zone(mut self, staff_zone: bool) -> Self {
        if staff_zone {
            self.flags |= STAFF_ZONE;
        } else {
            self.flags &= !STAFF_ZONE;
        }
        self
    }

    /// What the cell is used for.
    #[must_use]
    pub fn kind(self) -> AreaKind {
        AreaKind::of_flags(self.flags)
    }

    /// `true` if the cell belongs to a staff zone.
    #[must_use]
    pub fn staff_zone(self) -> bool {
        self.flags & STAFF_ZONE != 0
    }

    /// Returns `true` if the cell can be walked on.
    #[must_use]
    pub fn walkable(self) -> bool {
        self.flags & WALL == 0
    }
}

/// Why a cell could not be changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridError {
    /// The position or rectangle lies outside the map.
    OutOfMap,
    /// The floor is not in −2 … +2.
    InvalidFloor,
    /// The zone number is unknown.
    InvalidZone,
    /// The cost factor is 0 or above [`MAX_COST`].
    InvalidCost,
    /// The flag byte has a bit that is not in [`FLAG_BITS`].
    InvalidFlags,
}

/// 32 × 32 cells of one floor, stored by field.
#[derive(Clone)]
pub struct Block {
    flags: [u8; BLOCK_CELLS],
    zone: [u8; BLOCK_CELLS],
    cost: [u8; BLOCK_CELLS],
    walkable: u16,
}

impl Block {
    fn void() -> Self {
        Self {
            flags: [Cell::VOID.flags; BLOCK_CELLS],
            zone: [Cell::VOID.zone; BLOCK_CELLS],
            cost: [Cell::VOID.cost; BLOCK_CELLS],
            walkable: 0,
        }
    }

    /// Flag bytes in row order (`y · 32 + x`).
    #[must_use]
    pub fn flags(&self) -> &[u8; BLOCK_CELLS] {
        &self.flags
    }

    /// Zone numbers in row order.
    #[must_use]
    pub fn zones(&self) -> &[u8; BLOCK_CELLS] {
        &self.zone
    }

    /// Cost factors in row order.
    #[must_use]
    pub fn costs(&self) -> &[u8; BLOCK_CELLS] {
        &self.cost
    }

    /// Number of walkable cells.
    #[must_use]
    pub fn walkable_cells(&self) -> usize {
        usize::from(self.walkable)
    }

    /// Block from the three cell fields of a save game; `None` if a cell
    /// carries an unused flag bit, an unknown zone or a cost out of range.
    pub(crate) fn from_fields(
        flags: [u8; BLOCK_CELLS],
        zone: [u8; BLOCK_CELLS],
        cost: [u8; BLOCK_CELLS],
    ) -> Option<Self> {
        let valid = (0..BLOCK_CELLS).all(|i| {
            flags[i] & !FLAG_BITS == 0
                && zone[i] <= Zone::Exit as u8
                && (1..=MAX_COST).contains(&cost[i])
        });
        if !valid {
            return None;
        }
        let walkable = flags.iter().filter(|f| *f & WALL == 0).count();
        Some(Self {
            flags,
            zone,
            cost,
            walkable: u16::try_from(walkable).ok()?,
        })
    }

    fn cell(&self, local: usize) -> Cell {
        Cell {
            flags: self.flags[local],
            zone: self.zone[local],
            cost: self.cost[local],
        }
    }

    fn set(&mut self, local: usize, cell: Cell) {
        let was = self.flags[local] & WALL == 0;
        let is = cell.walkable();
        self.flags[local] = cell.flags;
        self.zone[local] = cell.zone;
        self.cost[local] = cell.cost;
        match (was, is) {
            (false, true) => self.walkable += 1,
            (true, false) => self.walkable -= 1,
            _ => {}
        }
    }
}

/// Index of a floor in the floor tables, `None` outside −2 … +2.
#[must_use]
pub fn floor_index(floor: i8) -> Option<usize> {
    is_valid_floor(floor).then(|| usize::try_from(floor - MIN_FLOOR).unwrap_or(0))
}

/// Immutable view of the block table; cheap to take, safe to hand to
/// background threads.
#[derive(Clone)]
pub struct GridSnapshot {
    map: MapSize,
    blocks_x: usize,
    blocks_y: usize,
    table: Vec<Option<Arc<Block>>>,
}

impl GridSnapshot {
    /// Size of the map.
    #[must_use]
    pub fn map(&self) -> MapSize {
        self.map
    }

    /// Blocks per row and per column.
    #[must_use]
    pub fn block_counts(&self) -> (usize, usize) {
        (self.blocks_x, self.blocks_y)
    }

    /// Block at table index `floor_index · blocks_x · blocks_y + by · blocks_x
    /// + bx`, if stored.
    #[must_use]
    pub fn block(&self, table_index: usize) -> Option<&Block> {
        self.table.get(table_index).and_then(|b| b.as_deref())
    }

    /// Number of table entries (all floors).
    #[must_use]
    pub fn table_len(&self) -> usize {
        self.table.len()
    }

    /// The shared blocks of the table, for save games (S-5).
    pub(crate) fn entries(&self) -> &[Option<Arc<Block>>] {
        &self.table
    }

    /// Snapshot of `map` from a block table of a save game; `None` if the
    /// table length does not fit the map or a ground floor block is
    /// missing.
    pub(crate) fn from_entries(map: MapSize, table: Vec<Option<Arc<Block>>>) -> Option<Self> {
        let blocks_x = usize::from(map.width).div_ceil(BLOCK);
        let blocks_y = usize::from(map.height).div_ceil(BLOCK);
        let per_floor = blocks_x * blocks_y;
        if table.len() != per_floor * FLOORS {
            return None;
        }
        let ground = &table[GROUND_INDEX * per_floor..(GROUND_INDEX + 1) * per_floor];
        if ground.iter().any(Option::is_none) {
            return None;
        }
        Some(Self {
            map,
            blocks_x,
            blocks_y,
            table,
        })
    }
}

/// The world grid with change counter.
pub struct Grid {
    snapshot: GridSnapshot,
    version: u32,
    stored: usize,
}

impl Grid {
    /// Empty grid: all blocks of the ground floor exist, everything is void.
    #[must_use]
    pub fn new(map: MapSize) -> Self {
        let blocks_x = usize::from(map.width).div_ceil(BLOCK);
        let blocks_y = usize::from(map.height).div_ceil(BLOCK);
        let per_floor = blocks_x * blocks_y;
        let mut table = vec![None; per_floor * FLOORS];
        let empty = Arc::new(Block::void());
        for slot in &mut table[GROUND_INDEX * per_floor..(GROUND_INDEX + 1) * per_floor] {
            *slot = Some(Arc::clone(&empty));
        }
        Self {
            snapshot: GridSnapshot {
                map,
                blocks_x,
                blocks_y,
                table,
            },
            version: 0,
            stored: per_floor,
        }
    }

    /// Grid of a save game with its change counter.
    pub(crate) fn from_snapshot(snapshot: GridSnapshot, version: u32) -> Self {
        let stored = snapshot.table.iter().filter(|b| b.is_some()).count();
        Self {
            snapshot,
            version,
            stored,
        }
    }

    /// Size of the map.
    #[must_use]
    pub fn map(&self) -> MapSize {
        self.snapshot.map
    }

    /// Counter that rises with every change.
    #[must_use]
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Number of stored blocks (all floors).
    #[must_use]
    pub fn stored_blocks(&self) -> usize {
        self.stored
    }

    fn locate(&self, pos: CellPos) -> Result<(usize, usize), GridError> {
        let floor = floor_index(pos.floor).ok_or(GridError::InvalidFloor)?;
        if !self.snapshot.map.contains(pos) {
            return Err(GridError::OutOfMap);
        }
        let (x, y) = (usize::from(pos.x), usize::from(pos.y));
        let table_index =
            (floor * self.snapshot.blocks_y + y / BLOCK) * self.snapshot.blocks_x + x / BLOCK;
        Ok((table_index, (y % BLOCK) * BLOCK + x % BLOCK))
    }

    /// Content of a cell; `None` outside the map or on an invalid floor.
    /// Cells without a stored block are void.
    #[must_use]
    pub fn cell(&self, pos: CellPos) -> Option<Cell> {
        let (table_index, local) = self.locate(pos).ok()?;
        Some(
            self.snapshot
                .block(table_index)
                .map_or(Cell::VOID, |b| b.cell(local)),
        )
    }

    /// Returns `true` if the cell exists and can be walked on.
    #[must_use]
    pub fn walkable(&self, pos: CellPos) -> bool {
        self.cell(pos).is_some_and(Cell::walkable)
    }

    /// Sets a cell. Blocks outside the ground floor are created on demand.
    ///
    /// # Errors
    ///
    /// Fails if the position is not on the map, the floor or the zone is
    /// invalid, the flags have an unused bit or the cost factor is out of
    /// range; the grid stays unchanged.
    pub fn set(&mut self, pos: CellPos, cell: Cell) -> Result<(), GridError> {
        if cell.flags & !FLAG_BITS != 0 {
            return Err(GridError::InvalidFlags);
        }
        if cell.zone > Zone::Exit as u8 {
            return Err(GridError::InvalidZone);
        }
        if cell.cost == 0 || cell.cost > MAX_COST {
            return Err(GridError::InvalidCost);
        }
        let (table_index, local) = self.locate(pos)?;
        let entry = &mut self.snapshot.table[table_index];
        if entry.is_none() {
            if cell == Cell::VOID {
                return Ok(());
            }
            *entry = Some(Arc::new(Block::void()));
            self.stored += 1;
        }
        if let Some(block) = entry {
            if block.cell(local) == cell {
                return Ok(());
            }
            Arc::make_mut(block).set(local, cell);
        }
        self.version = self.version.wrapping_add(1);
        Ok(())
    }

    /// Sets all cells of a rectangle to `cell`.
    ///
    /// # Errors
    ///
    /// Fails, without changing anything, if the rectangle is empty, not on
    /// the map or on an invalid floor, or if `cell` is invalid.
    pub fn fill(&mut self, rect: CellRect, cell: Cell) -> Result<(), GridError> {
        if floor_index(rect.floor).is_none() {
            return Err(GridError::InvalidFloor);
        }
        if !rect.is_non_empty() || !self.snapshot.map.contains_rect(rect) {
            return Err(GridError::OutOfMap);
        }
        for y in rect.y0..rect.y1 {
            for x in rect.x0..rect.x1 {
                self.set(
                    CellPos {
                        x,
                        y,
                        floor: rect.floor,
                    },
                    cell,
                )?;
            }
        }
        Ok(())
    }

    /// Cheap copy of the block table for background computations.
    #[must_use]
    pub fn snapshot(&self) -> GridSnapshot {
        self.snapshot.clone()
    }

    /// Feeds the complete grid content into `hasher`.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(u32::try_from(self.stored).unwrap_or(u32::MAX));
        for (index, entry) in self.snapshot.table.iter().enumerate() {
            if let Some(block) = entry {
                hasher.u32(u32::try_from(index).unwrap_or(u32::MAX));
                hasher.bytes(&block.flags);
                hasher.bytes(&block.zone);
                hasher.bytes(&block.cost);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Table index of the first block of the ground floor (4 × 3 blocks per floor).
    const GROUND: usize = GROUND_INDEX * 12;

    fn pos(x: u16, y: u16, floor: i8) -> CellPos {
        CellPos { x, y, floor }
    }

    fn grid() -> Grid {
        Grid::new(MapSize::new(100, 70).unwrap())
    }

    #[test]
    fn only_the_ground_floor_is_stored_at_first() {
        let g = grid();
        // 100 × 70 cells → 4 × 3 blocks on the ground floor.
        assert_eq!(g.stored_blocks(), 12);
        assert_eq!(g.cell(pos(5, 5, 0)), Some(Cell::VOID));
        assert!(!g.walkable(pos(5, 5, 0)));
        assert_eq!(g.cell(pos(5, 5, 1)), Some(Cell::VOID));
        assert_eq!(g.cell(pos(100, 0, 0)), None);
        assert_eq!(g.cell(pos(0, 0, 3)), None);
    }

    #[test]
    fn setting_a_cell_creates_its_block_on_demand() {
        let mut g = grid();
        let before = g.version();
        g.set(pos(40, 40, 1), Cell::floor(Zone::Airside)).unwrap();
        assert_eq!(g.stored_blocks(), 13);
        assert!(g.walkable(pos(40, 40, 1)));
        assert!(!g.walkable(pos(41, 40, 1)));
        assert_eq!(g.cell(pos(40, 40, 1)).unwrap().zone, Zone::Airside as u8);
        assert_eq!(g.version(), before + 1);
        // Same content again changes nothing.
        g.set(pos(40, 40, 1), Cell::floor(Zone::Airside)).unwrap();
        assert_eq!(g.version(), before + 1);
        // Setting void where no block exists creates no block.
        g.set(pos(90, 60, -2), Cell::VOID).unwrap();
        assert_eq!(g.stored_blocks(), 13);
    }

    #[test]
    fn invalid_input_is_refused_and_changes_nothing() {
        let mut g = grid();
        let ok = Cell::floor(Zone::Landside);
        assert_eq!(g.set(pos(100, 0, 0), ok), Err(GridError::OutOfMap));
        assert_eq!(g.set(pos(0, 0, 3), ok), Err(GridError::InvalidFloor));
        assert_eq!(
            g.set(pos(0, 0, 0), Cell { zone: 4, ..ok }),
            Err(GridError::InvalidZone)
        );
        assert_eq!(
            g.set(pos(0, 0, 0), Cell { cost: 0, ..ok }),
            Err(GridError::InvalidCost)
        );
        assert_eq!(
            g.set(
                pos(0, 0, 0),
                Cell {
                    cost: MAX_COST + 1,
                    ..ok
                }
            ),
            Err(GridError::InvalidCost)
        );
        assert_eq!(g.version(), 0);
        assert_eq!(g.stored_blocks(), 12);
    }

    #[test]
    fn area_kind_and_staff_zone_live_in_the_flag_byte() {
        let cell = Cell::floor(Zone::Airside)
            .with_kind(AreaKind::StandArea)
            .with_staff_zone(true);
        assert_eq!(cell.kind(), AreaKind::StandArea);
        assert!(cell.staff_zone() && cell.walkable());
        assert_eq!(cell.flags, (3 << KIND_SHIFT) | STAFF_ZONE);
        let plain = cell.with_kind(AreaKind::PublicWalk).with_staff_zone(false);
        assert_eq!(plain, Cell::floor(Zone::Airside), "0 means public walkway");
        for (n, kind) in AreaKind::ALL.into_iter().enumerate() {
            assert_eq!(kind as usize, n);
            assert_eq!(AreaKind::from_number(n as u8), Some(kind));
            assert_eq!(Cell::floor(Zone::Landside).with_kind(kind).kind(), kind);
        }
        assert_eq!(AreaKind::from_number(8), None);
        // Walls keep their kind.
        let wall = Cell::VOID.with_kind(AreaKind::Runway);
        assert!(!wall.walkable() && wall.kind() == AreaKind::Runway);
    }

    #[test]
    fn cells_with_unused_flag_bits_are_rejected() {
        let mut g = grid();
        let bad = Cell {
            flags: 1 << 6,
            ..Cell::floor(Zone::Landside)
        };
        assert_eq!(g.set(pos(0, 0, 0), bad), Err(GridError::InvalidFlags));
        let rect = CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: 4,
            y1: 4,
        };
        assert_eq!(g.fill(rect, bad), Err(GridError::InvalidFlags));
        assert_eq!(g.version(), 0);
        // The kind bits and the staff zone are valid in a block of a save game.
        let kinds = [Cell::floor(Zone::Landside)
            .with_kind(AreaKind::Terrain)
            .with_staff_zone(true)
            .flags; BLOCK_CELLS];
        let zones = [0; BLOCK_CELLS];
        let costs = [COST_NORMAL; BLOCK_CELLS];
        assert!(Block::from_fields(kinds, zones, costs).is_some());
        let unused = [1 << 6; BLOCK_CELLS];
        assert!(Block::from_fields(unused, zones, costs).is_none());
    }

    #[test]
    fn walkable_count_follows_changes_in_both_directions() {
        let mut g = grid();
        let p = pos(3, 3, 0);
        g.set(p, Cell::floor(Zone::Landside)).unwrap();
        g.set(
            p,
            Cell {
                flags: STAIRS,
                ..Cell::floor(Zone::Landside)
            },
        )
        .unwrap();
        let snap = g.snapshot();
        let block = snap.block(GROUND).unwrap();
        assert_eq!(block.walkable_cells(), 1);
        assert_eq!(block.flags()[3 * BLOCK + 3], STAIRS);
        assert_eq!(block.zones()[3 * BLOCK + 3], Zone::Landside as u8);
        assert_eq!(block.costs()[3 * BLOCK + 3], COST_NORMAL);
        g.set(p, Cell::VOID).unwrap();
        assert_eq!(g.snapshot().block(GROUND).unwrap().walkable_cells(), 0);
    }

    #[test]
    fn fill_covers_a_rectangle_and_validates_it() {
        let mut g = grid();
        let rect = CellRect {
            floor: 0,
            x0: 30,
            y0: 30,
            x1: 70,
            y1: 34,
        };
        g.fill(rect, Cell::floor(Zone::Landside)).unwrap();
        assert!(g.walkable(pos(30, 30, 0)) && g.walkable(pos(69, 33, 0)));
        assert!(!g.walkable(pos(70, 33, 0)) && !g.walkable(pos(30, 34, 0)));
        let empty = CellRect { x1: 30, ..rect };
        assert_eq!(g.fill(empty, Cell::VOID), Err(GridError::OutOfMap));
        let outside = CellRect { x1: 101, ..rect };
        assert_eq!(g.fill(outside, Cell::VOID), Err(GridError::OutOfMap));
        let floor = CellRect { floor: 9, ..rect };
        assert_eq!(g.fill(floor, Cell::VOID), Err(GridError::InvalidFloor));
    }

    #[test]
    fn a_snapshot_is_not_affected_by_later_changes() {
        let mut g = grid();
        g.set(pos(1, 1, 0), Cell::floor(Zone::Landside)).unwrap();
        let snap = g.snapshot();
        g.set(pos(1, 1, 0), Cell::VOID).unwrap();
        g.set(pos(2, 2, 2), Cell::floor(Zone::Landside)).unwrap();
        assert_eq!(snap.block(GROUND).unwrap().walkable_cells(), 1);
        assert_eq!(snap.map(), g.map());
        assert_eq!(snap.block_counts(), (4, 3));
        assert_eq!(snap.table_len(), 12 * FLOORS);
        assert!(snap.block(4 * 12 + 1).is_none());
        assert!(snap.block(usize::MAX).is_none());
    }

    #[test]
    fn hash_reflects_content() {
        let hash = |g: &Grid| {
            let mut h = StateHasher::new();
            g.hash_into(&mut h);
            h.finish()
        };
        let mut a = grid();
        let b = grid();
        assert_eq!(hash(&a), hash(&b));
        a.set(pos(1, 1, 0), Cell::floor(Zone::Airside)).unwrap();
        assert_ne!(hash(&a), hash(&b));
    }

    #[test]
    fn zone_steps_follow_the_one_way_rules() {
        let l = Zone::Landside as u8;
        let a = Zone::Airside as u8;
        let s = Zone::Security as u8;
        let e = Zone::Exit as u8;
        assert!(Zone::step_allowed(l, l));
        assert!(Zone::step_allowed(l, s) && Zone::step_allowed(s, a));
        assert!(Zone::step_allowed(a, e) && Zone::step_allowed(e, l));
        assert!(!Zone::step_allowed(a, s) && !Zone::step_allowed(s, l));
        assert!(!Zone::step_allowed(l, a) && !Zone::step_allowed(l, e));
    }

    #[test]
    fn floor_index_maps_minus_two_to_zero() {
        assert_eq!(floor_index(-2), Some(0));
        assert_eq!(floor_index(0), Some(GROUND_INDEX));
        assert_eq!(floor_index(2), Some(4));
        assert_eq!(floor_index(-3), None);
    }
}
