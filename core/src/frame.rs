//! State buffer S-1: snapshots of the visible state, captured by the
//! simulation thread and handed to the reader through the triple buffer of
//! `runner`, and their compact binary encoding with a filter on floor and camera rectangle
//! (`docs/architecture.md` section 4, spike 1.3).
//!
//! Layout version 2 (ADR-038, step 3.1b), little endian. Header of
//! [`HEADER_BYTES`] bytes:
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
//! | 32 | u32 | world revision of this frame, 0 if it carries no block |
//! | 36 | u32 | vehicles in this frame |
//! | 40 | u32 | aircraft in this frame |
//! | 44 | u32 | baggage items in this frame |
//!
//! Then the sections in fixed order, each on a 4-byte boundary: n persons of
//! [`PERSON_BYTES`] bytes (x and y as u16 in 1/32 cell [`SUBCELL`], floor
//! i8, heading u8 in 256 steps, group u8, state u8), vehicles
//! ([`VEHICLE_BYTES`]), aircraft ([`AIRCRAFT_BYTES`]) and baggage
//! ([`BAGGAGE_BYTES`]) with the counts of the header, and the world section
//! of `display` (always present, at least its header). Group and state of
//! persons are 0 until step 3.2; the sections of vehicles, aircraft and
//! baggage are empty until steps 3.4 and 3.5.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::build::grid::floor_index;
use crate::datapack::Catalog;
use crate::display::{WorldSender, WorldView};
use crate::geometry::MapSize;
use crate::time::Tick;
use crate::world::World;

/// Version of the frame layout; changes with every non-additive change.
pub const LAYOUT_VERSION: u16 = 2;
/// Size of the frame header in bytes.
pub const HEADER_BYTES: usize = 48;
/// Size of one person in a frame in bytes.
pub const PERSON_BYTES: usize = 8;
/// Size of one vehicle in a frame in bytes (section empty until step 3.4).
pub const VEHICLE_BYTES: usize = 12;
/// Size of one aircraft in a frame in bytes (section empty until step 3.4).
pub const AIRCRAFT_BYTES: usize = 16;
/// Size of one baggage item in a frame in bytes (section empty until step 3.5).
pub const BAGGAGE_BYTES: usize = 8;
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

/// A person of a snapshot as seen by the query service (S-7).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PersonPoint {
    /// Position east in cells.
    pub x: f32,
    /// Position south in cells.
    pub y: f32,
    /// Floor.
    pub floor: i8,
    /// Direction of the current step.
    pub heading: u8,
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
    /// Content of the session (S-7 `catalog`); it changes only with the
    /// session, so a capture only counts a reference.
    content: Arc<Catalog>,
    /// The resting world; taken again only when it changed.
    world: WorldView,
}

fn fixed(cells: f32, limit: u16) -> u16 {
    let max = f32::from(limit) * SUBCELL - 1.0;
    // Clamped into 0 … u16::MAX before the conversion.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let value = (cells * SUBCELL).clamp(0.0, max) as u16;
    value
}

impl Snapshot {
    /// Overwrites the snapshot with the state of `world` and the content of
    /// the session, reusing the buffer.
    pub fn capture(&mut self, session: u32, tick: Tick, world: &World, content: &Arc<Catalog>) {
        let map = world.map();
        if !Arc::ptr_eq(&self.content, content) {
            self.content = Arc::clone(content);
        }
        self.session = session;
        self.tick = tick;
        self.width = map.width;
        self.height = map.height;
        self.fields_revision = world.wayfinding().fields_revision();
        self.world.refresh(session, world.build(), content);
        self.persons.clear();
        self.persons.reserve(world.persons().len());
        let persons = &mut self.persons;
        world.persons().visit(|p| {
            persons.push(Packed {
                x: fixed(p.x, map.width),
                y: fixed(p.y, map.height),
                floor: p.floor,
                heading: p.heading,
            });
        });
    }

