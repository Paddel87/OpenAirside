//! Continuous-building benchmark "wandering wall in the reference terminal"
//! (roadmap step 3.1a, ADR-041, `docs/research/3.1-vorlage.md` section 3).
//!
//! A wall cell is set and removed again in the hall of the reference
//! terminal (768² × 5 in a world 2048², 10,000 persons, 3× in real-time
//! pace), one building action every 0.25 s for 30 s, then the game rests
//! until the flow fields of the last state are adopted. Fails with exit
//! code 1 if
//!
//! - more than 8 s pass between two adoptions of new flow fields during
//!   the building, or from the last building action to the adoption of its
//!   state,
//! - more than 2 recomputations are in flight at the same time (one running,
//!   one waiting), more than 3 flow field sets are alive (in use, in
//!   computation, in release) or more than 2 snapshot copies are held.
//!
//! The peak of the process memory is printed (Linux), but not judged: it
//! depends on the platform. The building is stopped early if more than
//! `OPENAIRSIDE_BENCH_ABORT_JOBS` recomputations are in flight (code
//! without the fix of step 3.1a would fill the memory of a runner).
//!
//! Usage: `cargo bench -p openairside-core --bench continuous_build --
//! [--output FILE]` (cargo runs the program in `core/`; relative paths start
//! there). Environment (optional): `OPENAIRSIDE_BENCH_MAX_GAP_MS`,
//! `OPENAIRSIDE_BENCH_MAX_SETTLE_MS`, `OPENAIRSIDE_BENCH_MAX_JOBS`,
//! `OPENAIRSIDE_BENCH_MAX_FIELD_SETS`, `OPENAIRSIDE_BENCH_MAX_SNAPSHOTS`,
//! `OPENAIRSIDE_BENCH_ABORT_JOBS`, `OPENAIRSIDE_BENCH_BUILD_SECONDS`,
//! `OPENAIRSIDE_BENCH_BUILD_PERIOD_MS`, `OPENAIRSIDE_BENCH_SETTLE_TIMEOUT_MS`,
//! `OPENAIRSIDE_BENCH_THREADS`, `OPENAIRSIDE_BENCH_BACKGROUND_THREADS`.

use std::process::ExitCode;
use std::time::{Duration, Instant};

use openairside_core::adoption::AdoptionMode;
use openairside_core::command::{Command, DebugAction};
use openairside_core::event::Event;
use openairside_core::geometry::{CellPos, CellRect, MapSize};
use openairside_core::scenario::reference_terminal;
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::Speed;

mod machine;

const PERSONS: u32 = 10_000;
const TERMINAL_EDGE: u16 = 768;
/// 45 gates and 19 other targets make 64 targets.
const GATES: u8 = 45;
const DEFAULT_SIM_THREADS: usize = 2;
const DEFAULT_BACKGROUND_THREADS: usize = 2;
const WARM_UP: usize = 100;
/// Real time per step at 3×: 1 s / 180.
const STEP_PERIOD: Duration = Duration::from_nanos(5_555_556);
/// Cells the wall wanders over.
const WALL_CELLS: usize = 64;

struct Limits {
    gap_ms: f64,
    settle_ms: f64,
    jobs: u32,
    field_sets: u32,
    snapshots: u32,
}

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

fn quantile(sorted: &[f64], q: f64) -> f64 {
    let rank = (q * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

/// Peak resident memory of the process in MB (`VmHWM`), where the platform
/// tells.
fn peak_memory_mb() -> Option<f64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    let kb: f64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb / 1024.0)
}

/// Walkable ground-floor cells in the first row of the terminal that has
/// enough of them.
fn wall_cells(sim: &Simulation) -> Vec<CellPos> {
    let map = sim.world().map();
    let half = TERMINAL_EDGE / 2;
    for y in map.height / 2 - half..map.height / 2 + half {
        let cells: Vec<CellPos> = (map.width / 2 - half..map.width / 2 + half)
            .map(|x| CellPos { x, y, floor: 0 })
            .filter(|&pos| sim.world().build().grid().walkable(pos))
            .take(WALL_CELLS)
            .collect();
        if cells.len() == WALL_CELLS {
            return cells;
        }
    }
    Vec::new()
}

