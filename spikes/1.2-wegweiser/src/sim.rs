//! Persons following direction fields, with plans (check-in → security → gate,
//! arrivals → exit), dwell times, density-dependent speed, build commands with
//! background field recomputation applied at a fixed tick (deterministic).

use crate::field::{ARRIVED, Cause, DIRS, DOWN, Fields, UNREACHABLE, UP};
use crate::grid::{AIRLINES, COST_AVOID, COST_NORMAL, Grid, SECURITY, TargetKind, WALL};
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};
use std::sync::mpsc;
use std::time::Instant;

/// Data touched every tick (16 bytes).
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Hot {
    pub x: f32,
    pub y: f32,
    /// Cell currently counted in the density map.
    pub cell: u32,
    pub field: u8,
    pub floor: u8,
    pub wait: u16,
}

/// Data touched only on events (arrival at a target, respawn).
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Cold {
    pub rng: u32,
    pub speed: f32,
    pub needs: [f32; 4],
    pub departing: bool,
    pub screened: bool,
    pub airline: u8,
    pub gate: u8,
    pub plan: u8,
    pub trips: u32,
}

/// Array-of-structs variant: hot and cold in one record.
#[derive(Clone, Copy, Debug, Default)]
pub struct Person {
    pub hot: Hot,
    pub cold: Cold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// One Vec<Person>.
    Aos,
    /// Vec<Hot> + Vec<Cold> (hot/cold split).
    Split,
}

enum Store {
    Aos(Vec<Person>),
    Split(Vec<Hot>, Vec<Cold>),
}

/// Build command, applied to the grid immediately (walls are visible in the
/// next frame); new fields take effect `APPLY_DELAY` ticks later.
#[derive(Clone, Copy, Debug)]
pub enum Command {
    CloseDoors,
    OpenDoors,
    CloseSecurity,
    OpenSecurity,
    MarkAvoidHall,
    ClearHall,
}

pub const APPLY_DELAY: u64 = 30;

#[derive(Default, Debug, Clone)]
pub struct Counters {
    pub arrivals: u64,
    pub trips: u64,
    pub unreachable_zone: u64,
    pub unreachable_enclosed: u64,
}

struct AtomicCounters {
    arrivals: AtomicU64,
    trips: AtomicU64,
    unreachable_zone: AtomicU64,
    unreachable_enclosed: AtomicU64,
}

pub struct RecomputeReport {
    pub changed_cells: usize,
    pub command: Command,
    pub issued_tick: u64,
    pub recompute_ms: f64,
    pub stall_ms: f64,
}

/// Background recomputation started by `set_walls` (spike 1.3).
struct AsyncPending {
    issued_tick: u64,
    started: Instant,
    rx: mpsc::Receiver<Fields>,
}

/// Result of taking over background fields (spike 1.3, ADR-010 option A).
#[derive(Clone, Copy, Debug)]
pub struct AsyncApplied {
    /// Ticks between start of the recomputation and takeover.
    pub ticks: u64,
    /// Wall-clock time between start of the recomputation and takeover.
    pub ms: f64,
    /// Pure computation time reported by the field builder.
    pub recompute_ms: f64,
}

struct Pending {
    changed_cells: usize,
    command: Command,
    issued_tick: u64,
    rx: mpsc::Receiver<Fields>,
}

pub struct Sim {
    pub grid: Arc<Grid>,
    pub fields: Arc<Fields>,
    store: Store,
    density: Vec<AtomicU16>,
    counters: AtomicCounters,
    pub tick: u64,
    pending: Option<Pending>,
    bg: Arc<rayon::ThreadPool>,
    pub reports: Vec<RecomputeReport>,
    /// Incremental repair instead of full recomputation after build commands.
    pub incremental: bool,
    /// Spike 1.3: cells changed by `set_walls` that the current fields do not
    /// yet reflect and no running recomputation covers.
    unapplied: Vec<u32>,
    async_pending: Option<AsyncPending>,
    /// Ids of frequently used targets.
    exit: u8,
    toilet_land: u8,
    toilet_air: u8,
    shop_air: u8,
    checkin0: u8,
    gate0: u8,
    gates: u8,
}

