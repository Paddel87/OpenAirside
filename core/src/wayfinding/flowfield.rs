//! Flow fields ("Wegweiser", ADR-004, ADR-010, ADR-020): one field per
//! target, computed backwards from the target with Dijkstra over integer
//! costs (bucket queue). Each walkable cell stores one byte: the next step
//! towards the target.
//!
//! Fields exist only in blocks of 32 × 32 cells that contain walkable cells.
//! A [`FieldSet`] is built completely from a snapshot of the grid, on any
//! thread, and never changed afterwards; the simulation takes over a new set
//! as a whole (see `adoption`). Building is deterministic: the same snapshot
//! gives the same bytes regardless of thread count.

use rayon::prelude::*;

use crate::build::access::Rules;
use crate::build::grid::{BLOCK, BLOCK_CELLS, FLOORS, GridSnapshot, STAIRS, floor_index};
use crate::build::target::Target;
use crate::geometry::CellPos;

use super::gauge::Token;

/// Planar directions 0 … 7 as (dx, dy): east, south-east, south, …
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
/// Step to the floor above (stairs).
pub const UP: u8 = 8;
/// Step to the floor below (stairs).
pub const DOWN: u8 = 9;
/// The cell belongs to the target.
pub const ARRIVED: u8 = 254;
/// No way to the target from this cell.
pub const UNREACHABLE: u8 = 255;

/// Cost of a step on stairs.
const STAIR_COST: u32 = 40;
/// Bucket count of the queue; larger than every step cost
/// (14 · `MAX_COST` / 10 = 56, `STAIR_COST` = 40).
const BUCKETS: usize = 64;
const NONE: u32 = u32::MAX;

/// Everything a computation needs; cheap to create, owned by the job.
#[derive(Clone)]
pub struct FieldInput {
    /// Snapshot of the grid.
    pub grid: GridSnapshot,
    /// Targets; the index in this list is the field number.
    pub targets: Vec<Target>,
}

/// Index of the direction `(dx, dy)` in [`DIRS`].
const fn dir_index(dx: i32, dy: i32) -> usize {
    match (dx, dy) {
        (1, 0) => 0,
        (1, 1) => 1,
        (0, 1) => 2,
        (-1, 1) => 3,
        (-1, 0) => 4,
        (-1, -1) => 5,
        (0, -1) => 6,
        _ => 7,
    }
}

/// Compact numbering of the walkable blocks and their neighbourhood.
struct Topology {
    blocks_x: usize,
    blocks_y: usize,
    /// Table index of the grid → slot, or `NONE`.
    slot_of: Vec<u32>,
    /// Planar neighbour slots of every slot, in the order of [`DIRS`].
    neighbours: Vec<[u32; 8]>,
    up: Vec<u32>,
    down: Vec<u32>,
    flags: Vec<u8>,
    zone: Vec<u8>,
    cost: Vec<u8>,
}

impl Topology {
    fn build(grid: &GridSnapshot) -> Self {
        let (blocks_x, blocks_y) = grid.block_counts();
        let per_floor = blocks_x * blocks_y;
        let mut slot_of = vec![NONE; grid.table_len()];
        let mut keys = Vec::new();
        for (table_index, slot) in slot_of.iter_mut().enumerate() {
            if grid
                .block(table_index)
                .is_some_and(|b| b.walkable_cells() > 0)
            {
                *slot = u32::try_from(keys.len()).unwrap_or(NONE);
                keys.push(table_index);
            }
        }
        let lookup = |floor: usize, bx: i64, by: i64| -> u32 {
            let inside = (0..blocks_x as i64).contains(&bx) && (0..blocks_y as i64).contains(&by);
            if !inside || floor >= FLOORS {
                return NONE;
            }
            slot_of[floor * per_floor + by as usize * blocks_x + bx as usize]
        };
        let mut neighbours = Vec::with_capacity(keys.len());
        let mut up = Vec::with_capacity(keys.len());
        let mut down = Vec::with_capacity(keys.len());
        for &key in &keys {
            let (floor, rest) = (key / per_floor, key % per_floor);
            let (bx, by) = ((rest % blocks_x) as i64, (rest / blocks_x) as i64);
            neighbours
                .push(DIRS.map(|(dx, dy)| lookup(floor, bx + i64::from(dx), by + i64::from(dy))));
            up.push(lookup(floor + 1, bx, by));
            down.push(if floor == 0 {
                NONE
            } else {
                lookup(floor - 1, bx, by)
            });
        }
        let mut flags = Vec::with_capacity(keys.len() * BLOCK_CELLS);
        let mut zone = Vec::with_capacity(keys.len() * BLOCK_CELLS);
        let mut cost = Vec::with_capacity(keys.len() * BLOCK_CELLS);
        for &key in &keys {
            if let Some(block) = grid.block(key) {
                flags.extend_from_slice(block.flags());
                zone.extend_from_slice(block.zones());
                cost.extend_from_slice(block.costs());
            }
        }
        Self {
            blocks_x,
            blocks_y,
            slot_of,
            neighbours,
            up,
            down,
            flags,
            zone,
            cost,
        }
    }

