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
    AreaKind, BLOCK, COST_AVOID, COST_MARKED, COST_NORMAL, Cell, FLOORS, GROUND_INDEX, STAIRS,
    WALL, Zone,
};
use crate::build::target::Target;
use crate::command::Scenario;
use crate::geometry::{CellPos, CellRect, MapSize, Rotation};
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
// The three results of the one-off scenario builder are used once, by `reference_terminal`; a named
// struct would only rename the tuple (fahrplan-ref: 3.10).
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
        Scenario::FurnishedTerminal => {
            let mut world =
                reference_terminal(map, REFERENCE_EDGE, REFERENCE_GATES, persons, seed)?;
            furnish(&mut world, REFERENCE_EDGE, FURNISHED_OBJECTS, seed);
            Some(world)
        }
    }
}

/// Types the furnished terminal uses; ids of the base pack (step 3.1b).
const FURNISH_FLOOR: &str = "base:floor_concrete";
const FURNISH_WALL: &str = "base:wall_concrete";
const FURNISH_OBJECT: &str = "base:bench";
/// Objects of the furnished terminal: the estimate of step 2.10 for the
/// reference scenario (several thousand seats, counters, plants and signs,
/// doubled as headroom; re-measured in 4.1).
pub const FURNISHED_OBJECTS: usize = 20_000;
/// Footprint of a bench in cells (4 m × 2 m).
const BENCH_SIZE: (u16, u16) = (2, 1);

/// Origin of the terminal of `edge` cells in the middle of `map`.
fn terminal_origin(map: MapSize, edge: u16) -> (u16, u16) {
    ((map.width - edge) / 2, (map.height - edge) / 2)
}

/// The field of the measurement's build actions in the terminal, in cells
/// `x0, y0, x1, y1` (half open): the landside hall below the security line
/// (`app/renderer/measure.ts` `buildArea` draws its 3 × 3 areas there). The
/// furnished terminal keeps it free of objects, so that walls can be built
/// and demolished there.
#[must_use]
pub fn measurement_field(map: MapSize, edge: u16) -> (u16, u16, u16, u16) {
    let (ox, oy) = terminal_origin(map, edge);
    let x0 = ox + edge / 8;
    let y0 = oy + edge / 2 + 15;
    (x0, y0, x0 + edge / 4 * 3 + 3, y0 + (edge / 2 - 90) + 3)
}

