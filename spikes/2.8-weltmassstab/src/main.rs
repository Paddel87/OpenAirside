//! Measurement driver for roadmap step 2.8 (world scale, airside, time model).
//! Throwaway code. Prints JSON lines.
//!
//! world  W=2048 H=2048 t=512 band=128 gates=17 threads=4 block=32
//!   Embeds the five-floor reference terminal of spike 1.5 (t × t cells) into a
//!   world of W × H cells. Outside the terminal the ground floor is airside
//!   (apron, taxiways, runways: not walkable for persons) except a landside
//!   band of `band` rows south of the terminal (forecourt, parking, station).
//!   Upper and lower floors exist only above/below the terminal.
//!   Measures: dense direction fields over the whole world (status quo of
//!   ADR-013) and the cropped rectangle that a block-sparse store would touch;
//!   counts occupied blocks for a block-sparse store.
//!
//! step   W=2048 H=2048 t=512 band=128 persons=10000 ticks=3000 threads=4
//!   Simulation step of 10,000 persons in the embedded world.
//!
//! lookup W=2048 H=2048 t=512 band=128 block=32 persons=10000 rounds=2000
//!   Cost of reading one direction per person through a block table compared
//!   with a dense index (the inner access of the movement phase).

use std::collections::HashMap;
use std::time::Instant;
use wegweiser::field::Fields;
use wegweiser::grid::{Grid, LANDSIDE, Target, WALL};
use wegweiser::sim::{Layout, Sim};

/// Peak resident memory (`VmHWM`) in MB.
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

/// Embeds `term` (five floors, ground floor 2) at (`ox`, `oy`) into a world of
/// `ww` × `wh` cells. Adds a walkable landside band of `band` rows directly
/// south of the terminal and opens the terminal's south wall towards it.
fn embed(term: &Grid, ww: usize, wh: usize, ox: usize, oy: usize, band: usize) -> Grid {
    let floors = term.floors;
    let n = ww * wh * floors;
    let mut g = Grid {
        w: ww,
        h: wh,
        floors,
        ground: term.ground,
        flags: vec![WALL; n],
        zone: vec![LANDSIDE; n],
        cost: vec![term.cost[0]; n],
        targets: Vec::new(),
        entrances: Vec::new(),
        security_cells: Vec::new(),
        doors: Vec::new(),
    };
    let map = |c: u32| -> u32 {
        let (x, y, f) = term.xyf(c as usize);
        (f * ww * wh + (y + oy) * ww + (x + ox)) as u32
    };
    for f in 0..floors {
        for y in 0..term.h {
            for x in 0..term.w {
                let s = term.idx(x, y, f);
                let d = f * ww * wh + (y + oy) * ww + (x + ox);
                g.flags[d] = term.flags[s];
                g.zone[d] = term.zone[s];
                g.cost[d] = term.cost[s];
            }
        }
    }
    let gf = term.ground;
    // Open the south wall of the terminal on the ground floor.
    for x in 1..term.w - 1 {
        let d = gf * ww * wh + (term.h - 1 + oy) * ww + (x + ox);
        g.flags[d] = 0;
    }
    for y in term.h + oy..(term.h + oy + band).min(wh - 1) {
        for x in ox..ox + term.w {
            let d = gf * ww * wh + y * ww + x;
            g.flags[d] = 0;
            g.zone[d] = LANDSIDE;
        }
    }
    g.targets = term
        .targets
        .iter()
        .map(|t| Target {
            kind: t.kind,
            cells: t.cells.iter().map(|&c| map(c)).collect(),
        })
        .collect();
    g.entrances = term.entrances.iter().map(|&c| map(c)).collect();
    g.security_cells = term.security_cells.iter().map(|&c| map(c)).collect();
    g.doors = term.doors.iter().map(|&c| map(c)).collect();
    g
}