    fn cell_count(&self) -> usize {
        self.flags.len()
    }

    /// `true` if a person with `rules` may stand on cell `c`.
    fn walkable(&self, c: u32, rules: Rules) -> bool {
        rules.walkable(self.flags[c as usize])
    }

    /// Compact index of a cell, `None` if its block holds no walkable cell.
    fn index_of(&self, pos: CellPos) -> Option<u32> {
        let floor = floor_index(pos.floor)?;
        let (x, y) = (usize::from(pos.x), usize::from(pos.y));
        let (bx, by) = (x / BLOCK, y / BLOCK);
        if bx >= self.blocks_x || by >= self.blocks_y {
            return None;
        }
        let slot = self.slot_of[(floor * self.blocks_y + by) * self.blocks_x + bx];
        (slot != NONE).then(|| slot * BLOCK_CELLS as u32 + ((y % BLOCK) * BLOCK + x % BLOCK) as u32)
    }

    /// Neighbour of `c` in planar direction `j`, if its block is stored.
    fn neighbour(&self, c: u32, j: usize) -> Option<u32> {
        let slot = c as usize / BLOCK_CELLS;
        let local = c as usize % BLOCK_CELLS;
        let (dx, dy) = DIRS[j];
        let nx = (local % BLOCK) as i32 + dx;
        let ny = (local / BLOCK) as i32 + dy;
        // Fast path: most neighbours lie in the same block.
        if (0..BLOCK as i32).contains(&nx) && (0..BLOCK as i32).contains(&ny) {
            return Some((slot * BLOCK_CELLS) as u32 + (ny as u32) * BLOCK as u32 + nx as u32);
        }
        let block = (nx.div_euclid(BLOCK as i32), ny.div_euclid(BLOCK as i32));
        let target = if block == (0, 0) {
            slot as u32
        } else {
            self.neighbours[slot][dir_index(block.0, block.1)]
        };
        (target != NONE).then(|| {
            target * BLOCK_CELLS as u32
                + (ny.rem_euclid(BLOCK as i32) as u32) * BLOCK as u32
                + nx.rem_euclid(BLOCK as i32) as u32
        })
    }

    /// Neighbour across the corner rule: a diagonal step needs both
    /// orthogonal neighbours of `c` to be walkable.
    fn diagonal_open(&self, c: u32, j: usize, rules: Rules) -> bool {
        let (dx, dy) = DIRS[j];
        if dx == 0 || dy == 0 {
            return true;
        }
        [dir_index(dx, 0), dir_index(0, dy)].into_iter().all(|k| {
            self.neighbour(c, k)
                .is_some_and(|n| self.walkable(n, rules))
        })
    }

    fn vertical(&self, c: u32, direction: u8) -> Option<u32> {
        let slot = c as usize / BLOCK_CELLS;
        let other = if direction == UP {
            self.up[slot]
        } else {
            self.down[slot]
        };
        (other != NONE).then(|| other * BLOCK_CELLS as u32 + c % BLOCK_CELLS as u32)
    }

    /// Cell reached by the step `direction` from `p` and its cost, if the
    /// step is allowed (walls, corners, access rules, stairs).
    fn step(&self, p: u32, direction: u8, rules: Rules) -> Option<(u32, u32)> {
        let n = match direction {
            UP | DOWN => {
                let n = self.vertical(p, direction)?;
                if self.flags[p as usize] & STAIRS == 0 || self.flags[n as usize] & STAIRS == 0 {
                    return None;
                }
                n
            }
            0..=7 => {
                let j = usize::from(direction);
                if !self.diagonal_open(p, j, rules) {
                    return None;
                }
                self.neighbour(p, j)?
            }
            _ => return None,
        };
        if !rules.step_allowed(
            self.flags[p as usize],
            self.zone[p as usize],
            self.flags[n as usize],
            self.zone[n as usize],
        ) {
            return None;
        }
        let cost = match direction {
            UP | DOWN => STAIR_COST,
            _ => {
                let (dx, dy) = DIRS[usize::from(direction)];
                let base = if dx != 0 && dy != 0 { 14 } else { 10 };
                (base * u32::from(self.cost[p as usize]) / 10).max(1)
            }
        };
        Some((n, cost))
    }
}