/// Gives the ground floor of the terminal floor coverings, walls and
/// objects of the base pack ([`Scenario::FurnishedTerminal`]): the
/// walkable cells get the floor, void cells next to them the wall, and
/// `objects` benches stand on free floor in the blocks with enough room,
/// outside the [`measurement_field`]. Deterministic in `seed`.
pub fn furnish(world: &mut World, edge: u16, objects: usize, seed: u64) {
    let map = world.map();
    let (ox, oy) = terminal_origin(map, edge);
    // Floor under the walkable cells, per run of a row: a floor drawn over
    // void would pave it (`buildArea` semantics).
    for y in oy..oy + edge {
        let mut run: Option<u16> = None;
        for x in ox..=ox + edge {
            let walkable =
                x < ox + edge && world.build().grid().walkable(CellPos { x, y, floor: 0 });
            match (run, walkable) {
                (None, true) => run = Some(x),
                (Some(start), false) => {
                    let rect = CellRect {
                        floor: 0,
                        x0: start,
                        y0: y,
                        x1: x,
                        y1: y + 1,
                    };
                    let _ =
                        world.build_floor(FURNISH_FLOOR, AreaKind::PublicWalk, COST_NORMAL, rect);
                    run = None;
                }
                _ => {}
            }
        }
    }
    // Walls where void touches walkable cells, merged into runs per row.
    for y in oy..oy + edge {
        let mut run: Option<u16> = None;
        for x in ox..=ox + edge {
            let at_wall = x < ox + edge
                && {
                    let grid = world.build().grid();
                    let pos = |x: u16, y: u16| CellPos { x, y, floor: 0 };
                    let void = grid
                        .cell(pos(x, y))
                        .is_some_and(|c| !c.walkable() && c.wall_kind == 0);
                    void && [
                    (x.checked_sub(1), Some(y)),
                    (Some(x + 1), Some(y)),
                    (Some(x), y.checked_sub(1)),
                    (Some(x), Some(y + 1)),
                ]
                .into_iter()
                .any(|(nx, ny)| matches!((nx, ny), (Some(nx), Some(ny)) if grid.walkable(pos(nx, ny))))
                };
            match (run, at_wall) {
                (None, true) => run = Some(x),
                (Some(start), false) => {
                    let rect = CellRect {
                        floor: 0,
                        x0: start,
                        y0: y,
                        x1: x,
                        y1: y + 1,
                    };
                    let _ = world.build_wall(FURNISH_WALL, rect);
                    run = None;
                }
                _ => {}
            }
        }
    }
    let (fx0, fy0, fx1, fy1) = measurement_field(map, edge);
    let blocks_x = usize::from(map.width).div_ceil(BLOCK);
    let ground = GROUND_INDEX * blocks_x * usize::from(map.height).div_ceil(BLOCK);
    let walkable: Vec<usize> = {
        let snapshot = world.build().grid().snapshot();
        (0..blocks_x * usize::from(map.height).div_ceil(BLOCK))
            .filter(|&i| {
                snapshot
                    .block(ground + i)
                    .is_some_and(|b| b.walkable_cells() >= 100)
            })
            .filter(|&i| {
                let (bx0, by0) = ((i % blocks_x * BLOCK) as u16, (i / blocks_x * BLOCK) as u16);
                // Two cells of margin: a bench reaches into the next block.
                bx0 + BLOCK as u16 + 2 <= fx0
                    || bx0 >= fx1 + 2
                    || by0 + BLOCK as u16 + 2 <= fy0
                    || by0 >= fy1 + 2
            })
            .collect()
    };
    if walkable.is_empty() {
        return;
    }
    let per_block = objects.div_ceil(walkable.len());
    let mut rng = SplitMix64(seed ^ 0x9e37_79b9_7f4a_7c15);
    let mut placed = 0usize;
    for &i in &walkable {
        let (bx0, by0) = ((i % blocks_x * BLOCK) as u16, (i / blocks_x * BLOCK) as u16);
        let mut tries = 0usize;
        let mut here = 0usize;
        while here < per_block && placed < objects && tries < per_block * 6 {
            tries += 1;
            let r = rng.next();
            let at = CellPos {
                x: bx0 + (r % BLOCK as u64) as u16,
                y: by0 + ((r >> 8) % BLOCK as u64) as u16,
                floor: 0,
            };
            let rotation = Rotation(((r >> 16) % 4) as u8);
            if world
                .build_object(FURNISH_OBJECT, BENCH_SIZE, false, at, rotation)
                .is_ok()
            {
                here += 1;
                placed += 1;
            }
        }
    }
    world.discard_events();
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
                        ..Cell::VOID
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

    #[test]
    fn the_furnished_terminal_has_floors_walls_and_free_build_field() {
        let map = MapSize::new(2048, 2048).unwrap();
        let world = scenario_world(Scenario::FurnishedTerminal, map, 0, 7).unwrap();
        let build = world.build();
        let objects = build.objects().as_slice();
        assert!(
            (FURNISHED_OBJECTS - 200..=FURNISHED_OBJECTS).contains(&objects.len()),
            "{} objects",
            objects.len()
        );
        assert_eq!(
            build.types().ids(),
            [FURNISH_FLOOR, FURNISH_WALL, FURNISH_OBJECT]
        );
        // A walkable cell of the hall has the floor, a wall cell next to it the wall.
        let (x0, y0, x1, y1) = measurement_field(map, REFERENCE_EDGE);
        let hall = CellPos {
            x: x0 + 10,
            y: y0 + 10,
            floor: 0,
        };
        let cell = build.grid().cell(hall).unwrap();
        assert!(cell.walkable());
        assert_eq!(cell.floor_kind, 1);
        let walls = (0..build.grid().snapshot().table_len())
            .filter_map(|i| {
                build
                    .grid()
                    .snapshot()
                    .block(i)
                    .map(|b| b.wall_kinds().iter().filter(|w| **w == 2).count())
            })
            .sum::<usize>();
        assert!(walls > 1000, "{walls} wall cells");
        // The field of the measurement's build actions stays free.
        assert!(
            objects
                .iter()
                .all(|o| o.x + o.width <= x0 || o.x >= x1 || o.y + o.depth <= y0 || o.y >= y1),
            "an object stands in the build field"
        );
        // The persons can still walk: no cell under a bench is blocked.
        assert!(objects.iter().all(|o| !o.blocks));
    }

    #[test]
    fn the_furnished_terminal_is_deterministic_and_keeps_its_targets() {
        let map = MapSize::new(2048, 2048).unwrap();
        let hash = |seed| {
            let world = scenario_world(Scenario::FurnishedTerminal, map, 100, seed).unwrap();
            let mut hasher = crate::hash::StateHasher::new();
            world.hash_into(&mut hasher);
            (hasher.finish(), world.build().targets().len())
        };
        let (a, targets) = hash(5);
        assert_eq!((a, targets), hash(5));
        assert_ne!(a, hash(6).0);
        assert_eq!(targets, super::target_count(usize::from(REFERENCE_GATES)));
    }
}
