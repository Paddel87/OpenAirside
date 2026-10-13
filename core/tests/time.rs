//! Time control FR-003 (META-08, META-09) with the time model of ADR-020:
//! pause, 1×–3×, night skip and calendar through the command channel S-2.

use openairside_core::adoption::AdoptionMode;
use openairside_core::command::{Command, code};
use openairside_core::event::Event;
use openairside_core::geometry::{CellRect, MapSize};
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::{Speed, TICKS_PER_DAY, TICKS_PER_GAME_HOUR, Tick};

const HOUR: Tick = TICKS_PER_GAME_HOUR;

fn simulation_at(start_tick: Tick, persons: u32) -> Simulation {
    Simulation::new(SimulationConfig {
        map: MapSize::new(128, 128).unwrap(),
        threads: 2,
        adoption: AdoptionMode::WhenReady,
        persons,
        seed: 11,
        start_tick,
    })
    .unwrap()
}

/// Sends one command, runs one pass and returns the events of that pass.
fn command(sim: &mut Simulation, id: u32, command: Command) -> Vec<Event> {
    sim.submit(sim.session(), id, command);
    sim.step();
    drain(sim)
}

fn drain(sim: &mut Simulation) -> Vec<Event> {
    let records = sim.take_events(usize::MAX);
    if let Some(last) = records.last() {
        sim.ack(sim.session(), last.seq);
    }
    records.into_iter().map(|r| r.event).collect()
}

fn run_until(sim: &mut Simulation, tick: Tick) -> Vec<Event> {
    let mut events = Vec::new();
    while sim.tick() < tick {
        sim.step();
        events.extend(drain(sim));
    }
    events
}

fn rejection_code(events: &[Event]) -> Option<&str> {
    events.iter().find_map(|e| match e {
        Event::CommandRejected { reason, .. } => Some(reason.code.as_str()),
        _ => None,
    })
}

#[test]
fn a_new_game_starts_paused_and_a_paused_game_does_not_tick() {
    let mut sim = simulation_at(0, 10);
    assert_eq!(sim.speed(), Speed::Paused);
    for _ in 0..10 {
        sim.step();
    }
    assert_eq!(sim.tick(), 0);
}

#[test]
fn speeds_change_on_command_and_are_reported_once() {
    let mut sim = simulation_at(0, 0);
    let events = command(&mut sim, 1, Command::SetSpeed { speed: Speed::X3 });
    assert_eq!(
        events,
        vec![
            Event::CommandAccepted { id: 1, tick: 1 },
            Event::SpeedChanged {
                speed: Speed::X3,
                reason: None
            },
        ],
        "answer first, then the effect; the step already ran at 3×"
    );
    assert_eq!(sim.speed().ticks_per_second(), 180);
    let events = command(&mut sim, 2, Command::SetSpeed { speed: Speed::X3 });
    assert_eq!(events, vec![Event::CommandAccepted { id: 2, tick: 2 }]);
    let events = command(
        &mut sim,
        3,
        Command::SetSpeed {
            speed: Speed::Paused,
        },
    );
    assert_eq!(events[0], Event::CommandAccepted { id: 3, tick: 2 });
    assert_eq!(sim.tick(), 2);
    for speed in [Speed::X1, Speed::X2] {
        let events = command(
            &mut sim,
            4 + speed.ticks_per_second(),
            Command::SetSpeed { speed },
        );
        assert!(events.contains(&Event::SpeedChanged {
            speed,
            reason: None
        }));
    }
}

#[test]
fn commands_are_processed_in_the_pause_without_advancing_time() {
    let mut sim = simulation_at(0, 0);
    let area = CellRect {
        floor: 0,
        x0: 0,
        y0: 0,
        x1: 4,
        y1: 4,
    };
    let events = command(&mut sim, 1, Command::Demolish { area });
    assert_eq!(events, vec![Event::CommandAccepted { id: 1, tick: 0 }]);
    assert_eq!(sim.tick(), 0);
}

#[test]
fn each_game_day_starts_with_an_event_and_twelve_days_make_a_year() {
    let mut sim = simulation_at(TICKS_PER_DAY - 3, 0);
    command(&mut sim, 1, Command::SetSpeed { speed: Speed::X1 });
    let events = run_until(&mut sim, TICKS_PER_DAY + 5);
    assert_eq!(events, vec![Event::DayStarted { day: 2, year: 1 }]);

    let mut sim = simulation_at(12 * TICKS_PER_DAY - 2, 0);
    command(&mut sim, 1, Command::SetSpeed { speed: Speed::X2 });
    let events = run_until(&mut sim, 12 * TICKS_PER_DAY + 1);
    assert_eq!(events, vec![Event::DayStarted { day: 1, year: 2 }]);
}

#[test]
fn night_skip_is_refused_during_the_day() {
    for start in [6 * HOUR, 12 * HOUR, 18 * HOUR - 1] {
        let mut sim = simulation_at(start, 0);
        let events = command(&mut sim, 1, Command::SkipNight {});
        assert_eq!(rejection_code(&events), Some(code::NOT_NIGHT), "at {start}");
        assert_eq!(sim.tick(), start);
    }
}

#[test]
fn night_skip_jumps_from_evening_to_six_in_the_morning() {
    let mut sim = simulation_at(19 * HOUR, 0);
    let events = command(&mut sim, 1, Command::SkipNight {});
    let morning = TICKS_PER_DAY + 6 * HOUR;
    assert_eq!(
        events,
        vec![
            Event::CommandAccepted {
                id: 1,
                tick: morning
            },
            Event::DayStarted { day: 2, year: 1 },
        ]
    );
    assert_eq!(sim.tick(), morning);
}

#[test]
fn night_skip_after_midnight_stays_on_the_same_day() {
    let mut sim = simulation_at(2 * HOUR, 0);
    command(&mut sim, 1, Command::SetSpeed { speed: Speed::X3 });
    let events = command(&mut sim, 2, Command::SkipNight {});
    // Running at 3×, the effect is visible one tick after 06:00.
    assert_eq!(
        events,
        vec![Event::CommandAccepted {
            id: 2,
            tick: 6 * HOUR + 1
        }]
    );
}

#[test]
fn night_skip_is_blocked_while_persons_are_present() {
    let mut sim = simulation_at(20 * HOUR, 5);
    let events = command(&mut sim, 1, Command::SkipNight {});
    assert_eq!(rejection_code(&events), Some(code::NIGHT_SKIP_BLOCKED));
}

#[test]
fn a_restarted_session_begins_at_the_start_tick() {
    let mut sim = simulation_at(7 * HOUR, 0);
    command(&mut sim, 1, Command::SetSpeed { speed: Speed::X3 });
    run_until(&mut sim, 7 * HOUR + 10);
    sim.restart();
    assert_eq!(sim.tick(), 7 * HOUR);
}
