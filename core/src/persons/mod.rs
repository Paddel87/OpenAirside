//! Module "Personen" (ADR-039): persons with their hot fields, movement
//! along the flow fields and the choice of the next target, with their part
//! of the save game, the checks of a loaded state and their contribution to
//! the state hash.
//!
//! Persons keep their hot fields in one small record each and move in two
//! phases (ADR-010): phase 1 computes every person from the state of the
//! previous tick, phase 2 computes commutative counts. The module reads the
//! modules "Welt & Bau" (walkable cells, number of targets) and
//! "Wegfindung" (flow fields).

use rayon::ThreadPool;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::build::Build;
use crate::build::access::AccessProfile;
use crate::build::grid::{FLOORS, floor_index};
use crate::geometry::{CellPos, MapSize};
use crate::hash::StateHasher;
use crate::pool::{Kind, Pool};
use crate::wayfinding::flowfield::{ARRIVED, DIRS, DOWN, FieldSet, UNREACHABLE, UP};

/// Walking speed in cells per tick: 0.3 cells per game second, the guide
/// value of ADR-020 (a 300 m walk takes about 8 game minutes).
pub const WALK_SPEED: f32 = 0.1;
const DIAGONAL: f32 = std::f32::consts::FRAC_1_SQRT_2;

/// Most persons a world holds; the debug command `spawnPersons` stops
/// there, and a save game with more is rejected (S-5).
pub const MAX_PERSONS: usize = 200_000;

/// Hot fields of one person, needed in every tick; stored as they are in a
/// save game.
// TODO(fahrplan-ref: 3.2): needs, patience, tickets and other cold fields of
// passengers and staff.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct Person {
    x: u16,
    y: u16,
    floor: i8,
    /// Step towards the target: direction 0 … 9, `ARRIVED` or `UNREACHABLE`.
    heading: u8,
    target: u8,
    /// Share of the current step already walked, 0 … 1.
    progress: f32,
    /// State of the generator that picks the next target.
    rng: u32,
}

/// What a loaded person is checked against.
pub(crate) struct PersonContext {
    map: MapSize,
    /// Number of targets, at least 1 (a person of a world without targets
    /// keeps target 0).
    targets: usize,
}

impl Kind for Person {
    const MAX: usize = MAX_PERSONS;
    const LIMIT: &'static str = "persons";
    const CHECK: &'static str = "person";
    type Context = PersonContext;

    fn is_valid(&self, context: &PersonContext) -> bool {
        let pos = self.pos();
        let heading_ok = self.heading <= DOWN || matches!(self.heading, ARRIVED | UNREACHABLE);
        context.map.contains(pos)
            && floor_index(pos.floor).is_some()
            && heading_ok
            && usize::from(self.target) < context.targets
            && (0.0..=1.0).contains(&self.progress)
    }

    fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(u32::from(self.x));
        hasher.u32(u32::from(self.y));
        hasher.bytes(&[self.floor.to_le_bytes()[0], self.heading, self.target]);
        hasher.f32(self.progress);
        hasher.u32(self.rng);
    }
}

impl Person {
    fn pos(&self) -> CellPos {
        CellPos {
            x: self.x,
            y: self.y,
            floor: self.floor,
        }
    }

    fn view(&self) -> PersonView {
        let (dx, dy) = DIRS
            .get(usize::from(self.heading))
            .copied()
            .unwrap_or((0, 0));
        #[allow(clippy::cast_precision_loss)] // dx, dy are −1, 0 or 1
        let (dx, dy) = (dx as f32, dy as f32);
        PersonView {
            x: f32::from(self.x) + 0.5 + dx * self.progress,
            y: f32::from(self.y) + 0.5 + dy * self.progress,
            floor: self.floor,
            heading: self.heading,
        }
    }
}

/// Visible state of one person for the state buffer S-1: position in cells
/// with the share of the current step already walked.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PersonView {
    /// Position east of the map origin in cells (cell centre = x + 0.5).
    pub x: f32,
    /// Position south of the map origin in cells.
    pub y: f32,
    /// Floor −2 … +2.
    pub floor: i8,
    /// Direction 0 … 9, `ARRIVED` or `UNREACHABLE` (see `flowfield`).
    pub heading: u8,
}

/// State of the module: the persons, the second buffer of the step and the
/// count per floor.
#[derive(Default)]
pub struct Persons {
    pool: Pool<Person>,
    next: Vec<Person>,
    per_floor: [u32; FLOORS],
}

