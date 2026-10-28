//! Construction orders and construction workers (ADR-045, step 3.1c):
//! `build` and `buildArea` place orders that change nothing about who may
//! walk where; the construction workers of a delivery point carry material
//! to the sites and work there until the order takes effect; `demolish`
//! cancels orders and takes a delivery point away with its workers; all of
//! it survives a save game bit for bit.

use std::path::PathBuf;
use std::sync::Arc;

use openairside_core::adoption::AdoptionMode;
use openairside_core::build::grid::{AreaKind, Cell, WALL};
use openairside_core::build::orders::{MAX_DELIVERY_POINTS, OrderState, TICKS_PER_MINUTE};
use openairside_core::command::{Command, code};
use openairside_core::datapack::{self, Catalog, game_version};
use openairside_core::event::Event;
use openairside_core::geometry::{CellPos, CellRect, MapSize, Rotation};
use openairside_core::persons::workers::{GROUP_WORKER, LOAD, WorkerState};
use openairside_core::savegame::{Limits, SaveKind, SaveRequest, decode};
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::text::ParamValue;
use openairside_core::time::Speed;
use openairside_core::world::World;

const EDGE: u16 = 64;
const FLOOR: &str = "base:floor_concrete";
const WALL_TYPE: &str = "base:wall_concrete";
const BENCH: &str = "base:bench";
const DELIVERY: &str = "base:delivery_point";
const LIMITS: Limits = Limits { map_edges: &[EDGE] };

fn base_content() -> Arc<Catalog> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../content/base");
    let folders = datapack::discover(Some(&base), None);
    Arc::new(datapack::load(&folders, &[], &game_version()).catalog)
}

fn config(adoption: AdoptionMode) -> SimulationConfig {
    SimulationConfig {
        map: MapSize::new(EDGE, EDGE).unwrap(),
        threads: 1,
        background_threads: 1,
        adoption,
        persons: 0,
        seed: 0,
        start_tick: 0,
    }
}

/// A new game (terrain on the ground floor) with the base pack.
struct Game {
    sim: Simulation,
    next_id: u32,
}

impl Game {
    fn new() -> Self {
        Self::with_adoption(AdoptionMode::WhenReady)
    }

    fn with_adoption(adoption: AdoptionMode) -> Self {
        let map = MapSize::new(EDGE, EDGE).unwrap();
        Self {
            sim: Simulation::with_world(config(adoption), World::new_game(map))
                .unwrap()
                .with_content(base_content()),
            next_id: 1,
        }
    }

    /// Runs one command; `None` if accepted, else the rejection code and
    /// its parameters.
    fn run(&mut self, command: Command) -> Option<(String, Option<ParamValue>)> {
        let id = self.next_id;
        self.next_id += 1;
        self.sim.submit(self.sim.session(), id, command);
        self.sim.step();
        let mut answer = None;
        for record in self.sim.take_events(usize::MAX) {
            self.sim.ack(self.sim.session(), record.seq);
            if let Event::CommandRejected {
                id: got, reason, ..
            } = record.event
            {
                assert_eq!(got, id);
                answer = Some((reason.code, reason.params.get("max").cloned()));
            }
        }
        answer
    }

    fn accepted(&mut self, command: Command) {
        assert_eq!(self.run(command), None);
    }

    fn refused(&mut self, command: Command) -> String {
        self.run(command).expect("the command must be refused").0
    }

    /// Lets the game run at 3× for `ticks` steps.
    fn play(&mut self, ticks: u32) {
        if self.sim.speed() != Speed::X3 {
            self.accepted(Command::SetSpeed { speed: Speed::X3 });
        }
        for _ in 0..ticks {
            self.sim.step();
            for record in self.sim.take_events(usize::MAX) {
                self.sim.ack(self.sim.session(), record.seq);
            }
        }
    }

    /// Plays until no order is open; returns the ticks it took.
    fn play_until_built(&mut self, limit: u32) -> u32 {
        for tick in 0..limit {
            if self.sim.world().build().orders().is_empty() {
                return tick;
            }
            self.play(1);
        }
        panic!(
            "{} orders still open after {limit} ticks",
            self.sim.world().build().orders().len()
        );
    }

