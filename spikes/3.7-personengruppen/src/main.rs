//! Measurement driver for roadmap step 3.7 (groups of persons, walkable
//! areas, crossings). Throwaway code. Prints JSON lines.
//!
//! fields threads=4 runs=3
//!   Builds the reference terminal of the core (768² × 5, 17 gates, 36
//!   targets) in a world of 2048² and adds an apron north of it (service
//!   road, 17 stands, taxilane, taxiway, runway) and a landside forecourt
//!   south of it. For each access profile it derives a grid with the cells
//!   that profile may walk on and computes the production flow fields
//!   (`FieldSet::compute`) for the targets of that profile:
//!     base  terminal only, the 36 targets of today (benchmark of 2.3)
//!     pax   passengers: + forecourt, walk-out walkways and crossings, 6
//!           aircraft doors as extra targets (42)
//!     staff staff inside buildings: same cells as pax (staff rooms are
//!           inside the terminal), 16 staff targets
//!     ramp  ramp staff: pax cells + all 17 stand surfaces, 21 targets
//!     build construction workers: whole ground floor except the runway,
//!           8 construction sites
//!     union one set over the build cells with all 87 targets (variant
//!           "one topology for every group")
//!     people one set over the ramp cells with the 79 targets of pax, staff
//!           and ramp (variant "one topology for passengers and staff")
//!     build2 build cells with two fields: to the delivery point and to the
//!           nearest of all construction sites (one target with all sites)
//!
//! astar threads=1 trips=1000 seed=1
//!   Single route search (A*, same step rules and costs as the flow fields)
//!   on a dense copy of the profile grids: ramp staff between staff rooms
//!   and stands, staff between random cells of the terminal, construction
//!   workers from the delivery point to random cells of the ground floor.
//!   Admissible heuristic (cost factor 0.7) and heuristic with the normal
//!   cost (faster, not always shortest). Checks the admissible costs against
//!   the flow fields of the ramp profile.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;

use openairside_core::flowfield::{FieldInput, FieldSet};
use openairside_core::geometry::{CellPos, CellRect, MapSize};
use openairside_core::grid::{
    BLOCK, BLOCK_CELLS, Cell, FLOORS, GROUND_INDEX, Grid, STAIRS, WALL, Zone,
};
use openairside_core::scenario::reference_terminal;
use openairside_core::target::Target;
use openairside_core::world::World;

const MAP: u16 = 2048;
const EDGE: u16 = 768;
/// Terminal origin; the terminal covers 640 … 1407 in x and y.
const O: u16 = (MAP - EDGE) / 2;
const T1: u16 = O + EDGE;
/// Row of the terminal between landside (south) and airside (north).
const YB: u16 = O + EDGE / 2;

// Apron north of the terminal (rows): service road 628 … 639 (walkable only
// where a walkway crosses it), stands 584 … 627, taxilane 544 … 583, taxiway
// 504 … 543, runway 374 … 403. Only the build profile walks on road, taxilane
// and taxiway; nobody walks on the runway.
const STANDS: (u16, u16) = (584, 628);
const RUNWAY: (u16, u16) = (374, 404);
const RUNWAY_X: (u16, u16) = (274, 1774);
const FORECOURT: (u16, u16) = (T1, T1 + 128);
const STAND_W: u16 = 44;
const STAND_COUNT: u16 = 17;
const STAND_X0: u16 = 650;
/// Stands without a passenger bridge (walk-out, FR-052).
const WALKOUT: [u16; 6] = [0, 3, 6, 9, 12, 15];
/// Rows of the aircraft door on a stand.
const DOOR_Y: u16 = 600;
/// Staff rooms: west edges of four rooms inside the airside ground floor.
const STAFF_ROOMS: [u16; 4] = [O + 97, O + 289, O + 481, O + 673];
const STAFF_Y: (u16, u16) = (O + 289, O + 352);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Profile {
    Base,
    Pax,
    Staff,
    Ramp,
    Build,
}

impl Profile {
    fn name(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Pax => "pax",
            Self::Staff => "staff",
            Self::Ramp => "ramp",
            Self::Build => "build",
        }
    }
}

fn pos(x: u16, y: u16, floor: i8) -> CellPos {
    CellPos { x, y, floor }
}

fn rect(x0: u16, x1: u16, y0: u16, y1: u16) -> CellRect {
    CellRect {
        floor: 0,
        x0,
        y0,
        x1,
        y1,
    }
}

