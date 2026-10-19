//! World state of the simulation core: the grid, the targets, the path
//! networks, the flow fields in use and the persons that follow them.
//!
//! Persons keep their hot fields in one small record each and move in two
//! phases (ADR-010): phase 1 computes every person from the state of the
//! previous tick, phase 2 computes commutative counts. Flow fields are taken
//! over as a whole by [`World::adopt_fields`] (see `adoption`).

use rayon::ThreadPool;
use rayon::prelude::*;

use crate::flowfield::{ARRIVED, DIRS, DOWN, FieldInput, FieldSet, UNREACHABLE, UP};
use crate::geometry::{CellPos, CellRect, MapSize};
use crate::grid::{Cell, FLOORS, Grid, GridError, WALL, Zone, floor_index};
use crate::hash::StateHasher;
use crate::network::{Network, NetworkKind};
use crate::savegame::state::SavedPerson;
use crate::target::{MAX_TARGETS, Target};

/// Walking speed in cells per tick: 0.3 cells per game second, the guide
/// value of ADR-020 (a 300 m walk takes about 8 game minutes).
pub const WALK_SPEED: f32 = 0.1;
const DIAGONAL: f32 = std::f32::consts::FRAC_1_SQRT_2;

/// Why a target could not be added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetError {
    /// The world has [`MAX_TARGETS`] targets already.
    TooMany,
    /// The target has no cell.
    NoCells,
}

/// Hot fields of one person, needed in every tick.
// TODO(fahrplan-ref: 3.2): needs, patience, tickets and other cold fields of
// passengers and staff; the target choice moves to the module "Personen".
#[derive(Debug, Clone, Copy, PartialEq)]
struct Person {
    x: u16,
    y: u16,
    floor: i8,
    /// Step towards the target: direction 0 … 9, `ARRIVED` or `UNREACHABLE`.
    heading: u8,
    target: u8,
    /// Share of the current step already walked, 0 … 1.
    progress: f32,
    /// State of the generator that picks the next target.
    rng: u32,
}

impl Person {
    fn pos(&self) -> CellPos {
        CellPos {
            x: self.x,
            y: self.y,
            floor: self.floor,
        }
    }

    fn view(&self) -> PersonView {
        let (dx, dy) = DIRS
            .get(usize::from(self.heading))
            .copied()
            .unwrap_or((0, 0));
        #[allow(clippy::cast_precision_loss)] // dx, dy are −1, 0 or 1
        let (dx, dy) = (dx as f32, dy as f32);
        PersonView {
            x: f32::from(self.x) + 0.5 + dx * self.progress,
            y: f32::from(self.y) + 0.5 + dy * self.progress,
            floor: self.floor,
            heading: self.heading,
        }
    }
}

/// Visible state of one person for the state buffer S-1: position in cells
/// with the share of the current step already walked.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PersonView {
    /// Position east of the map origin in cells (cell centre = x + 0.5).
    pub x: f32,
    /// Position south of the map origin in cells.
    pub y: f32,
    /// Floor −2 … +2.
    pub floor: i8,
    /// Direction 0 … 9, `ARRIVED` or `UNREACHABLE` (see `flowfield`).
    pub heading: u8,
}

/// Most persons a world holds; the debug command `spawnPersons` stops
/// there, and a save game with more is rejected (S-5).
pub const MAX_PERSONS: usize = 200_000;

/// Population, grid and derived data of one session.
pub struct World {
    grid: Grid,
    targets: Vec<Target>,
    networks: [Network; 3],
    fields: FieldSet,
    persons: Vec<Person>,
    next: Vec<Person>,
    per_floor: [u32; FLOORS],
}

impl World {
    /// Empty world: nothing is walkable, there are no targets and no
    /// persons.
    #[must_use]
    pub fn new(map: MapSize) -> Self {
        let grid = Grid::new(map);
        let fields = FieldSet::empty(&grid.snapshot(), 0);
        Self {
            grid,
            targets: Vec::new(),
            networks: NetworkKind::ALL.map(Network::new),
            fields,
            persons: Vec::new(),
            next: Vec::new(),
            per_floor: [0; FLOORS],
        }
    }

