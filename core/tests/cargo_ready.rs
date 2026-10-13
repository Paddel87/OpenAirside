//! FR-046: the data model is prepared for freight. The load kind "freight"
//! exists in the schema and targets can serve it; there are no game
//! mechanics for it yet (FR-048, after 1.0).

use openairside_core::geometry::{CellPos, CellRect, MapSize};
use openairside_core::grid::{Cell, Zone};
use openairside_core::target::{LoadKind, Target};
use openairside_core::world::World;

#[test]
fn freight_is_part_of_the_schema() {
    // The kind is a plain value of the schema; the TypeScript type is
    // generated from it (`app/src/generated/LoadKind.ts`, checked in the CI).
    let kinds = [LoadKind::Passengers, LoadKind::Freight];
    assert_eq!(kinds.len(), 2);
    assert_ne!(kinds[0], kinds[1]);
}

#[test]
fn a_world_holds_freight_targets_next_to_passenger_targets() {
    let mut world = World::new(MapSize::new(64, 64).unwrap());
    let rect = CellRect {
        floor: 0,
        x0: 0,
        y0: 0,
        x1: 16,
        y1: 16,
    };
    world
        .grid_mut()
        .fill(rect, Cell::floor(Zone::Landside))
        .unwrap();
    let cell = CellPos {
        x: 3,
        y: 3,
        floor: 0,
    };
    let passengers = world.add_target(Target::passengers(vec![cell])).unwrap();
    let freight = world
        .add_target(Target {
            cells: vec![cell],
            load: LoadKind::Freight,
        })
        .unwrap();
    assert_eq!(
        world.targets()[usize::from(passengers)].load,
        LoadKind::Passengers
    );
    assert_eq!(
        world.targets()[usize::from(freight)].load,
        LoadKind::Freight
    );
}