fn fill(grid: &mut Grid, r: CellRect, cell: Cell) {
    grid.fill(r, cell).expect("rectangle inside the map");
}

fn walkout_x(k: u16) -> u16 {
    STAND_X0 + k * STAND_W + 12
}

fn stand_centre(k: u16) -> (u16, u16) {
    (
        STAND_X0 + k * STAND_W + STAND_W / 2,
        (STANDS.0 + STANDS.1) / 2,
    )
}

/// World with the grid of one profile and the reference targets of the core.
fn build(profile: Profile) -> (World, Vec<Target>) {
    let map = MapSize::new(MAP, MAP).expect("valid map");
    let mut world = reference_terminal(map, EDGE, 17, 0, 1).expect("terminal fits");
    let reference = world.targets().to_vec();
    let grid = world.grid_mut();
    let air = Cell::floor(Zone::Airside);
    let land = Cell::floor(Zone::Landside);
    if profile == Profile::Base {
        return (world, reference);
    }
    if profile == Profile::Build {
        // Whole ground floor outside the terminal: airside north of the
        // boundary row, landside south of it; a security row west and an
        // exit row east of the terminal (gates in the fence).
        fill(grid, rect(0, MAP, 0, O), air);
        fill(grid, rect(0, O, O, YB), air);
        fill(grid, rect(T1, MAP, O, YB), air);
        fill(grid, rect(0, MAP, T1, MAP), land);
        fill(grid, rect(0, O, YB + 1, T1), land);
        fill(grid, rect(T1, MAP, YB + 1, T1), land);
        fill(grid, rect(0, O, YB, YB + 1), Cell::floor(Zone::Security));
        fill(grid, rect(T1, MAP, YB, YB + 1), Cell::floor(Zone::Exit));
        fill(
            grid,
            rect(RUNWAY_X.0, RUNWAY_X.1, RUNWAY.0, RUNWAY.1),
            Cell::VOID,
        );
    }
    // Forecourt south of the terminal with doors every 32 cells.
    fill(grid, rect(O, T1, FORECOURT.0, FORECOURT.1), land);
    for x in (O + 8..T1 - 8).step_by(32) {
        fill(grid, rect(x, x + 4, T1 - 1, T1), land);
    }
    // Walk-out walkways: door in the north wall, crossing over the service
    // road, walkway on the stand up to the aircraft door.
    for k in WALKOUT {
        let x = walkout_x(k);
        fill(grid, rect(x - 1, x + 2, DOOR_Y, O + 1), air);
    }
    if profile == Profile::Ramp {
        fill(
            grid,
            rect(
                STAND_X0,
                STAND_X0 + STAND_COUNT * STAND_W,
                STANDS.0,
                STANDS.1,
            ),
            air,
        );
    }
    (world, reference)
}

/// Walkable cells of the 3 × 3 square around (x, y) on `floor`.
fn square(grid: &Grid, x: u16, y: u16, floor: i8) -> Vec<CellPos> {
    let mut cells = Vec::new();
    for dy in 0..3 {
        for dx in 0..3 {
            let p = pos(x + dx - 1, y + dy - 1, floor);
            if grid.walkable(p) {
                cells.push(p);
            }
        }
    }
    assert!(!cells.is_empty(), "no walkable cell at {x},{y},{floor}");
    cells
}

fn staff_room_targets(grid: &Grid) -> Vec<Target> {
    STAFF_ROOMS
        .iter()
        .map(|&x| Target::passengers(square(grid, x + 31, (STAFF_Y.0 + STAFF_Y.1) / 2, 0)))
        .collect()
}

fn walkout_targets(grid: &Grid) -> Vec<Target> {
    WALKOUT
        .iter()
        .map(|&k| Target::passengers(square(grid, walkout_x(k), DOOR_Y + 1, 0)))
        .collect()
}

fn stand_targets(grid: &Grid) -> Vec<Target> {
    (0..STAND_COUNT)
        .map(|k| {
            let (x, y) = stand_centre(k);
            Target::passengers(square(grid, x, y, 0))
        })
        .collect()
}

