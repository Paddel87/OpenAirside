//! Reference terminal for benchmarks and tests (module "Benchmark &
//! Leistung"): a procedural airport terminal with five floors, landside
//! hall, check-in, security lanes, airside area with gates, one-way exits,
//! stairs and floor markings. Ported from the load scenario of spike 1.5
//! (`spikes/1.2-wegweiser/src/grid.rs`), embedded in a larger map as in the
//! measurement of step 2.8 (ADR-020).
// TODO(fahrplan-ref: 4.1): the reference scenario of the NFR (3 terminal
// areas, 2 runways, ~17 gates) replaces this procedural terminal; it is
// built from data packs and a reference save game.

use crate::build::grid::{
    COST_AVOID, COST_MARKED, COST_NORMAL, Cell, FLOORS, GROUND_INDEX, STAIRS, WALL, Zone,
};
use crate::build::target::Target;
use crate::command::Scenario;
use crate::geometry::{CellPos, MapSize};
use crate::persons::SplitMix64;
use crate::world::World;

/// Smallest and largest terminal edge in cells. The layout repeats walls
/// every 64 cells and needs room for the hall, the security line and the
/// airside area.
pub const MIN_EDGE: u16 = 256;

/// Number of check-in groups (airlines).
const AIRLINES: usize = 8;

/// The terminal as dense arrays, indexed `(floor · h + y) · w + x`.
struct Dense {
    w: usize,
    h: usize,
    flags: Vec<u8>,
    zone: Vec<u8>,
    cost: Vec<u8>,
}

impl Dense {
    fn idx(&self, x: usize, y: usize, f: usize) -> usize {
        (f * self.h + y) * self.w + x
    }

    fn walkable(&self, x: usize, y: usize, f: usize) -> bool {
        self.flags[self.idx(x, y, f)] & WALL == 0
    }

    fn open(&mut self, x: usize, y: usize, f: usize, zone: Zone) {
        let i = self.idx(x, y, f);
        self.flags[i] = 0;
        self.zone[i] = zone as u8;
    }

    /// Walkable cells of the 3 × 3 square at `(x, y)`; the cell `(x, y)`
    /// itself if none is walkable.
    fn free(&self, x: usize, y: usize, f: usize) -> Vec<(usize, usize, usize)> {
        let cells = self.square(x, y, f);
        if cells.is_empty() {
            vec![(x, y, f)]
        } else {
            cells
        }
    }

    fn square(&self, x: usize, y: usize, f: usize) -> Vec<(usize, usize, usize)> {
        let mut cells = Vec::new();
        for dy in 0..3 {
            for dx in 0..3 {
                if self.walkable(x + dx, y + dy, f) {
                    cells.push((x + dx, y + dy, f));
                }
            }
        }
        cells
    }
}

