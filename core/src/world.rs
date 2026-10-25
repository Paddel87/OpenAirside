//! World state of the simulation core (module "Takt & Weltzustand",
//! ADR-039): the domain modules in their fixed order "Welt & Bau" →
//! "Wegfindung" → "Personen", and the internal events between them.
//!
//! The order holds for the stages of a step, for loading, for the state
//! hash and for the save game. Each module keeps its state, its part of the
//! save game, the checks of a loaded state and its contribution to the
//! state hash; this file only puts them together.

use rayon::ThreadPool;

use crate::adoption::AdoptionMode;
use crate::build::grid::{AreaKind, Cell, GridError, Zone};
use crate::build::objects::BuildRefusal;
use crate::build::target::Target;
use crate::build::{Blocks, Build, BuildSnapshot, SavedBuild};
use crate::geometry::{CellPos, CellRect, MapSize, Rotation};
use crate::hash::StateHasher;
use crate::internal::{Entry, InternalEvents, Stage};
use crate::persons::{Person, Persons, SplitMix64};
use crate::savegame::invalid;
use crate::text::Reason;
use crate::time::Tick;
use crate::wayfinding::flowfield::FieldSet;
use crate::wayfinding::{RestoredWayfinding, SavedWayfinding, Wayfinding, WayfindingSnapshot};

/// The domain modules of one session and the internal events between them.
pub struct World {
    build: Build,
    wayfinding: Wayfinding,
    persons: Persons,
    events: InternalEvents,
}

impl World {
    /// Empty world: nothing is walkable, there are no targets and no
    /// persons.
    #[must_use]
    pub fn new(map: MapSize) -> Self {
        let build = Build::new(map);
        let wayfinding = Wayfinding::new(&build);
        Self {
            build,
            wayfinding,
            persons: Persons::default(),
            events: InternalEvents::default(),
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
            .build
            .grid_mut()
            .fill(rect, Cell::floor(Zone::Landside))
            .expect("the whole map is a valid rectangle");
        let middle = CellPos {
            x: map.width / 2,
            y: map.height / 2,
            floor: 0,
        };
        world
            .build
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
        world.persons.spawn(&world.build, &cells, seed);
        let input = Wayfinding::input(&world.build);
        world.wayfinding.adopt_fields(FieldSet::compute(&input, 0));
        world.persons.resample_headings(world.wayfinding.fields());
        world
    }

    /// Size of the map.
    #[must_use]
    pub fn map(&self) -> MapSize {
        self.build.map()
    }

    /// Module "Welt & Bau".
    #[must_use]
    pub fn build(&self) -> &Build {
        &self.build
    }

    /// Module "Welt & Bau" for building. Changes reach the flow fields with
    /// the next recomputation (`Simulation::request_recompute` or an
    /// internal event of the module).
    pub fn build_mut(&mut self) -> &mut Build {
        &mut self.build
    }

    /// Module "Wegfindung & Passagierführung".
    #[must_use]
    pub fn wayfinding(&self) -> &Wayfinding {
        &self.wayfinding
    }

    /// Module "Personen".
    #[must_use]
    pub fn persons(&self) -> &Persons {
        &self.persons
    }

    /// Adds one person on each of `cells` (see [`Persons::spawn`]).
    pub fn spawn_persons(&mut self, cells: &[CellPos], seed: u64) {
        self.persons.spawn(&self.build, cells, seed);
    }

    /// Adds up to `count` persons on random walkable cells of the ground
    /// floor (see [`Persons::spawn_random`]).
    pub fn spawn_random(&mut self, count: u32, seed: u64) -> u32 {
        self.persons.spawn_random(&self.build, count, seed)
    }

    /// Removes all persons (debug command).
    pub fn clear_persons(&mut self) {
        self.persons.clear();
    }

    /// Sets or clears walls in `rect` as a command (S-2); the flow fields
    /// follow through the internal event of the module "Welt & Bau".
    ///
    /// # Errors
    ///
    /// See [`Build::set_walls`].
    pub fn set_walls(&mut self, rect: CellRect, wall: bool) -> Result<(), GridError> {
        self.build
            .set_walls(rect, wall, &mut self.events, Stage::Commands)
    }

    /// Places a single object as a command (S-2); with `blocks` the flow
    /// fields follow through the internal event of the module "Welt & Bau".
    ///
    /// # Errors
    ///
    /// See [`Build::build_object`].
    #[allow(clippy::too_many_arguments)] // mirrors `Build::build_object`
    pub fn build_object(
        &mut self,
        type_id: &str,
        size: (u16, u16),
        blocks: bool,
        at: CellPos,
        rotation: Rotation,
    ) -> Result<u32, BuildRefusal> {
        self.build.build_object(
            type_id,
            size,
            blocks,
            at,
            rotation,
            &mut self.events,
            Stage::Commands,
        )
    }

    /// Draws a floor as a command (S-2).
    ///
    /// # Errors
    ///
    /// See [`Build::build_floor`].
    pub fn build_floor(
        &mut self,
        type_id: &str,
        area_kind: AreaKind,
        cost: u8,
        rect: CellRect,
    ) -> Result<(), BuildRefusal> {
        self.build.build_floor(
            type_id,
            area_kind,
            cost,
            rect,
            &mut self.events,
            Stage::Commands,
        )
    }

    /// Draws a wall as a command (S-2).
    ///
    /// # Errors
    ///
    /// See [`Build::build_wall`].
    pub fn build_wall(&mut self, type_id: &str, rect: CellRect) -> Result<(), BuildRefusal> {
        self.build
            .build_wall(type_id, rect, &mut self.events, Stage::Commands)
    }

    /// Demolishes everything in `rect` as a command (S-2).
    ///
    /// # Errors
    ///
    /// See [`Build::demolish`].
    pub fn demolish(&mut self, rect: CellRect) -> Result<(), BuildRefusal> {
        self.build.demolish(rect, &mut self.events, Stage::Commands)
    }

    /// Starts a session: adoption of the flow fields with `mode`.
    pub(crate) fn reset_adoption(&mut self, mode: AdoptionMode) {
        self.wayfinding.reset(mode);
    }

    /// Computes the flow fields now, on `pool`, with revision 0, and points
    /// every person along them. Used when a session starts, before the first
    /// tick.
    pub fn rebuild_fields_now(&mut self, pool: &ThreadPool) {
        self.wayfinding.rebuild_now(&self.build, pool);
        self.persons.resample_headings(self.wayfinding.fields());
    }

    /// Starts recomputing the flow fields from the current grid and targets
    /// in the background at `tick` and returns the revision.
    pub(crate) fn request_recompute(&mut self, background: &ThreadPool, tick: Tick) -> u32 {
        self.wayfinding.request(&self.build, background, tick)
    }

    /// One step (ADR-039): the stages of the domain modules in their order.
    /// Each stage takes its internal events, also while the game is paused;
    /// the simulation step of a module runs only if `running`. Returns the
    /// revision of adopted flow fields.
    ///
    /// - "Welt & Bau": no events and no step of its own (building happens
    ///   in the commands).
    /// - "Wegfindung": recomputation for changed grids, then adoption of a
    ///   result that is due.
    /// - "Personen": movement and target choice.
    pub fn step(
        &mut self,
        pool: &ThreadPool,
        background: &ThreadPool,
        tick: Tick,
        running: bool,
    ) -> Option<u32> {
        let adopted = self
            .wayfinding
            .step(&self.build, &self.events, background, tick);
        if running {
            self.persons
                .advance(self.wayfinding.fields(), self.build.targets().len(), pool);
        }
        self.events.end_step();
        adopted
    }

    /// Feeds the complete world state into `hasher`, module by module in
    /// their order, then the internal events carried into the next step.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        self.build.hash_into(hasher);
        self.wayfinding.hash_into(hasher);
        self.persons.hash_into(hasher);
        self.events.hash_into(hasher);
    }