/// Backwards Dijkstra from `seeds` with the access `rules`: `dir[p]` becomes
/// the step from `p` towards the nearest seed. `dist` is scratch space of the
/// same length.
fn dijkstra(topo: &Topology, seeds: &[u32], rules: Rules, dir: &mut [u8], dist: &mut [u32]) {
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); BUCKETS];
    let mut pending = 0usize;
    for &s in seeds {
        if topo.walkable(s, rules) && dist[s as usize] != 0 {
            dist[s as usize] = 0;
            dir[s as usize] = ARRIVED;
            buckets[0].push(s);
            pending += 1;
        }
    }
    let mut cur: u32 = 0;
    let mut batch: Vec<u32> = Vec::new();
    while pending > 0 {
        let slot = cur as usize % BUCKETS;
        if buckets[slot].is_empty() {
            cur += 1;
            continue;
        }
        // Swap with a spare buffer so that bucket capacity is reused.
        std::mem::swap(&mut buckets[slot], &mut batch);
        pending -= batch.len();
        for c in batch.drain(..) {
            if dist[c as usize] != cur {
                continue;
            }
            let mut relax = |p: u32, step: u32, d: u8| {
                if !rules.step_allowed(
                    topo.flags[p as usize],
                    topo.zone[p as usize],
                    topo.flags[c as usize],
                    topo.zone[c as usize],
                ) {
                    return;
                }
                let nd = cur + step;
                if nd < dist[p as usize] {
                    dist[p as usize] = nd;
                    dir[p as usize] = d;
                    buckets[nd as usize % BUCKETS].push(p);
                    pending += 1;
                }
            };
            // All eight neighbours once; the corner rule reads the
            // orthogonal ones from this array instead of looking them up again.
            let around: [Option<u32>; 8] = std::array::from_fn(|j| topo.neighbour(c, j));
            let open = |k: usize| around[k].is_some_and(|n| topo.walkable(n, rules));
            for (j, &(dx, dy)) in DIRS.iter().enumerate() {
                let Some(p) = around[j] else {
                    continue;
                };
                if dx != 0 && dy != 0 && !(open(dir_index(dx, 0)) && open(dir_index(0, dy))) {
                    continue;
                }
                let base = if dx != 0 && dy != 0 { 14 } else { 10 };
                let step = (base * u32::from(topo.cost[p as usize]) / 10).max(1);
                relax(p, step, ((j + 4) % 8) as u8);
            }
            if topo.flags[c as usize] & STAIRS != 0 {
                // From the floor above the step leads down to c, from the
                // floor below it leads up.
                if let Some(p) = topo.vertical(c, UP)
                    && topo.flags[p as usize] & STAIRS != 0
                {
                    relax(p, STAIR_COST, DOWN);
                }
                if let Some(p) = topo.vertical(c, DOWN)
                    && topo.flags[p as usize] & STAIRS != 0
                {
                    relax(p, STAIR_COST, UP);
                }
            }
        }
        cur += 1;
    }
}

/// All flow fields for one grid state, immutable once built.
pub struct FieldSet {
    revision: u32,
    /// Input the set was computed from; a save game stores it instead of
    /// the fields (S-5), and the grid snapshot shares unchanged blocks.
    source: FieldInput,
    topo: Topology,
    targets: usize,
    data: Vec<u8>,
    unreachable: Vec<u32>,
    /// Counts the set in a [`super::gauge::Gauge`] while it lives (ADR-041).
    token: Option<Token>,
}

impl FieldSet {
    /// A set without any field, for a world that has no targets yet.
    #[must_use]
    pub fn empty(grid: &GridSnapshot, revision: u32) -> Self {
        Self::compute(
            &FieldInput {
                grid: grid.clone(),
                targets: Vec::new(),
            },
            revision,
        )
    }

    /// Computes all fields; fields are computed in parallel on the current
    /// rayon pool, each one sequentially, so the result does not depend on
    /// the number of threads.
    #[must_use]
    pub fn compute(input: &FieldInput, revision: u32) -> Self {
        let topo = Topology::build(&input.grid);
        let cells = topo.cell_count();
        let targets = input.targets.len();
        let mut data = vec![UNREACHABLE; cells * targets];
        let mut unreachable = vec![0u32; targets];
        if cells > 0 && targets > 0 {
            let seeds: Vec<Vec<u32>> = input
                .targets
                .iter()
                .map(|t| t.cells.iter().filter_map(|&p| topo.index_of(p)).collect())
                .collect();
            let rules: Vec<Rules> = input.targets.iter().map(|t| t.profile.rules()).collect();
            data.par_chunks_mut(cells)
                .zip(unreachable.par_iter_mut())
                .zip(seeds.par_iter().zip(rules.par_iter()))
                .for_each_init(
                    || vec![u32::MAX; cells],
                    |dist, ((dir, count), (seeds, &rules))| {
                        dist.fill(u32::MAX);
                        dijkstra(&topo, seeds, rules, dir, dist);
                        *count = (0..cells as u32)
                            .filter(|&c| topo.walkable(c, rules) && dir[c as usize] == UNREACHABLE)
                            .count() as u32;
                    },
                );
        }
        Self {
            revision,
            source: input.clone(),
            topo,
            targets,
            data,
            unreachable,
            token: None,
        }
    }

