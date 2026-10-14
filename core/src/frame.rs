//! State buffer S-1: snapshots of the visible state, captured by the
//! simulation thread and handed to the reader through the triple buffer of
//! `runner`, and their compact binary encoding with a filter on floor and camera rectangle
//! (`docs/architecture.md` section 4, spike 1.3).
//!
//! Layout version 1, little endian. Header of [`HEADER_BYTES`] bytes:
//!
//! | Offset | Type | Field |
//! |---|---|---|
//! | 0 | u16 | layout version ([`LAYOUT_VERSION`]) |
//! | 2 | u16 | header size in bytes |
//! | 4 | u32 | session |
//! | 8 | u32 | tick |
//! | 12 | u16 | map width in cells |
//! | 14 | u16 | map height in cells |
//! | 16 | u32 | persons in the world |
//! | 20 | u32 | persons in this frame (n) |
//! | 24 | i8 | floor of the filter, [`ALL_FLOORS`] = no floor filter |
//! | 25 | u8 | bytes per person ([`PERSON_BYTES`]) |
//! | 26 | u16 | reserved, 0 |
//! | 28 | u32 | revision of the flow fields in use |
//!
//! Then n persons of [`PERSON_BYTES`] bytes: x and y as u16 in 1/32 cell
//! ([`SUBCELL`]), floor as i8, heading as u8.

use serde::{Deserialize, Serialize};

use crate::geometry::MapSize;
use crate::grid::floor_index;
use crate::time::Tick;
use crate::world::World;

/// Version of the frame layout; changes with every non-additive change.
pub const LAYOUT_VERSION: u16 = 1;
/// Size of the frame header in bytes.
pub const HEADER_BYTES: usize = 32;
/// Size of one person in a frame in bytes.
pub const PERSON_BYTES: usize = 6;
/// Steps per cell of the fixed-point positions.
pub const SUBCELL: f32 = 32.0;
/// Floor value in the header when the frame holds all floors.
pub const ALL_FLOORS: i8 = i8::MAX;
/// Extra cells around the camera rectangle, so that persons walking into
/// the picture do not pop up at its edge.
pub const VIEW_MARGIN: f32 = 4.0;

/// Camera rectangle in cells; half open like `CellRect`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct ViewRect {
    /// West edge.
    pub x0: f32,
    /// North edge.
    pub y0: f32,
    /// East edge.
    pub x1: f32,
    /// South edge.
    pub y1: f32,
}

impl ViewRect {
    fn is_valid(self) -> bool {
        [self.x0, self.y0, self.x1, self.y1]
            .iter()
            .all(|v| v.is_finite())
            && self.x1 > self.x0
            && self.y1 > self.y0
    }
}

/// What the renderer asks for with a frame request.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct FrameFilter {
    /// Only persons on this floor; all floors if absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub floor: Option<i8>,
    /// Only persons inside this rectangle (plus [`VIEW_MARGIN`]); the whole
    /// map if absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub view: Option<ViewRect>,
}

impl FrameFilter {
    /// `true` if the floor is valid and the rectangle is finite and not
    /// empty.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.floor.is_none_or(|f| floor_index(f).is_some())
            && self.view.is_none_or(ViewRect::is_valid)
    }
}

/// One person as stored in a snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Packed {
    x: u16,
    y: u16,
    floor: i8,
    heading: u8,
}

/// Visible state of one tick, captured by the simulation thread.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Snapshot {
    session: u32,
    tick: Tick,
    width: u16,
    height: u16,
    fields_revision: u32,
    persons: Vec<Packed>,
}

fn fixed(cells: f32, limit: u16) -> u16 {
    let max = f32::from(limit) * SUBCELL - 1.0;
    // Clamped into 0 … u16::MAX before the conversion.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let value = (cells * SUBCELL).clamp(0.0, max) as u16;
    value
}