    /// World with a walkable landside ground floor, one target in the middle
    /// and `persons` persons spread over it. Used where a world is needed
    /// without a building.
    #[must_use]
    pub fn open_hall(map: MapSize, persons: u32, seed: u64) -> Self {
        let mut world = Self::new(map);
        let rect = CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: map.width,
            y1: map.height,
        };
        world
            .grid
            .fill(rect, Cell::floor(Zone::Landside))
            .expect("the whole map is a valid rectangle");
        let middle = CellPos {
            x: map.width / 2,
            y: map.height / 2,
            floor: 0,
        };
        world
            .add_target(Target::passengers(vec![middle]))
            .expect("an empty world has room for a target");
        let mut rng = SplitMix64(seed);
        let cells: Vec<CellPos> = (0..persons)
            .map(|_| CellPos {
                x: u16::try_from(rng.next() % u64::from(map.width)).unwrap_or(0),
                y: u16::try_from(rng.next() % u64::from(map.height)).unwrap_or(0),
                floor: 0,
            })
            .collect();
        world.spawn_persons(&cells, seed);
        world.fields = FieldSet::compute(&world.field_input(), 0);
        world.resample_headings();
        world
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
    /// (`Simulation::request_recompute`), not automatically.
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

    /// Input for a background computation of the flow fields.
    #[must_use]
    pub fn field_input(&self) -> FieldInput {
        FieldInput {
            grid: self.grid.snapshot(),
            targets: self.targets.clone(),
        }
    }

    /// The flow fields in use.
    #[must_use]
    pub fn fields(&self) -> &FieldSet {
        &self.fields
    }

    /// Revision of the flow fields in use.
    #[must_use]
    pub fn fields_revision(&self) -> u32 {
        self.fields.revision()
    }

    /// Takes over a new set of flow fields as a whole and returns the old
    /// one, so that the caller can drop its large buffers off the tick.
    pub fn adopt_fields(&mut self, fields: FieldSet) -> FieldSet {
        std::mem::replace(&mut self.fields, fields)
    }

    /// Computes the flow fields now, on `pool`, with revision 0. Used when a
    /// session starts, before the first tick.
    pub fn rebuild_fields_now(&mut self, pool: &ThreadPool) {
        let input = self.field_input();
        self.fields = pool.install(|| FieldSet::compute(&input, 0));
        self.resample_headings();
    }

    fn resample_headings(&mut self) {
        let fields = &self.fields;
        for p in &mut self.persons {
            p.heading = fields.dir(p.target, p.pos());
        }
    }

    /// Adds one person on each of `cells` (cells that are not walkable are
    /// skipped), with a target and a generator state derived from `seed`.
    pub fn spawn_persons(&mut self, cells: &[CellPos], seed: u64) {
        let mut rng = SplitMix64(seed ^ 0x5eed);
        let targets = self.targets.len().max(1) as u64;
        for &pos in cells {
            if !self.grid.walkable(pos) {
                continue;
            }
            let state = u32::try_from(rng.next() >> 32).unwrap_or(1) | 1;
            self.persons.push(Person {
                x: pos.x,
                y: pos.y,
                floor: pos.floor,
                heading: UNREACHABLE,
                target: u8::try_from(rng.next() % targets).unwrap_or(0),
                progress: 0.0,
                rng: state,
            });
        }
        self.next.clone_from(&self.persons);
        self.count_per_floor_serial();
    }

    /// Number of persons.
    #[must_use]
    pub fn persons(&self) -> usize {
        self.persons.len()
    }

    /// Calls `visit` with the visible state of every person, in storage
    /// order.
    pub fn visit_persons(&self, mut visit: impl FnMut(PersonView)) {
        for p in &self.persons {
            visit(p.view());
        }
    }

