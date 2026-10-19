//! World state of a save game: what is stored, how it is captured from the
//! simulation and how a loaded state is checked and turned back into a
//! world (S-5 "Was nicht gespeichert wird", "Reihenfolge der Prüfung").

use std::collections::HashMap;
use std::sync::Arc;

use rayon::ThreadPool;
use serde::{Deserialize, Serialize};

use super::{SaveMeta, SavedPack, invalid, limit};
use crate::flowfield::{FieldInput, FieldSet};
use crate::geometry::{CellPos, MapSize};
use crate::grid::{BLOCK_CELLS, Block, Grid, GridSnapshot};
use crate::network::{Network, NetworkKind, NodeId};
use crate::target::{LoadKind, MAX_TARGETS, Target};
use crate::text::Reason;
use crate::time::{Speed, Tick};
use crate::world::{MAX_PERSONS, World};

/// Hot fields of one person as stored.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct SavedPerson {
    pub x: u16,
    pub y: u16,
    pub floor: i8,
    pub heading: u8,
    pub target: u8,
    pub progress: f32,
    pub rng: u32,
}

/// One block: the three cell fields in row order, 1,024 bytes each.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SavedBlock {
    flags: Vec<u8>,
    zone: Vec<u8>,
    cost: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SavedTarget {
    cells: Vec<CellPos>,
    load: LoadKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SavedNetwork {
    nodes: Vec<CellPos>,
    edges: Vec<(u32, u32)>,
}

/// Input of a flow field computation: a block table (0 = no block,
/// n = block n − 1 of [`SavedState::blocks`]) and the targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SavedInput {
    grid: Vec<u32>,
    targets: Vec<SavedTarget>,
}

/// A recomputation that was running when the game was saved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SavedPending {
    revision: u32,
    /// Adoption tick in the verification mode.
    due: Tick,
    input: SavedInput,
}

/// The complete world state section, serialised with postcard (ADR-027).
/// Blocks are stored once even if several tables share them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct SavedState {
    tick: Tick,
    speed: Speed,
    map: MapSize,
    blocks: Vec<SavedBlock>,
    grid: Vec<u32>,
    grid_version: u32,
    targets: Vec<SavedTarget>,
    networks: Vec<SavedNetwork>,
    persons: Vec<SavedPerson>,
    /// Revision of the flow fields in use.
    fields_revision: u32,
    /// Input of the flow fields in use (grid and targets at that revision).
    fields: SavedInput,
    next_revision: u32,
    pending: Option<SavedPending>,
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
    pub(crate) grid: GridSnapshot,
    pub(crate) grid_version: u32,
    pub(crate) targets: Vec<Target>,
    pub(crate) networks: Vec<Network>,
    pub(crate) persons: Vec<SavedPerson>,
    pub(crate) fields_revision: u32,
    pub(crate) fields: FieldInput,
    pub(crate) next_revision: u32,
    pub(crate) pending: Option<(u32, Tick, FieldInput)>,
    pub(crate) packs: Vec<SavedPack>,
}

impl SaveSnapshot {
    /// Tick of the snapshot.
    #[must_use]
    pub fn tick(&self) -> Tick {
        self.tick
    }

    /// Active data packs of the session.
    #[must_use]
    pub fn packs(&self) -> &[SavedPack] {
        &self.packs
    }

    pub(crate) fn to_saved(&self) -> SavedState {
        let mut pool = BlockPool::default();
        let grid = pool.table(&self.grid);
        let fields_grid = pool.table(&self.fields.grid);
        let pending = self.pending.as_ref().map(|(revision, due, input)| {
            let grid = pool.table(&input.grid);
            SavedPending {
                revision: *revision,
                due: *due,
                input: SavedInput {
                    grid,
                    targets: saved_targets(&input.targets),
                },
            }
        });
        SavedState {
            tick: self.tick,
            speed: self.speed,
            map: self.grid.map(),
            blocks: pool.blocks,
            grid,
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
            persons: self.persons.clone(),
            fields_revision: self.fields_revision,
            fields: SavedInput {
                grid: fields_grid,
                targets: saved_targets(&self.fields.targets),
            },
            next_revision: self.next_revision,
            pending,
        }
    }
}

fn saved_targets(targets: &[Target]) -> Vec<SavedTarget> {
    targets
        .iter()
        .map(|t| SavedTarget {
            cells: t.cells.clone(),
            load: t.load,
        })
        .collect()
}