/// Builds the dense terminal and the targets (as dense coordinates) and the
/// entrance cells.
#[allow(clippy::type_complexity)]
fn build_dense(
    edge: usize,
    gates: usize,
) -> (
    Dense,
    Vec<Vec<(usize, usize, usize)>>,
    Vec<(usize, usize, usize)>,
) {
    let (w, h) = (edge, edge);
    let n = FLOORS * w * h;
    let mut d = Dense {
        w,
        h,
        flags: vec![WALL; n],
        zone: vec![Zone::Landside as u8; n],
        cost: vec![COST_NORMAL; n],
    };
    let ground = GROUND_INDEX;
    let yb = h / 2;
    // Ground floor (landside hall below, airside above the boundary row) and
    // the departures level above it, which exists only over the airside part.
    for bf in 0..2 {
        let f = ground + bf;
        for y in 0..h {
            for x in 0..w {
                let border = x == 0 || y == 0 || x == w - 1 || y == h - 1;
                let void = bf == 1 && y >= yb;
                // Interior walls every 64 cells (offset 32), doors every 32 cells.
                let vwall = x % 64 == 32 && y % 32 >= 3;
                let hwall = y % 64 == 32 && y != yb && x % 32 >= 3;
                let boundary = bf == 0 && y == yb;
                if !(border || void || vwall || hwall || boundary) {
                    d.open(
                        x,
                        y,
                        f,
                        if y < yb {
                            Zone::Airside
                        } else {
                            Zone::Landside
                        },
                    );
                }
            }
        }
    }
    // Security lanes (6 × 2 cells) and exit passages (2 × 3 cells) in the boundary row.
    for k in 0..6 {
        let x0 = w / 8 + k * (w * 3 / 4) / 6 + 6;
        for x in x0..x0 + 2 {
            d.open(x, yb, ground, Zone::Security);
        }
    }
    for x0 in [w / 16, w - w / 16 - 3] {
        for x in x0..x0 + 3 {
            d.open(x, yb, ground, Zone::Exit);
        }
    }
    // Floor markings: preferred corridor in front of the security line, an
    // "avoid" area in the landside hall.
    for y in yb + 1..yb + 12 {
        for x in w / 8..w - w / 8 {
            let i = d.idx(x, y, ground);
            d.cost[i] = COST_MARKED;
        }
    }
    for y in h * 3 / 4..h * 3 / 4 + 20 {
        for x in w / 3..w / 2 {
            let i = d.idx(x, y, ground);
            d.cost[i] = COST_AVOID;
        }
    }
    // Airside basement (floor -1) and lounge level (+2): copies of the
    // airside part of the ground floor (walls with doors every 64 cells).
    for f in [ground - 1, ground + 2] {
        for y in 0..yb {
            for x in 0..w {
                if d.walkable(x, y, ground) {
                    d.open(x, y, f, Zone::Airside);
                }
            }
        }
    }
    // Metro station (floor -2, landside).
    for y in yb + 20..yb + 60 {
        for x in w / 4..w * 3 / 4 {
            d.open(x, y, ground - 2, Zone::Landside);
        }
    }
    // Airside stairs through floors -1 … +2.
    for k in 0..8 {
        let x = w * (2 * k + 1) / 16;
        for f in ground - 1..=ground + 2 {
            let i = d.idx(x, yb - 20, f);
            d.flags[i] = STAIRS;
            d.zone[i] = Zone::Airside as u8;
        }
    }
    // Landside stairs: metro through a shaft on floor -1 to the hall.
    for k in 0..4 {
        let x = w / 4 + (2 * k + 1) * w / 16 + 3;
        for f in ground - 2..=ground {
            let i = d.idx(x, yb + 40, f);
            d.flags[i] = STAIRS;
            d.zone[i] = Zone::Landside as u8;
        }
    }

    let mut targets: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    for k in 0..gates {
        let x = 4 + k * (w - 12) / gates.max(1);
        targets.push(d.free(x, 4, ground + 1));
    }
    for a in 0..AIRLINES {
        let x = w / 8 + a * (w * 3 / 4) / AIRLINES;
        targets.push(d.free(x, h - 60, ground));
    }
    let multi = |d: &Dense, pts: &[(usize, usize, usize)]| -> Vec<(usize, usize, usize)> {
        pts.iter().flat_map(|&(x, y, f)| d.free(x, y, f)).collect()
    };
    targets.push(multi(
        &d,
        &[
            (w / 5, h - 30, ground),
            (w * 2 / 5, h - 30, ground),
            (w * 3 / 5, h - 30, ground),
            (w * 4 / 5, h - 30, ground),
        ],
    ));
    targets.push(multi(
        &d,
        &[
            (w / 5, yb - 40, ground),
            (w * 4 / 5, yb - 40, ground),
            (w / 5, 40, ground + 1),
            (w * 4 / 5, 40, ground + 1),
        ],
    ));
    targets.push(multi(
        &d,
        &[
            (w / 4, yb - 60, ground),
            (w / 2, yb - 60, ground),
            (w * 3 / 4, yb - 60, ground),
            (w / 6, 60, ground + 1),
            (w / 3, 60, ground + 1),
            (w / 2, 60, ground + 1),
            (w * 2 / 3, 60, ground + 1),
            (w * 5 / 6, 60, ground + 1),
            (w * 5 / 6 - 20, 90, ground + 1),
        ],
    ));
    let exit_row: Vec<(usize, usize, usize)> = (1..w - 1).map(|x| (x, h - 2, ground)).collect();
    // Persons start on the walkable cells of the entrance row.
    let entrances: Vec<(usize, usize, usize)> = exit_row
        .iter()
        .copied()
        .filter(|&(x, y, f)| d.walkable(x, y, f))
        .collect();
    targets.push(exit_row);
    // Baggage claims (airside basement), lounges (top floor), metro station.
    for k in 0..4 {
        let x = w * (2 * k + 1) / 8 + 5;
        targets.push(d.square(x, yb - 40, ground - 1));
    }
    for x in [w / 4 + 5, w * 3 / 4 + 5] {
        targets.push(d.square(x, yb / 2 + 5, ground + 2));
    }
    targets.push(
        (w / 4 + 2..w * 3 / 4 - 2)
            .map(|x| (x, yb + 50, ground - 2))
            .collect(),
    );
    (d, targets, entrances)
}

