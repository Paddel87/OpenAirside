//! Core benchmark "tick with 10,000 persons" (roadmap step 2.2, ADR-022).
//!
//! Runs headless: 200 warm-up and 2,000 measured simulation steps at 3× on a
//! 2048² map with 4 worker threads. Fails with exit code 1 if the mean step
//! time exceeds 1 ms or the 99th percentile exceeds 4 ms (3× leaves 5.5 ms
//! per step). Writes the result as JSON for the trend display in the CI.
//!
//! Usage: `cargo bench -p openairside-core --bench tick -- [--output FILE]`
//! Environment (optional, e.g. for the forced failure of the gate):
//! `OPENAIRSIDE_BENCH_MAX_MEAN_MS`, `OPENAIRSIDE_BENCH_MAX_P99_MS`,
//! `OPENAIRSIDE_BENCH_THREADS`.

use std::process::ExitCode;
use std::time::Instant;

use openairside_core::adoption::AdoptionMode;
use openairside_core::command::Command;
use openairside_core::geometry::MapSize;
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::Speed;

const PERSONS: u32 = 10_000;
const WARM_UP: usize = 200;
const MEASURED: usize = 2_000;
const DEFAULT_THREADS: usize = 4;
const DEFAULT_MAX_MEAN_MS: f64 = 1.0;
const DEFAULT_MAX_P99_MS: f64 = 4.0;

fn env_number<T: std::str::FromStr>(name: &str, default: T) -> Result<T, String> {
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

/// Value at quantile `q` of sorted samples (nearest rank).
fn quantile(sorted: &[f64], q: f64) -> f64 {
    let rank = (q * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

fn run() -> Result<bool, String> {
    let threads = env_number("OPENAIRSIDE_BENCH_THREADS", DEFAULT_THREADS)?;
    let max_mean = env_number("OPENAIRSIDE_BENCH_MAX_MEAN_MS", DEFAULT_MAX_MEAN_MS)?;
    let max_p99 = env_number("OPENAIRSIDE_BENCH_MAX_P99_MS", DEFAULT_MAX_P99_MS)?;
    let map = MapSize::new(2048, 2048).ok_or("invalid map size")?;
    let mut sim = Simulation::new(SimulationConfig {
        map,
        threads,
        adoption: AdoptionMode::WhenReady,
        persons: PERSONS,
        seed: 2026,
        start_tick: 0,
    })
    .map_err(|err| err.to_string())?;
    sim.submit(sim.session(), 1, Command::SetSpeed { speed: Speed::X3 });

    let mut samples = Vec::with_capacity(MEASURED);
    for i in 0..WARM_UP + MEASURED {
        let start = Instant::now();
        sim.step();
        let elapsed = start.elapsed();
        for record in sim.take_events(usize::MAX) {
            sim.ack(sim.session(), record.seq);
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
    let passed = mean <= max_mean && p99 <= max_p99;

    let cpus = std::thread::available_parallelism().map_or(0, std::num::NonZero::get);
    let json = format!(
        concat!(
            "{{\"benchmark\":\"tick\",\"persons\":{},\"threads\":{},\"cpus\":{},",
            "\"os\":\"{}\",\"ticks\":{},\"meanMs\":{:.4},\"p50Ms\":{:.4},",
            "\"p99Ms\":{:.4},\"maxMs\":{:.4},\"limits\":{{\"meanMs\":{},\"p99Ms\":{}}},",
            "\"finalTick\":{},\"passed\":{}}}\n"
        ),
        PERSONS,
        threads,
        cpus,
        std::env::consts::OS,
        MEASURED,
        mean,
        p50,
        p99,
        max,
        max_mean,
        max_p99,
        sim.tick(),
        passed
    );
    if let Some(path) = output_path() {
        std::fs::write(&path, &json).map_err(|err| format!("{path}: {err}"))?;
    }
    println!(
        "tick with {PERSONS} persons, {threads} threads, {MEASURED} steps: \
         mean {mean:.3} ms (limit {max_mean}), p50 {p50:.3} ms, \
         p99 {p99:.3} ms (limit {max_p99}), max {max:.3} ms"
    );
    print!("{json}");
    Ok(passed)
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => {
            eprintln!("benchmark limit exceeded (ADR-022)");
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("benchmark failed: {err}");
            ExitCode::FAILURE
        }
    }
}