/// 4 staff rooms and 12 workplaces (behind check-in, security, exits).
fn staff_targets(grid: &Grid) -> Vec<Target> {
    let mut targets = staff_room_targets(grid);
    for i in 0..6u16 {
        targets.push(Target::passengers(square(
            grid,
            O + 110 + 100 * i,
            T1 - 70,
            0,
        )));
        targets.push(Target::passengers(square(
            grid,
            O + 105 + 100 * i,
            YB - 6,
            0,
        )));
    }
    targets
}

const BUILD_SITES: [(u16, u16); 8] = [
    (300, 450),
    (1700, 470),
    (1000, 560),
    (1200, 520),
    (500, 1200),
    (1600, 1300),
    (1000, 300),
    (1024, 2000),
];

fn build_targets(grid: &Grid) -> Vec<Target> {
    BUILD_SITES
        .iter()
        .map(|&(x, y)| Target::passengers(square(grid, x, y, 0)))
        .collect()
}

fn targets_for(profile: Profile, grid: &Grid, reference: &[Target]) -> Vec<Target> {
    match profile {
        Profile::Base => reference.to_vec(),
        Profile::Pax => {
            let mut t = reference.to_vec();
            t.extend(walkout_targets(grid));
            t
        }
        Profile::Staff => staff_targets(grid),
        Profile::Ramp => {
            let mut t = stand_targets(grid);
            t.extend(staff_room_targets(grid));
            t
        }
        Profile::Build => build_targets(grid),
    }
}

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

