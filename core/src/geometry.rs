//! World coordinates of the interface contracts (`docs/architecture.md`
//! section 4, "Weltkoordinaten"; ADR-020: 2 m per cell, maps up to 2048²).

use serde::{Deserialize, Serialize};

/// Lowest floor (two basement levels, BAU-07).
pub const MIN_FLOOR: i8 = -2;
/// Highest floor.
pub const MAX_FLOOR: i8 = 2;
/// Largest supported map edge in cells (ADR-020).
pub const MAX_MAP_EDGE: u16 = 2048;

/// Size of the map in cells; the same for every floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct MapSize {
    /// Number of cells from west to east.
    pub width: u16,
    /// Number of cells from north to south.
    pub height: u16,
}

impl MapSize {
    /// Creates a map size; `None` if an edge is zero or larger than
    /// [`MAX_MAP_EDGE`].
    #[must_use]
    pub fn new(width: u16, height: u16) -> Option<Self> {
        let valid = |edge: u16| (1..=MAX_MAP_EDGE).contains(&edge);
        (valid(width) && valid(height)).then_some(Self { width, height })
    }

    /// Returns `true` if the cell lies on the map.
    #[must_use]
    pub fn contains(self, pos: CellPos) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    /// Returns `true` if the half-open rectangle lies completely on the map.
    #[must_use]
    pub fn contains_rect(self, rect: CellRect) -> bool {
        rect.x1 <= self.width && rect.y1 <= self.height
    }
}

/// A cell of the world grid. Origin north-west, x to the east, y to the south.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CellPos {
    /// Column, 0 … map width − 1.
    pub x: u16,
    /// Row, 0 … map height − 1.
    pub y: u16,
    /// Floor, −2 … +2, 0 = ground floor.
    pub floor: i8,
}

/// Half-open rectangle of cells on one floor: `x0 ≤ x < x1`, `y0 ≤ y < y1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CellRect {
    /// Floor, −2 … +2.
    pub floor: i8,
    /// First column (inclusive).
    pub x0: u16,
    /// First row (inclusive).
    pub y0: u16,
    /// Last column (exclusive).
    pub x1: u16,
    /// Last row (exclusive).
    pub y1: u16,
}

impl CellRect {
    /// Returns `true` if the rectangle covers at least one cell.
    #[must_use]
    pub fn is_non_empty(self) -> bool {
        self.x1 > self.x0 && self.y1 > self.y0
    }

    /// Width in cells (0 for an inverted rectangle).
    #[must_use]
    pub fn width(self) -> u16 {
        self.x1.saturating_sub(self.x0)
    }

    /// Height in cells (0 for an inverted rectangle).
    #[must_use]
    pub fn height(self) -> u16 {
        self.y1.saturating_sub(self.y0)
    }
}

/// Rotation in multiples of 90° clockwise, 0 = north. Valid values 0 … 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export, type = "0 | 1 | 2 | 3"))]
pub struct Rotation(pub u8);

impl Rotation {
    /// Returns `true` for the values 0 … 3.
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 <= 3
    }
}

/// Reference to an object in the world state; stale handles are detected by
/// their generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Handle {
    /// Slot in the object table.
    pub index: u32,
    /// Generation of the slot when the handle was issued.
    pub generation: u32,
}

/// Returns `true` if the floor lies in −2 … +2.
#[must_use]
pub fn is_valid_floor(floor: i8) -> bool {
    (MIN_FLOOR..=MAX_FLOOR).contains(&floor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_size_rejects_zero_and_oversized_edges() {
        assert!(MapSize::new(0, 10).is_none());
        assert!(MapSize::new(10, 2049).is_none());
        assert_eq!(
            MapSize::new(2048, 1536),
            Some(MapSize {
                width: 2048,
                height: 1536
            })
        );
    }

    #[test]
    fn map_contains_cells_and_half_open_rectangles() {
        let map = MapSize::new(16, 8).unwrap();
        assert!(map.contains(CellPos {
            x: 15,
            y: 7,
            floor: 0
        }));
        assert!(!map.contains(CellPos {
            x: 16,
            y: 0,
            floor: 0
        }));
        let rect = CellRect {
            floor: 0,
            x0: 0,
            y0: 0,
            x1: 16,
            y1: 8,
        };
        assert!(map.contains_rect(rect));
        assert!(!map.contains_rect(CellRect { y1: 9, ..rect }));
    }

    #[test]
    fn rect_size_and_emptiness() {
        let rect = CellRect {
            floor: 0,
            x0: 2,
            y0: 3,
            x1: 5,
            y1: 4,
        };
        assert!(rect.is_non_empty());
        assert_eq!((rect.width(), rect.height()), (3, 1));
        let inverted = CellRect { x1: 1, ..rect };
        assert!(!inverted.is_non_empty());
        assert_eq!(inverted.width(), 0);
    }

    #[test]
    fn rotation_and_floor_ranges() {
        assert!(Rotation(3).is_valid());
        assert!(!Rotation(4).is_valid());
        assert!(is_valid_floor(-2) && is_valid_floor(2));
        assert!(!is_valid_floor(-3) && !is_valid_floor(3));
    }
}