/// World with the terminal centred horizontally and the landside band at the
/// south edge (airside to the north, east and west of the terminal).
fn world(ww: usize, wh: usize, t: usize, band: usize, gates: u8) -> Grid {
    let term = Grid::reference_scenario(t, t, gates);
    let ox = (ww - t) / 2;
    let oy = wh.saturating_sub(t + band + 1);
    embed(&term, ww, wh, ox, oy, band)
}

/// Blocks of `b` × `b` cells per floor that contain at least one walkable cell.
fn occupied_blocks(g: &Grid, b: usize) -> (usize, Vec<usize>) {
    let bw = g.w.div_ceil(b);
    let bh = g.h.div_ceil(b);
    let mut per_floor = vec![0; g.floors];
    for (f, count) in per_floor.iter_mut().enumerate() {
        let mut occ = vec![false; bw * bh];
        for y in 0..g.h {
            for x in 0..g.w {
                if g.walkable(g.idx(x, y, f)) {
                    occ[(y / b) * bw + x / b] = true;
                }
            }
        }
        *count = occ.iter().filter(|&&o| o).count();
    }
    (per_floor.iter().sum(), per_floor)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().cloned().unwrap_or_else(|| "world".into());
    let kv: HashMap<String, String> = args
        .iter()
        .skip(1)
        .filter_map(|a| {
            a.split_once('=')
                .map(|(k, v)| (k.to_string(), v.to_string()))
        })
        .collect();
    let get = |k: &str, d: usize| kv.get(k).and_then(|v| v.parse().ok()).unwrap_or(d);
    let (ww, wh) = (get("W", 2048), get("H", 2048));
    let t = get("t", 512);
    let band = get("band", 128);
    let gates = get("gates", 17) as u8;
    let block = get("block", 32);
    let threads = get("threads", 4);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("pool");

    match mode.as_str() {
        "world" => {
            let g = world(ww, wh, t, band, gates);
            let walkable = (0..g.cells()).filter(|&i| g.walkable(i)).count();
            let (blocks, per_floor) = occupied_blocks(&g, block);
            let nfields = g.targets.len();
            let dense = pool.install(|| Fields::build_opts(&g, false));
            let dense_ms = dense.build_ms;
            let dense_mb = dense.bytes() as f64 / 1_048_576.0;
            let unreachable = dense.unreachable_cells.iter().max().copied().unwrap_or(0);
            drop(dense);
            let peak_dense = peak_mb();
            // Cropped rectangle = what a block-sparse store would touch.
            let crop = world(t, t + band + 1, t, band, gates);
            let sparse = pool.install(|| Fields::build_opts(&crop, false));
            let sparse_mb = (blocks * block * block * nfields) as f64 / 1_048_576.0;
            // Per-cell base data: flags, zone, cost (3 bytes) + density (2 bytes).
            let base_dense_mb = (g.cells() * 5) as f64 / 1_048_576.0;
            let ground_cells = ww * wh;
            let base_sparse_mb = ((ground_cells + (blocks - per_floor[g.ground]) * block * block)
                * 5) as f64
                / 1_048_576.0;
            println!(
                "{{\"mode\":\"world\",\"W\":{ww},\"H\":{wh},\"t\":{t},\"band\":{band},\"floors\":{},\"cells\":{},\"walkable\":{walkable},\"fields\":{nfields},\"threads\":{threads},\"dense_build_ms\":{dense_ms:.1},\"dense_field_mb\":{dense_mb:.1},\"unreachable_max\":{unreachable},\"peak_mb_after_dense\":{peak_dense:.1},\"crop_build_ms\":{:.1},\"crop_field_mb\":{:.1},\"block\":{block},\"blocks\":{blocks},\"blocks_per_floor\":{per_floor:?},\"sparse_field_mb\":{sparse_mb:.1},\"base_dense_mb\":{base_dense_mb:.1},\"base_sparse_mb\":{base_sparse_mb:.1}}}",
                g.floors,
                g.cells(),
                sparse.build_ms,
                sparse.bytes() as f64 / 1_048_576.0,
            );
        }
        "step" => {
            let persons = get("persons", 10_000);
            let ticks = get("ticks", 3000);
            let g = world(ww, wh, t, band, gates);
            let mut sim =
                pool.install(|| Sim::with_incremental(g, persons, Layout::Split, 1, 42, false));
            for _ in 0..300 {
                pool.install(|| sim.step(256));
            }
            let mut v: Vec<f64> = Vec::with_capacity(ticks);
            for _ in 0..ticks {
                let t0 = Instant::now();
                pool.install(|| sim.step(256));
                v.push(t0.elapsed().as_secs_f64() * 1e3);
            }
            v.sort_by(f64::total_cmp);
            let mean = v.iter().sum::<f64>() / v.len() as f64;
            let p99 = v[(v.len() - 1) * 99 / 100];
            println!(
                "{{\"mode\":\"step\",\"W\":{ww},\"H\":{wh},\"t\":{t},\"persons\":{persons},\"threads\":{threads},\"ticks\":{ticks},\"step_ms_mean\":{mean:.3},\"step_ms_p99\":{p99:.3},\"peak_mb\":{:.1}}}",
                peak_mb()
            );
        }
        "lookup" => {
            let persons = get("persons", 10_000);
            let rounds = get("rounds", 2000);
            let g = world(ww, wh, t, band, gates);
            let plane = ww * wh;
            let bw = ww.div_ceil(block);
            let bh = wh.div_ceil(block);
            // Block table: per floor and block the slot in the compact store or u32::MAX.
            let mut table = vec![u32::MAX; bw * bh * g.floors];
            let mut slots = 0u32;
            for f in 0..g.floors {
                for by in 0..bh {
                    for bx in 0..bw {
                        let any = (by * block..((by + 1) * block).min(wh)).any(|y| {
                            (bx * block..((bx + 1) * block).min(ww))
                                .any(|x| g.walkable(f * plane + y * ww + x))
                        });
                        if any {
                            table[f * bw * bh + by * bw + bx] = slots;
                            slots += 1;
                        }
                    }
                }
            }
            let dense: Vec<u8> = (0..g.cells()).map(|i| (i % 251) as u8).collect();
            let compact: Vec<u8> = (0..slots as usize * block * block)
                .map(|i| (i % 251) as u8)
                .collect();
            // Person positions: random walkable cells.
            let mut rng = 0x9e37_79b9u32;
            let mut pos: Vec<(u32, u32, u32)> = Vec::with_capacity(persons);
            while pos.len() < persons {
                rng ^= rng << 13;
                rng ^= rng >> 17;
                rng ^= rng << 5;
                let i = rng as usize % g.cells();
                if g.walkable(i) {
                    let (x, y, f) = g.xyf(i);
                    pos.push((x as u32, y as u32, f as u32));
                }
            }
            let t0 = Instant::now();
            let mut acc = 0u64;
            for r in 0..rounds {
                for &(x, y, f) in &pos {
                    let i = f as usize * plane + y as usize * ww + x as usize;
                    acc += u64::from(dense[(i + r) % dense.len()]);
                }
            }
            let dense_ns = t0.elapsed().as_secs_f64() * 1e9 / (rounds * persons) as f64;
            let t1 = Instant::now();
            let mut acc2 = 0u64;
            for r in 0..rounds {
                for &(x, y, f) in &pos {
                    let (x, y, f) = (x as usize, y as usize, f as usize);
                    let s = table[f * bw * bh + (y / block) * bw + x / block] as usize;
                    let i = s * block * block + (y % block) * block + (x % block);
                    acc2 += u64::from(compact[(i + r) % compact.len()]);
                }
            }
            let sparse_ns = t1.elapsed().as_secs_f64() * 1e9 / (rounds * persons) as f64;
            println!(
                "{{\"mode\":\"lookup\",\"W\":{ww},\"H\":{wh},\"block\":{block},\"persons\":{persons},\"dense_ns\":{dense_ns:.2},\"sparse_ns\":{sparse_ns:.2},\"check\":{}}}",
                (acc ^ acc2) & 1
            );
        }
        _ => eprintln!("unknown mode"),
    }
}
