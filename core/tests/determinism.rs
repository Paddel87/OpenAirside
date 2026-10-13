//! Verification mode of ADR-010: with a fixed adoption tick the simulation
//! is bit-identical across thread counts, including background recomputes.

use openairside_core::adoption::AdoptionMode;
use openairside_core::command::Command;
use openairside_core::event::Event;
use openairside_core::geometry::MapSize;
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::Speed;

fn run(threads: usize) -> (u64, Vec<(u32, u32)>) {
    let mut sim = Simulation::new(SimulationConfig {
        map: MapSize::new(512, 512).unwrap(),
        threads,
        adoption: AdoptionMode::FixedTick { delay: 20 },
        persons: 10_000,
        seed: 2026,
        start_tick: 0,
    })
    .unwrap();
    sim.submit(1, 1, Command::SetSpeed { speed: Speed::X3 });
    let mut adopted = Vec::new();
    for _ in 0..400 {
        if matches!(sim.tick(), 50 | 60 | 200) {
            sim.request_recompute();
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
    let (hash, adopted) = run(1);
    // The request at tick 60 replaces the one at 50; results arrive exactly
    // 20 ticks after their request (reported with the tick of that step).
    assert_eq!(adopted, vec![(2, 81), (3, 221)]);
    for threads in [2, 4, 7] {
        assert_eq!(run(threads), (hash, adopted.clone()), "{threads} threads");
    }
}