    fn cell(&self, x: u16, y: u16) -> Cell {
        self.sim.world().build().grid().cell(pos(x, y)).unwrap()
    }

    fn workers(&self) -> Vec<(CellPos, u8, u32)> {
        let persons = self.sim.world().persons();
        (0..persons.workers())
            .map(|i| persons.worker(i).unwrap())
            .collect()
    }
}

fn pos(x: u16, y: u16) -> CellPos {
    CellPos { x, y, floor: 0 }
}

fn inside(r: CellRect, at: CellPos) -> bool {
    at.floor == r.floor && (r.x0..r.x1).contains(&at.x) && (r.y0..r.y1).contains(&at.y)
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

fn single(object: &str, x: u16, y: u16) -> Command {
    Command::Build {
        object: object.into(),
        at: pos(x, y),
        rotation: Rotation(0),
    }
}

/// The delivery point (4 × 4 cells) at the north-west corner.
fn delivery_point() -> Command {
    single(DELIVERY, 1, 1)
}

#[test]
fn a_new_game_is_terrain_on_which_objects_need_a_floor() {
    let mut game = Game::new();
    let cell = game.cell(30, 30);
    assert_eq!(cell, Cell::TERRAIN);
    assert_eq!(cell.kind(), AreaKind::Terrain);
    assert!(cell.walkable() && !cell.has_floor());
    assert_eq!(game.refused(single(BENCH, 30, 30)), code::NEEDS_FLOOR);
    // Persons other than construction workers never start on terrain.
    assert_eq!(game.sim.world_mut().spawn_random(10, 1), 0);
}

#[test]
fn an_order_is_visible_at_once_and_changes_nothing_yet() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(20, 20, 24, 24)));
    let build = game.sim.world().build();
    assert_eq!(build.orders().len(), 1);
    let order = build.orders().as_slice()[0];
    assert_eq!(order.state(), OrderState::Planned);
    assert_eq!(order.rect(), rect(20, 20, 24, 24));
    assert_eq!(order.work, 16 * TICKS_PER_MINUTE, "1 minute per cell");
    assert_eq!(game.cell(21, 21), Cell::TERRAIN, "nothing is built yet");
    let planned = build.planned_cell(pos(21, 21)).expect("planned");
    assert_eq!(planned.state, OrderState::Planned);
    assert_eq!(
        Some(planned.floor_kind),
        build.types().number(FLOOR),
        "the planned covering is shown"
    );
    // An open order occupies its cells.
    assert_eq!(
        game.refused(area(WALL_TYPE, rect(23, 23, 30, 24))),
        code::BLOCKED
    );
    assert_eq!(
        game.refused(single(BENCH, 22, 22)),
        code::BLOCKED,
        "furniture waits for its floor"
    );
    game.accepted(area(WALL_TYPE, rect(24, 20, 25, 24)));
    assert_eq!(
        game.sim.world().build().orders().as_slice()[1].work,
        4 * 3 * TICKS_PER_MINUTE,
        "3 minutes per wall cell"
    );
    // Without a delivery point nobody builds.
    game.play(2_000);
    assert_eq!(game.sim.world().build().orders().len(), 2);
    assert_eq!(game.cell(21, 21), Cell::TERRAIN);
}