    /// Adds up to `count` persons on random walkable cells of the ground
    /// floor (debug command, META-15). Gives up on a person after 64
    /// cells that are not walkable and never exceeds [`MAX_PERSONS`].
    /// Returns the number of persons added.
    pub fn spawn_random(&mut self, count: u32, seed: u64) -> u32 {
        let room = MAX_PERSONS.saturating_sub(self.persons.len());
        let wanted = usize::try_from(count).unwrap_or(usize::MAX).min(room);
        let map = self.map();
        let mut rng = SplitMix64(seed);
        let mut cells = Vec::with_capacity(wanted);
        for _ in 0..wanted {
            for _ in 0..64 {
                let pos = CellPos {
                    x: u16::try_from(rng.next() % u64::from(map.width)).unwrap_or(0),
                    y: u16::try_from(rng.next() % u64::from(map.height)).unwrap_or(0),
                    floor: 0,
                };
                if self.grid.walkable(pos) {
                    cells.push(pos);
                    break;
                }
            }
        }
        let before = self.persons.len();
        self.spawn_persons(&cells, seed);
        u32::try_from(self.persons.len() - before).unwrap_or(u32::MAX)
    }

    /// Removes all persons (debug command).
    pub fn clear_persons(&mut self) {
        self.persons.clear();
        self.next.clear();
        self.per_floor = [0; FLOORS];
    }

    /// Sets (`wall`) or clears the wall flag of every cell in `rect`, keeping
    /// zone and cost (debug command for building without data packs). The
    /// flow fields follow with the next recomputation.
    ///
    /// # Errors
    ///
    /// Fails, without changing anything, if the rectangle is empty, not on
    /// the map or on an invalid floor.
    pub fn set_walls(&mut self, rect: CellRect, wall: bool) -> Result<(), GridError> {
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
        Ok(())
    }

    /// Persons per floor, index 0 = floor −2.
    #[must_use]
    pub fn per_floor(&self) -> [u32; FLOORS] {
        self.per_floor
    }

    /// Position of person `index`, if it exists.
    #[must_use]
    pub fn person_pos(&self, index: usize) -> Option<CellPos> {
        self.persons.get(index).map(Person::pos)
    }

    /// `true` while persons are present, which blocks the night skip
    /// (META-08: no passenger in the terminal).
    #[must_use]
    pub fn has_active_persons(&self) -> bool {
        !self.persons.is_empty()
    }

    /// One simulation step: phase 1 moves every person from the state of
    /// the previous tick, phase 2 computes commutative counts.
    pub fn advance(&mut self, pool: &ThreadPool) {
        let fields = &self.fields;
        let targets = self.targets.len();
        let (persons, next) = (&self.persons, &mut self.next);
        pool.install(|| {
            next.par_iter_mut()
                .zip(persons.par_iter())
                .for_each(|(out, cur)| *out = step_person(*cur, fields, targets));
        });
        std::mem::swap(&mut self.persons, &mut self.next);
        self.per_floor = pool.install(|| {
            self.persons
                .par_iter()
                .fold(
                    || [0u32; FLOORS],
                    |mut acc, p| {
                        if let Some(index) = floor_index(p.floor) {
                            acc[index] += 1;
                        }
                        acc
                    },
                )
                .reduce(|| [0u32; FLOORS], add_counts)
        });
    }

    fn count_per_floor_serial(&mut self) {
        let mut counts = [0u32; FLOORS];
        for p in &self.persons {
            if let Some(index) = floor_index(p.floor) {
                counts[index] += 1;
            }
        }
        self.per_floor = counts;
    }

    /// Hot fields of all persons for a save game (S-5).
    pub(crate) fn saved_persons(&self) -> Vec<SavedPerson> {
        self.persons
            .iter()
            .map(|p| SavedPerson {
                x: p.x,
                y: p.y,
                floor: p.floor,
                heading: p.heading,
                target: p.target,
                progress: p.progress,
                rng: p.rng,
            })
            .collect()
    }

