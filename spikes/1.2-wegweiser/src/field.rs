//! Direction fields ("Wegweiser"): one per target, computed backwards from the
//! target with Dijkstra over integer costs (bucket queue). Each walkable cell
//! stores one byte: the next step towards the target.

use crate::grid::{Grid, STAIRS, zone_step_allowed};
use rayon::prelude::*;
use std::time::Instant;

/// Planar directions 0..8.
pub const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
pub const UP: u8 = 8;
pub const DOWN: u8 = 9;
pub const ARRIVED: u8 = 254;
pub const UNREACHABLE: u8 = 255;

const STAIR_COST: u32 = 40;
const BUCKETS: usize = 64;

/// Why a cell cannot reach a target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cause {
    /// Reachable only when ignoring zone rules (e.g. all security lanes closed).
    ZoneRule,
    /// Enclosed by walls (also ignoring zone rules no path exists).
    Enclosed,
}

/// All direction fields for one grid state, immutable once built.
pub struct Fields {
    pub cells: usize,
    pub dirs: Vec<u8>,
    /// Distances per field (kept for incremental repair).
    pub dist: Vec<u32>,
    /// Per field: bitset of cells reachable when zone rules are ignored;
    /// only built for fields that have unreachable walkable cells.
    pub relaxed: Vec<Option<Vec<u64>>>,
    pub unreachable_cells: Vec<u32>,
    pub build_ms: f64,
    pub per_field_ms: Vec<f64>,
}

impl Fields {
    #[must_use]
    pub fn dir(&self, field: u8, cell: usize) -> u8 {
        self.dirs[usize::from(field) * self.cells + cell]
    }

    #[must_use]
    pub fn bytes(&self) -> usize {
        self.dirs.len()
            + self
                .relaxed
                .iter()
                .flatten()
                .map(|b| b.len() * 8)
                .sum::<usize>()
    }

    #[must_use]
    pub fn cause(&self, field: u8, cell: usize) -> Cause {
        match &self.relaxed[usize::from(field)] {
            Some(bits) if bits[cell / 64] >> (cell % 64) & 1 == 1 => Cause::ZoneRule,
            _ => Cause::Enclosed,
        }
    }

    /// Builds all fields in parallel on the current rayon pool.
    #[must_use]
    pub fn build(grid: &Grid) -> Self {
        let start = Instant::now();
        let cells = grid.cells();
        let results: Vec<(Vec<u8>, Vec<u32>, Option<Vec<u64>>, u32, f64)> = (0..grid.targets.len())
            .into_par_iter()
            .map(|k| {
                let t0 = Instant::now();
                let mut dir = vec![UNREACHABLE; cells];
                let mut dist = vec![u32::MAX; cells];
                dijkstra(grid, &grid.targets[k].cells, &mut dir, &mut dist, true);
                let unreachable = (0..cells)
                    .filter(|&i| grid.walkable(i) && dir[i] == UNREACHABLE)
                    .count() as u32;
                let relaxed = (unreachable > 0).then(|| relaxed_bits(grid, k));
                (
                    dir,
                    dist,
                    relaxed,
                    unreachable,
                    t0.elapsed().as_secs_f64() * 1e3,
                )
            })
            .collect();
        let mut dirs = Vec::with_capacity(cells * results.len());
        let mut dists = Vec::with_capacity(cells * results.len());
        let mut relaxed = Vec::new();
        let mut unreachable_cells = Vec::new();
        let mut per_field_ms = Vec::new();
        for (d, di, r, u, ms) in results {
            dirs.extend_from_slice(&d);
            dists.extend_from_slice(&di);
            relaxed.push(r);
            unreachable_cells.push(u);
            per_field_ms.push(ms);
        }
        Self {
            cells,
            dirs,
            dist: dists,
            relaxed,
            unreachable_cells,
            build_ms: start.elapsed().as_secs_f64() * 1e3,
            per_field_ms,
        }
    }
}

fn relaxed_bits(grid: &Grid, k: usize) -> Vec<u64> {
    let cells = grid.cells();
    let mut rdir = vec![UNREACHABLE; cells];
    let mut dist = vec![u32::MAX; cells];
    dijkstra(grid, &grid.targets[k].cells, &mut rdir, &mut dist, false);
    let mut bits = vec![0u64; cells.div_ceil(64)];
    for (i, &d) in rdir.iter().enumerate() {
        if d != UNREACHABLE {
            bits[i / 64] |= 1 << (i % 64);
        }
    }
    bits
}