#[test]
fn construction_workers_carry_material_and_build_the_order() {
    let mut game = Game::new();
    game.accepted(delivery_point());
    assert_eq!(game.workers().len(), 10, "the delivery point brings 10");
    assert!(
        game.workers()
            .iter()
            .all(|&(at, state, _)| at == pos(1, 1) && state == WorkerState::Carrying as u8)
    );
    // The delivery point stands at once and keeps its cells walkable.
    let build = game.sim.world().build();
    assert_eq!(build.delivery_points().len(), 1);
    assert_eq!(build.objects().len(), 1);
    assert!(game.cell(2, 2).walkable());

    game.accepted(area(FLOOR, rect(30, 30, 34, 34)));
    let mut states = [false; 3];
    let mut started = false;
    let mut ticks = 0;
    while !game.sim.world().build().orders().is_empty() {
        game.play(1);
        ticks += 1;
        assert!(ticks < 20_000, "the floor is never finished");
        for (at, state, _) in game.workers() {
            states[usize::from(state)] = true;
            if state == WorkerState::Working as u8 {
                // Working from beside the footprint, never inside it.
                assert!(!inside(rect(30, 30, 34, 34), at), "{at:?}");
                assert!((29..=34).contains(&at.x) && (29..=34).contains(&at.y));
            }
        }
        if let Some(order) = game.sim.world().build().orders().as_slice().first() {
            started |= order.state() == OrderState::Building;
        }
    }
    assert!(started, "the order was in construction");
    assert_eq!(states, [true; 3], "carrying, working, returning");
    // 16 cells of 1 minute with 10 workers: at least 288 ticks of work.
    assert!(ticks > 16 * TICKS_PER_MINUTE / 10, "{ticks}");
    for (x, y) in [(30, 30), (33, 33)] {
        let cell = game.cell(x, y);
        assert!(cell.has_floor(), "({x}, {y})");
        assert_eq!(cell.kind(), AreaKind::PublicWalk);
    }
    // Without work the workers go back and wait at the delivery point.
    game.play(1_500);
    for (at, state, _) in game.workers() {
        assert!(inside(rect(1, 1, 5, 5), at), "{at:?}");
        assert_eq!(state, WorkerState::Carrying as u8);
    }
}

#[test]
fn a_wall_and_a_bench_take_effect_when_their_work_is_done() {
    let mut game = Game::new();
    game.accepted(delivery_point());
    game.accepted(area(FLOOR, rect(20, 20, 26, 24)));
    game.play_until_built(20_000);
    game.accepted(single(BENCH, 21, 21));
    game.accepted(area(WALL_TYPE, rect(20, 26, 26, 27)));
    let build = game.sim.world().build();
    assert_eq!(build.objects().len(), 1, "only the delivery point stands");
    assert_eq!(build.orders().len(), 2);
    game.play_until_built(40_000);
    let build = game.sim.world().build();
    assert_eq!(build.objects().len(), 2, "the bench stands");
    assert!(build.object_at(pos(22, 21)).is_some());
    assert!(!game.cell(21, 21).walkable(), "the bench blocks");
    let wall = game.cell(23, 26);
    assert!(wall.flags & WALL != 0 && wall.wall_kind != 0);
    // No worker stands in the finished wall or bench.
    game.play(10);
    for (at, ..) in game.workers() {
        assert!(game.cell(at.x, at.y).walkable(), "{at:?}");
    }
}

#[test]
fn demolishing_cancels_orders_and_workers_choose_again() {
    let mut game = Game::new();
    game.accepted(delivery_point());
    game.accepted(area(WALL_TYPE, rect(40, 40, 48, 41)));
    // Play until work has arrived at the wall.
    for _ in 0..20_000 {
        game.play(1);
        let orders = game.sim.world().build().orders();
        if orders.as_slice()[0].state() == OrderState::Building {
            break;
        }
    }
    assert!(
        game.workers()
            .iter()
            .any(|w| w.1 == WorkerState::Working as u8)
    );
    assert_eq!(
        game.sim.world().build().orders().as_slice()[0].state(),
        OrderState::Building
    );
    game.accepted(Command::Demolish {
        area: rect(41, 40, 42, 41),
    });
    assert!(game.sim.world().build().orders().is_empty(), "cancelled");
    assert_eq!(game.cell(44, 40), Cell::TERRAIN, "nothing was built");
    game.play(2);
    assert!(
        game.workers()
            .iter()
            .all(|w| w.1 != WorkerState::Working as u8),
        "nobody works on a cancelled order"
    );
    // A new order is built by the same workers.
    game.accepted(area(FLOOR, rect(10, 30, 12, 32)));
    game.play_until_built(20_000);
    assert!(game.cell(10, 30).has_floor());
}

