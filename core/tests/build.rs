//! Building with effect (ADR-042, step 3.1a): `build`, `buildArea` and
//! `demolish` change the grid and the objects of the world state, a refused
//! command changes nothing, and the result survives a save game.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use openairside_core::adoption::AdoptionMode;
use openairside_core::build::grid::{AreaKind, Cell, WALL};
use openairside_core::command::{Command, code};
use openairside_core::datapack::{self, Catalog, game_version};
use openairside_core::event::Event;
use openairside_core::geometry::{CellPos, CellRect, MapSize, Rotation};
use openairside_core::hash::StateHasher;
use openairside_core::savegame::{Limits, SaveKind, SaveRequest, decode};
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::text::ParamValue;
use openairside_core::world::World;

const EDGE: u16 = 64;
const FLOOR: &str = "base:floor_concrete";
const WALL_TYPE: &str = "base:wall_concrete";
const BENCH: &str = "base:bench";

fn config() -> SimulationConfig {
    SimulationConfig {
        map: MapSize::new(EDGE, EDGE).unwrap(),
        threads: 1,
        background_threads: 1,
        adoption: AdoptionMode::WhenReady,
        persons: 0,
        seed: 0,
        start_tick: 0,
    }
}

fn base_content() -> Arc<Catalog> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../content/base");
    let folders = datapack::discover(Some(&base), None);
    Arc::new(datapack::load(&folders, &[], &game_version()).catalog)
}

/// A simulation with the base pack that answers one command at a time.
struct Game {
    sim: Simulation,
    next_id: u32,
}

impl Game {
    fn new() -> Self {
        Self::with(base_content())
    }

    fn with(content: Arc<Catalog>) -> Self {
        Self {
            sim: Simulation::with_world(config(), World::new(MapSize::new(EDGE, EDGE).unwrap()))
                .unwrap()
                .with_content(content),
            next_id: 1,
        }
    }

    /// Runs one command; `None` if accepted, else the rejection code and
    /// the `object` parameter.
    fn run(&mut self, command: Command) -> Option<(String, Option<String>)> {
        let id = self.next_id;
        self.next_id += 1;
        self.sim.submit(self.sim.session(), id, command);
        self.sim.step();
        let mut answer = None;
        for record in self.sim.take_events(usize::MAX) {
            self.sim.ack(self.sim.session(), record.seq);
            match record.event {
                Event::CommandAccepted { id: got, .. } => assert_eq!(got, id),
                Event::CommandRejected {
                    id: got, reason, ..
                } => {
                    assert_eq!(got, id);
                    let object = match reason.params.get("object") {
                        Some(ParamValue::Key { key }) => Some(key.clone()),
                        _ => None,
                    };
                    answer = Some((reason.code, object));
                }
                _ => {}
            }
        }
        answer
    }

    fn accepted(&mut self, command: Command) {
        assert_eq!(self.run(command), None);
    }

    fn refused(&mut self, command: Command) -> String {
        let before = self.build_hash();
        let (code, _) = self.run(command).expect("the command must be refused");
        // A refused command changes nothing in the module "Welt & Bau".
        assert_eq!(self.build_hash(), before, "refused: {code}");
        code
    }

    fn build_hash(&self) -> u64 {
        let mut hasher = StateHasher::new();
        self.sim.world().build().hash_into(&mut hasher);
        hasher.finish()
    }

    fn cell(&self, x: u16, y: u16) -> Cell {
        self.sim
            .world()
            .build()
            .grid()
            .cell(CellPos { x, y, floor: 0 })
            .unwrap()
    }

    fn walkable(&self, x: u16, y: u16) -> bool {
        self.cell(x, y).walkable()
    }

    fn type_number(&self, id: &str) -> u16 {
        self.sim
            .world()
            .build()
            .types()
            .number(id)
            .unwrap_or_else(|| panic!("{id} was never built"))
    }
}

fn rect(x0: u16, y0: u16, x1: u16, y1: u16) -> CellRect {
    CellRect {
        floor: 0,
        x0,
        y0,
        x1,
        y1,
    }
}

