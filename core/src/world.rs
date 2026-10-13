//! World state of the tick in step 2.2: a population of persons with hot
//! fields, moved in two phases in parallel (ADR-010).
//!
//! The movement rule is synthetic: persons drift along a per-floor direction
//! taken from background-computed guidance data and bounce off the map edge.
//! It carries the load of the tick benchmark and the determinism tests.
// TODO(fahrplan-ref: 2.3): replace the synthetic guidance and movement by
// flow fields on the block grid (ADR-020) and by the passenger rules.

use std::sync::Arc;

use rayon::ThreadPool;
use rayon::prelude::*;

use crate::geometry::{MAX_FLOOR, MIN_FLOOR, MapSize};
use crate::hash::StateHasher;

/// Number of floors (−2 … +2).
pub const FLOORS: usize = (MAX_FLOOR - MIN_FLOOR + 1) as usize;
/// Walking speed in cells per tick (about 0.3 cells per game second, the
/// guide value of ADR-020: 300 m in about 8 game minutes).
pub const WALK_SPEED: f32 = 0.1;
const STEER: f32 = 0.05;

/// Hot fields of one person, needed in every tick.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Motion {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

/// Derived data computed in the background and adopted as a whole.
#[derive(Debug, Clone, PartialEq)]
pub struct Guidance {
    /// Revision of the world layout the data belong to.
    pub revision: u32,
    drift: [(f32, f32); FLOORS],
}

impl Guidance {
    /// Computes the guidance for a layout revision. Pure function, so the
    /// result is the same on every thread and machine.
    #[must_use]
    pub fn compute(revision: u32) -> Self {
        let mut rng = SplitMix64(u64::from(revision).wrapping_mul(0x9e37_79b9));
        let drift = std::array::from_fn(|_| {
            let angle = rng.unit() * std::f32::consts::TAU;
            (angle.cos(), angle.sin())
        });
        Self { revision, drift }
    }
}

/// Population and derived data of one session.
pub struct World {
    map: MapSize,
    motion: Vec<Motion>,
    next: Vec<Motion>,
    floor: Vec<i8>,
    guidance: Arc<Guidance>,
    per_floor: [u32; FLOORS],
}

impl World {
    /// Places `persons` persons pseudo-randomly from `seed` on all floors.
    #[must_use]
    pub fn new(map: MapSize, persons: u32, seed: u64) -> Self {
        let mut rng = SplitMix64(seed);
        let width = f32::from(map.width);
        let height = f32::from(map.height);
        let mut motion = Vec::with_capacity(persons as usize);
        let mut floor = Vec::with_capacity(persons as usize);
        for _ in 0..persons {
            let angle = rng.unit() * std::f32::consts::TAU;
            motion.push(Motion {
                x: rng.unit() * width,
                y: rng.unit() * height,
                vx: angle.cos() * WALK_SPEED,
                vy: angle.sin() * WALK_SPEED,
            });
            let level = i8::try_from(rng.next() % FLOORS as u64).unwrap_or(0);
            floor.push(MIN_FLOOR + level);
        }
        let mut world = Self {
            map,
            next: motion.clone(),
            motion,
            floor,
            guidance: Arc::new(Guidance::compute(0)),
            per_floor: [0; FLOORS],
        };
        world.count_per_floor_serial();
        world
    }

    /// Number of persons.
    #[must_use]
    pub fn persons(&self) -> usize {
        self.motion.len()
    }

    /// Map of this world.
    #[must_use]
    pub fn map(&self) -> MapSize {
        self.map
    }

    /// Persons per floor, index 0 = floor −2.
    #[must_use]
    pub fn per_floor(&self) -> [u32; FLOORS] {
        self.per_floor
    }

    /// Revision of the guidance in use.
    #[must_use]
    pub fn guidance_revision(&self) -> u32 {
        self.guidance.revision
    }

    /// Takes over newly computed guidance.
    pub fn adopt_guidance(&mut self, guidance: Guidance) {
        self.guidance = Arc::new(guidance);
    }

    /// `true` while persons are present, which blocks the night skip
    /// (META-08: no passenger in the terminal).
    #[must_use]
    pub fn has_active_persons(&self) -> bool {
        !self.motion.is_empty()
    }

    /// One simulation step: phase 1 moves every person from the state of
    /// the previous tick, phase 2 computes commutative counts.
    pub fn advance(&mut self, pool: &ThreadPool) {
        let width = f32::from(self.map.width);
        let height = f32::from(self.map.height);
        let guidance = Arc::clone(&self.guidance);
        let (motion, next, floor) = (&self.motion, &mut self.next, &self.floor);
        pool.install(|| {
            next.par_iter_mut()
                .zip(motion.par_iter())
                .zip(floor.par_iter())
                .for_each(|((out, cur), level)| {
                    let index = usize::try_from(*level - MIN_FLOOR).unwrap_or(0);
                    let (dx, dy) = guidance.drift[index];
                    *out = step_person(*cur, dx, dy, width, height);
                });
        });
        std::mem::swap(&mut self.motion, &mut self.next);
        self.per_floor = pool.install(|| {
            self.floor
                .par_iter()
                .fold(
                    || [0u32; FLOORS],
                    |mut acc, level| {
                        acc[usize::try_from(*level - MIN_FLOOR).unwrap_or(0)] += 1;
                        acc
                    },
                )
                .reduce(|| [0u32; FLOORS], add_counts)
        });
    }