    /// World of a loaded save game. `fields` were computed from the saved
    /// input of the flow fields in use, so the persons keep their headings.
    ///
    /// # Errors
    ///
    /// Names the first broken invariant: too many targets or persons, a
    /// target without cells, a cell, node or person off the map or on an
    /// invalid floor, an unknown target or heading, a step share outside
    /// 0 … 1, or flow fields of another map.
    pub(crate) fn from_saved(
        grid: Grid,
        targets: Vec<Target>,
        networks: [Network; 3],
        fields: FieldSet,
        persons: &[SavedPerson],
    ) -> Result<Self, &'static str> {
        let map = grid.map();
        let on_map = |p: CellPos| map.contains(p) && floor_index(p.floor).is_some();
        if targets.len() > MAX_TARGETS {
            return Err("targets");
        }
        if targets
            .iter()
            .any(|t| t.cells.is_empty() || !t.cells.iter().all(|&c| on_map(c)))
        {
            return Err("targetCells");
        }
        if networks
            .iter()
            .any(|n| !n.nodes().iter().all(|&c| on_map(c)))
        {
            return Err("networkNodes");
        }
        if fields.source().grid.map() != map {
            return Err("fieldsMap");
        }
        if persons.len() > MAX_PERSONS {
            return Err("persons");
        }
        let target_limit = targets.len().max(1);
        let heading_ok = |h: u8| h <= DOWN || h == ARRIVED || h == UNREACHABLE;
        let mut out = Vec::with_capacity(persons.len());
        for p in persons {
            let pos = CellPos {
                x: p.x,
                y: p.y,
                floor: p.floor,
            };
            if !on_map(pos)
                || !heading_ok(p.heading)
                || usize::from(p.target) >= target_limit
                || !(0.0..=1.0).contains(&p.progress)
            {
                return Err("person");
            }
            out.push(Person {
                x: p.x,
                y: p.y,
                floor: p.floor,
                heading: p.heading,
                target: p.target,
                progress: p.progress,
                rng: p.rng,
            });
        }
        let mut world = Self {
            grid,
            targets,
            networks,
            fields,
            next: out.clone(),
            persons: out,
            per_floor: [0; FLOORS],
        };
        world.count_per_floor_serial();
        Ok(world)
    }

    /// Feeds the complete world state into `hasher`.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(u32::from(self.map().width));
        hasher.u32(u32::from(self.map().height));
        self.grid.hash_into(hasher);
        for target in &self.targets {
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
        self.fields.hash_into(hasher);
        for p in &self.persons {
            hasher.u32(u32::from(p.x));
            hasher.u32(u32::from(p.y));
            hasher.bytes(&[p.floor.to_le_bytes()[0], p.heading, p.target]);
            hasher.f32(p.progress);
            hasher.u32(p.rng);
        }
        for count in self.per_floor {
            hasher.u32(count);
        }
    }
}

fn add_counts(mut a: [u32; FLOORS], b: [u32; FLOORS]) -> [u32; FLOORS] {
    for (x, y) in a.iter_mut().zip(b) {
        *x += y;
    }
    a
}

/// Next state of one person.
fn step_person(mut p: Person, fields: &FieldSet, targets: usize) -> Person {
    if p.heading > DOWN {
        // Standing: arrived at the target or without a way to it.
        if p.heading == ARRIVED && targets > 0 {
            p.rng ^= p.rng << 13;
            p.rng ^= p.rng >> 17;
            p.rng ^= p.rng << 5;
            p.target = u8::try_from(p.rng as usize % targets).unwrap_or(0);
        }
        p.heading = fields.dir(p.target, p.pos());
        p.progress = 0.0;
        return p;
    }
    let diagonal = matches!(p.heading, 1 | 3 | 5 | 7);
    p.progress += if diagonal {
        WALK_SPEED * DIAGONAL
    } else {
        WALK_SPEED
    };
    if p.progress >= 1.0 {
        if let Some(next) = fields.step_target(p.pos(), p.heading) {
            p.x = next.x;
            p.y = next.y;
            p.floor = next.floor;
        }
        p.progress = 0.0;
        p.heading = fields.dir(p.target, p.pos());
    }
    p
}