fn arg(args: &[String], key: &str, default: usize) -> usize {
    args.iter()
        .find_map(|a| a.strip_prefix(&format!("{key}=")))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn measure_fields(
    label: &str,
    grid: &Grid,
    targets: Vec<Target>,
    pool: &rayon::ThreadPool,
    runs: usize,
) {
    let input = FieldInput {
        grid: grid.snapshot(),
        targets,
    };
    let mut times = Vec::new();
    let mut last = None;
    for _ in 0..runs {
        let start = Instant::now();
        let set = pool.install(|| FieldSet::compute(&input, 0));
        times.push(start.elapsed().as_secs_f64());
        last = Some(set);
    }
    let set = last.expect("runs > 0");
    let unreachable: u32 = (0..set.targets())
        .map(|t| set.unreachable_cells(u8::try_from(t).expect("< 256 targets")))
        .sum();
    let cells = set.cells();
    println!(
        "{{\"mode\":\"fields\",\"profile\":\"{label}\",\"targets\":{},\"blocks\":{},\"cells\":{},\"mb\":{:.1},\"mb_per_target\":{:.2},\"s_median\":{:.3},\"s_all\":{:?},\"unreachable_cells\":{},\"peak_mb\":{:.0}}}",
        set.targets(),
        cells / BLOCK_CELLS,
        cells,
        set.bytes() as f64 / 1e6,
        cells as f64 / 1e6,
        median(times.clone()),
        times
            .iter()
            .map(|t| (t * 1000.0).round() / 1000.0)
            .collect::<Vec<_>>(),
        unreachable,
        peak_mb()
    );
}

fn fields(args: &[String]) {
    let threads = arg(args, "threads", 4);
    let runs = arg(args, "runs", 3);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("thread pool");
    for profile in [
        Profile::Base,
        Profile::Pax,
        Profile::Staff,
        Profile::Ramp,
        Profile::Build,
    ] {
        let (world, reference) = build(profile);
        let targets = targets_for(profile, world.grid(), &reference);
        measure_fields(profile.name(), world.grid(), targets, &pool, runs);
    }
    // Variant: one topology (the build cells) for the targets of all groups.
    let (world, reference) = build(Profile::Build);
    let grid = world.grid();
    let mut all = targets_for(Profile::Pax, build(Profile::Pax).0.grid(), &reference);
    all.extend(staff_targets(grid));
    all.extend(targets_for(
        Profile::Ramp,
        build(Profile::Ramp).0.grid(),
        &reference,
    ));
    all.extend(build_targets(grid));
    measure_fields("union", grid, all, &pool, runs);
    // Variant: one topology for passengers and staff (the ramp cells) with
    // the targets of pax, staff and ramp, and the build profile with two
    // fields (to the delivery point, to the nearest of all sites).
    let (world, reference) = build(Profile::Ramp);
    let grid = world.grid();
    let mut people = targets_for(Profile::Pax, grid, &reference);
    people.extend(staff_targets(grid));
    people.extend(targets_for(Profile::Ramp, grid, &reference));
    measure_fields("people", grid, people, &pool, runs);
    let (world, _) = build(Profile::Build);
    let grid = world.grid();
    let sites: Vec<CellPos> = build_targets(grid)
        .into_iter()
        .flat_map(|t| t.cells)
        .collect();
    let delivery = Target::passengers(square(grid, 1024, FORECOURT.0 + 100, 0));
    measure_fields(
        "build2",
        grid,
        vec![delivery, Target::passengers(sites)],
        &pool,
        runs,
    );
}

/// Dense copy of a grid for the route search, indexed `(f · h + y) · w + x`.
struct Dense {
    w: usize,
    h: usize,
    flags: Vec<u8>,
    zone: Vec<u8>,
    cost: Vec<u8>,
}

impl Dense {
    fn from(grid: &Grid) -> Self {
        let (w, h) = (usize::from(MAP), usize::from(MAP));
        let n = FLOORS * w * h;
        let mut d = Self {
            w,
            h,
            flags: vec![WALL; n],
            zone: vec![0; n],
            cost: vec![10; n],
        };
        let snap = grid.snapshot();
        let (bx_n, by_n) = snap.block_counts();
        let per_floor = bx_n * by_n;
        for ti in 0..snap.table_len() {
            let Some(block) = snap.block(ti) else {
                continue;
            };
            let (f, rest) = (ti / per_floor, ti % per_floor);
            let (bx, by) = (rest % bx_n, rest / bx_n);
            for ly in 0..BLOCK {
                for lx in 0..BLOCK {
                    let local = ly * BLOCK + lx;
                    let i = d.idx(bx * BLOCK + lx, by * BLOCK + ly, f);
                    d.flags[i] = block.flags()[local];
                    d.zone[i] = block.zones()[local];
                    d.cost[i] = block.costs()[local];
                }
            }
        }
        d
    }

    fn idx(&self, x: usize, y: usize, f: usize) -> usize {
        (f * self.h + y) * self.w + x
    }

    fn xyf(&self, i: usize) -> (usize, usize, usize) {
        let plane = self.w * self.h;
        (i % self.w, (i % plane) / self.w, i / plane)
    }

    fn walkable(&self, i: usize) -> bool {
        self.flags[i] & WALL == 0
    }

    fn of(&self, p: CellPos) -> usize {
        let f = usize::try_from(i32::from(p.floor) + GROUND_INDEX as i32).expect("valid floor");
        self.idx(usize::from(p.x), usize::from(p.y), f)
    }
}

const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

/// Steps from `p` with the rules of `flowfield::Topology::step`.
fn steps(d: &Dense, p: usize, out: &mut Vec<(usize, u32)>) {
    out.clear();
    if !d.walkable(p) {
        return;
    }
    let (x, y, f) = d.xyf(p);
    let ok = |n: usize| d.walkable(n) && Zone::step_allowed(d.zone[p], d.zone[n]);
    for (dx, dy) in DIRS {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 || nx >= d.w as i32 || ny >= d.h as i32 {
            continue;
        }
        if dx != 0 && dy != 0 {
            let a = d.idx((x as i32 + dx) as usize, y, f);
            let b = d.idx(x, (y as i32 + dy) as usize, f);
            if !d.walkable(a) || !d.walkable(b) {
                continue;
            }
        }
        let n = d.idx(nx as usize, ny as usize, f);
        if ok(n) {
            let base = if dx != 0 && dy != 0 { 14 } else { 10 };
            out.push((n, (base * u32::from(d.cost[p]) / 10).max(1)));
        }
    }
    if d.flags[p] & STAIRS != 0 {
        for nf in [f.wrapping_sub(1), f + 1] {
            if nf < FLOORS {
                let n = d.idx(x, y, nf);
                if d.flags[n] & STAIRS != 0 && ok(n) {
                    out.push((n, 40));
                }
            }
        }
    }
}

struct Search {
    g: Vec<u32>,
    stamp: Vec<u32>,
    goal: Vec<u32>,
    parent: Vec<u32>,
    round: u32,
    heap: BinaryHeap<Reverse<(u32, u32, u32)>>,
    buf: Vec<(usize, u32)>,
}

struct Route {
    cost: u32,
    expanded: u32,
    cells: u32,
}

impl Search {
    fn new(n: usize) -> Self {
        Self {
            g: vec![0; n],
            stamp: vec![0; n],
            goal: vec![0; n],
            parent: vec![0; n],
            round: 0,
            heap: BinaryHeap::new(),
            buf: Vec::new(),
        }
    }

    /// A* from `start` to any of `goals`; heuristic octile distance with
    /// straight cost `a` and diagonal cost `b` plus 40 per floor.
    fn run(&mut self, d: &Dense, start: usize, goals: &[usize], a: u32, b: u32) -> Option<Route> {
        self.round += 1;
        let round = self.round;
        for &g in goals {
            self.goal[g] = round;
        }
        let gxyf: Vec<(usize, usize, usize)> = goals.iter().map(|&g| d.xyf(g)).collect();
        let h = |i: usize| -> u32 {
            let (x, y, f) = d.xyf(i);
            gxyf.iter()
                .map(|&(gx, gy, gf)| {
                    let dx = x.abs_diff(gx) as u32;
                    let dy = y.abs_diff(gy) as u32;
                    let (lo, hi) = (dx.min(dy), dx.max(dy));
                    a * (hi - lo) + b * lo + 40 * f.abs_diff(gf) as u32
                })
                .min()
                .unwrap_or(0)
        };
        self.heap.clear();
        self.g[start] = 0;
        self.stamp[start] = round;
        self.parent[start] = u32::MAX;
        self.heap.push(Reverse((h(start), 0, start as u32)));
        let mut expanded = 0u32;
        while let Some(Reverse((_, gp, p))) = self.heap.pop() {
            let p = p as usize;
            if gp > self.g[p] {
                continue;
            }
            expanded += 1;
            if self.goal[p] == round {
                let mut cells = 1;
                let mut c = p;
                while self.parent[c] != u32::MAX {
                    c = self.parent[c] as usize;
                    cells += 1;
                }
                return Some(Route {
                    cost: gp,
                    expanded,
                    cells,
                });
            }
            let mut buf = std::mem::take(&mut self.buf);
            steps(d, p, &mut buf);
            for &(n, c) in &buf {
                let gn = gp + c;
                if self.stamp[n] != round || gn < self.g[n] {
                    self.stamp[n] = round;
                    self.g[n] = gn;
                    self.parent[n] = p as u32;
                    self.heap.push(Reverse((gn + h(n), gn, n as u32)));
                }
            }
            self.buf = buf;
        }
        None
    }
}

/// splitmix64 for reproducible trips.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn pick<T: Copy>(&mut self, v: &[T]) -> T {
        v[(self.next() % v.len() as u64) as usize]
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    let i = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[i]
}

