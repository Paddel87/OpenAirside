//! Verification mode of ADR-010: with a fixed adoption tick the simulation
//! is bit-identical across thread counts, including background recomputes of
//! the flow fields after build actions.

use openairside_core::adoption::AdoptionMode;
use openairside_core::build::grid::Cell;
use openairside_core::command::Command;
use openairside_core::event::Event;
use openairside_core::geometry::{CellPos, MapSize};
use openairside_core::scenario::reference_terminal;
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::Speed;

/// Terminal of 256 × 256 cells with 3,000 persons; build actions at ticks
/// 60 and 200 close and open doors and trigger a recomputation each.
fn run(threads: usize, background_threads: usize) -> (u64, Vec<(u32, u32)>) {
    let map = MapSize::new(256, 256).unwrap();
    let world = reference_terminal(map, 256, 17, 3_000, 2026).unwrap();
    let mut sim = Simulation::with_world(
        SimulationConfig {
            map,
            threads,
            background_threads,
            adoption: AdoptionMode::FixedTick { delay: 20 },
            persons: 0,
            seed: 0,
            start_tick: 0,
        },
        world,
    )
    .unwrap();
    sim.submit(1, 1, Command::SetSpeed { speed: Speed::X3 });
    let door = |x: u16| CellPos {
        x,
        y: 128 + 30,
        floor: 0,
    };
    let mut adopted = Vec::new();
    for _ in 0..400 {
        match sim.tick() {
            60 => {
                for x in 30..34 {
                    sim.world_mut()
                        .build_mut()
                        .grid_mut()
                        .set(door(x), Cell::VOID)
                        .unwrap();
                }
                sim.request_recompute();
            }
            200 => {
                for x in 30..34 {
                    sim.world_mut()
                        .build_mut()
                        .grid_mut()
                        .set(
                            door(x),
                            Cell::floor(openairside_core::build::grid::Zone::Landside),
                        )
                        .unwrap();
                }
                sim.request_recompute();
            }
            _ => {}
        }
        sim.step();
        for record in sim.take_events(usize::MAX) {
            sim.ack(sim.session(), record.seq);
            if let Event::WaypointsAdopted { revision } = record.event {
                adopted.push((revision, record.tick));
            }
        }
    }
    assert_eq!(sim.tick(), 400);
    (sim.state_hash(), adopted)
}

#[test]
fn verification_mode_is_bit_identical_over_thread_counts() {
    let (hash, adopted) = run(1, 1);
    // Results arrive exactly 20 ticks after their request (reported with
    // the tick of that step), whatever the speed of the background threads.
    assert_eq!(adopted, vec![(1, 81), (2, 221)]);
    for (threads, background) in [(3, 2), (7, 5)] {
        assert_eq!(
            run(threads, background),
            (hash, adopted.clone()),
            "{threads} simulation and {background} background threads"
        );
    }
}