/// Collects the distinct blocks of several snapshots, identified by their
/// shared allocation.
#[derive(Default)]
struct BlockPool {
    blocks: Vec<SavedBlock>,
    index: HashMap<*const Block, u32>,
}

impl BlockPool {
    fn table(&mut self, snapshot: &GridSnapshot) -> Vec<u32> {
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
                        });
                        u32::try_from(self.blocks.len()).unwrap_or(u32::MAX)
                    })
                })
            })
            .collect()
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

/// Parts of a loaded state after the structural checks; the flow fields
/// still have to be computed.
pub(crate) struct Restored {
    pub tick: Tick,
    pub speed: Speed,
    pub grid: Grid,
    pub targets: Vec<Target>,
    pub networks: [Network; 3],
    pub persons: Vec<SavedPerson>,
    pub fields_revision: u32,
    pub fields: FieldInput,
    pub next_revision: u32,
    pub pending: Option<(u32, Tick, FieldInput)>,
}

impl Restored {
    /// Computes the flow fields in use on `pool` and builds the world.
    ///
    /// # Errors
    ///
    /// `invalidState` if the world breaks an invariant.
    pub fn into_world(self, pool: &ThreadPool) -> Result<(World, RestoredRest), Reason> {
        let fields = pool.install(|| FieldSet::compute(&self.fields, self.fields_revision));
        let world = World::from_saved(
            self.grid,
            self.targets,
            self.networks,
            fields,
            &self.persons,
        )
        .map_err(invalid)?;
        Ok((
            world,
            RestoredRest {
                tick: self.tick,
                speed: self.speed,
                next_revision: self.next_revision,
                pending: self.pending,
            },
        ))
    }
}

/// Session state of a loaded game besides the world.
pub(crate) struct RestoredRest {
    pub tick: Tick,
    pub speed: Speed,
    pub next_revision: u32,
    pub pending: Option<(u32, Tick, FieldInput)>,
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
    if state.persons.len() > MAX_PERSONS {
        return Err(limit("persons", MAX_PERSONS as f64));
    }
    if state.targets.len() > MAX_TARGETS {
        return Err(limit("targets", MAX_TARGETS as f64));
    }
    let blocks = state
        .blocks
        .into_iter()
        .map(block)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| invalid("block"))?;
    let snapshot = |table: &[u32]| -> Result<GridSnapshot, Reason> {
        let entries = table
            .iter()
            .map(|&n| match n {
                0 => Some(None),
                n => blocks.get(n as usize - 1).map(|b| Some(Arc::clone(b))),
            })
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| invalid("blockIndex"))?;
        GridSnapshot::from_entries(map, entries).ok_or_else(|| invalid("table"))
    };
    let grid = Grid::from_snapshot(snapshot(&state.grid)?, state.grid_version);
    let fields = FieldInput {
        grid: snapshot(&state.fields.grid)?,
        targets: targets(state.fields.targets)?,
    };
    let pending = match state.pending {
        Some(p) => Some((
            p.revision,
            p.due,
            FieldInput {
                grid: snapshot(&p.input.grid)?,
                targets: targets(p.input.targets)?,
            },
        )),
        None => None,
    };
    if state.networks.len() != NetworkKind::ALL.len() {
        return Err(invalid("networks"));
    }
    let mut networks = Vec::with_capacity(3);
    for (kind, saved) in NetworkKind::ALL.into_iter().zip(state.networks) {
        let edges = saved
            .edges
            .into_iter()
            .map(|(a, b)| (NodeId(a), NodeId(b)))
            .collect();
        networks
            .push(Network::from_parts(kind, saved.nodes, edges).ok_or_else(|| invalid("edges"))?);
    }
    let networks: [Network; 3] = networks.try_into().map_err(|_| invalid("networks"))?;
    Ok(Restored {
        tick: state.tick,
        speed: state.speed,
        grid,
        targets: targets(state.targets)?,
        networks,
        persons: state.persons,
        fields_revision: state.fields_revision,
        fields,
        next_revision: state.next_revision,
        pending,
    })
}

fn block(saved: SavedBlock) -> Option<Arc<Block>> {
    let flags: [u8; BLOCK_CELLS] = saved.flags.try_into().ok()?;
    let zone: [u8; BLOCK_CELLS] = saved.zone.try_into().ok()?;
    let cost: [u8; BLOCK_CELLS] = saved.cost.try_into().ok()?;
    Block::from_fields(flags, zone, cost).map(Arc::new)
}