fn report(case: &str, heuristic: &str, results: &[(f64, Option<Route>)]) {
    let mut t: Vec<f64> = results.iter().map(|r| r.0).collect();
    t.sort_by(f64::total_cmp);
    let found: Vec<&Route> = results.iter().filter_map(|r| r.1.as_ref()).collect();
    let n = found.len().max(1) as f64;
    let mean_exp = found.iter().map(|r| f64::from(r.expanded)).sum::<f64>() / n;
    let mean_cells = found.iter().map(|r| f64::from(r.cells)).sum::<f64>() / n;
    let max_exp = found.iter().map(|r| r.expanded).max().unwrap_or(0);
    println!(
        "{{\"mode\":\"astar\",\"case\":\"{case}\",\"heuristic\":\"{heuristic}\",\"trips\":{},\"found\":{},\"ms_median\":{:.3},\"ms_p90\":{:.3},\"ms_p99\":{:.3},\"ms_max\":{:.3},\"ms_mean\":{:.3},\"expanded_mean\":{:.0},\"expanded_max\":{},\"path_cells_mean\":{:.0}}}",
        results.len(),
        found.len(),
        percentile(&t, 0.5) * 1e3,
        percentile(&t, 0.9) * 1e3,
        percentile(&t, 0.99) * 1e3,
        t[t.len() - 1] * 1e3,
        t.iter().sum::<f64>() / t.len() as f64 * 1e3,
        mean_exp,
        max_exp,
        mean_cells
    );
}