/// Cost of stepping from `p` to its neighbour in direction `d`, or None if
/// the step is not allowed on `grid` (walls, corners, zone rules).
fn step_cost(grid: &Grid, p: usize, d: u8) -> Option<(usize, u32)> {
    let w = grid.w as isize;
    let plane = grid.w * grid.h;
    let n = match d {
        UP | DOWN => {
            let n = match d {
                UP if p < plane => p + plane,
                DOWN if p >= plane => p - plane,
                _ => return None,
            };
            if grid.flags[p] & STAIRS == 0 || grid.flags[n] & STAIRS == 0 {
                return None;
            }
            n
        }
        _ => {
            let (dx, dy) = DIRS[usize::from(d)];
            if dx != 0
                && dy != 0
                && (!grid.walkable((p as isize + dx as isize) as usize)
                    || !grid.walkable((p as isize + dy as isize * w) as usize))
            {
                return None;
            }
            (p as isize + dy as isize * w + dx as isize) as usize
        }
    };
    if !grid.walkable(p) || !grid.walkable(n) || !zone_step_allowed(grid.zone[p], grid.zone[n]) {
        return None;
    }
    let cost = match d {
        UP | DOWN => STAIR_COST,
        _ => {
            let (dx, dy) = DIRS[usize::from(d)];
            let base = if dx != 0 && dy != 0 { 14 } else { 10 };
            (base * u32::from(grid.cost[p]) / 10).max(1)
        }
    };
    Some((n, cost))
}

/// Directions a person on `p` could take on this grid (planar plus stairs).
fn step_dirs(grid: &Grid, p: usize) -> impl Iterator<Item = u8> + '_ {
    let plane = grid.w * grid.h;
    let stairs = grid.flags[p] & STAIRS != 0;
    let other = if p >= plane { p - plane } else { p + plane };
    let vertical =
        (stairs && grid.flags[other] & STAIRS != 0).then_some(if p >= plane { DOWN } else { UP });
    (0..8u8).chain(vertical)
}

impl Fields {
    /// Incremental repair after a build change on `grid` (new state).
    /// `changed` are cells whose walkability or cost changed. Only the cells
    /// whose path leads through a changed cell are recomputed; distances
    /// equal a full recomputation (directions may differ only on ties).
    #[must_use]
    pub fn repair(&self, grid: &Grid, changed: &[u32]) -> Self {
        let start = Instant::now();
        let cells = self.cells;
        let n_fields = grid.targets.len();
        let mut dirs = self.dirs.clone();
        let mut dist = self.dist.clone();
        let results: Vec<(Option<Vec<u64>>, u32, f64)> = dirs
            .par_chunks_mut(cells)
            .zip(dist.par_chunks_mut(cells))
            .enumerate()
            .map(|(k, (dir, dist))| {
                let t0 = Instant::now();
                repair_field(grid, &grid.targets[k].cells, dir, dist, changed);
                let unreachable = (0..cells)
                    .filter(|&i| grid.walkable(i) && dir[i] == UNREACHABLE)
                    .count() as u32;
                let relaxed = (unreachable > 0).then(|| relaxed_bits(grid, k));
                (relaxed, unreachable, t0.elapsed().as_secs_f64() * 1e3)
            })
            .collect();
        debug_assert_eq!(results.len(), n_fields);
        let mut relaxed = Vec::new();
        let mut unreachable_cells = Vec::new();
        let mut per_field_ms = Vec::new();
        for (r, u, ms) in results {
            relaxed.push(r);
            unreachable_cells.push(u);
            per_field_ms.push(ms);
        }
        Self {
            cells,
            dirs,
            dist,
            relaxed,
            unreachable_cells,
            build_ms: start.elapsed().as_secs_f64() * 1e3,
            per_field_ms,
        }
    }
}