impl Persons {
    /// Number of persons.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pool.len()
    }

    /// `true` if there is no person.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }

    /// Calls `visit` with the visible state of every person, in storage
    /// order.
    pub fn visit(&self, mut visit: impl FnMut(PersonView)) {
        for p in self.pool.as_slice() {
            visit(p.view());
        }
    }

    /// Persons per floor, index 0 = floor −2.
    #[must_use]
    pub fn per_floor(&self) -> [u32; FLOORS] {
        self.per_floor
    }

    /// Position of person `index`, if it exists.
    #[must_use]
    pub fn position(&self, index: usize) -> Option<CellPos> {
        self.pool.as_slice().get(index).map(Person::pos)
    }

    /// `true` while persons are present, which blocks the night skip
    /// (META-08: no passenger in the terminal).
    #[must_use]
    pub fn has_active(&self) -> bool {
        !self.pool.is_empty()
    }

    /// Adds one person on each of `cells` (cells a passenger may not stand
    /// on in `build` are skipped, and none beyond [`MAX_PERSONS`]), with a target
    /// and a generator state derived from `seed`.
    pub fn spawn(&mut self, build: &Build, cells: &[CellPos], seed: u64) {
        let mut rng = SplitMix64(seed ^ 0x5eed);
        let targets = build.targets().len().max(1) as u64;
        for &pos in cells {
            if !walkable_for_passengers(build, pos) {
                continue;
            }
            let state = u32::try_from(rng.next() >> 32).unwrap_or(1) | 1;
            self.pool.push(Person {
                x: pos.x,
                y: pos.y,
                floor: pos.floor,
                heading: UNREACHABLE,
                target: u8::try_from(rng.next() % targets).unwrap_or(0),
                progress: 0.0,
                rng: state,
            });
        }
        self.next.clone_from(self.pool.items_mut());
        self.count_per_floor_serial();
    }

    /// Adds up to `count` persons on random cells of the ground floor that
    /// a passenger may stand on (debug command, META-15). Gives up on a person after 64
    /// cells that are not walkable and never exceeds [`MAX_PERSONS`].
    /// Returns the number of persons added.
    pub fn spawn_random(&mut self, build: &Build, count: u32, seed: u64) -> u32 {
        let wanted = usize::try_from(count)
            .unwrap_or(usize::MAX)
            .min(self.pool.room());
        let map = build.map();
        let mut rng = SplitMix64(seed);
        let mut cells = Vec::with_capacity(wanted);
        for _ in 0..wanted {
            for _ in 0..64 {
                let pos = CellPos {
                    x: u16::try_from(rng.next() % u64::from(map.width)).unwrap_or(0),
                    y: u16::try_from(rng.next() % u64::from(map.height)).unwrap_or(0),
                    floor: 0,
                };
                if walkable_for_passengers(build, pos) {
                    cells.push(pos);
                    break;
                }
            }
        }
        let before = self.pool.len();
        self.spawn(build, &cells, seed);
        u32::try_from(self.pool.len() - before).unwrap_or(u32::MAX)
    }

    /// Removes all persons (debug command).
    pub fn clear(&mut self) {
        self.pool.clear();
        self.next.clear();
        self.per_floor = [0; FLOORS];
    }

    /// Sets every heading from `fields` (start of a session, after the
    /// fields were computed).
    pub(crate) fn resample_headings(&mut self, fields: &FieldSet) {
        for p in self.pool.items_mut() {
            p.heading = fields.dir(p.target, p.pos());
        }
    }

    /// Simulation step of the module: phase 1 moves every person from the
    /// state of the previous tick along `fields`, phase 2 computes
    /// commutative counts.
    pub(crate) fn advance(&mut self, fields: &FieldSet, targets: usize, pool: &ThreadPool) {
        let (persons, next) = (self.pool.items_mut(), &mut self.next);
        pool.install(|| {
            next.par_iter_mut()
                .zip(persons.par_iter())
                .for_each(|(out, cur)| *out = step_person(*cur, fields, targets));
        });
        std::mem::swap(persons, next);
        self.per_floor = pool.install(|| {
            self.pool
                .as_slice()
                .par_iter()
                .fold(
                    || [0u32; FLOORS],
                    |mut acc, p| {
                        if let Some(index) = floor_index(p.floor) {
                            acc[index] += 1;
                        }
                        acc
                    },
                )
                .reduce(|| [0u32; FLOORS], add_counts)
        });
    }

    fn count_per_floor_serial(&mut self) {
        let mut counts = [0u32; FLOORS];
        for p in self.pool.as_slice() {
            if let Some(index) = floor_index(p.floor) {
                counts[index] += 1;
            }
        }
        self.per_floor = counts;
    }

    /// Feeds the state of the module into `hasher`: every person, then the
    /// count per floor.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        self.pool.hash_into(hasher);
        for count in self.per_floor {
            hasher.u32(count);
        }
    }

    /// The persons as stored in a save game (S-5).
    pub(crate) fn saved(&self) -> Vec<Person> {
        self.pool.saved()
    }

    /// `true` if `count` persons of a save game fit under [`MAX_PERSONS`].
    pub(crate) fn fits(count: usize) -> bool {
        Pool::<Person>::fits(count)
    }

    /// Persons of a loaded save game in a world with `map` and `targets`
    /// targets.
    ///
    /// # Errors
    ///
    /// `persons` for too many, `person` for the first person off the map, on
    /// an invalid floor, with an unknown target or heading, or with a step
    /// share outside 0 … 1.
    pub(crate) fn restore(
        saved: Vec<Person>,
        map: MapSize,
        targets: usize,
    ) -> Result<Self, &'static str> {
        let context = PersonContext {
            map,
            targets: targets.max(1),
        };
        let pool = Pool::restore(saved, &context)?;
        let mut persons = Self {
            next: pool.as_slice().to_vec(),
            pool,
            per_floor: [0; FLOORS],
        };
        persons.count_per_floor_serial();
        Ok(persons)
    }
}