    fn count_per_floor_serial(&mut self) {
        let mut counts = [0u32; FLOORS];
        for level in &self.floor {
            counts[usize::try_from(*level - MIN_FLOOR).unwrap_or(0)] += 1;
        }
        self.per_floor = counts;
    }

    /// Feeds the complete world state into `hasher`.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(u32::from(self.map.width));
        hasher.u32(u32::from(self.map.height));
        hasher.u32(self.guidance.revision);
        for m in &self.motion {
            hasher.f32(m.x);
            hasher.f32(m.y);
            hasher.f32(m.vx);
            hasher.f32(m.vy);
        }
        for level in &self.floor {
            hasher.bytes(&level.to_le_bytes());
        }
        for count in self.per_floor {
            hasher.u32(count);
        }
    }
}

fn add_counts(mut a: [u32; FLOORS], b: [u32; FLOORS]) -> [u32; FLOORS] {
    for (x, y) in a.iter_mut().zip(b) {
        *x += y;
    }
    a
}

fn step_person(cur: Motion, dx: f32, dy: f32, width: f32, height: f32) -> Motion {
    let mut vx = cur.vx + dx * STEER;
    let mut vy = cur.vy + dy * STEER;
    let speed = vx.hypot(vy);
    if speed > 0.0 {
        vx = vx / speed * WALK_SPEED;
        vy = vy / speed * WALK_SPEED;
    }
    let (x, vx) = bounce(cur.x + vx, vx, width);
    let (y, vy) = bounce(cur.y + vy, vy, height);
    Motion { x, y, vx, vy }
}

fn bounce(pos: f32, vel: f32, limit: f32) -> (f32, f32) {
    if pos < 0.0 {
        (-pos, -vel)
    } else if pos >= limit {
        (2.0 * limit - pos - f32::EPSILON * limit, -vel)
    } else {
        (pos, vel)
    }
}

/// Small deterministic generator for reproducible start states.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform value in `[0, 1)`.
    fn unit(&mut self) -> f32 {
        // 24 random bits fit exactly into the mantissa of an f32.
        let bits = (self.next() >> 40) as f32;
        bits / 16_777_216.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool(threads: usize) -> ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
    }

    fn hash(world: &World) -> u64 {
        let mut h = StateHasher::new();
        world.hash_into(&mut h);
        h.finish()
    }

    #[test]
    fn persons_are_spread_over_all_floors_and_stay_on_the_map() {
        let map = MapSize::new(64, 32).unwrap();
        let mut world = World::new(map, 2_000, 7);
        assert_eq!(world.persons(), 2_000);
        assert_eq!(world.per_floor().iter().sum::<u32>(), 2_000);
        assert!(world.per_floor().iter().all(|&n| n > 300));
        let pool = pool(2);
        for _ in 0..2_000 {
            world.advance(&pool);
        }
        for m in &world.motion {
            assert!((0.0..64.0).contains(&m.x), "x = {}", m.x);
            assert!((0.0..32.0).contains(&m.y), "y = {}", m.y);
            assert!((m.vx.hypot(m.vy) - WALK_SPEED).abs() < 1e-4);
        }
        assert_eq!(world.per_floor().iter().sum::<u32>(), 2_000);
    }

    #[test]
    fn advancing_is_bit_identical_for_any_thread_count() {
        let map = MapSize::new(256, 256).unwrap();
        let run = |threads| {
            let pool = pool(threads);
            let mut world = World::new(map, 5_000, 42);
            for tick in 0..300 {
                if tick == 100 {
                    world.adopt_guidance(Guidance::compute(3));
                }
                world.advance(&pool);
            }
            hash(&world)
        };
        let single = run(1);
        assert_eq!(run(3), single);
        assert_eq!(run(8), single);
    }

    #[test]
    fn guidance_changes_the_course_and_is_reproducible() {
        assert_eq!(Guidance::compute(5), Guidance::compute(5));
        assert_ne!(Guidance::compute(5), Guidance::compute(6));
        let map = MapSize::new(128, 128).unwrap();
        let pool = pool(1);
        let mut a = World::new(map, 100, 1);
        let mut b = World::new(map, 100, 1);
        b.adopt_guidance(Guidance::compute(9));
        assert_eq!(b.guidance_revision(), 9);
        a.advance(&pool);
        b.advance(&pool);
        assert_ne!(hash(&a), hash(&b));
    }

    #[test]
    fn empty_world_has_no_active_persons() {
        let world = World::new(MapSize::new(8, 8).unwrap(), 0, 0);
        assert!(!world.has_active_persons());
        assert_eq!(world.map(), MapSize::new(8, 8).unwrap());
    }

    #[test]
    fn bounce_reflects_at_both_edges() {
        assert_eq!(bounce(-0.25, -0.1, 10.0), (0.25, 0.1));
        let (pos, vel) = bounce(10.0, 0.1, 10.0);
        assert!(pos < 10.0 && vel < 0.0);
        assert_eq!(bounce(5.0, 0.1, 10.0), (5.0, 0.1));
    }
}