    /// Content of the session of the snapshot; empty before the first
    /// capture.
    #[must_use]
    pub fn content(&self) -> &Catalog {
        &self.content
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

    /// Person `index` with its position in cells (resolution 1/32 cell).
    #[must_use]
    pub fn person(&self, index: usize) -> Option<PersonPoint> {
        self.persons.get(index).map(|p| PersonPoint {
            x: f32::from(p.x) / SUBCELL,
            y: f32::from(p.y) / SUBCELL,
            floor: p.floor,
            heading: p.heading,
        })
    }

    /// The resting world of the snapshot.
    #[must_use]
    pub fn world(&self) -> &WorldView {
        &self.world
    }

    /// Bytes of the frame without the blocks of the world section.
    #[must_use]
    pub fn max_frame_bytes(&self) -> usize {
        HEADER_BYTES + self.persons.len() * PERSON_BYTES + crate::display::SECTION_HEADER
    }

    /// Encodes the persons matching `filter` and the changed blocks of the
    /// world that `world` has not handed out yet into `out` (cleared
    /// first). An invalid filter yields a frame without persons and with
    /// the blocks of the ground floor.
    pub fn encode(&self, filter: &FrameFilter, world: &mut WorldSender, out: &mut Vec<u8>) {
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
                // Heading, group and state (group and state are 0 until 3.2).
                out.extend_from_slice(&[p.heading, 0, 0]);
                count += 1;
            }
        }
        let revision = world.write(
            &self.world,
            self.session,
            floor,
            bounds.map(|[x0, y0, x1, y1]| ViewRect {
                x0: x0 / SUBCELL,
                y0: y0 / SUBCELL,
                x1: x1 / SUBCELL,
                y1: y1 / SUBCELL,
            }),
            out,
        );
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
        header[32..36].copy_from_slice(&revision.to_le_bytes());
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
        let count = u32::from_le_bytes(frame[20..24].try_into().unwrap()) as usize;
        frame[HEADER_BYTES..HEADER_BYTES + count * PERSON_BYTES]
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
        snap.capture(3, 99, &world(), &Arc::default());
        let mut out = Vec::new();
        snap.encode(
            &FrameFilter::default(),
            &mut WorldSender::default(),
            &mut out,
        );
        // Persons, then the world section with the four blocks of the hall.
        let section = HEADER_BYTES + 3 * PERSON_BYTES;
        assert_eq!(
            header_u32(&out, section).map(|bytes| section + bytes as usize),
            Some(out.len())
        );
        assert_eq!(header_u32(&out, section + 4), Some(4));
        assert_eq!(
            header_u32(&out, 32),
            Some(1),
            "the first frame carries the ground floor"
        );
        assert_eq!(
            (
                header_u32(&out, 36),
                header_u32(&out, 40),
                header_u32(&out, 44)
            ),
            (Some(0), Some(0), Some(0)),
            "no vehicles, aircraft or baggage yet"
        );
        assert_eq!(u16::from_le_bytes([out[0], out[1]]), LAYOUT_VERSION);
        assert_eq!(u16::from_le_bytes([out[2], out[3]]), 48);
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
        snap.capture(1, 0, &world(), &Arc::default());
        let mut out = Vec::new();
        snap.encode(
            &FrameFilter::default(),
            &mut WorldSender::default(),
            &mut out,
        );
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
        snap.capture(1, 0, &world(), &Arc::default());
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
            &mut WorldSender::default(),
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
            &mut WorldSender::default(),
            &mut out,
        );
        assert!(persons(&out).is_empty());
        assert_eq!(header_u32(&out, 16), Some(3));
    }

    #[test]
    fn invalid_filter_yields_no_persons() {
        let mut snap = Snapshot::default();
        snap.capture(1, 0, &world(), &Arc::default());
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
            snap.encode(&filter, &mut WorldSender::default(), &mut out);
            assert_eq!(header_u32(&out, 20), Some(0));
        }
    }
}