fn targets(saved: Vec<SavedTarget>) -> Result<Vec<Target>, Reason> {
    if saved.len() > MAX_TARGETS {
        return Err(limit("targets", MAX_TARGETS as f64));
    }
    Ok(saved
        .into_iter()
        .map(|t| Target {
            cells: t.cells,
            load: t.load,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adoption::AdoptionMode;
    use crate::geometry::CellPos;
    use crate::grid::{Cell, Zone};
    use crate::simulation::{Simulation, SimulationConfig};

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
        world.network_mut(NetworkKind::Road).add_node(CellPos {
            x: 1,
            y: 1,
            floor: 0,
        });
        world
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
        let result = restore(state, EDGES).and_then(|r| r.into_world(&pool()).map(|_| ()));
        let reason = result.expect_err("the state must be rejected");
        let part = reason
            .params
            .get("check")
            .or_else(|| reason.params.get("limit"))
            .map(|v| match v {
                crate::text::ParamValue::Text(t) => t.clone(),
                crate::text::ParamValue::Number(n) => n.to_string(),
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
        assert_eq!(state.fields.grid, state.grid);
        let distinct: std::collections::BTreeSet<u32> =
            state.grid.iter().copied().filter(|&n| n > 0).collect();
        assert_eq!(state.blocks.len(), distinct.len());
        // An empty ground floor shares one void block.
        let map = MapSize::new(64, 64).unwrap();
        let empty = Simulation::with_world(simulation().config(), World::new(map)).unwrap();
        assert_eq!(empty.save_snapshot().to_saved().blocks.len(), 1);
        let (world, rest) = restore(state, EDGES).unwrap().into_world(&pool()).unwrap();
        let mut a = crate::hash::StateHasher::new();
        world.hash_into(&mut a);
        let mut b = crate::hash::StateHasher::new();
        sim.world().hash_into(&mut b);
        assert_eq!(a.finish(), b.finish());
        assert_eq!(rest.next_revision, 1);
        assert!(rest.pending.is_none());
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
        s.targets = vec![s.targets[0].clone(); MAX_TARGETS + 1];
        fails(s, "limitExceeded", "targets");
        let mut s = saved();
        s.fields.targets = vec![s.targets[0].clone(); MAX_TARGETS + 1];
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
        s.grid[0] = 999;
        fails(s, "invalidState", "blockIndex");
        let mut s = saved();
        s.grid.pop();
        fails(s, "invalidState", "table");
        let mut s = saved();
        let ground = 2 * 4; // floor index 2, 2 × 2 blocks per floor
        s.grid[ground] = 0;
        fails(s, "invalidState", "table");
        let mut s = saved();
        let pending_grid = s.fields.grid.clone();
        s.pending = Some(SavedPending {
            revision: 1,
            due: 0,
            input: SavedInput {
                grid: vec![0; 3],
                targets: Vec::new(),
            },
        });
        fails(s.clone(), "invalidState", "table");
        s.pending.as_mut().unwrap().input.grid = pending_grid;
        assert!(restore(s, EDGES).is_ok());
    }

    #[test]
    fn networks_are_checked() {
        let mut s = saved();
        s.networks.pop();
        fails(s, "invalidState", "networks");
        let mut s = saved();
        s.networks[0].edges.push((0, 0));
        fails(s, "invalidState", "edges");
        let mut s = saved();
        s.networks[0].edges.push((0, 7));
        fails(s, "invalidState", "edges");
        let mut s = saved();
        s.networks[1].nodes.push(CellPos {
            x: 64,
            y: 0,
            floor: 0,
        });
        fails(s, "invalidState", "networkNodes");
    }

    #[test]
    fn targets_and_persons_are_checked() {
        let mut s = saved();
        s.targets[0].cells.clear();
        fails(s, "invalidState", "targetCells");
        let mut s = saved();
        s.targets[0].cells.push(CellPos {
            x: 0,
            y: 0,
            floor: 3,
        });
        fails(s, "invalidState", "targetCells");
        for change in [
            |p: &mut SavedPerson| p.x = 64,
            |p: &mut SavedPerson| p.floor = -3,
            |p: &mut SavedPerson| p.heading = 11,
            |p: &mut SavedPerson| p.target = 1,
            |p: &mut SavedPerson| p.progress = f32::NAN,
            |p: &mut SavedPerson| p.progress = 1.5,
        ] {
            let mut s = saved();
            change(&mut s.persons[0]);
            fails(s, "invalidState", "person");
        }
    }
}
