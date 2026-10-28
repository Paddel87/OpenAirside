//! Flow field benchmark "terminal 768² × 5 in a world 2048²" (roadmap step
//! 2.3, FR-017, ADR-020), since step 3.1c as a construction site (ADR-045):
//! terrain on the ground floor outside the terminal, one delivery point and
//! open floor orders, so that every recomputation also computes the set of
//! the construction workers.
//!
//! Measures (1) the full recomputation of all flow fields for 64 targets and
//! the set of the construction workers on all cores (median of 5 runs) with
//! memory, the set of the construction workers alone for information, and
//! (2) the
//! simulation step with 10,000 persons at 3× in real-time pace while
//! recomputations run on those threads and are adopted. The defaults split
//! 4 cores into 2 simulation and 2 background threads: more threads than
//! cores make the simulation wait for the operating system.
//! Fails with exit code 1 if the recomputation takes longer than 4 s
//! (values of spike 1.5 and measurement 2.8: 1.6–1.7 s with 4 threads on
//! 4 vCPU) or the step time exceeds 4 ms at the 99th percentile (FR-017).
//!
//! Usage: `cargo bench -p openairside-core --bench flowfield -- [--output FILE]`
//! (cargo runs the program in `core/`; relative paths start there).
//! Environment (optional): `OPENAIRSIDE_BENCH_MAX_RECOMPUTE_MS`,
//! `OPENAIRSIDE_BENCH_MAX_P99_MS`, `OPENAIRSIDE_BENCH_THREADS`,
//! `OPENAIRSIDE_BENCH_BACKGROUND_THREADS`.

use std::process::ExitCode;
use std::time::Instant;

use openairside_core::adoption::AdoptionMode;
use openairside_core::command::Command;
use openairside_core::event::Event;
use openairside_core::geometry::MapSize;
use openairside_core::scenario::{add_construction_site, reference_terminal, target_count};
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::Speed;
use openairside_core::wayfinding::Fields;
use openairside_core::wayfinding::flowfield::FieldSet;

const PERSONS: u32 = 10_000;
const TERMINAL_EDGE: u16 = 768;
/// 45 gates and 19 other targets make 64 targets.
const GATES: u8 = 45;
const DEFAULT_SIM_THREADS: usize = 2;
const DEFAULT_BACKGROUND_THREADS: usize = 2;
const RUNS: usize = 5;
const WARM_UP: usize = 100;
const MEASURED: usize = 2_000;
const REQUEST_EVERY: usize = 1_000;
/// Real time per step at 3×: 1 s / 180.
const STEP_PERIOD: std::time::Duration = std::time::Duration::from_nanos(5_555_556);
const DEFAULT_MAX_RECOMPUTE_MS: f64 = 4_000.0;
const DEFAULT_MAX_P99_MS: f64 = 4.0;

fn env_threads(name: &str, default: usize) -> Result<usize, String> {
    match std::env::var(name) {
        Ok(value) => value
            .parse()
            .map_err(|_| format!("{name}: not a number: {value:?}")),
        Err(_) => Ok(default),
    }
}

fn env_number(name: &str, default: f64) -> Result<f64, String> {
    match std::env::var(name) {
        Ok(value) => value
            .parse()
            .map_err(|_| format!("{name}: not a number: {value:?}")),
        Err(_) => Ok(default),
    }
}