    /// The set, counted by `token` for as long as it lives.
    #[must_use]
    pub fn tracked(mut self, token: Token) -> Self {
        self.token = Some(token);
        self
    }

    /// `true` if the set is counted by a gauge.
    #[must_use]
    pub fn is_tracked(&self) -> bool {
        self.token.is_some()
    }

    /// Input this set was computed from.
    #[must_use]
    pub fn source(&self) -> &FieldInput {
        &self.source
    }

    /// Revision of the request this set was computed for.
    #[must_use]
    pub fn revision(&self) -> u32 {
        self.revision
    }

    /// Number of fields.
    #[must_use]
    pub fn targets(&self) -> usize {
        self.targets
    }

    /// Number of cells covered (cells of all blocks with walkable cells).
    #[must_use]
    pub fn cells(&self) -> usize {
        self.topo.cell_count()
    }

    /// Memory of the direction bytes.
    #[must_use]
    pub fn bytes(&self) -> usize {
        self.data.len()
    }

    /// Walkable cells without a way to the target.
    #[must_use]
    pub fn unreachable_cells(&self, target: u8) -> u32 {
        self.unreachable
            .get(usize::from(target))
            .copied()
            .unwrap_or(0)
    }

    /// Direction byte for `pos` towards `target`: 0 … 7 planar, [`UP`],
    /// [`DOWN`], [`ARRIVED`] or [`UNREACHABLE`] (also for cells without a
    /// field and for unknown targets).
    #[must_use]
    pub fn dir(&self, target: u8, pos: CellPos) -> u8 {
        let field = usize::from(target);
        if field >= self.targets {
            return UNREACHABLE;
        }
        self.topo.index_of(pos).map_or(UNREACHABLE, |c| {
            self.data[field * self.topo.cell_count() + c as usize]
        })
    }

    /// Cost of the way from `pos` to the target when following the field;
    /// `None` if there is no way. Costs are in tenths of a cell step.
    #[must_use]
    pub fn path_cost(&self, target: u8, pos: CellPos) -> Option<u32> {
        let field = usize::from(target);
        if field >= self.targets {
            return None;
        }
        let rules = self.rules_of(field);
        let base = field * self.topo.cell_count();
        let mut c = self.topo.index_of(pos)?;
        let mut total = 0u32;
        // A valid field has no cycle; the bound guards against corrupt data.
        for _ in 0..=self.topo.cell_count() {
            match self.data[base + c as usize] {
                ARRIVED => return Some(total),
                UNREACHABLE => return None,
                d => {
                    let (next, cost) = self.topo.step(c, d, rules)?;
                    total += cost;
                    c = next;
                }
            }
        }
        None
    }

    /// Access rules of the persons walking to field `field`.
    fn rules_of(&self, field: usize) -> Rules {
        self.source.targets[field].profile.rules()
    }

    /// Cell reached by the step `direction` from `pos` for a person walking
    /// to `target`, if the step is allowed under the access rules of that
    /// target (`None` for an unknown target).
    #[must_use]
    pub fn step_target(&self, target: u8, pos: CellPos, direction: u8) -> Option<CellPos> {
        let field = usize::from(target);
        if field >= self.targets {
            return None;
        }
        let c = self.topo.index_of(pos)?;
        self.topo.step(c, direction, self.rules_of(field))?;
        let (dx, dy, df) = match direction {
            UP => (0, 0, 1),
            DOWN => (0, 0, -1),
            d => {
                let (dx, dy) = DIRS[usize::from(d)];
                (dx, dy, 0)
            }
        };
        Some(CellPos {
            x: u16::try_from(i32::from(pos.x) + dx).ok()?,
            y: u16::try_from(i32::from(pos.y) + dy).ok()?,
            floor: pos.floor + df,
        })
    }