fn area(object: &str, area: CellRect) -> Command {
    Command::BuildArea {
        object: object.into(),
        area,
    }
}

fn bench(x: u16, y: u16, rotation: u8) -> Command {
    Command::Build {
        object: BENCH.into(),
        at: CellPos { x, y, floor: 0 },
        rotation: Rotation(rotation),
    }
}

fn demolish(area: CellRect) -> Command {
    Command::Demolish { area }
}

#[test]
fn a_floor_makes_cells_walkable_and_names_its_covering() {
    let mut game = Game::new();
    assert!(!game.walkable(5, 5));
    game.accepted(area(FLOOR, rect(4, 4, 10, 8)));
    let number = game.type_number(FLOOR);
    for (x, y) in [(4, 4), (9, 7), (6, 5)] {
        let cell = game.cell(x, y);
        assert!(cell.walkable(), "({x}, {y})");
        assert_eq!((cell.floor_kind, cell.wall_kind), (number, 0));
        assert_eq!((cell.kind(), cell.cost), (AreaKind::PublicWalk, 10));
    }
    for (x, y) in [(3, 4), (10, 4), (4, 8)] {
        assert_eq!(game.cell(x, y), Cell::VOID, "outside stays void");
    }
}

#[test]
fn a_wall_blocks_cells_and_names_its_type() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 10, 10)));
    game.accepted(area(WALL_TYPE, rect(5, 0, 6, 10)));
    let number = game.type_number(WALL_TYPE);
    let cell = game.cell(5, 3);
    assert!(!cell.walkable() && cell.flags & WALL != 0);
    assert_eq!(cell.wall_kind, number);
    assert_eq!(
        cell.floor_kind,
        game.type_number(FLOOR),
        "the floor stays under it"
    );
    assert!(game.walkable(4, 3) && game.walkable(6, 3));
    // A wall also stands on void.
    game.accepted(area(WALL_TYPE, rect(20, 20, 22, 22)));
    assert_eq!(game.cell(20, 20).wall_kind, number);
}

#[test]
fn a_later_floor_leaves_walls_alone() {
    let mut game = Game::new();
    game.accepted(area(WALL_TYPE, rect(5, 5, 6, 6)));
    let wall = game.cell(5, 5);
    game.accepted(area(FLOOR, rect(0, 0, 10, 10)));
    assert_eq!(game.cell(5, 5), wall);
    assert!(game.walkable(4, 5) && game.walkable(6, 5));
}

#[test]
fn an_object_needs_floor_under_its_whole_footprint() {
    let mut game = Game::new();
    // The bench is 4 m × 2 m, that is 2 × 1 cells.
    assert_eq!(game.refused(bench(10, 10, 0)), code::NEEDS_FLOOR);
    game.accepted(area(FLOOR, rect(10, 10, 11, 11)));
    assert_eq!(
        game.refused(bench(10, 10, 0)),
        code::NEEDS_FLOOR,
        "one of two cells has floor"
    );
    game.accepted(area(FLOOR, rect(10, 10, 12, 11)));
    game.accepted(bench(10, 10, 0));
    assert_eq!(game.sim.world().build().objects().len(), 1);
}

#[test]
fn the_rejection_names_the_object_type() {
    let mut game = Game::new();
    let (reason, object) = game.run(bench(10, 10, 0)).expect("refused");
    assert_eq!(reason, code::NEEDS_FLOOR);
    assert_eq!(object.as_deref(), Some("object.bench"));
}

#[test]
fn a_blocking_object_makes_its_cells_impassable_and_turns_with_the_rotation() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 30, 30)));
    game.accepted(bench(10, 10, 0));
    assert!(!game.walkable(10, 10) && !game.walkable(11, 10));
    assert!(game.walkable(12, 10) && game.walkable(10, 11));
    let object = *game
        .sim
        .world()
        .build()
        .object_at(CellPos {
            x: 11,
            y: 10,
            floor: 0,
        })
        .unwrap();
    assert_eq!((object.width, object.depth, object.blocks), (2, 1, true));

    // Rotations 1 and 3 swap width and depth: 1 × 2 cells.
    game.accepted(bench(20, 10, 1));
    assert!(!game.walkable(20, 10) && !game.walkable(20, 11));
    assert!(game.walkable(21, 10));
    let turned = game
        .sim
        .world()
        .build()
        .object_at(CellPos {
            x: 20,
            y: 11,
            floor: 0,
        })
        .unwrap();
    assert_eq!((turned.width, turned.depth, turned.rotation), (1, 2, 1));
    assert!(turned.handle > object.handle, "handles rise");
    // The floor under an object keeps its covering.
    assert_eq!(game.cell(10, 10).floor_kind, game.type_number(FLOOR));
}

