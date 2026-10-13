//! Minimal simulation core for roadmap step 1.1 (spike, throwaway code).
//!
//! Advances a world of N persons per tick. Each person follows a precomputed
//! direction field ("Wegweiser") towards one of several targets, dwells there and
//! picks a new target from a per-person deterministic RNG. The workload is meant
//! to resemble step 1.2 closely enough to compare native vs. WebAssembly builds:
//! random reads into a few MB of field data plus a handful of float ops per person.
//!
//! Only IEEE-exact float operations (+, -, *, /, sqrt) are used so that native
//! and WebAssembly builds can be compared bit for bit via [`World::checksum`].

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Cells per side of the square test grid.
pub const GRID: usize = 512;
/// Number of distinct targets (check-in, security, gates, toilets, exit, ...).
pub const TARGETS: usize = 8;
/// Version of the frame layout written by [`World::write_frame`].
pub const LAYOUT_VERSION: u32 = 1;
/// Header: layout version, entity count, tick (low, high) as little-endian u32.
pub const FRAME_HEADER_BYTES: usize = 16;
/// Per person: x f32, y f32, direction u32, kind/state u32.
pub const FRAME_RECORD_BYTES: usize = 16;

const BLOCKED: u8 = 254;
const ARRIVED: u8 = 255;
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
const INV_SQRT2: f32 = 0.707_106_77;

/// Size in bytes of a frame for `count` persons.
#[must_use]
pub const fn frame_bytes(count: usize) -> usize {
    FRAME_HEADER_BYTES + count * FRAME_RECORD_BYTES
}

/// One simulated person (array-of-structs; the real data model is decided in step 1.2).
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Person {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub needs: [f32; 4],
    pub rng: u32,
    pub wait: u16,
    pub target: u8,
    pub dir: u8,
}

/// Static, read-only world data shared by all workers.
pub struct Fields {
    walls: Vec<bool>,
    /// `TARGETS` direction fields of `GRID * GRID` cells each.
    dirs: Vec<u8>,
    targets: [(usize, usize); TARGETS],
}

impl Fields {
    /// Builds walls with door gaps and one BFS direction field per target.
    #[must_use]
    pub fn new() -> Self {
        let mut walls = vec![false; GRID * GRID];
        // Vertical and horizontal walls every 64 cells with a door every 32 cells.
        for i in 0..GRID {
            for w in (64..GRID).step_by(64) {
                if i % 32 >= 3 {
                    walls[i * GRID + w] = true;
                    walls[w * GRID + i] = true;
                }
            }
        }
        let mut targets = [(0, 0); TARGETS];
        for (k, t) in targets.iter_mut().enumerate() {
            let x = 20 + (k * 61) % (GRID - 40);
            let y = 20 + (k * 137) % (GRID - 40);
            walls[y * GRID + x] = false;
            *t = (x, y);
        }
        let mut dirs = vec![BLOCKED; TARGETS * GRID * GRID];
        for (k, &(tx, ty)) in targets.iter().enumerate() {
            bfs(
                &walls,
                &mut dirs[k * GRID * GRID..(k + 1) * GRID * GRID],
                tx,
                ty,
            );
        }
        Self {
            walls,
            dirs,
            targets,
        }
    }

    /// Bytes held by the direction fields.
    #[must_use]
    pub fn field_bytes(&self) -> usize {
        self.dirs.len()
    }

    fn dir_at(&self, target: u8, x: f32, y: f32) -> u8 {
        let cx = (x as usize).min(GRID - 1);
        let cy = (y as usize).min(GRID - 1);
        self.dirs[usize::from(target) * GRID * GRID + cy * GRID + cx]
    }

    fn is_free(&self, x: usize, y: usize) -> bool {
        !self.walls[y * GRID + x]
    }
}

impl Default for Fields {
    fn default() -> Self {
        Self::new()
    }
}

/// Breadth-first search from the target; every reachable cell stores the
/// direction to its predecessor, i.e. the next step towards the target.
fn bfs(walls: &[bool], out: &mut [u8], tx: usize, ty: usize) {
    let mut queue = std::collections::VecDeque::with_capacity(GRID * GRID);
    out[ty * GRID + tx] = ARRIVED;
    queue.push_back((tx, ty));
    while let Some((x, y)) = queue.pop_front() {
        for (d, &(dx, dy)) in DIRS.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= GRID as i32 || ny >= GRID as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            let idx = ny * GRID + nx;
            // No corner cutting through walls on diagonals.
            if walls[idx]
                || out[idx] != BLOCKED
                || (dx != 0 && dy != 0 && (walls[y * GRID + nx] || walls[ny * GRID + x]))
            {
                continue;
            }
            // The neighbour moves opposite to the direction we came from.
            out[idx] = ((d + 4) % 8) as u8;
            queue.push_back((nx, ny));
        }
    }
}

