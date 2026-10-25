//! World state of a save game: what is stored, how it is captured from the
//! simulation and how a loaded state is checked and turned back into a
//! world (S-5 "Was nicht gespeichert wird", "Reihenfolge der Prüfung").
//!
//! Each domain module stores, checks and rebuilds its own part (ADR-039);
//! this file puts the parts together in the order of the modules.

use rayon::ThreadPool;
use serde::{Deserialize, Serialize};

use super::{SaveMeta, SavedPack, limit};
use crate::adoption::AdoptionMode;
use crate::build::target::MAX_TARGETS;
use crate::build::{BlockPool, Blocks, SavedBlock, SavedBuild};
use crate::geometry::MapSize;
use crate::internal::Entry;
use crate::persons::{MAX_PERSONS, Person, Persons};
use crate::text::Reason;
use crate::time::{Speed, Tick};
use crate::wayfinding::SavedWayfinding;
use crate::world::{RestoredWorld, SavedWorld, World, WorldSnapshot};

/// The complete world state section, serialised with postcard (ADR-027),
/// format version 2 (ADR-039): tick, speed, map, the blocks of the grid
/// (stored once even if several tables share them), then one part per
/// domain module in their order and the internal events carried into the
/// next step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct SavedState {
    tick: Tick,
    speed: Speed,
    map: MapSize,
    blocks: Vec<SavedBlock>,
    build: SavedBuild,
    wayfinding: SavedWayfinding,
    persons: Vec<Person>,
    events: Vec<Entry>,
}

impl SavedState {
    pub(crate) fn tick(&self) -> Tick {
        self.tick
    }

    pub(crate) fn map(&self) -> MapSize {
        self.map
    }
}

/// State of a simulation taken in one tick, cheap to take (blocks are
/// shared); encoding it into a file can run on another thread.
#[derive(Clone)]
pub struct SaveSnapshot {
    pub(crate) tick: Tick,
    pub(crate) speed: Speed,
    pub(crate) world: WorldSnapshot,
    pub(crate) packs: Vec<SavedPack>,
}

impl SaveSnapshot {
    /// Tick of the snapshot.
    #[must_use]
    pub fn tick(&self) -> Tick {
        self.tick
    }

    /// Size of the map.
    #[must_use]
    pub fn map(&self) -> MapSize {
        self.world.build.map()
    }

    /// Active data packs of the session.
    #[must_use]
    pub fn packs(&self) -> &[SavedPack] {
        &self.packs
    }

    pub(crate) fn to_saved(&self) -> SavedState {
        let mut blocks = BlockPool::default();
        let build = self.world.build.to_saved(&mut blocks);
        let wayfinding = self.world.wayfinding.to_saved(&mut blocks);
        SavedState {
            tick: self.tick,
            speed: self.speed,
            map: self.map(),
            blocks: blocks.into_blocks(),
            build,
            wayfinding,
            persons: self.world.persons.clone(),
            events: self.world.events.clone(),
        }
    }
}

/// A loaded and checked save game, ready to become a session
/// (`Simulation::load_game`).
pub struct LoadedGame {
    pub(crate) meta: SaveMeta,
    pub(crate) restored: Restored,
}

impl LoadedGame {
    /// Metadata of the save game.
    #[must_use]
    pub fn meta(&self) -> &SaveMeta {
        &self.meta
    }
}

/// A loaded state after the structural checks; the flow fields still have
/// to be computed.
pub(crate) struct Restored {
    pub tick: Tick,
    pub speed: Speed,
    world: RestoredWorld,
}

impl Restored {
    /// Full ids of the object types the loaded world refers to.
    pub fn type_ids(&self) -> &[String] {
        self.world.type_ids()
    }

    /// Computes the flow fields in use on `background` and builds the world;
    /// an open recomputation starts again with adoption in `mode`.
    ///
    /// # Errors
    ///
    /// `invalidState` if the world breaks an invariant.
    pub fn into_world(self, background: &ThreadPool, mode: AdoptionMode) -> Result<World, Reason> {
        self.world.into_world(background, mode)
    }
}