#[test]
fn objects_do_not_overlap_walls_or_each_other() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 30, 30)));
    game.accepted(bench(10, 10, 0));
    assert_eq!(
        game.refused(bench(11, 10, 0)),
        code::BLOCKED,
        "second bench half on the first"
    );
    assert_eq!(game.refused(bench(9, 10, 0)), code::BLOCKED);
    game.accepted(area(WALL_TYPE, rect(20, 20, 21, 21)));
    assert_eq!(
        game.refused(bench(19, 20, 0)),
        code::BLOCKED,
        "wall in the footprint"
    );
    assert_eq!(game.refused(bench(63, 10, 0)), code::OUT_OF_MAP);
    assert_eq!(game.sim.world().build().objects().len(), 1);
}

#[test]
fn a_wall_over_an_object_refuses_the_whole_command() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 30, 30)));
    game.accepted(bench(10, 10, 0));
    // The area covers free floor and one cell of the bench: nothing is built.
    assert_eq!(
        game.refused(area(WALL_TYPE, rect(5, 10, 11, 11))),
        code::BLOCKED
    );
    assert!(game.walkable(5, 10));
}

#[test]
fn a_floor_over_an_object_leaves_the_object_in_place() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 30, 30)));
    game.accepted(bench(10, 10, 0));
    game.accepted(area(FLOOR, rect(8, 8, 14, 14)));
    assert!(!game.walkable(10, 10), "still blocked by the bench");
    assert_eq!(game.sim.world().build().objects().len(), 1);
    assert!(game.walkable(9, 9));
}

#[test]
fn demolishing_removes_every_touched_object_as_a_whole() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 30, 30)));
    game.accepted(bench(10, 10, 0));
    game.accepted(bench(20, 20, 0));
    let first = game.sim.world().build().objects().as_slice()[0].handle;
    // The frame touches only the left cell of the first bench.
    game.accepted(demolish(rect(10, 10, 11, 11)));
    let build = game.sim.world().build();
    assert_eq!(build.objects().len(), 1, "the second bench stands");
    assert!(
        build.object(first).is_none(),
        "a stale handle finds nothing"
    );
    assert_eq!(
        game.cell(10, 10),
        Cell::TERRAIN,
        "inside the frame: terrain"
    );
    let right = game.cell(11, 10);
    assert!(right.walkable(), "outside the frame: passable again");
    assert_eq!(right.floor_kind, game.type_number(FLOOR), "with its floor");
    assert!(!game.walkable(20, 20));
}

#[test]
fn demolishing_clears_floors_and_walls_inside_the_frame_only() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 10, 10)));
    game.accepted(area(WALL_TYPE, rect(5, 5, 6, 6)));
    game.accepted(demolish(rect(4, 4, 7, 7)));
    for (x, y) in [(4, 4), (5, 5), (6, 6)] {
        // The ground floor keeps terrain where something was (ADR-045).
        assert_eq!(game.cell(x, y), Cell::TERRAIN, "({x}, {y})");
    }
    assert!(game.walkable(3, 3) && game.walkable(7, 7));
    // Nothing to demolish is not an error.
    game.accepted(demolish(rect(40, 40, 50, 50)));
}

#[test]
fn only_the_ground_floor_can_be_built_on() {
    let mut game = Game::new();
    let upstairs = rect(0, 0, 4, 4);
    let upstairs = CellRect {
        floor: 1,
        ..upstairs
    };
    assert_eq!(game.refused(area(FLOOR, upstairs)), code::FLOOR_LOCKED);
    assert_eq!(game.refused(demolish(upstairs)), code::FLOOR_LOCKED);
    let bench_up = Command::Build {
        object: BENCH.into(),
        at: CellPos {
            x: 1,
            y: 1,
            floor: -1,
        },
        rotation: Rotation(0),
    };
    assert_eq!(game.refused(bench_up), code::FLOOR_LOCKED);
}