fn xorshift(s: &mut u32) -> u32 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *s = x;
    x
}

struct Ctx<'a> {
    grid: &'a Grid,
    fields: &'a Fields,
    density: &'a [AtomicU16],
    counters: &'a AtomicCounters,
    exit: u8,
    toilet_land: u8,
    toilet_air: u8,
    shop_air: u8,
    checkin0: u8,
    gate0: u8,
    gates: u8,
}

impl Ctx<'_> {
    fn spawn(&self, hot: &mut Hot, cold: &mut Cold) {
        let r = xorshift(&mut cold.rng);
        cold.departing = r % 10 < 6;
        cold.screened = false;
        cold.plan = 0;
        cold.airline = ((r >> 4) % u32::from(AIRLINES)) as u8;
        cold.gate = ((r >> 8) % u32::from(self.gates)) as u8;
        cold.speed = 0.08 + ((r >> 12) % 64) as f32 / 1600.0;
        cold.needs = [0.1, 0.2, 0.3, 0.4];
        let (cell, field) = if cold.departing {
            let e = &self.grid.entrances;
            (
                e[(r >> 16) as usize % e.len()] as usize,
                self.checkin0 + cold.airline,
            )
        } else {
            let t = &self.grid.targets[usize::from(self.gate0 + cold.gate)].cells;
            (t[(r >> 16) as usize % t.len()] as usize, self.exit)
        };
        let (x, y, f) = self.grid.xyf(cell);
        hot.x = x as f32 + 0.5;
        hot.y = y as f32 + 0.5;
        hot.floor = f as u8;
        hot.field = field;
        hot.wait = 0;
    }

    /// Next plan target after arriving; returns dwell ticks.
    fn arrive(&self, hot: &mut Hot, cold: &mut Cold) -> u16 {
        self.counters.arrivals.fetch_add(1, Ordering::Relaxed);
        let r = xorshift(&mut cold.rng);
        let dwell = 30 + (r % 90) as u16;
        if !cold.departing {
            // Arrived at landside exit: leave, respawn as a new person.
            cold.trips += 1;
            self.counters.trips.fetch_add(1, Ordering::Relaxed);
            self.spawn(hot, cold);
            return 0;
        }
        // Departing plan: 0 check-in → 1 landside toilet (optional) → 2/3 airside
        // shop (optional, path leads through security) → 4 gate → board.
        let gate = self.gate0 + cold.gate;
        let (stage, field) = match cold.plan {
            0 if r >> 8 & 3 == 0 => (1, self.toilet_land),
            0 | 1 if r >> 10 & 1 == 0 => (3, self.shop_air),
            0..=3 => (4, gate),
            _ => {
                // Boarded: respawn as a new person.
                cold.trips += 1;
                self.counters.trips.fetch_add(1, Ordering::Relaxed);
                self.spawn(hot, cold);
                return 0;
            }
        };
        cold.plan = stage;
        hot.field = field;
        let _ = self.toilet_air;
        dwell
    }

    /// Movement for one person. Reads density of the previous tick only.
    fn step(&self, hot: &mut Hot, cold: &mut Cold) {
        let g = self.grid;
        for n in &mut cold.needs {
            *n = (*n * 0.9995 + 0.0002).min(1.0);
        }
        if hot.wait > 0 {
            hot.wait -= 1;
            return;
        }
        let cell = g.idx(hot.x as usize, hot.y as usize, usize::from(hot.floor));
        if !g.walkable(cell) {
            // A wall was built on this cell: step to the first free neighbour.
            for &(dx, dy) in &DIRS {
                let (nx, ny) = (hot.x as i32 + dx, hot.y as i32 + dy);
                let n = g.idx(nx as usize, ny as usize, usize::from(hot.floor));
                if g.walkable(n) {
                    hot.x = nx as f32 + 0.5;
                    hot.y = ny as f32 + 0.5;
                    return;
                }
            }
            return;
        }
        if g.zone[cell] == SECURITY && !cold.screened {
            cold.screened = true;
            hot.wait = 90;
            return;
        }
        match self.fields.dir(hot.field, cell) {
            ARRIVED => hot.wait = self.arrive(hot, cold),
            UNREACHABLE => {
                match self.fields.cause(hot.field, cell) {
                    Cause::ZoneRule => self
                        .counters
                        .unreachable_zone
                        .fetch_add(1, Ordering::Relaxed),
                    Cause::Enclosed => self
                        .counters
                        .unreachable_enclosed
                        .fetch_add(1, Ordering::Relaxed),
                };
                // Wait and retry; a later build change may reopen the path.
                hot.wait = 60;
            }
            UP | DOWN => {
                hot.floor = 1 - hot.floor;
                hot.wait = 20;
            }
            d => {
                let (dx, dy) = DIRS[usize::from(d)];
                let scale = if dx != 0 && dy != 0 {
                    0.707_106_77
                } else {
                    1.0
                };
                let nx = hot.x + dx as f32 * cold.speed * scale;
                let ny = hot.y + dy as f32 * cold.speed * scale;
                let next = g.idx(nx as usize, ny as usize, usize::from(hot.floor));
                if !g.walkable(next) {
                    hot.wait = 5;
                    return;
                }
                let crowd = f32::from(self.density[next].load(Ordering::Relaxed));
                let slow = 1.0 / (1.0 + 0.15 * crowd);
                hot.x += dx as f32 * cold.speed * scale * slow;
                hot.y += dy as f32 * cold.speed * scale * slow;
            }
        }
    }

    /// Second phase: update density counts (commutative, therefore deterministic).
    fn recount(&self, hot: &mut Hot) {
        let cell = self
            .grid
            .idx(hot.x as usize, hot.y as usize, usize::from(hot.floor)) as u32;
        if cell != hot.cell {
            self.density[hot.cell as usize].fetch_sub(1, Ordering::Relaxed);
            self.density[cell as usize].fetch_add(1, Ordering::Relaxed);
            hot.cell = cell;
        }
    }
}

