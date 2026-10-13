//! Measurement driver for spike 1.2. Prints JSON lines.
//!
//! run   w=512 h=512 gates=17 persons=10000 ticks=10000 threads=4 bg=1 layout=split window=1000
//! fields w=512 h=512 gates=17 threads=4
//!
//! Spike 1.5: `scenario=5` uses the five-floor reference scenario,
//! `incremental=0` builds fields without distances (ADR-010).
//!
//! `run` issues build commands on a fixed schedule (every 5000 ticks one cycle:
//! close/open doors, close/open all security lanes, mark/clear an "avoid" area).

use std::collections::HashMap;
use std::time::Instant;
use wegweiser::grid::Grid;
use wegweiser::sim::{Command, Layout, Sim};

fn rss_mb() -> f64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .nth(1)
                .and_then(|v| v.parse::<f64>().ok())
        })
        .map_or(0.0, |pages| pages * 4096.0 / 1_048_576.0)
}

/// Peak resident memory (VmHWM) in MB.
fn peak_mb() -> f64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse::<f64>().ok())
        })
        .map_or(0.0, |kb| kb / 1024.0)
}

fn stats(v: &mut [f64]) -> (f64, f64, f64) {
    v.sort_by(f64::total_cmp);
    let mean = v.iter().sum::<f64>() / v.len().max(1) as f64;
    let p99 = v
        .get((v.len().saturating_sub(1) as f64 * 0.99) as usize)
        .copied()
        .unwrap_or(0.0);
    (mean, p99, v.last().copied().unwrap_or(0.0))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().cloned().unwrap_or_else(|| "run".into());
    let kv: HashMap<String, String> = args
        .iter()
        .skip(1)
        .filter_map(|a| {
            a.split_once('=')
                .map(|(k, v)| (k.to_string(), v.to_string()))
        })
        .collect();
    let get = |k: &str, d: usize| kv.get(k).and_then(|v| v.parse().ok()).unwrap_or(d);
    let (w, h) = (get("w", 512), get("h", 512));
    let gates = get("gates", 17) as u8;
    let five = get("scenario", 2) == 5;
    let make = |w, h, gates| {
        if five {
            Grid::reference_scenario(w, h, gates)
        } else {
            Grid::reference_like(w, h, gates)
        }
    };
    let threads = get("threads", 4);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("pool");

    if mode == "fields" {
        let grid = make(w, h, gates);
        let keep = get("incremental", 1) == 1;
        let fields = pool.install(|| wegweiser::field::Fields::build_opts(&grid, keep));
        let mut per = fields.per_field_ms.clone();
        let (mean, _, max) = stats(&mut per);
        println!(
            "{{\"mode\":\"fields\",\"w\":{w},\"h\":{h},\"floors\":{},\"walkable\":{},\"peak_mb\":{:.1},\"cells\":{},\"fields\":{},\"threads\":{threads},\"build_ms\":{:.1},\"per_field_ms_mean\":{mean:.2},\"per_field_ms_max\":{max:.2},\"field_mb\":{:.2},\"unreachable_cells_max\":{}}}",
            grid.floors,
            (0..grid.cells()).filter(|&i| grid.walkable(i)).count(),
            peak_mb(),
            grid.cells(),
            grid.targets.len(),
            fields.build_ms,
            fields.bytes() as f64 / 1_048_576.0,
            fields.unreachable_cells.iter().max().unwrap_or(&0)
        );
        return;
    }

    let persons = get("persons", 10_000);
    let ticks = get("ticks", 10_000) as u64;
    let window = get("window", 1000) as u64;
    let bg = get("bg", 1);
    let chunk = get("chunk", 256);
    let layout = if kv.get("layout").map(String::as_str) == Some("aos") {
        Layout::Aos
    } else {
        Layout::Split
    };
    let commands = get("commands", 1) == 1;
    let rss0 = rss_mb();
    let t0 = Instant::now();
    let incremental = get("incremental", 1) == 1;
    let mut sim = pool
        .install(|| Sim::with_incremental(make(w, h, gates), persons, layout, bg, 42, incremental));
    let setup_ms = t0.elapsed().as_secs_f64() * 1e3;
    let initial_fields_ms = sim.fields.build_ms;
    let schedule = [
        (2000, Command::CloseDoors),
        (2500, Command::OpenDoors),
        (3000, Command::CloseSecurity),
        (4000, Command::OpenSecurity),
        (4500, Command::MarkAvoidHall),
        (4800, Command::ClearHall),
    ];
    let mut win = Vec::with_capacity(window as usize);
    let mut all = Vec::with_capacity(ticks as usize);
    let mut windows = Vec::new();
    let mut security_closed_unreachable = (0, 0);
    let run0 = Instant::now();
    for t in 0..ticks {
        if commands {
            for &(at, cmd) in &schedule {
                if t % 5000 == at {
                    sim.command(cmd);
                    if matches!(cmd, Command::OpenSecurity) {
                        let c = sim.counters();
                        security_closed_unreachable = (c.unreachable_zone, c.unreachable_enclosed);
                    }
                }
            }
        }
        let s = Instant::now();
        pool.install(|| sim.step(chunk));
        let us = s.elapsed().as_secs_f64() * 1e6;
        win.push(us);
        all.push(us);
        if (t + 1) % window == 0 {
            let (mean, p99, max) = stats(&mut win);
            windows.push(format!("{{\"tick\":{},\"mean_us\":{mean:.0},\"p99_us\":{p99:.0},\"max_us\":{max:.0},\"rss_mb\":{:.1}}}", t + 1, rss_mb()));
            win.clear();
        }
    }
    let run_s = run0.elapsed().as_secs_f64();
    let (mean, p99, max) = stats(&mut all);
    let c = sim.counters();
    let reports: Vec<String> = sim
        .reports
        .iter()
        .map(|r| format!("{{\"cmd\":\"{:?}\",\"changed\":{},\"tick\":{},\"recompute_ms\":{:.1},\"stall_ms\":{:.2}}}", r.command, r.changed_cells, r.issued_tick, r.recompute_ms, r.stall_ms))
        .collect();
    println!(
        "{{\"mode\":\"run\",\"w\":{w},\"h\":{h},\"floors\":{},\"peak_mb\":{:.1},\"gates\":{gates},\"fields\":{},\"persons\":{persons},\"ticks\":{ticks},\"threads\":{threads},\"bg\":{bg},\"chunk\":{chunk},\"layout\":\"{layout:?}\",\"incremental\":{},\"setup_ms\":{setup_ms:.0},\"initial_fields_ms\":{:.1},\"field_mb\":{:.1},\"persons_kb\":{:.0},\"rss_start_mb\":{rss0:.1},\"rss_end_mb\":{:.1},\"step_us_mean\":{mean:.0},\"step_us_p99\":{p99:.0},\"step_us_max\":{max:.0},\"ticks_per_s\":{:.0},\"arrivals\":{},\"trips\":{},\"unreachable_events_until_security_reopened\":{{\"zone\":{},\"enclosed\":{}}},\"unreachable_events_total\":{{\"zone\":{},\"enclosed\":{}}},\"checksum\":\"{:016x}\",\"recompute\":[{}],\"windows\":[{}]}}",
        sim.grid.floors,
        peak_mb(),
        sim.grid.targets.len(),
        sim.incremental,
        initial_fields_ms,
        sim.fields.bytes() as f64 / 1_048_576.0,
        sim.persons_bytes() as f64 / 1024.0,
        rss_mb(),
        ticks as f64 / run_s,
        c.arrivals,
        c.trips,
        security_closed_unreachable.0,
        security_closed_unreachable.1,
        c.unreachable_zone,
        c.unreachable_enclosed,
        sim.checksum(),
        reports.join(","),
        windows.join(",")
    );
}