/// Checks the structure of a loaded state and rebuilds its parts.
///
/// # Errors
///
/// `limitExceeded` for a map edge that is not allowed or too many
/// persons or targets, `invalidState` naming the broken invariant.
pub(crate) fn restore(state: SavedState, map_edges: &[u16]) -> Result<Restored, Reason> {
    let map = state.map;
    if map.width != map.height || !map_edges.contains(&map.width) {
        return Err(limit(
            "map",
            f64::from(*map_edges.iter().max().unwrap_or(&0)),
        ));
    }
    if !Persons::fits(state.persons.len()) {
        return Err(limit("persons", MAX_PERSONS as f64));
    }
    if state.build.targets.len() > MAX_TARGETS {
        return Err(limit("targets", MAX_TARGETS as f64));
    }
    let blocks = Blocks::restore(state.blocks)?;
    let world = RestoredWorld::restore(
        SavedWorld {
            build: state.build,
            wayfinding: state.wayfinding,
            persons: state.persons,
            events: state.events,
        },
        map,
        &blocks,
    )?;
    Ok(Restored {
        tick: state.tick,
        speed: state.speed,
        world,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::grid::{Cell, Zone};
    use crate::build::network::NetworkKind;
    use crate::geometry::CellPos;
    use crate::simulation::{Simulation, SimulationConfig};
    use crate::wayfinding::{SavedInput, SavedPending};

    const EDGES: &[u16] = &[64];

    fn pool() -> ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap()
    }

    fn simulation() -> Simulation {
        let map = MapSize::new(64, 64).unwrap();
        let config = SimulationConfig {
            map,
            threads: 1,
            background_threads: 1,
            adoption: AdoptionMode::WhenReady,
            persons: 0,
            seed: 0,
            start_tick: 0,
        };
        let mut world = World::open_hall(map, 50, 3);
        world
            .build_mut()
            .network_mut(NetworkKind::Road)
            .add_node(CellPos {
                x: 1,
                y: 1,
                floor: 0,
            });
        world
            .build_mut()
            .grid_mut()
            .set(
                CellPos {
                    x: 3,
                    y: 3,
                    floor: 1,
                },
                Cell::floor(Zone::Airside),
            )
            .unwrap();
        Simulation::with_world(config, world).unwrap()
    }

    fn saved() -> SavedState {
        simulation().save_snapshot().to_saved()
    }

    /// Code and checked part of a failed restore (structure or world).
    fn failure(state: SavedState) -> (String, Option<String>) {
        let result = restore(state, EDGES)
            .and_then(|r| r.into_world(&pool(), AdoptionMode::WhenReady).map(|_| ()));
        let reason = result.expect_err("the state must be rejected");
        let part = reason
            .params
            .get("check")
            .or_else(|| reason.params.get("limit"))
            .map(|v| match v {
                crate::text::ParamValue::Text(t) => t.clone(),
                crate::text::ParamValue::Number(n) => n.to_string(),
                crate::text::ParamValue::Key { key } => key.clone(),
            });
        (reason.code, part)
    }

    fn fails(state: SavedState, code: &str, part: &str) {
        assert_eq!(failure(state), (code.to_owned(), Some(part.to_owned())));
    }

    #[test]
    fn a_saved_state_restores_the_same_world() {
        let sim = simulation();
        let state = sim.save_snapshot().to_saved();
        // The fields in use share all blocks with the grid: stored once.
        assert_eq!(state.wayfinding.fields.grid, state.build.grid);
        let distinct: std::collections::BTreeSet<u32> = state
            .build
            .grid
            .iter()
            .copied()
            .filter(|&n| n > 0)
            .collect();
        assert_eq!(state.blocks.len(), distinct.len());
        // An empty ground floor shares one void block.
        let map = MapSize::new(64, 64).unwrap();
        let empty = Simulation::with_world(simulation().config(), World::new(map)).unwrap();
        assert_eq!(empty.save_snapshot().to_saved().blocks.len(), 1);
        let world = restore(state, EDGES)
            .unwrap()
            .into_world(&pool(), AdoptionMode::WhenReady)
            .unwrap();
        let mut a = crate::hash::StateHasher::new();
        world.hash_into(&mut a);
        let mut b = crate::hash::StateHasher::new();
        sim.world().hash_into(&mut b);
        assert_eq!(a.finish(), b.finish());
        assert!(!world.wayfinding().is_recomputing());
    }

    #[test]
    fn maps_counts_and_blocks_are_checked() {
        let mut s = saved();
        s.map = MapSize::new(64, 32).unwrap();
        fails(s, "limitExceeded", "map");
        assert_eq!(
            restore(saved(), &[1536]).err().map(|r| r.code),
            Some("limitExceeded".into())
        );
        let mut s = saved();
        s.persons = vec![s.persons[0]; MAX_PERSONS + 1];
        fails(s, "limitExceeded", "persons");
        let mut s = saved();
        s.build.targets = vec![s.build.targets[0].clone(); MAX_TARGETS + 1];
        fails(s, "limitExceeded", "targets");
        let mut s = saved();
        s.wayfinding.fields.targets = vec![s.build.targets[0].clone(); MAX_TARGETS + 1];
        fails(s, "limitExceeded", "targets");
        let mut s = saved();
        s.blocks[0].zone[5] = 9;
        fails(s, "invalidState", "block");
        let mut s = saved();
        s.blocks[0].cost[0] = 0;
        fails(s, "invalidState", "block");
        let mut s = saved();
        s.blocks[0].flags[7] = 0x80;
        fails(s, "invalidState", "block");
        let mut s = saved();
        s.blocks[0].flags.pop();
        fails(s, "invalidState", "block");
        let mut s = saved();
        s.build.grid[0] = 999;
        fails(s, "invalidState", "blockIndex");
        let mut s = saved();
        s.build.grid.pop();
        fails(s, "invalidState", "table");
        let mut s = saved();
        let ground = 2 * 4; // floor index 2, 2 × 2 blocks per floor
        s.build.grid[ground] = 0;
        fails(s, "invalidState", "table");
        let mut s = saved();
        let pending_grid = s.wayfinding.fields.grid.clone();
        s.wayfinding.pending = Some(SavedPending {
            revision: 1,
            due: 0,
            input: SavedInput {
                grid: vec![0; 3],
                targets: Vec::new(),
            },
        });
        fails(s.clone(), "invalidState", "table");
        s.wayfinding.pending.as_mut().unwrap().input.grid = pending_grid;
        assert!(restore(s, EDGES).is_ok());
    }

    /// The simulation of `simulation()` with a blocking bench at (10, 10)
    /// and a fern that does not block at (20, 20).
    fn furnished() -> SavedState {
        use crate::geometry::Rotation;
        use crate::internal::{InternalEvents, Stage};
        let mut sim = simulation();
        let mut events = InternalEvents::default();
        let build = sim.world_mut().build_mut();
        for (id, at, blocks) in [("base:bench", 10, true), ("plants:fern", 20, false)] {
            build
                .build_object(
                    id,
                    (2, 1),
                    blocks,
                    CellPos {
                        x: at,
                        y: at,
                        floor: 0,
                    },
                    Rotation(0),
                    &mut events,
                    Stage::Commands,
                )
                .unwrap();
        }
        sim.save_snapshot().to_saved()
    }

    #[test]
    fn objects_and_types_are_checked() {
        let s = furnished();
        assert_eq!(s.build.types, ["base:bench", "plants:fern"]);
        assert_eq!(s.build.objects.len(), 2);
        assert!(restore(s.clone(), EDGES).is_ok());

        let mut bad = s.clone();
        bad.build.types[1] = "base:bench".into();
        fails(bad, "invalidState", "types");
        let mut bad = s.clone();
        bad.build.types[0].clear();
        fails(bad, "invalidState", "types");
        let mut bad = s.clone();
        bad.build.types = vec!["x".into(); crate::build::objects::MAX_TYPES + 1];
        fails(bad, "limitExceeded", "types");
        // Cells may only refer to types of the table.
        let mut bad = s.clone();
        bad.blocks[0].floor_kind[0] = 3;
        fails(bad, "invalidState", "blockKind");
        let mut bad = s.clone();
        bad.blocks[0].wall_kind[0] = 3;
        fails(bad, "invalidState", "blockKind");

        let mut bad = s.clone();
        bad.build.objects[0].ty = 0;
        fails(bad, "invalidState", "objects");
        let mut bad = s.clone();
        bad.build.objects[0].ty = 3;
        fails(bad, "invalidState", "objects");
        let mut bad = s.clone();
        bad.build.objects[0].rotation = 4;
        fails(bad, "invalidState", "objects");
        let mut bad = s.clone();
        bad.build.objects[0].x = 63;
        fails(bad, "invalidState", "objects");
        let mut bad = s.clone();
        bad.build.objects[0].floor = 1;
        fails(bad, "invalidState", "objects");
        let mut bad = s.clone();
        bad.build.objects[0].width = 0;
        fails(bad, "invalidState", "objects");
        let mut bad = s.clone();
        bad.build.objects = vec![s.build.objects[0]; crate::build::objects::MAX_OBJECTS + 1];
        fails(bad, "limitExceeded", "objects");

        // Handles rise and stay below the counter.
        let mut bad = s.clone();
        bad.build.objects.swap(0, 1);
        fails(bad, "invalidState", "objectHandles");
        let mut bad = s.clone();
        bad.build.next_handle = 1;
        fails(bad, "invalidState", "objectHandles");

        // Objects do not overlap, and a blocking object has impassable cells.
        let mut bad = s.clone();
        bad.build.objects[1].x = 10;
        bad.build.objects[1].y = 10;
        fails(bad, "invalidState", "objectCells");
        let mut bad = s.clone();
        bad.build.objects[1].blocks = true;
        fails(bad, "invalidState", "objectCells");
    }

    #[test]
    fn networks_are_checked() {
        let mut s = saved();
        s.build.networks.pop();
        fails(s, "invalidState", "networks");
        let mut s = saved();
        s.build.networks[0].edges.push((0, 0));
        fails(s, "invalidState", "edges");
        let mut s = saved();
        s.build.networks[0].edges.push((0, 7));
        fails(s, "invalidState", "edges");
        let mut s = saved();
        s.build.networks[1].nodes.push(CellPos {
            x: 64,
            y: 0,
            floor: 0,
        });
        fails(s, "invalidState", "networkNodes");
    }

    #[test]
    fn targets_persons_and_events_are_checked() {
        let mut s = saved();
        s.build.targets[0].cells.clear();
        fails(s, "invalidState", "targetCells");
        let mut s = saved();
        s.build.targets[0].cells.push(CellPos {
            x: 0,
            y: 0,
            floor: 3,
        });
        fails(s, "invalidState", "targetCells");
        // The checks of a person are the module's (`persons` tests); here
        // only that they run. x is the first field: write 64 (off the map).
        let mut s = saved();
        let mut bytes = postcard::to_stdvec(&s.persons[0]).unwrap();
        bytes[0] = 64;
        s.persons[0] = postcard::from_bytes(&bytes).unwrap();
        fails(s, "invalidState", "person");
        let mut s = saved();
        s.events = vec![(
            crate::internal::Stage::Commands,
            crate::internal::InternalEvent::GridChanged,
        )];
        fails(s, "invalidState", "events");
    }
}