fn xorshift(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

/// Advances a slice of persons by one tick. Independent per person, therefore
/// any split into chunks yields bit-identical results (deterministic in parallel).
///
/// `work` adds synthetic per-person decision work (distance evaluation to all
/// targets, repeated `work` times) to emulate a heavier future simulation.
pub fn step_persons(fields: &Fields, persons: &mut [Person], work: u32) {
    for p in persons {
        for n in &mut p.needs {
            *n = (*n * 0.999 + 0.0005).min(1.0);
        }
        if work > 0 {
            let mut acc = 0.0f32;
            for r in 0..work {
                for (k, &(tx, ty)) in fields.targets.iter().enumerate() {
                    let dx = tx as f32 - p.x + r as f32 * 0.01;
                    let dy = ty as f32 - p.y;
                    acc += (dx * dx + dy * dy).sqrt() * p.needs[k % 4];
                }
            }
            p.needs[3] = (p.needs[3] + acc * 1e-9).min(1.0);
        }
        if p.wait > 0 {
            p.wait -= 1;
            continue;
        }
        let d = fields.dir_at(p.target, p.x, p.y);
        if d == ARRIVED || d == BLOCKED {
            let r = xorshift(&mut p.rng);
            p.target = (r % TARGETS as u32) as u8;
            p.wait = (r >> 8) as u16 % 120;
            let k = usize::from(p.target) % 4;
            p.needs[k] *= 0.5;
            continue;
        }
        let (dx, dy) = DIRS[usize::from(d)];
        let scale = if dx != 0 && dy != 0 { INV_SQRT2 } else { 1.0 };
        let fatigue = 1.0 - 0.3 * (p.needs[0] * p.needs[0] + p.needs[1] * p.needs[1]).sqrt();
        let v = p.speed * scale * fatigue;
        p.x = (p.x + dx as f32 * v).clamp(0.0, (GRID - 1) as f32);
        p.y = (p.y + dy as f32 * v).clamp(0.0, (GRID - 1) as f32);
        p.dir = d;
    }
}

/// Mutable simulation state.
pub struct World {
    pub fields: Fields,
    pub persons: Vec<Person>,
    pub tick: u64,
    /// Synthetic extra work per person and tick, see [`step_persons`].
    pub work: u32,
}

impl World {
    /// Creates `count` persons on free cells, deterministic from `seed`.
    #[must_use]
    pub fn new(count: usize, seed: u32) -> Self {
        let fields = Fields::new();
        let mut rng = seed.max(1);
        let mut persons = Vec::with_capacity(count);
        while persons.len() < count {
            let x = xorshift(&mut rng) as usize % GRID;
            let y = xorshift(&mut rng) as usize % GRID;
            if !fields.is_free(x, y) {
                continue;
            }
            let r = xorshift(&mut rng);
            persons.push(Person {
                x: x as f32 + 0.5,
                y: y as f32 + 0.5,
                speed: 0.6 + (r % 64) as f32 / 160.0,
                needs: [0.1, 0.2, 0.3, 0.4],
                rng: r | 1,
                wait: 0,
                target: (r % TARGETS as u32) as u8,
                dir: 0,
            });
        }
        Self {
            fields,
            persons,
            tick: 0,
            work: 0,
        }
    }

    /// One tick on the calling thread.
    pub fn step_serial(&mut self) {
        step_persons(&self.fields, &mut self.persons, self.work);
        self.tick += 1;
    }

    /// One tick split into chunks on the current rayon pool.
    #[cfg(feature = "parallel")]
    pub fn step_parallel(&mut self, chunk: usize) {
        let fields = &self.fields;
        let work = self.work;
        self.persons
            .par_chunks_mut(chunk.max(1))
            .for_each(|c| step_persons(fields, c, work));
        self.tick += 1;
    }

    /// Writes the visible state in the frame layout (see module constants).
    /// `out` must hold at least [`frame_bytes`] bytes.
    pub fn write_frame(&self, out: &mut [u8]) {
        let n = self.persons.len();
        out[0..4].copy_from_slice(&LAYOUT_VERSION.to_le_bytes());
        out[4..8].copy_from_slice(&(n as u32).to_le_bytes());
        out[8..12].copy_from_slice(&(self.tick as u32).to_le_bytes());
        out[12..16].copy_from_slice(&((self.tick >> 32) as u32).to_le_bytes());
        for (i, p) in self.persons.iter().enumerate() {
            let o = FRAME_HEADER_BYTES + i * FRAME_RECORD_BYTES;
            out[o..o + 4].copy_from_slice(&p.x.to_le_bytes());
            out[o + 4..o + 8].copy_from_slice(&p.y.to_le_bytes());
            out[o + 8..o + 12].copy_from_slice(&u32::from(p.dir).to_le_bytes());
            let state = u32::from(p.target) | (u32::from(p.wait > 0) << 8);
            out[o + 12..o + 16].copy_from_slice(&state.to_le_bytes());
        }
    }

    /// FNV-1a over the bit patterns of all person fields; used for determinism checks.
    #[must_use]
    pub fn checksum(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut mix = |v: u32| {
            h ^= u64::from(v);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        };
        for p in &self.persons {
            mix(p.x.to_bits());
            mix(p.y.to_bits());
            for n in p.needs {
                mix(n.to_bits());
            }
            mix(p.rng);
            mix(u32::from(p.wait) | (u32::from(p.target) << 16) | (u32::from(p.dir) << 24));
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_target_reachable_from_most_cells() {
        let f = Fields::new();
        for k in 0..TARGETS {
            let field = &f.dirs[k * GRID * GRID..(k + 1) * GRID * GRID];
            let blocked = field.iter().filter(|&&d| d == BLOCKED).count();
            let walls = f.walls.iter().filter(|&&w| w).count();
            assert_eq!(blocked, walls, "target {k}: unreachable free cells");
        }
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn parallel_matches_serial() {
        let mut a = World::new(2_000, 7);
        let mut b = World::new(2_000, 7);
        a.work = 3;
        b.work = 3;
        for _ in 0..300 {
            a.step_serial();
            b.step_parallel(97);
        }
        assert_eq!(a.checksum(), b.checksum());
    }

    #[test]
    fn frame_layout() {
        let w = World::new(3, 1);
        let mut buf = vec![0u8; frame_bytes(3)];
        w.write_frame(&mut buf);
        assert_eq!(
            u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            LAYOUT_VERSION
        );
        assert_eq!(u32::from_le_bytes(buf[4..8].try_into().unwrap()), 3);
    }
}