    /// State for a save game, taken between two steps; cheap.
    pub(crate) fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot {
            build: self.build.snapshot(),
            wayfinding: self.wayfinding.snapshot(),
            persons: self.persons.saved(),
            events: self.events.saved(),
        }
    }
}

/// The world state taken for a save game.
#[derive(Clone)]
pub(crate) struct WorldSnapshot {
    pub build: BuildSnapshot,
    pub wayfinding: WayfindingSnapshot,
    pub persons: Vec<Person>,
    pub events: Vec<Entry>,
}

/// The parts of a save game that make up the world, before they are checked.
pub(crate) struct SavedWorld {
    pub build: SavedBuild,
    pub wayfinding: SavedWayfinding,
    pub persons: Vec<Person>,
    pub events: Vec<Entry>,
}

/// A world of a loaded save game after the structural checks; the flow
/// fields still have to be computed.
pub(crate) struct RestoredWorld {
    build: Build,
    wayfinding: RestoredWayfinding,
    persons: Vec<Person>,
    events: InternalEvents,
}

impl RestoredWorld {
    /// Full ids of the object types the world refers to.
    pub fn type_ids(&self) -> &[String] {
        self.build.types().ids()
    }

    /// Structural checks of the modules in their order.
    ///
    /// # Errors
    ///
    /// The first broken invariant (`invalidState`) or limit
    /// (`limitExceeded`) of a module.
    pub fn restore(saved: SavedWorld, map: MapSize, blocks: &Blocks) -> Result<Self, Reason> {
        let build = Build::restore(saved.build, map, blocks)?;
        let wayfinding = Wayfinding::restore(saved.wayfinding, map, blocks)?;
        let events = InternalEvents::restore(saved.events).map_err(invalid)?;
        Ok(Self {
            build,
            wayfinding,
            persons: saved.persons,
            events,
        })
    }