/// `true` if a passenger may stand on `pos`: persons other than
/// construction workers never start on terrain (ADR-045).
fn walkable_for_passengers(build: &Build, pos: CellPos) -> bool {
    build
        .grid()
        .cell(pos)
        .is_some_and(|c| AccessProfile::Passenger.rules().walkable(c.flags))
}

fn add_counts(mut a: [u32; FLOORS], b: [u32; FLOORS]) -> [u32; FLOORS] {
    for (x, y) in a.iter_mut().zip(b) {
        *x += y;
    }
    a
}

/// Next state of one person.
fn step_person(mut p: Person, fields: &FieldSet, targets: usize) -> Person {
    if p.heading > DOWN {
        // Standing: arrived at the target or without a way to it.
        if p.heading == ARRIVED && targets > 0 {
            p.rng ^= p.rng << 13;
            p.rng ^= p.rng >> 17;
            p.rng ^= p.rng << 5;
            p.target = u8::try_from(p.rng as usize % targets).unwrap_or(0);
        }
        p.heading = fields.dir(p.target, p.pos());
        p.progress = 0.0;
        return p;
    }
    let diagonal = matches!(p.heading, 1 | 3 | 5 | 7);
    p.progress += if diagonal {
        WALK_SPEED * DIAGONAL
    } else {
        WALK_SPEED
    };
    if p.progress >= 1.0 {
        if let Some(next) = fields.step_target(p.target, p.pos(), p.heading) {
            p.x = next.x;
            p.y = next.y;
            p.floor = next.floor;
        }
        p.progress = 0.0;
        p.heading = fields.dir(p.target, p.pos());
    }
    p
}

/// Small deterministic generator for reproducible start states.
pub(crate) struct SplitMix64(pub u64);

impl SplitMix64 {
    pub(crate) fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

// The step rule depends on the numbering of the directions.
const _: () = assert!(DIRS.len() == 8 && UP == 8 && DOWN == 9);

#[cfg(test)]
mod tests {
    use super::*;

    fn person() -> Person {
        Person {
            x: 3,
            y: 4,
            floor: 1,
            heading: 2, // south
            target: 0,
            progress: 0.5,
            rng: 1,
        }
    }

    #[test]
    fn view_interpolates_the_current_step() {
        let p = person();
        let v = p.view();
        assert!((v.x - 3.5).abs() < 1e-6 && (v.y - 5.0).abs() < 1e-6);
        let standing = Person {
            heading: ARRIVED,
            ..p
        };
        assert!((standing.view().y - 4.5).abs() < 1e-6);
    }

    #[test]
    fn loaded_persons_are_checked_against_map_and_targets() {
        let map = MapSize::new(64, 64).unwrap();
        assert!(Persons::restore(vec![person()], map, 1).is_ok());
        assert!(
            Persons::restore(vec![person()], map, 0).is_ok(),
            "target 0 without targets"
        );
        for change in [
            |p: &mut Person| p.x = 64,
            |p: &mut Person| p.floor = -3,
            |p: &mut Person| p.heading = 11,
            |p: &mut Person| p.target = 1,
            |p: &mut Person| p.progress = f32::NAN,
            |p: &mut Person| p.progress = 1.5,
        ] {
            let mut p = person();
            change(&mut p);
            assert_eq!(Persons::restore(vec![p], map, 1).err(), Some("person"));
        }
        assert!(!Persons::fits(MAX_PERSONS + 1));
        let restored = Persons::restore(vec![person(); 3], map, 1).unwrap();
        assert_eq!(restored.per_floor()[3], 3, "floor 1 counted");
        assert_eq!(restored.saved(), vec![person(); 3]);
    }
}