impl Snapshot {
    /// Overwrites the snapshot with the state of `world`, reusing the
    /// buffer.
    pub fn capture(&mut self, session: u32, tick: Tick, world: &World) {
        let map = world.map();
        self.session = session;
        self.tick = tick;
        self.width = map.width;
        self.height = map.height;
        self.fields_revision = world.fields_revision();
        self.persons.clear();
        self.persons.reserve(world.persons());
        let persons = &mut self.persons;
        world.visit_persons(|p| {
            persons.push(Packed {
                x: fixed(p.x, map.width),
                y: fixed(p.y, map.height),
                floor: p.floor,
                heading: p.heading,
            });
        });
    }

    /// Session of the snapshot; 0 before the first capture.
    #[must_use]
    pub fn session(&self) -> u32 {
        self.session
    }

    /// Tick of the snapshot.
    #[must_use]
    pub fn tick(&self) -> Tick {
        self.tick
    }

    /// Size of the map; `None` before the first capture.
    #[must_use]
    pub fn map(&self) -> Option<MapSize> {
        MapSize::new(self.width, self.height)
    }

    /// Persons in the snapshot.
    #[must_use]
    pub fn persons(&self) -> usize {
        self.persons.len()
    }

    /// Largest frame this snapshot can produce, in bytes.
    #[must_use]
    pub fn max_frame_bytes(&self) -> usize {
        HEADER_BYTES + self.persons.len() * PERSON_BYTES
    }

    /// Encodes the persons matching `filter` into `out` (cleared first).
    /// An invalid filter yields a frame without persons.
    pub fn encode(&self, filter: &FrameFilter, out: &mut Vec<u8>) {
        out.clear();
        out.resize(HEADER_BYTES, 0);
        let valid = filter.is_valid();
        let floor = if valid { filter.floor } else { None };
        let bounds = filter.view.filter(|_| valid).map(|v| {
            [
                (v.x0 - VIEW_MARGIN) * SUBCELL,
                (v.y0 - VIEW_MARGIN) * SUBCELL,
                (v.x1 + VIEW_MARGIN) * SUBCELL,
                (v.y1 + VIEW_MARGIN) * SUBCELL,
            ]
        });
        let mut count = 0u32;
        if valid {
            for p in &self.persons {
                if floor.is_some_and(|f| f != p.floor) {
                    continue;
                }
                if let Some([x0, y0, x1, y1]) = bounds {
                    let (x, y) = (f32::from(p.x), f32::from(p.y));
                    if x < x0 || x >= x1 || y < y0 || y >= y1 {
                        continue;
                    }
                }
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
                out.extend_from_slice(&p.floor.to_le_bytes());
                out.push(p.heading);
                count += 1;
            }
        }
        let total = u32::try_from(self.persons.len()).unwrap_or(u32::MAX);
        let header = &mut out[..HEADER_BYTES];
        header[0..2].copy_from_slice(&LAYOUT_VERSION.to_le_bytes());
        header[2..4].copy_from_slice(&u16::try_from(HEADER_BYTES).unwrap_or(0).to_le_bytes());
        header[4..8].copy_from_slice(&self.session.to_le_bytes());
        header[8..12].copy_from_slice(&self.tick.to_le_bytes());
        header[12..14].copy_from_slice(&self.width.to_le_bytes());
        header[14..16].copy_from_slice(&self.height.to_le_bytes());
        header[16..20].copy_from_slice(&total.to_le_bytes());
        header[20..24].copy_from_slice(&count.to_le_bytes());
        header[24..25].copy_from_slice(&floor.unwrap_or(ALL_FLOORS).to_le_bytes());
        header[25] = u8::try_from(PERSON_BYTES).unwrap_or(0);
        header[28..32].copy_from_slice(&self.fields_revision.to_le_bytes());
    }
}