    /// Computes the flow fields in use on `background`, checks the persons
    /// and builds the world; an open recomputation starts again with
    /// adoption in `mode`.
    ///
    /// # Errors
    ///
    /// `invalidState` naming the broken invariant (`fieldsMap`, `persons`,
    /// `person`).
    pub fn into_world(self, background: &ThreadPool, mode: AdoptionMode) -> Result<World, Reason> {
        let map = self.build.map();
        let prepared = self.wayfinding.into_fields(map, background)?;
        let persons =
            Persons::restore(self.persons, map, self.build.targets().len()).map_err(invalid)?;
        Ok(World {
            build: self.build,
            wayfinding: prepared.resume(mode, background),
            persons,
            events: self.events,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::grid::FLOORS;
    use crate::build::network::NetworkKind;

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
            .build_mut()
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
            .build_mut()
            .add_target(Target::passengers(vec![pos(0, 3)]))
            .unwrap();
        world
            .build_mut()
            .add_target(Target::passengers(vec![pos(39, 3)]))
            .unwrap();
        world
    }

    fn advance(world: &mut World, pool: &ThreadPool) {
        world.step(pool, pool, 0, true);
    }

    #[test]
    fn persons_walk_along_the_flow_field_and_choose_new_targets() {
        let pool = pool(2);
        let mut world = corridor();
        world.spawn_persons(&[pos(20, 3)], 1);
        world.rebuild_fields_now(&pool);
        assert_eq!(world.persons().len(), 1);
        let mut visited = std::collections::BTreeSet::new();
        for _ in 0..4_000 {
            advance(&mut world, &pool);
            let p = world.persons().position(0).unwrap();
            assert_eq!(p.y, 3, "stays in the corridor");
            assert!(p.x < 40);
            visited.insert(p.x);
        }
        // 4,000 ticks at 0.1 cells per tick are 400 cells: the person reaches
        // both ends and walks back and forth.
        assert!(visited.contains(&0) && visited.contains(&39));
        assert_eq!(world.persons().per_floor()[2], 1);
    }

    #[test]
    fn a_person_without_a_way_stays_and_is_counted_as_present() {
        let pool = pool(1);
        let mut world = corridor();
        world
            .build_mut()
            .grid_mut()
            .set(pos(50, 5), Cell::floor(Zone::Landside))
            .unwrap();
        world.spawn_persons(&[pos(50, 5), pos(10, 10)], 3);
        assert_eq!(
            world.persons().len(),
            1,
            "the cell (10,10) is void and skipped"
        );
        world.rebuild_fields_now(&pool);
        for _ in 0..50 {
            advance(&mut world, &pool);
        }
        assert_eq!(world.persons().position(0), Some(pos(50, 5)));
        assert!(world.persons().has_active());
        assert_eq!(world.persons().position(7), None);
    }

    #[test]
    fn paused_steps_take_events_but_move_nobody() {
        let pool = pool(2);
        let mut world = corridor();
        world.spawn_persons(&[pos(20, 3)], 1);
        world.rebuild_fields_now(&pool);
        world.reset_adoption(AdoptionMode::FixedTick { delay: 1 });
        let before = world.persons().position(0);
        let door = CellRect {
            floor: 0,
            x0: 10,
            y0: 3,
            x1: 11,
            y1: 4,
        };
        world.set_walls(door, true).unwrap();
        assert_eq!(world.step(&pool, &pool, 5, false), None);
        assert!(
            world.wayfinding().is_recomputing(),
            "requested while paused"
        );
        assert_eq!(world.step(&pool, &pool, 6, false), Some(1));
        assert_eq!(world.persons().position(0), before);
    }

    #[test]
    fn open_hall_is_a_working_world_for_any_thread_count() {
        let map = MapSize::new(128, 128).unwrap();
        let run = |threads| {
            let pool = pool(threads);
            let mut world = World::open_hall(map, 2_000, 42);
            for _ in 0..200 {
                advance(&mut world, &pool);
            }
            assert_eq!(world.persons().per_floor().iter().sum::<u32>(), 2_000);
            hash(&world)
        };
        let single = run(1);
        assert_eq!(run(3), single);
        assert_eq!(run(8), single);
        let world = World::open_hall(map, 10, 1);
        assert_eq!(world.map(), map);
        assert_eq!(world.build().grid().map(), map);
        assert_eq!(world.build().network(NetworkKind::Road).node_count(), 0);
    }

    #[test]
    fn debug_helpers_spawn_clear_and_wall() {
        let mut world = World::open_hall(MapSize::new(32, 32).unwrap(), 0, 1);
        assert_eq!(world.spawn_random(20, 9), 20);
        assert_eq!(world.persons().len(), 20);
        let mut seen = 0;
        world.persons().visit(|p| {
            assert!(p.x > 0.0 && p.x < 32.0 && p.floor == 0);
            seen += 1;
        });
        assert_eq!(seen, 20);
        world.clear_persons();
        assert_eq!(
            (world.persons().len(), world.persons().per_floor()),
            (0, [0; FLOORS])
        );
        let rect = CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: 32,
            y1: 32,
        };
        world.set_walls(rect, true).unwrap();
        assert_eq!(world.spawn_random(5, 1), 0, "no walkable cell left");
    }
}