fn repair_field(grid: &Grid, seeds: &[u32], dir: &mut [u8], dist: &mut [u32], changed: &[u32]) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let w = grid.w as isize;
    let plane = grid.w * grid.h;
    // 1. Affected set: changed cells plus every cell whose direction chain
    //    leads into an affected cell (subtree of the shortest-path tree).
    let mut affected: Vec<usize> = Vec::new();
    let mut mark = vec![false; dir.len()];
    let insert = |m: &mut Vec<bool>, c: usize| !std::mem::replace(&mut m[c], true);
    // Roots: changed cells and their neighbours (a new wall can also forbid a
    // diagonal step of a neighbour by the no-corner-cutting rule).
    for &c in changed {
        let c = c as usize;
        let roots = std::iter::once(c).chain(
            DIRS.iter()
                .map(|&(dx, dy)| (c as isize + dy as isize * w + dx as isize) as usize),
        );
        for r in roots {
            if r < dir.len() && insert(&mut mark, r) {
                affected.push(r);
            }
        }
    }
    let mut i = 0;
    while i < affected.len() {
        let c = affected[i];
        i += 1;
        // Children: neighbours whose stored direction points to c.
        for (j, &(dx, dy)) in DIRS.iter().enumerate() {
            let p = (c as isize - dy as isize * w - dx as isize) as usize;
            if p < dir.len() && dir[p] == j as u8 && insert(&mut mark, p) {
                affected.push(p);
            }
        }
        for (p, d) in [(c + plane, DOWN), (c.wrapping_sub(plane), UP)] {
            if p < dir.len() && dir[p] == d && insert(&mut mark, p) {
                affected.push(p);
            }
        }
    }
    for &c in &affected {
        dir[c] = UNREACHABLE;
        dist[c] = u32::MAX;
    }
    // 2. Seed affected cells from unaffected neighbours and target cells.
    let mut heap = BinaryHeap::new();
    for &s in seeds {
        let s = s as usize;
        if grid.walkable(s) && mark[s] {
            dist[s] = 0;
            dir[s] = ARRIVED;
            heap.push(Reverse((0u32, s)));
        }
    }
    for &p in &affected {
        if !grid.walkable(p) || dist[p] == 0 {
            continue;
        }
        for d in step_dirs(grid, p) {
            if let Some((n, cost)) = step_cost(grid, p, d)
                && !mark[n]
                && dist[n] != u32::MAX
                && dist[n] + cost < dist[p]
            {
                dist[p] = dist[n] + cost;
                dir[p] = d;
            }
        }
        if dist[p] != u32::MAX {
            heap.push(Reverse((dist[p], p)));
        }
    }
    // 3. Dijkstra from the seeds; relaxation may also improve unaffected cells
    //    (cost decreases, opened walls).
    while let Some(Reverse((d, c))) = heap.pop() {
        if d != dist[c] {
            continue;
        }
        let cf = c / plane;
        for (j, &(dx, dy)) in DIRS.iter().enumerate() {
            let p = (c as isize - dy as isize * w - dx as isize) as usize;
            if p >= dir.len() || p / plane != cf {
                continue;
            }
            if let Some((n, cost)) = step_cost(grid, p, j as u8)
                && n == c
                && d + cost < dist[p]
            {
                dist[p] = d + cost;
                dir[p] = j as u8;
                heap.push(Reverse((dist[p], p)));
            }
        }
        for (p, dd) in [(c + plane, DOWN), (c.wrapping_sub(plane), UP)] {
            if p < dir.len()
                && let Some((n, cost)) = step_cost(grid, p, dd)
                && n == c
                && d + cost < dist[p]
            {
                dist[p] = d + cost;
                dir[p] = dd;
                heap.push(Reverse((dist[p], p)));
            }
        }
    }
}