fn run() -> Result<bool, String> {
    let sim_threads = env_number("OPENAIRSIDE_BENCH_THREADS", DEFAULT_SIM_THREADS)?;
    let background_threads = env_number(
        "OPENAIRSIDE_BENCH_BACKGROUND_THREADS",
        DEFAULT_BACKGROUND_THREADS,
    )?;
    let limits = Limits {
        gap_ms: env_number("OPENAIRSIDE_BENCH_MAX_GAP_MS", 8_000.0)?,
        settle_ms: env_number("OPENAIRSIDE_BENCH_MAX_SETTLE_MS", 8_000.0)?,
        jobs: env_number("OPENAIRSIDE_BENCH_MAX_JOBS", 2)?,
        field_sets: env_number("OPENAIRSIDE_BENCH_MAX_FIELD_SETS", 3)?,
        snapshots: env_number("OPENAIRSIDE_BENCH_MAX_SNAPSHOTS", 2)?,
    };
    let abort_jobs: u32 = env_number("OPENAIRSIDE_BENCH_ABORT_JOBS", 12)?;
    let build_seconds: f64 = env_number("OPENAIRSIDE_BENCH_BUILD_SECONDS", 30.0)?;
    let period_ms: f64 = env_number("OPENAIRSIDE_BENCH_BUILD_PERIOD_MS", 250.0)?;
    let settle_timeout_ms: f64 = env_number("OPENAIRSIDE_BENCH_SETTLE_TIMEOUT_MS", 60_000.0)?;
    let period = Duration::from_secs_f64(period_ms / 1e3);
    let actions = (build_seconds * 1e3 / period_ms).round() as usize;

    let map = MapSize::new(2048, 2048).ok_or("invalid map size")?;
    let world = reference_terminal(map, TERMINAL_EDGE, GATES, PERSONS, 2026)
        .ok_or("the terminal does not fit on the map")?;
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
    let cells = wall_cells(&sim);
    if cells.len() < WALL_CELLS {
        return Err("no room for the wandering wall in the hall".into());
    }
    sim.submit(sim.session(), 1, Command::SetSpeed { speed: Speed::X3 });

    let origin = Instant::now();
    let mut step_index = 0u32;
    let mut next_id = 2u32;
    let mut samples = Vec::new();
    // Instants of the first building action, the adoptions and the last action.
    let mut first_action: Option<Instant> = None;
    let mut last_action: Option<Instant> = None;
    let mut adoptions: Vec<Instant> = Vec::new();
    let mut done_actions = 0usize;
    let mut settled: Option<Instant> = None;
    let mut aborted = false;
    let mut timed_out = false;
    let mut building_started: Option<Instant> = None;

    loop {
        let due = origin + STEP_PERIOD * step_index;
        step_index += 1;
        if let Some(wait) = due.checked_duration_since(Instant::now()) {
            std::thread::sleep(wait);
        }
        let now = Instant::now();
        let warm = (step_index as usize) <= WARM_UP;
        if !warm && building_started.is_none() {
            building_started = Some(now);
        }
        if let Some(started) = building_started
            && !aborted
            && done_actions < actions
            && now >= started + period * u32::try_from(done_actions).unwrap_or(u32::MAX)
        {
            let cell = cells[(done_actions / 2) % WALL_CELLS];
            let command = Command::Debug {
                action: DebugAction::SetWalls {
                    area: CellRect {
                        floor: cell.floor,
                        x0: cell.x,
                        y0: cell.y,
                        x1: cell.x + 1,
                        y1: cell.y + 1,
                    },
                    wall: done_actions.is_multiple_of(2),
                },
            };
            sim.submit(sim.session(), next_id, command);
            next_id += 1;
            done_actions += 1;
            first_action.get_or_insert(now);
            last_action = Some(now);
        }
        let start = Instant::now();
        sim.step();
        let elapsed = start.elapsed();
        let after = Instant::now();
        for record in sim.take_events(usize::MAX) {
            sim.ack(sim.session(), record.seq);
            if matches!(record.event, Event::WaypointsAdopted { .. }) {
                adoptions.push(after);
            }
        }
        if building_started.is_some() {
            samples.push(elapsed.as_secs_f64() * 1e3);
        }
        let gauges = sim.world().wayfinding().gauges();
        if gauges.jobs_live > abort_jobs && !aborted {
            aborted = true;
            done_actions = actions;
        }
        if done_actions >= actions {
            if !sim.is_recomputing() && !sim.has_pending_commands() && settled.is_none() {
                settled = Some(after);
                break;
            }
            if let Some(last) = last_action
                && after.duration_since(last).as_secs_f64() * 1e3 > settle_timeout_ms
            {
                timed_out = true;
                break;
            }
            if aborted && settled.is_none() {
                // Do not wait for a backlog that only grows.
                timed_out = true;
                break;
            }
        }
    }

    let first = first_action.ok_or("no building action was made")?;
    let last = last_action.ok_or("no building action was made")?;
    let end = settled.unwrap_or_else(Instant::now);
    // Gaps between the marks: first action, every adoption, end of the wait.
    let mut marks = vec![first];
    marks.extend(adoptions.iter().copied().filter(|&t| t >= first));
    if settled.is_none() {
        marks.push(end);
    }
    let max_gap = marks
        .windows(2)
        .map(|w| w[1].duration_since(w[0]).as_secs_f64() * 1e3)
        .fold(0.0, f64::max);
    let settle = end.duration_since(last).as_secs_f64() * 1e3;
    let building_adoptions = adoptions
        .iter()
        .filter(|&&t| t >= first && t <= last)
        .count();
    let gauges = sim.world().wayfinding().gauges();
    samples.sort_by(f64::total_cmp);
    let p99 = if samples.is_empty() {
        0.0
    } else {
        quantile(&samples, 0.99)
    };
    let memory = peak_memory_mb();
    let passed = !aborted
        && !timed_out
        && max_gap <= limits.gap_ms
        && settle <= limits.settle_ms
        && gauges.jobs_peak <= limits.jobs
        && gauges.field_sets_peak <= limits.field_sets
        && gauges.snapshots_peak <= limits.snapshots;

    let cpu = machine::cpu_model();
    let cpu_json = serde_json::to_string(&cpu).map_err(|err| err.to_string())?;
    let json = format!(
        concat!(
            "{{\"benchmark\":\"continuousBuild\",\"persons\":{},\"actions\":{},\"periodMs\":{},",
            "\"threads\":{},\"backgroundThreads\":{},\"cpus\":{},\"os\":\"{}\",\"cpuModel\":{},\"ticks\":{},",
            "\"adoptionsWhileBuilding\":{},\"adoptionsTotal\":{},\"maxGapMs\":{:.0},\"settleMs\":{:.0},",
            "\"jobsPeak\":{},\"fieldSetsPeak\":{},\"snapshotsPeak\":{},",
            "\"peakMemoryMb\":{},\"stepP99Ms\":{:.4},\"aborted\":{},\"timedOut\":{},",
            "\"limits\":{{\"gapMs\":{},\"settleMs\":{},\"jobs\":{},\"fieldSets\":{},\"snapshots\":{}}},",
            "\"passed\":{}}}\n"
        ),
        PERSONS,
        done_actions,
        period_ms,
        sim_threads,
        background_threads,
        std::thread::available_parallelism().map_or(1, std::num::NonZero::get),
        std::env::consts::OS,
        cpu_json,
        step_index,
        building_adoptions,
        adoptions.len(),
        max_gap,
        settle,
        gauges.jobs_peak,
        gauges.field_sets_peak,
        gauges.snapshots_peak,
        memory.map_or_else(|| "null".to_string(), |m| format!("{m:.0}")),
        p99,
        aborted,
        timed_out,
        limits.gap_ms,
        limits.settle_ms,
        limits.jobs,
        limits.field_sets,
        limits.snapshots,
        passed
    );
    if let Some(path) = output_path() {
        std::fs::write(&path, &json).map_err(|err| format!("{path}: {err}"))?;
    }
    let settle_text = if settled.is_some() {
        format!("last state adopted after {settle:.0} ms")
    } else {
        "last state not adopted".to_string()
    };
    println!(
        "continuous building, {done_actions} actions every {period_ms} ms: {building_adoptions} adoptions \
         while building, longest gap {max_gap:.0} ms (limit {}), {settle_text} \
         (limit {}); peaks: {} recomputations in flight (limit {}), {} flow field sets (limit {}), \
         {} snapshot copies (limit {}); process memory peak {} MB; step p99 {p99:.3} ms{}",
        limits.gap_ms,
        limits.settle_ms,
        gauges.jobs_peak,
        limits.jobs,
        gauges.field_sets_peak,
        limits.field_sets,
        gauges.snapshots_peak,
        limits.snapshots,
        memory.map_or_else(|| "unknown".to_string(), |m| format!("{m:.0}")),
        if aborted {
            " (building stopped: backlog of recomputations)"
        } else if timed_out {
            " (last state not adopted in time)"
        } else {
            ""
        },
    );
    println!("processor: {cpu}");
    print!("{json}");
    Ok(passed)
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => {
            eprintln!("benchmark limit exceeded (step 3.1a, ADR-041)");
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("benchmark failed: {err}");
            ExitCode::FAILURE
        }
    }
}