fn run_trips(
    case: &str,
    d: &Dense,
    search: &mut Search,
    trips: &[(usize, Vec<usize>)],
) -> Vec<Option<u32>> {
    let mut costs = Vec::new();
    for (heuristic, a, b) in [("admissible", 7, 9), ("normal_cost", 10, 14)] {
        let mut results = Vec::with_capacity(trips.len());
        for (start, goals) in trips {
            let t0 = Instant::now();
            let route = search.run(d, *start, goals, a, b);
            results.push((t0.elapsed().as_secs_f64(), route));
        }
        if heuristic == "admissible" {
            costs = results
                .iter()
                .map(|r| r.1.as_ref().map(|r| r.cost))
                .collect();
        }
        report(case, heuristic, &results);
    }
    costs
}

fn walkable_cells(
    d: &Dense,
    floor_only: Option<usize>,
    area: (usize, usize, usize, usize),
) -> Vec<usize> {
    let mut v = Vec::new();
    for f in 0..FLOORS {
        if floor_only.is_some_and(|g| g != f) {
            continue;
        }
        for y in area.2..area.3 {
            for x in area.0..area.1 {
                let i = d.idx(x, y, f);
                if d.walkable(i) {
                    v.push(i);
                }
            }
        }
    }
    v
}

fn astar(args: &[String]) {
    let trips = arg(args, "trips", 1000);
    let mut rng = Rng(arg(args, "seed", 1) as u64);
    let n = FLOORS * usize::from(MAP) * usize::from(MAP);
    let mut search = Search::new(n);

    // Ramp staff: staff rooms and stands, both directions; checked against
    // the flow fields of the ramp profile.
    let (world, reference) = build(Profile::Ramp);
    let grid = world.grid();
    let targets = targets_for(Profile::Ramp, grid, &reference);
    let d = Dense::from(grid);
    let cells: Vec<Vec<usize>> = targets
        .iter()
        .map(|t| t.cells.iter().map(|&p| d.of(p)).collect())
        .collect();
    let mut list = Vec::new();
    let mut check = Vec::new();
    for _ in 0..trips {
        let a = (rng.next() % targets.len() as u64) as usize;
        let mut b = (rng.next() % targets.len() as u64) as usize;
        if a == b {
            b = (b + 1) % targets.len();
        }
        let start = rng.pick(&cells[a]);
        list.push((start, cells[b].clone()));
        check.push((
            targets[a].cells[cells[a].iter().position(|&c| c == start).unwrap_or(0)],
            b,
        ));
    }
    let costs = run_trips("ramp_rooms_stands", &d, &mut search, &list);
    let input = FieldInput {
        grid: grid.snapshot(),
        targets,
    };
    let set = FieldSet::compute(&input, 0);
    let mismatches = check
        .iter()
        .zip(&costs)
        .filter(|((p, b), c)| set.path_cost(u8::try_from(*b).expect("small"), *p) != **c)
        .count();
    println!(
        "{{\"mode\":\"astar_check\",\"case\":\"ramp_rooms_stands\",\"trips\":{},\"cost_mismatches_vs_flow_field\":{}}}",
        check.len(),
        mismatches
    );

    // Staff inside the terminal: random cell to random cell, all floors.
    let (world, _) = build(Profile::Staff);
    let d = Dense::from(world.grid());
    let term = walkable_cells(
        &d,
        None,
        (
            usize::from(O),
            usize::from(T1),
            usize::from(O),
            usize::from(T1),
        ),
    );
    let list: Vec<(usize, Vec<usize>)> = (0..trips)
        .map(|_| (rng.pick(&term), vec![rng.pick(&term)]))
        .collect();
    run_trips("staff_terminal_random", &d, &mut search, &list);

    // Construction workers: delivery point on the forecourt to random cells
    // of the ground floor (apron, grass, taxiways, landside).
    let (world, _) = build(Profile::Build);
    let d = Dense::from(world.grid());
    let delivery = d.of(pos(1024, FORECOURT.0 + 100, 0));
    let ground = walkable_cells(
        &d,
        Some(GROUND_INDEX),
        (0, usize::from(MAP), 0, usize::from(MAP)),
    );
    let list: Vec<(usize, Vec<usize>)> = (0..trips)
        .map(|_| (delivery, vec![rng.pick(&ground)]))
        .collect();
    run_trips("build_delivery_to_site", &d, &mut search, &list);
    println!("{{\"mode\":\"astar_memory\",\"peak_mb\":{:.0}}}", peak_mb());
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("fields") => fields(&args[1..]),
        Some("astar") => astar(&args[1..]),
        _ => eprintln!("usage: personengruppen fields|astar [key=value …]"),
    }
}