impl Sim {
    #[must_use]
    pub fn new(grid: Grid, persons: usize, layout: Layout, bg_threads: usize, seed: u32) -> Self {
        let fields = Arc::new(Fields::build(&grid));
        let gates = grid
            .targets
            .iter()
            .filter(|t| matches!(t.kind, TargetKind::Gate(_)))
            .count() as u8;
        let density = (0..grid.cells()).map(|_| AtomicU16::new(0)).collect();
        let bg = Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(bg_threads)
                .build()
                .expect("pool"),
        );
        let mut sim = Self {
            exit: grid.target_index(TargetKind::ExitLandside),
            toilet_land: grid.target_index(TargetKind::ToiletLandside),
            toilet_air: grid.target_index(TargetKind::ToiletAirside),
            shop_air: grid.target_index(TargetKind::ShopAirside),
            checkin0: grid.target_index(TargetKind::CheckIn(0)),
            gate0: grid.target_index(TargetKind::Gate(0)),
            gates,
            grid: Arc::new(grid),
            fields,
            store: Store::Aos(Vec::new()),
            density,
            counters: AtomicCounters {
                arrivals: AtomicU64::new(0),
                trips: AtomicU64::new(0),
                unreachable_zone: AtomicU64::new(0),
                unreachable_enclosed: AtomicU64::new(0),
            },
            tick: 0,
            pending: None,
            bg,
            reports: Vec::new(),
            incremental: true,
            unapplied: Vec::new(),
            async_pending: None,
        };
        let mut hots = vec![Hot::default(); persons];
        let mut colds = vec![Cold::default(); persons];
        {
            let ctx = sim.ctx();
            let mut rng = seed.max(1);
            for (h, c) in hots.iter_mut().zip(colds.iter_mut()) {
                c.rng = xorshift(&mut rng) | 1;
                ctx.spawn(h, c);
                // Spread the initial population over the plan.
                h.wait = (xorshift(&mut rng) % 600) as u16;
                let cell = ctx
                    .grid
                    .idx(h.x as usize, h.y as usize, usize::from(h.floor));
                h.cell = cell as u32;
                ctx.density[cell].fetch_add(1, Ordering::Relaxed);
            }
        }
        sim.store = match layout {
            Layout::Aos => Store::Aos(
                hots.into_iter()
                    .zip(colds)
                    .map(|(hot, cold)| Person { hot, cold })
                    .collect(),
            ),
            Layout::Split => Store::Split(hots, colds),
        };
        sim
    }

    fn ctx(&self) -> Ctx<'_> {
        Ctx {
            grid: &self.grid,
            fields: &self.fields,
            density: &self.density,
            counters: &self.counters,
            exit: self.exit,
            toilet_land: self.toilet_land,
            toilet_air: self.toilet_air,
            shop_air: self.shop_air,
            checkin0: self.checkin0,
            gate0: self.gate0,
            gates: self.gates,
        }
    }

    /// One tick on the current rayon pool. `chunk` = persons per task.
    pub fn step(&mut self, chunk: usize) {
        self.apply_pending_if_due();
        let ctx = Ctx {
            grid: &self.grid,
            fields: &self.fields,
            density: &self.density,
            counters: &self.counters,
            exit: self.exit,
            toilet_land: self.toilet_land,
            toilet_air: self.toilet_air,
            shop_air: self.shop_air,
            checkin0: self.checkin0,
            gate0: self.gate0,
            gates: self.gates,
        };
        match &mut self.store {
            Store::Aos(p) => {
                p.par_chunks_mut(chunk)
                    .for_each(|c| c.iter_mut().for_each(|p| ctx.step(&mut p.hot, &mut p.cold)));
                p.par_chunks_mut(chunk)
                    .for_each(|c| c.iter_mut().for_each(|p| ctx.recount(&mut p.hot)));
            }
            Store::Split(h, c) => {
                h.par_chunks_mut(chunk)
                    .zip(c.par_chunks_mut(chunk))
                    .for_each(|(hs, cs)| {
                        hs.iter_mut()
                            .zip(cs.iter_mut())
                            .for_each(|(h, c)| ctx.step(h, c))
                    });
                h.par_chunks_mut(chunk)
                    .for_each(|hs| hs.iter_mut().for_each(|h| ctx.recount(h)));
            }
        }
        self.tick += 1;
    }

    /// Applies a build command now and starts the field recomputation in the
    /// background on a snapshot of the grid.
    pub fn command(&mut self, cmd: Command) {
        assert!(
            self.pending.is_none(),
            "one command at a time in this spike"
        );
        let mut grid = (*self.grid).clone();
        match cmd {
            Command::CloseDoors | Command::OpenDoors => {
                let close = matches!(cmd, Command::CloseDoors);
                // Every second door in the landside hall.
                for (k, &d) in grid.doors.clone().iter().enumerate() {
                    let (_, y, f) = grid.xyf(d as usize);
                    if f == 0 && y > grid.h / 2 && k % 2 == 0 {
                        if close {
                            grid.flags[d as usize] |= WALL;
                        } else {
                            grid.flags[d as usize] &= !WALL;
                        }
                    }
                }
            }
            Command::CloseSecurity | Command::OpenSecurity => {
                for &s in &grid.security_cells.clone() {
                    if matches!(cmd, Command::CloseSecurity) {
                        grid.flags[s as usize] |= WALL;
                    } else {
                        grid.flags[s as usize] &= !WALL;
                    }
                }
            }
            Command::MarkAvoidHall | Command::ClearHall => {
                let c = if matches!(cmd, Command::MarkAvoidHall) {
                    COST_AVOID
                } else {
                    COST_NORMAL
                };
                let (w, h) = (grid.w, grid.h);
                for y in h / 2 + 20..h / 2 + 80 {
                    for x in w / 2..w * 3 / 4 {
                        let i = grid.idx(x, y, 0);
                        grid.cost[i] = c;
                    }
                }
            }
        }
        let old = &self.grid;
        let changed: Vec<u32> = (0..grid.cells())
            .filter(|&i| old.flags[i] != grid.flags[i] || old.cost[i] != grid.cost[i])
            .map(|i| i as u32)
            .collect();
        let changed_cells = changed.len();
        let grid = Arc::new(grid);
        self.grid = Arc::clone(&grid);
        let base = Arc::clone(&self.fields);
        let incremental = self.incremental;
        let (tx, rx) = mpsc::channel();
        self.bg.spawn(move || {
            let f = if incremental {
                base.repair(&grid, &changed)
            } else {
                Fields::build(&grid)
            };
            let _ = tx.send(f);
        });
        self.pending = Some(Pending {
            changed_cells,
            command: cmd,
            issued_tick: self.tick,
            rx,
        });
    }

    fn apply_pending_if_due(&mut self) {
        let due = matches!(&self.pending, Some(p) if self.tick >= p.issued_tick + APPLY_DELAY);
        if !due {
            return;
        }
        let p = self.pending.take().expect("pending");
        let t0 = Instant::now();
        let fields = p.rx.recv().expect("background result");
        let stall_ms = t0.elapsed().as_secs_f64() * 1e3;
        self.reports.push(RecomputeReport {
            changed_cells: p.changed_cells,
            command: p.command,
            issued_tick: p.issued_tick,
            recompute_ms: fields.build_ms,
            stall_ms,
        });
        self.fields = Arc::new(fields);
    }

    /// Spike 1.3: sets or clears walls at arbitrary cells. The grid changes
    /// immediately (visible in the next frame); new fields are computed in the
    /// background and taken over by `poll_async` as soon as they are ready
    /// (ADR-010, option A). Returns the cells that actually changed.
    pub fn set_walls(&mut self, cells: &[u32], wall: bool) -> Vec<u32> {
        let mut grid = (*self.grid).clone();
        let mut changed = Vec::new();
        for &c in cells {
            let i = c as usize;
            if i >= grid.cells() {
                continue;
            }
            let before = grid.flags[i];
            if wall {
                grid.flags[i] |= WALL;
            } else {
                grid.flags[i] &= !WALL;
            }
            if grid.flags[i] != before {
                changed.push(c);
            }
        }
        if !changed.is_empty() {
            self.grid = Arc::new(grid);
            self.unapplied.extend_from_slice(&changed);
            self.start_async_if_idle();
        }
        changed
    }

    fn start_async_if_idle(&mut self) {
        if self.async_pending.is_some() || self.unapplied.is_empty() {
            return;
        }
        let changed = std::mem::take(&mut self.unapplied);
        let grid = Arc::clone(&self.grid);
        let base = Arc::clone(&self.fields);
        let incremental = self.incremental;
        let (tx, rx) = mpsc::channel();
        self.bg.spawn(move || {
            let f = if incremental {
                base.repair(&grid, &changed)
            } else {
                Fields::build(&grid)
            };
            let _ = tx.send(f);
        });
        self.async_pending = Some(AsyncPending {
            issued_tick: self.tick,
            started: Instant::now(),
            rx,
        });
    }

    /// Spike 1.3: takes over finished background fields without waiting.
    /// Starts the next recomputation if more walls changed in the meantime.
    pub fn poll_async(&mut self) -> Option<AsyncApplied> {
        let p = self.async_pending.as_ref()?;
        let fields = p.rx.try_recv().ok()?;
        let applied = AsyncApplied {
            ticks: self.tick - p.issued_tick,
            ms: p.started.elapsed().as_secs_f64() * 1e3,
            recompute_ms: fields.build_ms,
        };
        self.fields = Arc::new(fields);
        self.async_pending = None;
        self.start_async_if_idle();
        Some(applied)
    }

    /// Spike 1.3: whether walls changed that the fields do not reflect yet.
    #[must_use]
    pub fn fields_outdated(&self) -> bool {
        self.async_pending.is_some() || !self.unapplied.is_empty()
    }

    /// Spike 1.3: read access to the per-tick data of all persons, independent
    /// of the memory layout.
    pub fn visit_hot(&self, mut f: impl FnMut(&Hot)) {
        match &self.store {
            Store::Aos(p) => p.iter().for_each(|p| f(&p.hot)),
            Store::Split(h, _) => h.iter().for_each(f),
        }
    }

    #[must_use]
    pub fn counters(&self) -> Counters {
        let c = &self.counters;
        Counters {
            arrivals: c.arrivals.load(Ordering::Relaxed),
            trips: c.trips.load(Ordering::Relaxed),
            unreachable_zone: c.unreachable_zone.load(Ordering::Relaxed),
            unreachable_enclosed: c.unreachable_enclosed.load(Ordering::Relaxed),
        }
    }

    /// FNV-1a over all person state and density counts.
    #[must_use]
    pub fn checksum(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut mix = |v: u32| {
            h ^= u64::from(v);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        };
        let mut person = |hot: &Hot, cold: &Cold| {
            mix(hot.x.to_bits());
            mix(hot.y.to_bits());
            mix(hot.cell);
            mix(u32::from(hot.field) | u32::from(hot.floor) << 8 | u32::from(hot.wait) << 16);
            mix(cold.rng);
            mix(cold.trips);
            for n in cold.needs {
                mix(n.to_bits());
            }
        };
        match &self.store {
            Store::Aos(p) => p.iter().for_each(|p| person(&p.hot, &p.cold)),
            Store::Split(hs, cs) => hs.iter().zip(cs).for_each(|(h, c)| person(h, c)),
        }
        for d in &self.density {
            mix(u32::from(d.load(Ordering::Relaxed)));
        }
        h
    }

    #[must_use]
    pub fn persons_bytes(&self) -> usize {
        match &self.store {
            Store::Aos(p) => p.len() * size_of::<Person>(),
            Store::Split(h, c) => h.len() * size_of::<Hot>() + c.len() * size_of::<Cold>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spike 1.3: walls built while a recomputation is still running must end
    /// up in the fields; after the last takeover the fields equal a full build.
    #[test]
    fn async_walls_converge_to_full_build() {
        let grid = Grid::reference_like(128, 128, 4);
        let mut sim = Sim::new(grid, 500, Layout::Split, 2, 7);
        let a: Vec<u32> = (40..60).map(|x| sim.grid.idx(x, 90, 0) as u32).collect();
        let b: Vec<u32> = (40..60).map(|y| sim.grid.idx(70, y, 0) as u32).collect();
        assert_eq!(sim.set_walls(&a, true).len(), a.len());
        // Second change while the first recomputation may still be running.
        assert!(!sim.set_walls(&b, true).is_empty());
        let mut takeovers = 0;
        for _ in 0..10_000 {
            sim.step(64);
            if sim.poll_async().is_some() {
                takeovers += 1;
            }
            if !sim.fields_outdated() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        assert!(!sim.fields_outdated(), "fields never caught up");
        assert!(takeovers >= 1);
        let full = Fields::build(&sim.grid);
        assert_eq!(sim.fields.dist, full.dist);
        // Clearing the walls again restores the original distances.
        sim.set_walls(&a, false);
        sim.set_walls(&b, false);
        while sim.fields_outdated() {
            sim.poll_async();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let original = Fields::build(&Grid::reference_like(128, 128, 4));
        assert_eq!(sim.fields.dist, original.dist);
    }
}