fn cell_pos(origin: (usize, usize), (x, y, f): (usize, usize, usize)) -> CellPos {
    CellPos {
        x: u16::try_from(origin.0 + x).unwrap_or(u16::MAX),
        y: u16::try_from(origin.1 + y).unwrap_or(u16::MAX),
        floor: i8::try_from(f).unwrap_or(0) - i8::try_from(GROUND_INDEX).unwrap_or(0),
    }
}

/// Number of targets of [`reference_terminal`] with `gates` gates: gates,
/// 8 check-in groups, 3 groups of toilets and shops, the landside exit, 4
/// baggage claims, 2 lounges and the metro station.
#[must_use]
pub fn target_count(gates: usize) -> usize {
    gates + AIRLINES + 3 + 1 + 4 + 2 + 1
}

/// Edge of the reference terminal of spike 1.5 (ADR-020).
pub const REFERENCE_EDGE: u16 = 768;
/// Gates of the reference terminal of spike 1.5 (36 targets).
pub const REFERENCE_GATES: u8 = 17;

/// `true` if the reference terminal of spike 1.5 fits on `map`.
#[must_use]
pub fn fits_reference_terminal(map: MapSize) -> bool {
    map.width >= REFERENCE_EDGE && map.height >= REFERENCE_EDGE
}

/// World of a debug scenario (`loadScenario`, FR-009) with `persons`
/// persons. The flow fields are computed when the session starts.
///
/// Returns `None` if the scenario does not fit on `map`.
#[must_use]
pub fn scenario_world(scenario: Scenario, map: MapSize, persons: u32, seed: u64) -> Option<World> {
    match scenario {
        Scenario::OpenHall => Some(World::open_hall(map, persons, seed)),
        Scenario::ReferenceTerminal => {
            reference_terminal(map, REFERENCE_EDGE, REFERENCE_GATES, persons, seed)
        }
    }
}