#[test]
fn the_placement_of_the_type_must_fit_the_command() {
    let mut game = Game::new();
    // A floor is no single object, a bench no area.
    let floor_as_object = Command::Build {
        object: FLOOR.into(),
        at: CellPos {
            x: 1,
            y: 1,
            floor: 0,
        },
        rotation: Rotation(0),
    };
    assert_eq!(game.refused(floor_as_object), code::INVALID_ARGUMENT);
    assert_eq!(
        game.refused(area(BENCH, rect(0, 0, 3, 3))),
        code::INVALID_ARGUMENT
    );
    assert_eq!(
        game.refused(area("base:nothing", rect(0, 0, 3, 3))),
        code::UNKNOWN_CONTENT
    );
}

#[test]
fn a_non_blocking_object_leaves_its_cells_passable() {
    let dir = scratch("plant");
    fs::create_dir_all(dir.join("plants/objects")).unwrap();
    fs::write(
        dir.join("plants/manifest.json"),
        r#"{"id":"plants","name":"Plants","version":"1.0.0","format":1,"game":">=0.0.0",
            "dependencies":[{"id":"base","version":"*"}]}"#,
    )
    .unwrap();
    fs::write(
        dir.join("plants/objects/plant.json"),
        r#"{"objects":[{"id":"fern","name":"object.fern",
            "placement":{"kind":"single","width":2,"depth":2},"blocks":false}]}"#,
    )
    .unwrap();
    let content = load_with(&dir, &["plants"]);
    let mut game = Game::with(content);
    game.accepted(area(FLOOR, rect(0, 0, 10, 10)));
    let fern = Command::Build {
        object: "plants:fern".into(),
        at: CellPos {
            x: 3,
            y: 3,
            floor: 0,
        },
        rotation: Rotation(0),
    };
    game.accepted(fern);
    assert!(game.walkable(3, 3), "persons walk over a fern");
    let object = game.sim.world().build().objects().as_slice()[0];
    assert!(!object.blocks);
    // It still occupies the place.
    assert_eq!(game.refused(bench(3, 3, 0)), code::BLOCKED);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn building_changes_the_flow_fields() {
    let map = MapSize::new(EDGE, EDGE).unwrap();
    let world = World::open_hall(map, 0, 1);
    let mut game = Game {
        sim: Simulation::with_world(config(), world)
            .unwrap()
            .with_content(base_content()),
        next_id: 1,
    };
    let before = game.sim.world().wayfinding().fields_revision();
    game.accepted(bench(10, 10, 0));
    // The bench stands on floor; the hall has floor without a covering, so
    // build it explicitly.
    assert_eq!(game.sim.world().build().objects().len(), 1);
    assert!(!game.walkable(10, 10));
    // The recomputation runs in the background; wait for it.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while game.sim.world().wayfinding().fields_revision() == before
        && std::time::Instant::now() < deadline
    {
        game.sim.step();
        let _ = game.sim.take_events(usize::MAX);
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    assert_ne!(
        game.sim.world().wayfinding().fields_revision(),
        before,
        "the new flow fields were adopted"
    );
}

fn scratch(name: &str) -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "openairside-build-{}-{name}-{n}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn load_with(user: &Path, active: &[&str]) -> Arc<Catalog> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../content/base");
    let folders = datapack::discover(Some(&base), Some(user));
    let active: Vec<String> = active.iter().map(|s| (*s).to_owned()).collect();
    let outcome = datapack::load(&folders, &active, &game_version());
    for report in &outcome.reports {
        assert!(report.errors.is_empty(), "{:?}", report.errors);
    }
    Arc::new(outcome.catalog)
}

const LIMITS: Limits = Limits { map_edges: &[EDGE] };