/// Reads a frame header; used by tests and by tools.
#[must_use]
pub fn header_u32(frame: &[u8], offset: usize) -> Option<u32> {
    let bytes = frame.get(offset..offset + 4)?;
    Some(u32::from_le_bytes(bytes.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::CellPos;

    fn world() -> World {
        let mut world = World::open_hall(MapSize::new(64, 64).unwrap(), 0, 1);
        world.spawn_persons(
            &[
                CellPos {
                    x: 1,
                    y: 1,
                    floor: 0,
                },
                CellPos {
                    x: 40,
                    y: 50,
                    floor: 0,
                },
                CellPos {
                    x: 63,
                    y: 63,
                    floor: 0,
                },
            ],
            7,
        );
        world
    }

    fn persons(frame: &[u8]) -> Vec<(u16, u16, i8)> {
        frame[HEADER_BYTES..]
            .as_chunks::<PERSON_BYTES>()
            .0
            .iter()
            .map(|c| {
                (
                    u16::from_le_bytes([c[0], c[1]]),
                    u16::from_le_bytes([c[2], c[3]]),
                    i8::from_le_bytes([c[4]]),
                )
            })
            .collect()
    }

    #[test]
    fn header_describes_the_frame() {
        let mut snap = Snapshot::default();
        assert_eq!(snap.map(), None);
        snap.capture(3, 99, &world());
        let mut out = Vec::new();
        snap.encode(&FrameFilter::default(), &mut out);
        assert_eq!(out.len(), HEADER_BYTES + 3 * PERSON_BYTES);
        assert_eq!(out.len(), snap.max_frame_bytes());
        assert_eq!(u16::from_le_bytes([out[0], out[1]]), LAYOUT_VERSION);
        assert_eq!(u16::from_le_bytes([out[2], out[3]]), 32);
        assert_eq!(header_u32(&out, 4), Some(3));
        assert_eq!(header_u32(&out, 8), Some(99));
        assert_eq!(u16::from_le_bytes([out[12], out[13]]), 64);
        assert_eq!(header_u32(&out, 16), Some(3));
        assert_eq!(header_u32(&out, 20), Some(3));
        assert_eq!(out[24], 0x7f);
        assert_eq!(usize::from(out[25]), PERSON_BYTES);
        assert_eq!(header_u32(&out, 28), Some(0));
        assert_eq!((snap.session(), snap.tick(), snap.persons()), (3, 99, 3));
        assert_eq!(header_u32(&out, 200), None);
    }

    #[test]
    fn positions_are_cell_centres_in_fixed_point() {
        let mut snap = Snapshot::default();
        snap.capture(1, 0, &world());
        let mut out = Vec::new();
        snap.encode(&FrameFilter::default(), &mut out);
        let got = persons(&out);
        assert_eq!(got[0], (48, 48, 0)); // (1 + 0.5) × 32
        assert_eq!(got[1], (1296, 1616, 0));
        // The last cell stays inside the map.
        assert_eq!(got[2], (2032, 2032, 0));
        assert_eq!(fixed(80.0, 64), 2047);
        assert_eq!(fixed(-1.0, 64), 0);
    }

    #[test]
    fn filter_selects_floor_and_view_with_margin() {
        let mut snap = Snapshot::default();
        snap.capture(1, 0, &world());
        let mut out = Vec::new();
        let view = ViewRect {
            x0: 30.0,
            y0: 40.0,
            x1: 38.0,
            y1: 48.0,
        };
        snap.encode(
            &FrameFilter {
                floor: Some(0),
                view: Some(view),
            },
            &mut out,
        );
        // (40.5, 50.5) lies inside the rectangle plus 4 cells margin.
        assert_eq!(persons(&out), vec![(1296, 1616, 0)]);
        assert_eq!(out[24], 0);
        snap.encode(
            &FrameFilter {
                floor: Some(1),
                view: None,
            },
            &mut out,
        );
        assert!(persons(&out).is_empty());
        assert_eq!(header_u32(&out, 16), Some(3));
    }

    #[test]
    fn invalid_filter_yields_no_persons() {
        let mut snap = Snapshot::default();
        snap.capture(1, 0, &world());
        let mut out = Vec::new();
        for filter in [
            FrameFilter {
                floor: Some(3),
                view: None,
            },
            FrameFilter {
                floor: None,
                view: Some(ViewRect {
                    x0: 1.0,
                    y0: 1.0,
                    x1: 1.0,
                    y1: 5.0,
                }),
            },
            FrameFilter {
                floor: None,
                view: Some(ViewRect {
                    x0: f32::NAN,
                    y0: 0.0,
                    x1: 5.0,
                    y1: 5.0,
                }),
            },
        ] {
            assert!(!filter.is_valid());
            snap.encode(&filter, &mut out);
            assert_eq!(out.len(), HEADER_BYTES);
        }
    }
}