/// Small deterministic generator for reproducible start states.
pub(crate) struct SplitMix64(pub u64);

impl SplitMix64 {
    pub(crate) fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

// The step rule depends on the numbering of the directions.
const _: () = assert!(DIRS.len() == 8 && UP == 8 && DOWN == 9);

#[cfg(test)]
mod tests {
    use super::*;

    fn pool(threads: usize) -> ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
    }

    fn hash(world: &World) -> u64 {
        let mut h = StateHasher::new();
        world.hash_into(&mut h);
        h.finish()
    }

    fn pos(x: u16, y: u16) -> CellPos {
        CellPos { x, y, floor: 0 }
    }

    /// A 40-cell corridor with a target at each end.
    fn corridor() -> World {
        let mut world = World::new(MapSize::new(64, 8).unwrap());
        world
            .grid_mut()
            .fill(
                CellRect {
                    floor: 0,
                    x0: 0,
                    y0: 3,
                    x1: 40,
                    y1: 4,
                },
                Cell::floor(Zone::Landside),
            )
            .unwrap();
        world
            .add_target(Target::passengers(vec![pos(0, 3)]))
            .unwrap();
        world
            .add_target(Target::passengers(vec![pos(39, 3)]))
            .unwrap();
        world
    }

    #[test]
    fn persons_walk_along_the_flow_field_and_choose_new_targets() {
        let pool = pool(2);
        let mut world = corridor();
        world.spawn_persons(&[pos(20, 3)], 1);
        world.rebuild_fields_now(&pool);
        assert_eq!(world.persons(), 1);
        let mut visited = std::collections::BTreeSet::new();
        for _ in 0..4_000 {
            world.advance(&pool);
            let p = world.person_pos(0).unwrap();
            assert_eq!(p.y, 3, "stays in the corridor");
            assert!(p.x < 40);
            visited.insert(p.x);
        }
        // 4,000 ticks at 0.1 cells per tick are 400 cells: the person reaches
        // both ends and walks back and forth.
        assert!(visited.contains(&0) && visited.contains(&39));
        assert_eq!(world.per_floor()[2], 1);
    }

    #[test]
    fn a_person_without_a_way_stays_and_is_counted_as_present() {
        let pool = pool(1);
        let mut world = corridor();
        world
            .grid_mut()
            .set(pos(50, 5), Cell::floor(Zone::Landside))
            .unwrap();
        world.spawn_persons(&[pos(50, 5), pos(10, 10)], 3);
        assert_eq!(world.persons(), 1, "the cell (10,10) is void and skipped");
        world.rebuild_fields_now(&pool);
        for _ in 0..50 {
            world.advance(&pool);
        }
        assert_eq!(world.person_pos(0), Some(pos(50, 5)));
        assert!(world.has_active_persons());
        assert_eq!(world.person_pos(7), None);
    }

    #[test]
    fn a_new_field_set_is_taken_over_as_a_whole() {
        let pool = pool(2);
        let mut world = corridor();
        world.spawn_persons(&[pos(20, 3)], 1);
        world.rebuild_fields_now(&pool);
        assert_eq!(world.fields_revision(), 0);
        // Wall off the west half and recompute for revision 5.
        world.grid_mut().set(pos(10, 3), Cell::VOID).unwrap();
        let input = world.field_input();
        let old = world.adopt_fields(pool.install(|| FieldSet::compute(&input, 5)));
        assert_eq!(old.revision(), 0);
        assert_eq!(world.fields_revision(), 5);
        assert_eq!(
            world.fields().dir(1, pos(20, 3)),
            0,
            "east end is reachable"
        );
        assert_eq!(
            world.fields().dir(0, pos(20, 3)),
            UNREACHABLE,
            "west end is cut off"
        );
        assert_eq!(world.fields().dir(0, pos(9, 3)), 4);
        assert_eq!(world.fields().targets(), 2);
    }