/// Backwards Dijkstra from `seeds`. For each reached cell p, `dir[p]` is the
/// step from p towards the target. With `zones == false` zone rules are ignored.
fn dijkstra(grid: &Grid, seeds: &[u32], dir: &mut [u8], dist: &mut [u32], zones: bool) {
    let w = grid.w as isize;
    let plane = grid.w * grid.h;
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); BUCKETS];
    let mut pending = 0usize;
    for &s in seeds {
        let s = s as usize;
        if grid.walkable(s) {
            dist[s] = 0;
            dir[s] = ARRIVED;
            buckets[0].push(s as u32);
            pending += 1;
        }
    }
    let mut cur: u32 = 0;
    while pending > 0 {
        let slot = cur as usize % BUCKETS;
        if buckets[slot].is_empty() {
            cur += 1;
            continue;
        }
        let batch = std::mem::take(&mut buckets[slot]);
        pending -= batch.len();
        for c in batch {
            let c = c as usize;
            if dist[c] != cur {
                continue;
            }
            let zc = grid.zone[c];
            let mut relax = |p: usize, step: u32, d: u8, dist: &mut [u32], dir: &mut [u8]| {
                if !grid.walkable(p) || (zones && !zone_step_allowed(grid.zone[p], zc)) {
                    return;
                }
                let nd = cur + step;
                if nd < dist[p] {
                    dist[p] = nd;
                    dir[p] = d;
                    buckets[nd as usize % BUCKETS].push(p as u32);
                    pending += 1;
                }
            };
            // Border cells are walls and never expanded, so neighbours of an
            // expanded cell are always inside the grid.
            for (j, &(dx, dy)) in DIRS.iter().enumerate() {
                let p = (c as isize + dy as isize * w + dx as isize) as usize;
                // No corner cutting along walls.
                if dx != 0
                    && dy != 0
                    && (!grid.walkable((c as isize + dx as isize) as usize)
                        || !grid.walkable((c as isize + dy as isize * w) as usize))
                {
                    continue;
                }
                let base = if dx != 0 && dy != 0 { 14 } else { 10 };
                let step = base * u32::from(grid.cost[p]) / 10;
                relax(p, step.max(1), ((j + 4) % 8) as u8, dist, dir);
            }
            if grid.flags[c] & STAIRS != 0 {
                let cf = c / plane;
                let other = if cf == 0 { c + plane } else { c - plane };
                if grid.flags[other] & STAIRS != 0 {
                    // From the other floor the step leads up (to floor 1) or down.
                    let d = if cf == 1 { UP } else { DOWN };
                    relax(other, STAIR_COST, d, dist, dir);
                }
            }
        }
        cur += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FNV-1a over all direction bytes; reference value from the first
    /// (coordinate-based) implementation on the 256 × 256 layout.
    fn hash(f: &Fields) -> u64 {
        f.dirs.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| {
            (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
        })
    }

    #[test]
    fn fields_stable() {
        let g = Grid::reference_like(256, 256, 17);
        let f = Fields::build(&g);
        assert_eq!(f.unreachable_cells.iter().max(), Some(&0));
        let expected = std::env::var("FIELDS_HASH").ok();
        let h = format!("{:016x}", hash(&f));
        eprintln!("fields hash {h}");
        if let Some(e) = expected {
            assert_eq!(h, e);
        }
    }

    fn assert_same(a: &[u32], b: &[u32], what: &str) {
        let diff = a.iter().zip(b).filter(|(x, y)| x != y).count();
        let first = a.iter().zip(b).position(|(x, y)| x != y);
        assert!(
            diff == 0,
            "{what}: {diff} cells differ, first at {first:?}: {:?}",
            first.map(|i| (a[i], b[i], i % (2 * 256 * 256)))
        );
    }

    #[test]
    fn repair_matches_full() {
        let g0 = Grid::reference_like(256, 256, 6);
        let f0 = Fields::build(&g0);
        let mut g1 = g0.clone();
        let mut changed = Vec::new();
        // Close every second landside door, close all security lanes, raise a cost area.
        for (k, &d) in g0.doors.iter().enumerate() {
            if k % 2 == 0 {
                g1.flags[d as usize] |= crate::grid::WALL;
                changed.push(d);
            }
        }
        for &s in &g0.security_cells {
            g1.flags[s as usize] |= crate::grid::WALL;
            changed.push(s);
        }
        for y in 150..170 {
            for x in 100..140 {
                let i = g1.idx(x, y, 0);
                g1.cost[i] = crate::grid::COST_AVOID;
                changed.push(i as u32);
            }
        }
        let full1 = Fields::build(&g1);
        let rep1 = f0.repair(&g1, &changed);
        assert_same(&full1.dist, &rep1.dist, "repair after closing");
        assert_eq!(full1.unreachable_cells, rep1.unreachable_cells);
        // And back (opening walls, lowering costs).
        let full0 = Fields::build(&g0);
        let rep0 = rep1.repair(&g0, &changed);
        assert_same(&full0.dist, &rep0.dist, "repair after reopening");
        // Every stored direction must be a valid step to a cell with lower distance.
        for k in 0..g0.targets.len() {
            for c in 0..g0.cells() {
                let d = rep0.dir(k as u8, c);
                if d != UNREACHABLE && d != ARRIVED {
                    let (n, cost) = step_cost(&g0, c, d).expect("valid step");
                    assert_eq!(
                        rep0.dist[k * g0.cells() + n] + cost,
                        rep0.dist[k * g0.cells() + c]
                    );
                }
            }
        }
    }
}