    /// Feeds the complete set into `hasher`.
    pub fn hash_into(&self, hasher: &mut crate::hash::StateHasher) {
        hasher.u32(self.revision);
        hasher.u32(self.targets as u32);
        hasher.bytes(&self.data);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    use super::*;
    use crate::build::access::AccessProfile;
    use crate::build::grid::{
        AreaKind, COST_AVOID, COST_MARKED, COST_NORMAL, Cell, Grid, WALL, Zone,
    };
    use crate::geometry::MapSize;
    use crate::hash::StateHasher;

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        }

        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }

    fn pos(x: u16, y: u16, floor: i8) -> CellPos {
        CellPos { x, y, floor }
    }

    /// Random layout over several block borders and three floors.
    fn random_grid(seed: u64) -> Grid {
        let mut rng = Rng(seed);
        let mut grid = Grid::new(MapSize::new(100, 70).unwrap());
        for floor in -1..=1i8 {
            for y in 0..70u16 {
                for x in 0..100u16 {
                    if floor != 0 && rng.below(3) == 0 {
                        continue; // void
                    }
                    let wall = rng.below(100) < 22;
                    let flags = if wall {
                        WALL
                    } else if rng.below(100) < 6 {
                        STAIRS
                    } else {
                        0
                    };
                    let zone = if x < 35 {
                        Zone::Landside
                    } else if x < 40 {
                        Zone::Security
                    } else if x < 90 {
                        Zone::Airside
                    } else {
                        Zone::Exit
                    };
                    let cost =
                        [COST_NORMAL, COST_NORMAL, COST_MARKED, COST_AVOID][rng.below(4) as usize];
                    grid.set(
                        pos(x, y, floor),
                        Cell {
                            flags,
                            zone: zone as u8,
                            cost,
                            ..Cell::VOID
                        },
                    )
                    .unwrap();
                }
            }
        }
        grid
    }

    /// Straightforward dense reference: Dijkstra with a binary heap over
    /// positions, the rules written directly against the `Grid` API.
    fn reference(grid: &Grid, seeds: &[CellPos]) -> std::collections::HashMap<CellPos, u32> {
        let mut dist = std::collections::HashMap::new();
        let mut heap = BinaryHeap::new();
        for &s in seeds {
            if grid.walkable(s) && !dist.contains_key(&s) {
                dist.insert(s, 0u32);
                heap.push(Reverse((0u32, s.floor, s.y, s.x)));
            }
        }
        let cell_at = |x: i32, y: i32, f: i8| -> Option<Cell> {
            let x = u16::try_from(x).ok()?;
            let y = u16::try_from(y).ok()?;
            grid.cell(pos(x, y, f)).filter(|c| c.walkable())
        };
        while let Some(Reverse((d, f, y, x))) = heap.pop() {
            let c = pos(x, y, f);
            if dist[&c] != d {
                continue;
            }
            let here = grid.cell(c).unwrap();
            let offer =
                |p: CellPos,
                 step: u32,
                 from_zone: u8,
                 dist: &mut std::collections::HashMap<CellPos, u32>,
                 heap: &mut BinaryHeap<Reverse<(u32, i8, u16, u16)>>| {
                    if !Zone::step_allowed(from_zone, here.zone) {
                        return;
                    }
                    let nd = d + step;
                    if dist.get(&p).is_none_or(|&old| nd < old) {
                        dist.insert(p, nd);
                        heap.push(Reverse((nd, p.floor, p.y, p.x)));
                    }
                };
            for &(dx, dy) in &DIRS {
                let (px, py) = (i32::from(x) + dx, i32::from(y) + dy);
                let Some(pc) = cell_at(px, py, f) else {
                    continue;
                };
                if dx != 0
                    && dy != 0
                    && (cell_at(i32::from(x) + dx, i32::from(y), f).is_none()
                        || cell_at(i32::from(x), i32::from(y) + dy, f).is_none())
                {
                    continue;
                }
                let base = if dx != 0 && dy != 0 { 14 } else { 10 };
                let step = (base * u32::from(pc.cost) / 10).max(1);
                offer(
                    pos(px as u16, py as u16, f),
                    step,
                    pc.zone,
                    &mut dist,
                    &mut heap,
                );
            }
            if here.flags & STAIRS != 0 {
                for df in [-1i8, 1] {
                    if let Some(pc) = cell_at(i32::from(x), i32::from(y), f + df)
                        && pc.flags & STAIRS != 0
                    {
                        offer(pos(x, y, f + df), STAIR_COST, pc.zone, &mut dist, &mut heap);
                    }
                }
            }
        }
        dist
    }

    fn pool(threads: usize) -> rayon::ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
    }

    fn compute(grid: &Grid, targets: Vec<Target>, threads: usize) -> FieldSet {
        let input = FieldInput {
            grid: grid.snapshot(),
            targets,
        };
        pool(threads).install(|| FieldSet::compute(&input, 1))
    }

    fn random_targets(grid: &Grid, rng: &mut Rng, count: usize) -> Vec<Target> {
        let mut targets = Vec::new();
        while targets.len() < count {
            let seeds: Vec<CellPos> = (0..3)
                .map(|_| {
                    pos(
                        rng.below(100) as u16,
                        rng.below(70) as u16,
                        rng.below(3) as i8 - 1,
                    )
                })
                .filter(|&p| grid.walkable(p))
                .collect();
            if !seeds.is_empty() {
                targets.push(Target::passengers(seeds));
            }
        }
        targets
    }

    /// Distances of the block-based computation equal the dense reference
    /// for every cell, also after changing the grid (full recomputation).
    #[test]
    fn distances_equal_the_dense_reference_before_and_after_building() {
        for seed in 1..=4u64 {
            let mut grid = random_grid(seed);
            let mut rng = Rng(seed * 77);
            let targets = random_targets(&grid, &mut rng, 5);
            for round in 0..2 {
                let set = compute(&grid, targets.clone(), 3);
                for (k, target) in targets.iter().enumerate() {
                    let reference = reference(&grid, &target.cells);
                    for floor in -1..=1i8 {
                        for y in 0..70u16 {
                            for x in 0..100u16 {
                                let p = pos(x, y, floor);
                                if !grid.walkable(p) {
                                    continue;
                                }
                                let got = set.path_cost(k as u8, p);
                                assert_eq!(
                                    got,
                                    reference.get(&p).copied(),
                                    "seed {seed} round {round} target {k} at {p:?}"
                                );
                                assert_eq!(set.dir(k as u8, p) == UNREACHABLE, got.is_none());
                            }
                        }
                    }
                    let unreachable = reference_unreachable(&grid, &reference);
                    assert_eq!(set.unreachable_cells(k as u8), unreachable);
                }
                // Build actions: close doors and open a corridor, then recompute.
                for i in 0..40u16 {
                    grid.set(pos(20 + i, 33, 0), Cell::VOID).unwrap();
                    grid.set(pos(50, 10 + i, 0), Cell::floor(Zone::Airside))
                        .unwrap();
                    grid.set(pos(51, 10 + i, 1), Cell::floor(Zone::Airside))
                        .unwrap();
                }
            }
        }
    }

    fn reference_unreachable(
        grid: &Grid,
        reference: &std::collections::HashMap<CellPos, u32>,
    ) -> u32 {
        let mut n = 0;
        for floor in -2..=2i8 {
            for y in 0..70u16 {
                for x in 0..100u16 {
                    let p = pos(x, y, floor);
                    // Only cells in blocks that hold walkable cells have a field.
                    if grid.walkable(p) && !reference.contains_key(&p) {
                        n += 1;
                    }
                }
            }
        }
        n
    }

    #[test]
    fn result_is_bit_identical_for_any_thread_count() {
        let grid = random_grid(9);
        let mut rng = Rng(5);
        let targets = random_targets(&grid, &mut rng, 7);
        let hash = |set: &FieldSet| {
            let mut h = StateHasher::new();
            set.hash_into(&mut h);
            h.finish()
        };
        let one = hash(&compute(&grid, targets.clone(), 1));
        for threads in [2, 5, 8] {
            assert_eq!(
                hash(&compute(&grid, targets.clone(), threads)),
                one,
                "{threads} threads"
            );
        }
    }

    #[test]
    fn fields_exist_only_in_blocks_with_walkable_cells() {
        let mut grid = Grid::new(MapSize::new(2048, 2048).unwrap());
        for x in 100..140u16 {
            grid.set(pos(x, 100, 0), Cell::floor(Zone::Landside))
                .unwrap();
            grid.set(pos(x, 100, 1), Cell::floor(Zone::Landside))
                .unwrap();
        }
        let set = compute(&grid, vec![Target::passengers(vec![pos(100, 100, 0)])], 2);
        // Blocks x = 3 and 4 (cells 96 … 159) on two floors → 4 blocks.
        assert_eq!(set.cells(), 4 * BLOCK_CELLS);
        assert_eq!(set.bytes(), set.cells());
        assert_eq!(set.targets(), 1);
        assert_eq!(set.dir(0, pos(100, 100, 0)), ARRIVED);
        assert_eq!(set.dir(0, pos(139, 100, 0)), 4, "walk west");
        assert_eq!(set.dir(0, pos(139, 100, 1)), UNREACHABLE, "no stairs");
        assert_eq!(set.unreachable_cells(0), 40);
        assert_eq!(set.path_cost(0, pos(139, 100, 0)), Some(390));
        // Cells without a field and unknown targets.
        assert_eq!(set.dir(0, pos(1000, 1000, 0)), UNREACHABLE);
        assert_eq!(set.dir(0, pos(5000, 5, 0)), UNREACHABLE);
        assert_eq!(set.dir(0, pos(5, 5, 7)), UNREACHABLE);
        assert_eq!(set.dir(3, pos(100, 100, 0)), UNREACHABLE);
        assert_eq!(set.path_cost(3, pos(100, 100, 0)), None);
        assert_eq!(set.path_cost(0, pos(1000, 1000, 0)), None);
        assert_eq!(set.unreachable_cells(9), 0);
    }

    #[test]
    fn stairs_connect_floors_and_zones_are_one_way() {
        let mut grid = Grid::new(MapSize::new(64, 8).unwrap());
        let stairs = Cell {
            flags: STAIRS,
            ..Cell::floor(Zone::Airside)
        };
        grid.fill(
            crate::geometry::CellRect {
                floor: 0,
                x0: 0,
                y0: 2,
                x1: 30,
                y1: 3,
            },
            Cell::floor(Zone::Landside),
        )
        .unwrap();
        grid.set(pos(30, 2, 0), Cell::floor(Zone::Security))
            .unwrap();
        grid.fill(
            crate::geometry::CellRect {
                floor: 0,
                x0: 31,
                y0: 2,
                x1: 40,
                y1: 3,
            },
            Cell::floor(Zone::Airside),
        )
        .unwrap();
        grid.set(pos(40, 2, 0), stairs).unwrap();
        grid.set(pos(40, 2, 1), stairs).unwrap();
        grid.fill(
            crate::geometry::CellRect {
                floor: 1,
                x0: 41,
                y0: 2,
                x1: 50,
                y1: 3,
            },
            Cell::floor(Zone::Airside),
        )
        .unwrap();
        let gate = Target::passengers(vec![pos(49, 2, 1)]);
        let hall = Target::passengers(vec![pos(0, 2, 0)]);
        let set = compute(&grid, vec![gate, hall], 2);
        // Landside → security → airside → stairs → gate.
        assert_eq!(set.dir(0, pos(5, 2, 0)), 0);
        assert_eq!(set.dir(0, pos(40, 2, 0)), UP);
        assert_eq!(set.dir(0, pos(40, 2, 1)), 0);
        assert!(set.path_cost(0, pos(5, 2, 0)).is_some());
        // Back to the hall is blocked: airside → landside needs an exit.
        assert_eq!(set.path_cost(1, pos(45, 2, 1)), None);
        assert_eq!(set.dir(1, pos(31, 2, 0)), UNREACHABLE);
        // Stairs step helper.
        assert_eq!(set.step_target(0, pos(40, 2, 0), UP), Some(pos(40, 2, 1)));
        assert_eq!(set.step_target(0, pos(40, 2, 1), DOWN), Some(pos(40, 2, 0)));
        assert_eq!(set.step_target(0, pos(5, 2, 0), 0), Some(pos(6, 2, 0)));
        assert_eq!(set.step_target(0, pos(5, 2, 0), UP), None);
        assert_eq!(set.step_target(0, pos(5, 2, 0), 200), None);
        assert_eq!(set.step_target(0, pos(5, 5, 0), 0), None);
    }

    #[test]
    fn diagonal_steps_do_not_cut_corners() {
        let mut grid = Grid::new(MapSize::new(8, 8).unwrap());
        grid.fill(
            crate::geometry::CellRect {
                floor: 0,
                x0: 0,
                y0: 0,
                x1: 8,
                y1: 8,
            },
            Cell::floor(Zone::Landside),
        )
        .unwrap();
        grid.set(pos(3, 2, 0), Cell::VOID).unwrap();
        let set = compute(&grid, vec![Target::passengers(vec![pos(3, 3, 0)])], 1);
        // From (2,2) the diagonal to (3,3) would pass the wall at (3,2).
        assert_ne!(set.dir(0, pos(2, 2, 0)), 1);
        assert_eq!(set.path_cost(0, pos(2, 2, 0)), Some(20));
        // Next to an open diagonal the shortcut is taken.
        assert_eq!(set.dir(0, pos(4, 4, 0)), 5);
        assert_eq!(set.path_cost(0, pos(4, 4, 0)), Some(14));
    }

    #[test]
    fn a_world_without_targets_or_walkable_cells_gives_an_empty_set() {
        let grid = Grid::new(MapSize::new(64, 64).unwrap());
        let none = FieldSet::empty(&grid.snapshot(), 4);
        assert_eq!(
            (none.revision(), none.targets(), none.cells(), none.bytes()),
            (4, 0, 0, 0)
        );
        let void = compute(&grid, vec![Target::passengers(vec![pos(1, 1, 0)])], 1);
        assert_eq!(void.cells(), 0);
        assert_eq!(void.dir(0, pos(1, 1, 0)), UNREACHABLE);
    }

    /// One row of 40 cells on the ground floor: hall (0 … 9), apron walkway
    /// (10 … 19), stand area (20 … 29), runway (30 … 39).
    fn apron_row() -> Grid {
        let mut grid = Grid::new(MapSize::new(64, 8).unwrap());
        for x in 0..40u16 {
            let kind = match x / 10 {
                0 => AreaKind::PublicWalk,
                1 => AreaKind::ApronWalkway,
                2 => AreaKind::StandArea,
                _ => AreaKind::Runway,
            };
            grid.set(pos(x, 3, 0), Cell::floor(Zone::Landside).with_kind(kind))
                .unwrap();
        }
        grid
    }

    #[test]
    fn each_target_is_computed_with_the_rules_of_its_profile() {
        let grid = apron_row();
        let at = |x| vec![pos(x, 3, 0)];
        let set = compute(
            &grid,
            vec![
                Target::passengers(at(0)),
                Target::passengers(at(25)).with_profile(AccessProfile::ApronStaff),
                Target::passengers(at(25)),
                Target::passengers(at(35)).with_profile(AccessProfile::Worker),
                Target::passengers(at(0)).with_profile(AccessProfile::Worker),
            ],
            2,
        );
        // Passengers reach the hall from the apron walkway, not from the stand.
        assert_eq!(set.dir(0, pos(15, 3, 0)), 4);
        assert_eq!(set.path_cost(0, pos(15, 3, 0)), Some(150));
        assert_eq!(
            set.dir(0, pos(25, 3, 0)),
            UNREACHABLE,
            "stand is no walkway"
        );
        // Apron staff walk from the hall to the stand; the stand cell itself
        // is a legal seed, passengers have none there.
        assert_eq!(set.dir(1, pos(5, 3, 0)), 0);
        assert_eq!(set.path_cost(1, pos(5, 3, 0)), Some(200));
        assert_eq!(
            set.dir(2, pos(5, 3, 0)),
            UNREACHABLE,
            "seed on a stand: no field"
        );
        assert_eq!(set.dir(2, pos(25, 3, 0)), UNREACHABLE);
        // Workers cross the stand area but not the runway they walk to.
        assert_eq!(set.dir(3, pos(5, 3, 0)), UNREACHABLE, "seed on a runway");
        assert_eq!(set.dir(4, pos(29, 3, 0)), 4, "workers walk the stand area");
        assert_eq!(set.dir(4, pos(35, 3, 0)), UNREACHABLE, "runway");
        assert_eq!(
            set.unreachable_cells(2),
            40 - 20,
            "walkway cells cut off from the missing seed"
        );
        // Stepping checks the rules of the target as well.
        assert_eq!(set.step_target(1, pos(19, 3, 0), 0), Some(pos(20, 3, 0)));
        assert_eq!(set.step_target(0, pos(19, 3, 0), 0), None);
    }

    #[test]
    fn a_staff_zone_is_left_by_everyone_and_entered_by_staff_only() {
        // Hall 0 … 9, staff zone 10 … 19, hall 20 … 29.
        let mut grid = Grid::new(MapSize::new(64, 8).unwrap());
        for x in 0..30u16 {
            let cell = Cell::floor(Zone::Landside).with_staff_zone((10..20).contains(&x));
            grid.set(pos(x, 3, 0), cell).unwrap();
        }
        // The only way between the halls leads through the staff zone.
        let set = compute(
            &grid,
            vec![
                Target::passengers(vec![pos(25, 3, 0)]),
                Target::passengers(vec![pos(25, 3, 0)]).with_profile(AccessProfile::Staff),
                Target::passengers(vec![pos(15, 3, 0)]),
                Target::passengers(vec![pos(15, 3, 0)]).with_profile(AccessProfile::Staff),
            ],
            2,
        );
        assert_eq!(
            set.dir(0, pos(5, 3, 0)),
            UNREACHABLE,
            "passengers do not cross"
        );
        assert_eq!(set.dir(1, pos(5, 3, 0)), 0, "staff cross");
        assert_eq!(set.dir(0, pos(15, 3, 0)), 0, "passengers inside leave");
        assert_eq!(
            set.dir(2, pos(5, 3, 0)),
            UNREACHABLE,
            "passengers cannot enter"
        );
        assert_eq!(set.dir(2, pos(15, 3, 0)), ARRIVED, "inside the zone");
        assert_eq!(set.dir(3, pos(5, 3, 0)), 0, "staff enter");
    }

    #[test]
    fn dir_index_inverts_the_direction_table() {
        for (j, &(dx, dy)) in DIRS.iter().enumerate() {
            assert_eq!(dir_index(dx, dy), j);
        }
    }
}