    #[test]
    fn targets_are_validated() {
        let mut world = World::new(MapSize::new(64, 8).unwrap());
        assert_eq!(
            world.add_target(Target::passengers(Vec::new())),
            Err(TargetError::NoCells)
        );
        for i in 0..MAX_TARGETS {
            assert_eq!(
                world.add_target(Target::passengers(vec![pos(0, 0)])),
                Ok(u8::try_from(i).unwrap())
            );
        }
        assert_eq!(
            world.add_target(Target::passengers(vec![pos(0, 0)])),
            Err(TargetError::TooMany)
        );
        assert_eq!(world.targets().len(), MAX_TARGETS);
    }

    #[test]
    fn open_hall_is_a_working_world_for_any_thread_count() {
        let map = MapSize::new(128, 128).unwrap();
        let run = |threads| {
            let pool = pool(threads);
            let mut world = World::open_hall(map, 2_000, 42);
            for _ in 0..200 {
                world.advance(&pool);
            }
            assert_eq!(world.per_floor().iter().sum::<u32>(), 2_000);
            hash(&world)
        };
        let single = run(1);
        assert_eq!(run(3), single);
        assert_eq!(run(8), single);
        let world = World::open_hall(map, 10, 1);
        assert_eq!(world.map(), map);
        assert_eq!(world.grid().map(), map);
        assert_eq!(world.network(NetworkKind::Road).node_count(), 0);
    }

    #[test]
    fn networks_belong_to_the_world_and_are_part_of_its_state() {
        let mut world = World::new(MapSize::new(16, 16).unwrap());
        let before = hash(&world);
        let net = world.network_mut(NetworkKind::Taxiway);
        let a = net.add_node(pos(1, 1));
        let b = net.add_node(pos(5, 1));
        net.add_edge(a, b).unwrap();
        assert_ne!(hash(&world), before);
        assert_eq!(world.network(NetworkKind::Taxiway).edge_count(), 1);
        assert_eq!(world.network(NetworkKind::Road).edge_count(), 0);
    }

    #[test]
    fn debug_helpers_spawn_clear_and_wall() {
        let mut world = World::open_hall(MapSize::new(32, 32).unwrap(), 0, 1);
        assert_eq!(world.spawn_random(20, 9), 20);
        assert_eq!(world.persons(), 20);
        let mut seen = 0;
        world.visit_persons(|p| {
            assert!(p.x > 0.0 && p.x < 32.0 && p.floor == 0);
            seen += 1;
        });
        assert_eq!(seen, 20);
        world.clear_persons();
        assert_eq!((world.persons(), world.per_floor()), (0, [0; FLOORS]));
        let rect = CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: 32,
            y1: 32,
        };
        world.set_walls(rect, true).unwrap();
        assert_eq!(world.spawn_random(5, 1), 0, "no walkable cell left");
        let bad = CellRect { floor: 4, ..rect };
        assert_eq!(world.set_walls(bad, true), Err(GridError::InvalidFloor));
        let outside = CellRect { x1: 40, ..rect };
        assert_eq!(world.set_walls(outside, false), Err(GridError::OutOfMap));
    }

    #[test]
    fn view_interpolates_the_current_step() {
        let p = Person {
            x: 3,
            y: 4,
            floor: 1,
            heading: 2, // south
            target: 0,
            progress: 0.5,
            rng: 1,
        };
        let v = p.view();
        assert!((v.x - 3.5).abs() < 1e-6 && (v.y - 5.0).abs() < 1e-6);
        let standing = Person {
            heading: ARRIVED,
            ..p
        };
        assert!((standing.view().y - 4.5).abs() < 1e-6);
    }
}