fn save(game: &Game) -> Vec<u8> {
    let request = SaveRequest {
        name: "Test".into(),
        kind: SaveKind::Manual,
        saved_at: "2026-10-25T20:00:00Z".into(),
        preview: Vec::new(),
    };
    game.sim
        .save_snapshot()
        .encode(&request, "", "0.0.0")
        .unwrap()
}

fn furnished() -> Game {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 0, 30, 30)));
    game.accepted(area(WALL_TYPE, rect(15, 0, 16, 30)));
    game.accepted(bench(10, 10, 0));
    game.accepted(bench(20, 20, 1));
    game.accepted(demolish(rect(20, 20, 21, 21)));
    game.accepted(bench(5, 5, 2));
    game
}

#[test]
fn the_buildings_survive_a_save_game_and_go_on_identically() {
    let mut game = furnished();
    let before = game.sim.state_hash();
    let bytes = save(&game);
    let mut loaded = Game::new();
    loaded
        .sim
        .load_game(decode(&bytes, &LIMITS).unwrap())
        .unwrap();
    assert_eq!(loaded.sim.state_hash(), before);
    let (a, b) = (game.sim.world().build(), loaded.sim.world().build());
    assert_eq!(a.objects(), b.objects());
    assert_eq!(a.types(), b.types());
    assert!(
        b.object_at(CellPos {
            x: 10,
            y: 10,
            floor: 0
        })
        .is_some()
    );
    assert!(!loaded.walkable(10, 10) && loaded.walkable(12, 10));
    // Both go on: new handles continue the count, demolishing finds the objects.
    for g in [&mut game, &mut loaded] {
        g.accepted(bench(2, 20, 0));
        g.accepted(demolish(rect(10, 10, 11, 11)));
    }
    // Only the module "Welt & Bau" is compared: the timing of the background
    // recomputation of the flow fields belongs to the whole state hash and
    // differs between two running simulations.
    assert_eq!(game.build_hash(), loaded.build_hash());
    assert_eq!(
        game.sim.world().build().objects(),
        loaded.sim.world().build().objects()
    );
}

#[test]
fn a_save_game_with_a_type_the_content_lacks_is_refused() {
    let with_fern = scratch("with");
    let without_fern = scratch("without");
    for (dir, objects) in [
        (
            &with_fern,
            r#"{"objects":[{"id":"fern","name":"object.fern","placement":{"kind":"single","width":2,"depth":2}}]}"#,
        ),
        (&without_fern, r#"{"objects":[]}"#),
    ] {
        fs::create_dir_all(dir.join("plants/objects")).unwrap();
        fs::write(
            dir.join("plants/manifest.json"),
            r#"{"id":"plants","name":"Plants","version":"1.0.0","format":1,"game":">=0.0.0",
                "dependencies":[{"id":"base","version":"*"}]}"#,
        )
        .unwrap();
        fs::write(dir.join("plants/objects/plant.json"), objects).unwrap();
    }
    let mut game = Game::with(load_with(&with_fern, &["plants"]));
    game.accepted(area(FLOOR, rect(0, 0, 10, 10)));
    game.accepted(Command::Build {
        object: "plants:fern".into(),
        at: CellPos {
            x: 3,
            y: 3,
            floor: 0,
        },
        rotation: Rotation(0),
    });
    let bytes = save(&game);

    let mut loaded = Game::with(load_with(&without_fern, &["plants"]));
    let before = loaded.sim.state_hash();
    let error = loaded
        .sim
        .load_game(decode(&bytes, &LIMITS).unwrap())
        .expect_err("the type is gone");
    assert_eq!(error.code, "invalidState");
    assert_eq!(
        error.params.get("check"),
        Some(&ParamValue::Text("objectType".into()))
    );
    assert_eq!(loaded.sim.state_hash(), before, "the session is unchanged");
    // The same content loads.
    let mut again = Game::with(load_with(&with_fern, &["plants"]));
    again
        .sim
        .load_game(decode(&bytes, &LIMITS).unwrap())
        .unwrap();
    assert_eq!(again.sim.state_hash(), game.sim.state_hash());
    for dir in [&with_fern, &without_fern] {
        let _ = fs::remove_dir_all(dir);
    }
}