/// Builds a world with the reference terminal of `edge` × `edge` cells (at
/// least [`MIN_EDGE`]) in the middle of `map`, with `gates` gates and
/// `persons` persons starting at the entrance row. The flow fields are not
/// computed yet (`Simulation::with_world` does that).
///
/// Returns `None` if the terminal does not fit on the map or is too small.
#[must_use]
pub fn reference_terminal(
    map: MapSize,
    edge: u16,
    gates: u8,
    persons: u32,
    seed: u64,
) -> Option<World> {
    if edge < MIN_EDGE || edge > map.width || edge > map.height {
        return None;
    }
    let origin = (
        usize::from(map.width - edge) / 2,
        usize::from(map.height - edge) / 2,
    );
    let (dense, targets, entrances) = build_dense(usize::from(edge), usize::from(gates));
    let mut world = World::new(map);
    for f in 0..FLOORS {
        for y in 0..dense.h {
            for x in 0..dense.w {
                let i = dense.idx(x, y, f);
                if dense.flags[i] & WALL == 0 {
                    let cell = Cell {
                        flags: dense.flags[i],
                        zone: dense.zone[i],
                        cost: dense.cost[i],
                    };
                    world
                        .build_mut()
                        .grid_mut()
                        .set(cell_pos(origin, (x, y, f)), cell)
                        .ok()?;
                }
            }
        }
    }
    for cells in targets {
        let cells = cells.into_iter().map(|c| cell_pos(origin, c)).collect();
        world
            .build_mut()
            .add_target(Target::passengers(cells))
            .ok()?;
    }
    let mut rng = SplitMix64(seed);
    let starts: Vec<CellPos> = (0..persons)
        .map(|_| {
            cell_pos(
                origin,
                entrances[(rng.next() % entrances.len() as u64) as usize],
            )
        })
        .collect();
    world.spawn_persons(&starts, seed);
    Some(world)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wayfinding::flowfield::FieldSet;

    fn map() -> MapSize {
        MapSize::new(512, 512).unwrap()
    }

    fn pool(threads: usize) -> rayon::ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
    }

    #[test]
    fn the_terminal_must_fit_and_not_be_too_small() {
        assert!(reference_terminal(map(), 255, 17, 0, 1).is_none());
        assert!(reference_terminal(map(), 513, 17, 0, 1).is_none());
        assert!(reference_terminal(MapSize::new(512, 300).unwrap(), 301, 17, 0, 1).is_none());
        assert!(reference_terminal(map(), 256, 17, 0, 1).is_some());
    }

    #[test]
    fn the_terminal_has_the_documented_targets_and_floors() {
        let world = reference_terminal(map(), 256, 17, 100, 1).unwrap();
        assert_eq!(world.build().targets().len(), target_count(17));
        assert_eq!(world.persons().len(), 100);
        // The ground floor holds all persons at the entrance row.
        assert_eq!(world.persons().per_floor()[GROUND_INDEX], 100);
        // Floors -2 … +2 all have walkable cells (metro … lounges).
        let origin = (512 - 256) / 2;
        for floor in -2..=2i8 {
            let any = (0..256u16).any(|x| {
                (0..256u16).any(|y| {
                    world.build().grid().walkable(CellPos {
                        x: origin + x,
                        y: origin + y,
                        floor,
                    })
                })
            });
            assert!(any, "floor {floor} is empty");
        }
        assert!(!world.build().grid().walkable(CellPos {
            x: 0,
            y: 0,
            floor: 0
        }));
    }

    #[test]
    fn every_target_is_reachable_from_the_entrances() {
        let mut world = reference_terminal(map(), 256, 17, 50, 7).unwrap();
        world.rebuild_fields_now(&pool(2));
        let entrance = |x: u16| CellPos {
            x: 128 + x,
            y: 128 + 254,
            floor: 0,
        };
        for target in 0..world.build().targets().len() {
            let target = u8::try_from(target).unwrap();
            assert!(
                world
                    .wayfinding()
                    .fields()
                    .path_cost(target, entrance(5))
                    .is_some(),
                "target {target} not reachable"
            );
            assert!(
                world
                    .wayfinding()
                    .fields()
                    .path_cost(target, entrance(200))
                    .is_some()
            );
        }
        assert!(world.wayfinding().fields().cells() > 0);
    }

    #[test]
    fn terminal_with_its_persons_runs_and_floors_are_used() {
        let pool = pool(2);
        let mut world = reference_terminal(map(), 256, 17, 400, 3).unwrap();
        world.rebuild_fields_now(&pool);
        for _ in 0..30_000 {
            world.step(&pool, &pool, 0, true);
        }
        let per_floor = world.persons().per_floor();
        assert_eq!(per_floor.iter().sum::<u32>(), 400);
        assert!(
            per_floor.iter().filter(|&&n| n > 0).count() >= 3,
            "persons use several floors: {per_floor:?}"
        );
    }

    #[test]
    fn unreachable_counts_are_zero_for_the_reference_terminal() {
        let world = reference_terminal(map(), 256, 17, 0, 1).unwrap();
        let input = crate::wayfinding::Wayfinding::input(world.build());
        let set = pool(2).install(|| FieldSet::compute(&input, 0));
        assert_eq!(set.targets(), target_count(17));
        let stuck: u32 = (0..set.targets())
            .map(|t| set.unreachable_cells(u8::try_from(t).unwrap()))
            .sum();
        assert_eq!(stuck, 0);
    }
}