#[test]
fn demolishing_a_delivery_point_takes_its_workers_away() {
    let mut game = Game::new();
    game.accepted(delivery_point());
    game.accepted(single(DELIVERY, 50, 50));
    assert_eq!(game.workers().len(), 20);
    let second = game.sim.world().build().delivery_points().as_slice()[1].handle;
    game.accepted(Command::Demolish {
        area: rect(1, 1, 2, 2),
    });
    let workers = game.workers();
    assert_eq!(workers.len(), 10);
    assert!(workers.iter().all(|w| w.2 == second));
    assert_eq!(game.sim.world().build().delivery_points().len(), 1);
    assert_eq!(game.cell(1, 1), Cell::TERRAIN);
    assert!(
        game.cell(4, 4).walkable(),
        "outside the frame: terrain stays"
    );
}

#[test]
fn at_most_sixteen_delivery_points_and_not_on_orders() {
    let mut game = Game::new();
    game.accepted(area(FLOOR, rect(0, 60, 4, 64)));
    assert_eq!(game.refused(single(DELIVERY, 0, 60)), code::BLOCKED);
    for i in 0..MAX_DELIVERY_POINTS as u16 {
        game.accepted(single(DELIVERY, (i % 8) * 6, (i / 8) * 6));
        if i == 0 {
            // Delivery points do not stand on each other.
            assert_eq!(game.refused(single(DELIVERY, 2, 2)), code::BLOCKED);
        }
    }
    let (reason, max) = game.run(single(DELIVERY, 0, 40)).expect("refused");
    assert_eq!(reason, code::LIMIT_EXCEEDED);
    assert_eq!(max, Some(ParamValue::Number(16.0)));
    assert_eq!(game.workers().len(), 160);
}

#[test]
fn orders_and_workers_survive_a_save_game_bit_for_bit() {
    let mode = AdoptionMode::FixedTick { delay: 5 };
    let mut game = Game::with_adoption(mode);
    game.accepted(delivery_point());
    game.accepted(area(FLOOR, rect(30, 30, 36, 36)));
    game.accepted(single(DELIVERY, 50, 4));
    game.play(400);
    let build = game.sim.world().build();
    assert_eq!(build.orders().len(), 1);
    assert!(
        game.workers()
            .iter()
            .any(|w| w.1 != WorkerState::Carrying as u8),
        "workers are on their way"
    );
    let before = game.sim.state_hash();
    let request = SaveRequest {
        name: "Baustelle".into(),
        kind: SaveKind::Manual,
        saved_at: "2026-10-28T20:00:00Z".into(),
        preview: Vec::new(),
    };
    let bytes = game
        .sim
        .save_snapshot()
        .encode(&request, "", "0.0.0")
        .unwrap();
    let mut loaded = Game::with_adoption(mode);
    loaded
        .sim
        .load_game(decode(&bytes, &LIMITS).unwrap())
        .unwrap();
    assert_eq!(loaded.sim.state_hash(), before);
    assert_eq!(loaded.workers(), game.workers());
    // Both continue identically until the floor is built.
    game.play(3_000);
    loaded.play(3_000);
    assert_eq!(loaded.sim.state_hash(), game.sim.state_hash());
    assert!(game.sim.world().build().orders().is_empty(), "built");
    assert!(loaded.cell(33, 33).has_floor());
}

#[test]
fn construction_workers_appear_in_the_frame_with_group_and_state() {
    let mut game = Game::new();
    game.accepted(delivery_point());
    let mut seen = 0;
    game.sim.world().persons().visit(|p| {
        assert_eq!((p.group, p.state), (GROUP_WORKER, 0));
        assert!((p.x - 1.5).abs() < 1e-6 && (p.y - 1.5).abs() < 1e-6);
        seen += 1;
    });
    assert_eq!(seen, 10);
    assert_eq!(game.sim.world().persons().per_floor()[2], 10);
    // A load is 15 game minutes of work (ADR-045).
    assert_eq!(LOAD, 15 * TICKS_PER_MINUTE);
}