fn output_path() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--output" {
            return args.next();
        }
    }
    None
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    let rank = (q * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

fn run() -> Result<bool, String> {
    let sim_threads = env_threads("OPENAIRSIDE_BENCH_THREADS", DEFAULT_SIM_THREADS)?;
    let background_threads = env_threads(
        "OPENAIRSIDE_BENCH_BACKGROUND_THREADS",
        DEFAULT_BACKGROUND_THREADS,
    )?;
    let max_recompute = env_number(
        "OPENAIRSIDE_BENCH_MAX_RECOMPUTE_MS",
        DEFAULT_MAX_RECOMPUTE_MS,
    )?;
    let max_p99 = env_number("OPENAIRSIDE_BENCH_MAX_P99_MS", DEFAULT_MAX_P99_MS)?;
    let map = MapSize::new(2048, 2048).ok_or("invalid map size")?;
    let mut world = reference_terminal(map, TERMINAL_EDGE, GATES, PERSONS, 2026)
        .ok_or("the terminal does not fit on the map")?;
    if world.build().targets().len() != target_count(usize::from(GATES)) {
        return Err("unexpected number of targets".into());
    }
    add_construction_site(&mut world, TERMINAL_EDGE);
    if world.build().delivery_points().len() != 1 || world.build().orders().is_empty() {
        return Err("the construction site is missing".into());
    }

    // (1) Full recomputation on all cores, as measured in spike 1.5.
    let cpus = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let input = openairside_core::wayfinding::Wayfinding::input(world.build());
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cpus)
        .build()
        .map_err(|err| err.to_string())?;
    let mut times = Vec::new();
    let mut bytes = 0;
    let mut cells = 0;
    let mut worker_cells = 0;
    for run in 0..RUNS {
        let start = Instant::now();
        let fields = pool.install(|| Fields::compute(&input, 1));
        times.push(start.elapsed().as_secs_f64() * 1e3);
        if run == 0 {
            let (set, workers) = (fields.main(), fields.workers());
            bytes = set.bytes() + set.cells() * 3 + workers.bytes() + workers.cells() * 3;
            cells = set.cells();
            worker_cells = workers.cells();
        }
    }
    times.sort_by(f64::total_cmp);
    let recompute = times[times.len() / 2];
    let start = Instant::now();
    let workers = pool.install(|| FieldSet::compute(&input.workers, 1));
    let worker_ms = start.elapsed().as_secs_f64() * 1e3;
    let worker_targets = workers.targets();
    drop(workers);

    // (2) Simulation step while recomputations run in the background.
    let mut sim = Simulation::with_world(
        SimulationConfig {
            map,
            threads: sim_threads,
            background_threads,
            adoption: AdoptionMode::WhenReady,
            persons: PERSONS,
            seed: 2026,
            start_tick: 0,
        },
        world,
    )
    .map_err(|err| err.to_string())?;
    sim.submit(sim.session(), 1, Command::SetSpeed { speed: Speed::X3 });
    let mut samples = Vec::with_capacity(MEASURED);
    let mut adopted = 0u32;
    let mut adoption_ms: f64 = 0.0;
    let origin = Instant::now();
    for i in 0..WARM_UP + MEASURED {
        // Real-time pace of 3×: the background computation overlaps with the
        // steps as in a running game.
        let due = origin + STEP_PERIOD * u32::try_from(i).unwrap_or(u32::MAX);
        if let Some(wait) = due.checked_duration_since(Instant::now()) {
            std::thread::sleep(wait);
        }
        if i >= WARM_UP && (i - WARM_UP).is_multiple_of(REQUEST_EVERY) {
            sim.request_recompute();
        }
        let start = Instant::now();
        sim.step();
        let elapsed = start.elapsed();
        for record in sim.take_events(usize::MAX) {
            sim.ack(sim.session(), record.seq);
            if matches!(record.event, Event::WaypointsAdopted { .. }) {
                adopted += 1;
                adoption_ms = adoption_ms.max(elapsed.as_secs_f64() * 1e3);
            }
        }
        if i >= WARM_UP {
            samples.push(elapsed.as_secs_f64() * 1e3);
        }
    }
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    samples.sort_by(f64::total_cmp);
    let p50 = quantile(&samples, 0.50);
    let p99 = quantile(&samples, 0.99);
    let max = samples.last().copied().unwrap_or(0.0);
    let passed = recompute <= max_recompute && p99 <= max_p99;

    let json = format!(
        concat!(
            "{{\"benchmark\":\"flowfield\",\"targets\":{},\"persons\":{},\"cells\":{},",
            "\"workerTargets\":{},\"workerCells\":{},\"workerMs\":{:.1},",
            "\"fieldBytes\":{},\"threads\":{},\"backgroundThreads\":{},\"cpus\":{},",
            "\"os\":\"{}\",\"recomputeMs\":{:.1},\"recomputeMinMs\":{:.1},",
            "\"recomputeMaxMs\":{:.1},\"ticks\":{},\"adopted\":{},\"adoptionMaxMs\":{:.4},\"meanMs\":{:.4},",
            "\"p50Ms\":{:.4},\"p99Ms\":{:.4},\"maxMs\":{:.4},",
            "\"limits\":{{\"recomputeMs\":{},\"p99Ms\":{}}},\"passed\":{}}}\n"
        ),
        target_count(usize::from(GATES)),
        PERSONS,
        cells,
        worker_targets,
        worker_cells,
        worker_ms,
        bytes,
        sim_threads,
        background_threads,
        cpus,
        std::env::consts::OS,
        recompute,
        times[0],
        times[times.len() - 1],
        MEASURED,
        adopted,
        adoption_ms,
        mean,
        p50,
        p99,
        max,
        max_recompute,
        max_p99,
        passed
    );
    if let Some(path) = output_path() {
        std::fs::write(&path, &json).map_err(|err| format!("{path}: {err}"))?;
    }
    println!(
        "flow fields, {} targets, {cells} cells, workers {worker_targets} fields on \
         {worker_cells} cells ({worker_ms:.0} ms alone), {:.0} MB: recompute median \
         {recompute:.0} ms (limit {max_recompute}); step with {PERSONS} persons during recompute: \
         mean {mean:.3} ms, p50 {p50:.3} ms, p99 {p99:.3} ms (limit {max_p99}), max {max:.3} ms, \
         {adopted} adoptions (slowest step {adoption_ms:.3} ms)",
        target_count(usize::from(GATES)),
        bytes as f64 / 1e6,
    );
    print!("{json}");
    Ok(passed)
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => {
            eprintln!("benchmark limit exceeded (step 2.3, FR-017)");
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("benchmark failed: {err}");
            ExitCode::FAILURE
        }
    }
}
