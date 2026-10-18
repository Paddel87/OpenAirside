//! Command channel S-2 and event stream S-3 (ADR-021): exactly one answer
//! per command, sessions, loading state and sequence numbers; content ids
//! checked against the loaded data packs (S-4, step 2.4).

use std::path::Path;
use std::sync::Arc;

use openairside_core::adoption::AdoptionMode;
use openairside_core::command::{Command, MAX_COMMANDS_PER_STEP, code};
use openairside_core::datapack::{self, Catalog, game_version};
use openairside_core::event::Event;
use openairside_core::geometry::{CellPos, CellRect, MapSize, Rotation};
use openairside_core::simulation::{CoreState, Simulation, SimulationConfig};
use openairside_core::time::Speed;

fn simulation() -> Simulation {
    Simulation::new(SimulationConfig {
        map: MapSize::new(64, 64).unwrap(),
        threads: 1,
        background_threads: 1,
        adoption: AdoptionMode::WhenReady,
        persons: 0,
        seed: 0,
        start_tick: 0,
    })
    .unwrap()
}

fn demolish() -> Command {
    Command::Demolish {
        area: CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: 2,
            y1: 2,
        },
    }
}

fn answers(sim: &mut Simulation) -> Vec<(u32, Option<String>)> {
    let records = sim.take_events(usize::MAX);
    if let Some(last) = records.last() {
        sim.ack(sim.session(), last.seq);
    }
    records
        .into_iter()
        .filter_map(|r| match r.event {
            Event::CommandAccepted { id, .. } => Some((id, None)),
            Event::CommandRejected { id, reason, .. } => Some((id, Some(reason.code))),
            _ => None,
        })
        .collect()
}

#[test]
fn every_command_gets_exactly_one_answer_in_order() {
    let mut sim = simulation();
    sim.submit(1, 1, demolish());
    sim.submit(
        1,
        2,
        Command::Build {
            object: "base:bench".into(),
            at: CellPos {
                x: 99,
                y: 0,
                floor: 0,
            },
            rotation: Rotation(0),
        },
    );
    sim.submit(
        1,
        3,
        Command::BuildArea {
            object: "base:floor".into(),
            area: CellRect {
                floor: 0,
                x0: 0,
                y0: 0,
                x1: 3,
                y1: 3,
            },
        },
    );
    sim.submit(1, 4, Command::SetSpeed { speed: Speed::X1 });
    sim.step();
    assert_eq!(
        answers(&mut sim),
        vec![
            (1, None),
            (2, Some(code::OUT_OF_MAP.to_owned())),
            (3, Some(code::UNKNOWN_CONTENT.to_owned())),
            (4, None),
        ]
    );
}

#[test]
fn stale_ids_and_foreign_sessions_are_dropped_and_counted() {
    let mut sim = simulation();
    sim.submit(1, 5, demolish());
    sim.submit(1, 5, demolish());
    sim.submit(1, 3, demolish());
    sim.submit(7, 6, demolish());
    sim.ack(7, 1);
    sim.step();
    assert_eq!(answers(&mut sim), vec![(5, None)]);
    let counters = sim.counters();
    assert_eq!(counters.stale_commands, 2);
    assert_eq!(counters.foreign_session, 2);
}

#[test]
fn at_most_4096_commands_are_taken_per_step_and_none_is_lost() {
    let mut sim = simulation();
    let total = u32::try_from(MAX_COMMANDS_PER_STEP).unwrap() + 10;
    for id in 1..=total {
        sim.submit(1, id, demolish());
    }
    sim.step();
    assert_eq!(answers(&mut sim).len(), MAX_COMMANDS_PER_STEP);
    assert!(sim.has_pending_commands());
    sim.step();
    let rest = answers(&mut sim);
    assert_eq!(rest.first().map(|a| a.0), Some(total - 9));
    assert_eq!(rest.len(), 10);
}

#[test]
fn commands_during_loading_are_rejected_with_not_ready() {
    let mut sim = simulation();
    sim.begin_loading();
    assert_eq!(sim.state(), CoreState::Loading);
    sim.submit(1, 1, Command::SetSpeed { speed: Speed::X2 });
    sim.step();
    assert_eq!(
        answers(&mut sim),
        vec![(1, Some(code::NOT_READY.to_owned()))]
    );
    assert_eq!(sim.speed(), Speed::Paused);
    sim.end_loading();
    sim.submit(1, 2, Command::SetSpeed { speed: Speed::X2 });
    sim.step();
    assert_eq!(answers(&mut sim), vec![(2, None)]);
}

#[test]
fn a_new_session_restarts_ids_sequence_numbers_and_time() {
    let mut sim = simulation();
    sim.submit(1, 1, Command::SetSpeed { speed: Speed::X3 });
    sim.step();
    sim.step();
    sim.submit(1, 2, demolish()); // never processed
    sim.restart();
    assert_eq!(sim.session(), 2);
    assert_eq!((sim.tick(), sim.speed()), (0, Speed::Paused));
    sim.submit(1, 3, demolish());
    sim.submit(2, 1, demolish());
    sim.step();
    let records = sim.take_events(usize::MAX);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].seq, 1);
    assert_eq!(records[0].event, Event::CommandAccepted { id: 1, tick: 0 });
}

#[test]
fn acknowledging_unsent_events_is_counted() {
    let mut sim = simulation();
    sim.ack(1, 3);
    assert_eq!(sim.invalid_acks(), 1);
}

fn base_content() -> Arc<Catalog> {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/base");
    let folders = datapack::discover(Some(&base), None);
    let outcome = datapack::load(&folders, &[], &game_version());
    assert!(!outcome.catalog.base_failed());
    Arc::new(outcome.catalog)
}

fn build(object: &str) -> Command {
    Command::Build {
        object: object.into(),
        at: CellPos {
            x: 1,
            y: 1,
            floor: 0,
        },
        rotation: Rotation(0),
    }
}

fn build_area(object: &str) -> Command {
    Command::BuildArea {
        object: object.into(),
        area: CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: 3,
            y1: 3,
        },
    }
}

#[test]
fn build_commands_accept_only_ids_of_loaded_packs() {
    let mut sim = simulation().with_content(base_content());
    sim.submit(1, 1, build("base:bench"));
    sim.submit(1, 2, build_area("base:floor_concrete"));
    sim.submit(1, 3, build("base:missing"));
    sim.submit(1, 4, build_area("other:floor_concrete"));
    sim.step();
    assert_eq!(
        answers(&mut sim),
        vec![
            (1, None),
            (2, None),
            (3, Some(code::UNKNOWN_CONTENT.to_owned())),
            (4, Some(code::UNKNOWN_CONTENT.to_owned())),
        ]
    );
}

#[test]
fn new_content_applies_from_the_next_session_on() {
    let mut sim = simulation();
    sim.set_next_content(base_content());
    assert_eq!(sim.content().object_count(), 0);
    sim.submit(1, 1, build("base:bench"));
    sim.step();
    assert_eq!(
        answers(&mut sim),
        vec![(1, Some(code::UNKNOWN_CONTENT.to_owned()))]
    );
    sim.restart();
    assert_eq!(sim.content().object_count(), 3);
    sim.submit(2, 1, build("base:bench"));
    sim.step();
    assert_eq!(answers(&mut sim), vec![(1, None)]);
    // Without new content the next session keeps the current one.
    sim.restart();
    assert_eq!(sim.content().object_count(), 3);
}
