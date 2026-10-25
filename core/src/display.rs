//! Display data of the resting world for the state buffer S-1, layout
//! version 2 (ADR-038, step 3.1b): the blocks of all floors with floor
//! covering, wall type, flags and objects, and the sender that hands out the
//! blocks that changed since the ones the renderer acknowledged.
//!
//! The cells and objects of the module "Welt & Bau" refer to numbers of its
//! type table (ADR-042); the display data refer to the catalog of the
//! session instead. [`WorldView`] translates once per change: a value `n`
//! (≥ 1) of a cell or object becomes the catalog index plus 1, 0 means
//! "none", [`MISSING`] a type the catalog does not know (or an index that
//! does not fit 16 bits).
//!
//! World section of a frame (after the sections of the moving objects):
//! header of [`SECTION_HEADER`] bytes (`u32` bytes of the section including
//! the header, `u32` blocks in this frame, `u32` changed blocks that did not
//! fit, `u32` reserved 0), then block records:
//!
//! | Offset | Type | Field |
//! |---|---|---|
//! | 0 | u32 | table index of the block (`floor · blocks per floor + by · blocks per row + bx`) |
//! | 4 | u8 | encoding: [`UNIFORM`], [`PALETTE`] or [`RAW`] |
//! | 5 | u8 | palette length (1 for uniform, 2 … 16, 0 for raw) |
//! | 6 | u16 | objects in the block |
//! | 8 | u32 | payload bytes after the record header |
//!
//! Payload: the palette (8 bytes per value) and for [`PALETTE`] 4-bit
//! indices of the 1,024 cells (low nibble first, row order), or for [`RAW`]
//! the 1,024 cell values; padded to 4 bytes; then the objects, [`OBJECT_BYTES`]
//! each (`u16` type, `u8` x and `u8` y in the block, `u8` rotation, `u8`
//! state 0), padded to 4 bytes. A cell value is 8 bytes: flags, zone,
//! marking (0 none, 1 preferred route, 2 avoid), reserve 0, floor covering
//! `u16`, wall type `u16`. An object belongs to the block that holds the
//! cell with the smallest coordinates of its footprint.

use std::collections::HashMap;
use std::sync::Arc;

use crate::build::Build;
use crate::build::grid::{
    BLOCK, BLOCK_CELLS, Block, COST_AVOID, COST_MARKED, GROUND_INDEX, GridSnapshot, floor_index,
};
use crate::datapack::Catalog;
use crate::frame::ViewRect;

/// Most bytes of the world section per frame (ADR-038; re-measured in 4.1).
pub const FRAME_BUDGET: usize = 1 << 20;
/// Bytes of the section header.
pub const SECTION_HEADER: usize = 16;
/// Bytes of a block record header.
pub const RECORD_HEADER: usize = 12;
/// Bytes of an object record.
pub const OBJECT_BYTES: usize = 6;
/// Bytes of a cell value.
pub const CELL_BYTES: usize = 8;
/// Encoding: all cells of the block have the same value.
pub const UNIFORM: u8 = 0;
/// Encoding: palette of 2 … 16 values and 4 bits per cell.
pub const PALETTE: u8 = 1;
/// Encoding: one value per cell.
pub const RAW: u8 = 2;
/// Type value of a type the catalog does not know.
pub const MISSING: u16 = u16::MAX;
/// Blocks around the view rectangle that count as visible (scrolling).
const VIEW_MARGIN_BLOCKS: f32 = 1.0;

/// An object as the block record stores it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectRec {
    /// Catalog index plus 1, or [`MISSING`].
    pub ty: u16,
    /// Cell in the block, east.
    pub x: u8,
    /// Cell in the block, south.
    pub y: u8,
    /// Rotation 0 … 3.
    pub rotation: u8,
}

#[derive(Debug, Default)]
struct BlockObjects {
    records: Vec<ObjectRec>,
    /// Hash of the records; differs when the objects of the block change.
    hash: u64,
}

/// Objects by block of the table.
#[derive(Debug, Default)]
pub struct ObjectIndex {
    blocks: HashMap<usize, BlockObjects>,
}

impl ObjectIndex {
    fn build(build: &Build, type_map: &[u16], blocks_x: usize, per_floor: usize) -> Self {
        let mut blocks: HashMap<usize, BlockObjects> = HashMap::new();
        let side = BLOCK as u16;
        for object in build.objects().as_slice() {
            let Some(floor) = floor_index(object.floor) else {
                continue;
            };
            let index = floor * per_floor
                + usize::from(object.y / side) * blocks_x
                + usize::from(object.x / side);
            let ty = map_kind(type_map, object.ty);
            let entry = blocks.entry(index).or_default();
            entry.records.push(ObjectRec {
                ty,
                x: u8::try_from(object.x % side).unwrap_or(0),
                y: u8::try_from(object.y % side).unwrap_or(0),
                rotation: object.rotation,
            });
        }
        for entry in blocks.values_mut() {
            let mut hash = 0xcbf2_9ce4_8422_2325_u64;
            for r in &entry.records {
                for byte in [
                    r.ty.to_le_bytes()[0],
                    r.ty.to_le_bytes()[1],
                    r.x,
                    r.y,
                    r.rotation,
                ] {
                    hash = (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3);
                }
            }
            entry.hash = hash | 1;
        }
        Self { blocks }
    }

    /// Objects of the block at `index`.
    #[must_use]
    pub fn of(&self, index: usize) -> &[ObjectRec] {
        self.blocks.get(&index).map_or(&[], |b| &b.records)
    }

    fn hash(&self, index: usize) -> u64 {
        self.blocks.get(&index).map_or(0, |b| b.hash)
    }

    /// Number of objects in all blocks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.blocks.values().map(|b| b.records.len()).sum()
    }

    /// `true` if there is no object.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

fn map_kind(type_map: &[u16], n: u16) -> u16 {
    match n.checked_sub(1) {
        None => 0,
        Some(i) => type_map.get(usize::from(i)).copied().unwrap_or(MISSING),
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ViewKey {
    session: u32,
    grid: u32,
    /// Next handle and object count of the module: a build raises the
    /// handle, a demolition lowers the count, neither goes back.
    objects: (u32, usize),
    types: usize,
    content: usize,
}

/// The resting world of one snapshot (S-1): the grid and the display data
/// derived from the objects and the catalog. Taken again only when
/// something changed, so a capture per tick costs a comparison.
#[derive(Clone, Default)]
pub struct WorldView {
    key: Option<ViewKey>,
    grid: Option<GridSnapshot>,
    objects: Arc<ObjectIndex>,
    type_map: Arc<Vec<u16>>,
}

impl std::fmt::Debug for WorldView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WorldView")
    }
}

// A view carries derived data only; two snapshots are equal by their persons.
impl PartialEq for WorldView {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for WorldView {}

impl WorldView {
    /// Brings the view up to date with `build` and the `content` of the
    /// session; does nothing if nothing changed.
    pub fn refresh(&mut self, session: u32, build: &Build, content: &Arc<Catalog>) {
        let grid = build.grid();
        let key = ViewKey {
            session,
            grid: grid.version(),
            objects: (build.next_handle(), build.objects().len()),
            types: build.types().ids().len(),
            content: Arc::as_ptr(content).addr(),
        };
        let old = self.key;
        if old == Some(key) {
            return;
        }
        let same_session = old.is_some_and(|o| o.session == session && o.content == key.content);
        if !same_session || old.is_none_or(|o| o.types != key.types) {
            self.type_map = Arc::new(
                build
                    .types()
                    .ids()
                    .iter()
                    .map(|id| {
                        content
                            .index_of(id)
                            .and_then(|i| u16::try_from(i + 1).ok())
                            .filter(|n| *n != MISSING)
                            .unwrap_or(MISSING)
                    })
                    .collect(),
            );
        }
        if self.grid.is_none() || old.is_none_or(|o| o.grid != key.grid || o.session != session) {
            self.grid = Some(grid.snapshot());
        }
        let rebuild_objects =
            !same_session || old.is_none_or(|o| o.objects != key.objects || o.types != key.types);
        if rebuild_objects {
            let (blocks_x, blocks_y) = grid.snapshot().block_counts();
            self.objects = Arc::new(ObjectIndex::build(
                build,
                &self.type_map,
                blocks_x,
                blocks_x * blocks_y,
            ));
        }
        self.key = Some(key);
    }

    /// The grid of the view; `None` before the first refresh.
    #[must_use]
    pub fn grid(&self) -> Option<&GridSnapshot> {
        self.grid.as_ref()
    }

    /// Objects by block.
    #[must_use]
    pub fn objects(&self) -> &ObjectIndex {
        &self.objects
    }

    fn kind(&self, n: u16) -> u16 {
        map_kind(&self.type_map, n)
    }

    /// Appends the record of the block at `index` to `out`.
    pub fn encode_block(&self, index: usize, block: &Block, out: &mut Vec<u8>) {
        let start = out.len();
        out.resize(start + RECORD_HEADER, 0);
        let mut cells = [[0u8; CELL_BYTES]; BLOCK_CELLS];
        let mut palette: Vec<[u8; CELL_BYTES]> = Vec::with_capacity(16);
        let mut fits = true;
        for (i, cell) in cells.iter_mut().enumerate() {
            let cost = block.costs()[i];
            let marking = if cost == COST_MARKED {
                1
            } else {
                u8::from(cost == COST_AVOID) * 2
            };
            let floor = self.kind(block.floor_kinds()[i]).to_le_bytes();
            let wall = self.kind(block.wall_kinds()[i]).to_le_bytes();
            *cell = [
                block.flags()[i],
                block.zones()[i],
                marking,
                0,
                floor[0],
                floor[1],
                wall[0],
                wall[1],
            ];
            if fits && !palette.contains(cell) {
                if palette.len() == 16 {
                    fits = false;
                } else {
                    palette.push(*cell);
                }
            }
        }
        let (encoding, palette_len) = if !fits {
            for cell in &cells {
                out.extend_from_slice(cell);
            }
            (RAW, 0)
        } else {
            for value in &palette {
                out.extend_from_slice(value);
            }
            if palette.len() > 1 {
                for pair in cells.as_chunks::<2>().0 {
                    let lo = palette.iter().position(|p| *p == pair[0]).unwrap_or(0);
                    let hi = palette.iter().position(|p| *p == pair[1]).unwrap_or(0);
                    out.push(u8::try_from(lo | hi << 4).unwrap_or(0));
                }
                (PALETTE, palette.len())
            } else {
                (UNIFORM, 1)
            }
        };
        pad4(out, start);
        let objects = self.objects.of(index);
        for o in objects {
            out.extend_from_slice(&o.ty.to_le_bytes());
            out.extend_from_slice(&[o.x, o.y, o.rotation, 0]);
        }
        pad4(out, start);
        let payload = u32::try_from(out.len() - start - RECORD_HEADER).unwrap_or(0);
        let header = &mut out[start..start + RECORD_HEADER];
        header[0..4].copy_from_slice(&u32::try_from(index).unwrap_or(0).to_le_bytes());
        header[4] = encoding;
        header[5] = u8::try_from(palette_len).unwrap_or(0);
        header[6..8].copy_from_slice(
            &u16::try_from(objects.len())
                .unwrap_or(u16::MAX)
                .to_le_bytes(),
        );
        header[8..12].copy_from_slice(&payload.to_le_bytes());
    }
}

fn pad4(out: &mut Vec<u8>, start: usize) {
    while !(out.len() - start).is_multiple_of(4) {
        out.push(0);
    }
}

/// What the renderer holds of one block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Held {
    stamp: u32,
    objects: u64,
}

/// Blocks handed out on one port. A block counts as handed out only when
/// the renderer acknowledged the frame that carried it (`worldAck`, S-1);
/// the sender holds stamps, not the blocks, so the grid is not forced to
/// copy a block when it changes.
#[derive(Debug, Default)]
pub struct WorldSender {
    session: u32,
    table: usize,
    held: Vec<Option<Held>>,
    /// Revision of the last frame that carried blocks.
    revision: u32,
    /// Blocks of that frame, until it is acknowledged.
    pending: Vec<(usize, Held)>,
}

impl WorldSender {
    /// The renderer processed the frame with world revision `world_ack`.
    pub fn acknowledge(&mut self, world_ack: u32) {
        if world_ack != 0 && world_ack == self.revision {
            for (index, held) in self.pending.drain(..) {
                if let Some(slot) = self.held.get_mut(index) {
                    *slot = Some(held);
                }
            }
        }
    }

    /// Number of blocks sent and not yet acknowledged.
    #[must_use]
    pub fn unacknowledged(&self) -> usize {
        self.pending.len()
    }

    /// Appends the world section for `view` to `out` and returns the world
    /// revision of the frame: 0 if it carries no block, else a number that
    /// rises with every frame that does. Changed blocks of the shown floor
    /// inside the view rectangle come first; the section ends when it
    /// reaches [`FRAME_BUDGET`], what is left follows in the next frames.
    pub fn write(
        &mut self,
        view: &WorldView,
        session: u32,
        floor: Option<i8>,
        rect: Option<ViewRect>,
        out: &mut Vec<u8>,
    ) -> u32 {
        let start = out.len();
        out.resize(start + SECTION_HEADER, 0);
        let Some(grid) = view.grid() else {
            finish(out, start, 0, 0);
            return 0;
        };
        let entries = grid.entries();
        if self.session != session || self.table != entries.len() {
            self.session = session;
            self.table = entries.len();
            self.held = vec![None; entries.len()];
            self.revision = 0;
            self.pending.clear();
        }
        let current = |index: usize, block: &Block| Held {
            stamp: block.stamp(),
            objects: view.objects.hash(index),
        };
        let mut changed: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(index, entry)| {
                entry
                    .as_deref()
                    .is_some_and(|b| self.held[*index] != Some(current(*index, b)))
            })
            .map(|(index, _)| index)
            .collect();
        let (blocks_x, blocks_y) = grid.block_counts();
        let per_floor = (blocks_x * blocks_y).max(1);
        let shown = floor.and_then(floor_index).unwrap_or(GROUND_INDEX);
        let visible = |index: usize| -> bool {
            if index / per_floor != shown {
                return false;
            }
            let Some(v) = rect else { return true };
            let in_floor = index % per_floor;
            let margin = VIEW_MARGIN_BLOCKS * BLOCK as f32;
            let x0 = (in_floor % blocks_x * BLOCK) as f32;
            let y0 = (in_floor / blocks_x * BLOCK) as f32;
            x0 + BLOCK as f32 + margin >= v.x0
                && x0 - margin <= v.x1
                && y0 + BLOCK as f32 + margin >= v.y0
                && y0 - margin <= v.y1
        };
        changed.sort_by_key(|&index| (!visible(index), index));
        self.pending.clear();
        let mut blocks = 0u32;
        for &index in &changed {
            if out.len() - start >= FRAME_BUDGET {
                break;
            }
            if let Some(block) = entries[index].as_deref() {
                view.encode_block(index, block, out);
                self.pending.push((index, current(index, block)));
                blocks += 1;
            }
        }
        if blocks > 0 {
            self.revision = self.revision.wrapping_add(1).max(1);
        }
        let left = u32::try_from(changed.len()).unwrap_or(u32::MAX) - blocks;
        finish(out, start, blocks, left);
        if blocks > 0 { self.revision } else { 0 }
    }
}

fn finish(out: &mut [u8], start: usize, blocks: u32, left: u32) {
    let bytes = u32::try_from(out.len() - start).unwrap_or(u32::MAX);
    let header = &mut out[start..start + SECTION_HEADER];
    header[0..4].copy_from_slice(&bytes.to_le_bytes());
    header[4..8].copy_from_slice(&blocks.to_le_bytes());
    header[8..12].copy_from_slice(&left.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::grid::{AreaKind, Cell, WALL, Zone};
    use crate::geometry::{CellPos, CellRect, MapSize, Rotation};
    use crate::world::World;

    const BENCH: &str = "base:bench";
    const FLOOR: &str = "base:floor_concrete";
    const WALL_TYPE: &str = "base:wall_concrete";

    fn catalog() -> Arc<Catalog> {
        let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../content/base");
        let folders = crate::datapack::discover(Some(&base), None);
        Arc::new(crate::datapack::load(&folders, &[], &crate::datapack::game_version()).catalog)
    }

    fn rect(x0: u16, y0: u16, x1: u16, y1: u16) -> CellRect {
        CellRect {
            floor: 0,
            x0,
            y0,
            x1,
            y1,
        }
    }

    /// Decodes a section into (table index, cells, objects) per block.
    fn decode(section: &[u8]) -> Vec<(usize, Vec<[u8; CELL_BYTES]>, Vec<ObjectRec>)> {
        let u32_at = |at: usize| u32::from_le_bytes(section[at..at + 4].try_into().unwrap());
        assert_eq!(u32_at(0) as usize, section.len());
        let mut at = SECTION_HEADER;
        let mut blocks = Vec::new();
        for _ in 0..u32_at(4) {
            let index = u32_at(at) as usize;
            let encoding = section[at + 4];
            let palette = usize::from(section[at + 5]);
            let objects = usize::from(u16::from_le_bytes([section[at + 6], section[at + 7]]));
            let payload = u32_at(at + 8) as usize;
            let body = at + RECORD_HEADER;
            let value = |k: usize| -> [u8; CELL_BYTES] {
                section[body + k * CELL_BYTES..body + (k + 1) * CELL_BYTES]
                    .try_into()
                    .unwrap()
            };
            let (cells, mut end): (Vec<_>, usize) = match encoding {
                UNIFORM => (vec![value(0); BLOCK_CELLS], body + CELL_BYTES),
                PALETTE => {
                    let indices = body + palette * CELL_BYTES;
                    let cells = (0..BLOCK_CELLS)
                        .map(|c| {
                            value(usize::from(section[indices + (c >> 1)] >> ((c & 1) * 4)) & 15)
                        })
                        .collect();
                    (cells, indices + BLOCK_CELLS / 2)
                }
                RAW => (
                    (0..BLOCK_CELLS).map(value).collect(),
                    body + BLOCK_CELLS * CELL_BYTES,
                ),
                other => panic!("encoding {other}"),
            };
            end = end.next_multiple_of(4);
            let objs: Vec<ObjectRec> = (0..objects)
                .map(|k| {
                    let o = end + k * OBJECT_BYTES;
                    ObjectRec {
                        ty: u16::from_le_bytes([section[o], section[o + 1]]),
                        x: section[o + 2],
                        y: section[o + 3],
                        rotation: section[o + 4],
                    }
                })
                .collect();
            assert_eq!(
                body + payload,
                (end + objects * OBJECT_BYTES).next_multiple_of(4)
            );
            blocks.push((index, cells, objs));
            at = body + payload;
        }
        assert_eq!(at, section.len());
        blocks
    }

    fn world_view(world: &World, catalog: &Arc<Catalog>) -> WorldView {
        let mut view = WorldView::default();
        view.refresh(1, world.build(), catalog);
        view
    }

    fn bench(
        world: &mut World,
        x: u16,
        y: u16,
        blocks: bool,
    ) -> Result<u32, crate::build::objects::BuildRefusal> {
        world.build_object(
            BENCH,
            (4, 2),
            blocks,
            CellPos { x, y, floor: 0 },
            Rotation(1),
        )
    }

    fn built() -> (World, Arc<Catalog>) {
        let catalog = catalog();
        let mut world = World::new(MapSize::new(128, 96).unwrap());
        world
            .build_floor(FLOOR, AreaKind::PublicWalk, 10, rect(2, 2, 20, 10))
            .unwrap();
        (world, catalog)
    }

    #[test]
    fn a_floor_shows_the_catalog_index_plus_one() {
        let (world, catalog) = built();
        let view = world_view(&world, &catalog);
        let mut out = Vec::new();
        let mut sender = WorldSender::default();
        let revision = sender.write(&view, 1, Some(0), None, &mut out);
        assert_eq!(revision, 1);
        let blocks = decode(&out);
        // 4 × 3 blocks on the ground floor, every one is sent the first time.
        assert_eq!(blocks.len(), 12);
        let (_, cells, objects) = &blocks[0];
        assert!(objects.is_empty());
        let floor = catalog.index_of(FLOOR).unwrap() + 1;
        let cell = cells[2 * BLOCK + 2];
        assert_eq!(
            u16::from_le_bytes([cell[4], cell[5]]),
            u16::try_from(floor).unwrap()
        );
        assert_eq!(cell[0] & WALL, 0, "walkable");
        // The void around keeps no covering and stays a wall for walking.
        let void = cells[0];
        assert_eq!((void[0] & WALL, void[4], void[5]), (WALL, 0, 0));
    }

    #[test]
    fn walls_and_objects_are_translated_and_belong_to_their_anchor_block() {
        let (mut world, catalog) = built();
        world.build_wall(WALL_TYPE, rect(5, 5, 6, 8)).unwrap();
        bench(&mut world, 10, 3, true).unwrap();
        let view = world_view(&world, &catalog);
        let mut out = Vec::new();
        WorldSender::default().write(&view, 1, Some(0), None, &mut out);
        let blocks = decode(&out);
        let (_, cells, objects) = blocks.iter().find(|b| b.0 == GROUND_INDEX * 12).unwrap();
        let wall = catalog.index_of(WALL_TYPE).unwrap() + 1;
        let c = cells[5 * BLOCK + 5];
        assert_eq!(
            u16::from_le_bytes([c[6], c[7]]),
            u16::try_from(wall).unwrap()
        );
        let bench = u16::try_from(catalog.index_of(BENCH).unwrap() + 1).unwrap();
        assert_eq!(
            objects,
            &[ObjectRec {
                ty: bench,
                x: 10,
                y: 3,
                rotation: 1
            }]
        );
        // The blocking bench set its cells to blocked, its floor stays.
        let under = cells[3 * BLOCK + 10];
        assert_eq!(under[0] & WALL, WALL);
        assert_ne!(u16::from_le_bytes([under[4], under[5]]), 0);
    }

    #[test]
    fn unknown_types_show_missing() {
        let (world, _) = built();
        let view = world_view(&world, &Arc::new(Catalog::empty()));
        let mut out = Vec::new();
        WorldSender::default().write(&view, 1, Some(0), None, &mut out);
        let (_, cells, _) = &decode(&out)[0];
        let cell = cells[2 * BLOCK + 2];
        assert_eq!(u16::from_le_bytes([cell[4], cell[5]]), MISSING);
    }

    #[test]
    fn only_changed_blocks_follow_and_only_after_the_acknowledgement() {
        let (mut world, catalog) = built();
        let mut sender = WorldSender::default();
        let mut out = Vec::new();
        let first = sender.write(&world_view(&world, &catalog), 1, Some(0), None, &mut out);
        assert_eq!(first, 1);
        assert_eq!(sender.unacknowledged(), 12);
        // Not acknowledged: the same blocks are offered again.
        out.clear();
        let again = sender.write(&world_view(&world, &catalog), 1, Some(0), None, &mut out);
        assert_eq!((again, decode(&out).len()), (2, 12));
        // Acknowledging an old revision changes nothing.
        sender.acknowledge(first);
        assert_eq!(sender.unacknowledged(), 12);
        sender.acknowledge(again);
        assert_eq!(sender.unacknowledged(), 0);
        out.clear();
        assert_eq!(
            sender.write(&world_view(&world, &catalog), 1, Some(0), None, &mut out),
            0
        );
        assert!(decode(&out).is_empty());
        // One build touches one block.
        world.build_wall(WALL_TYPE, rect(40, 40, 42, 42)).unwrap();
        out.clear();
        let third = sender.write(&world_view(&world, &catalog), 1, Some(0), None, &mut out);
        assert_eq!(third, 3);
        let sent: Vec<usize> = decode(&out).iter().map(|b| b.0).collect();
        assert_eq!(sent, vec![GROUND_INDEX * 12 + 4 + 1]);
        sender.acknowledge(third);
        // A non-blocking object changes no cell, only the object list of its block.
        bench(&mut world, 14, 3, false).unwrap();
        out.clear();
        let fourth = sender.write(&world_view(&world, &catalog), 1, Some(0), None, &mut out);
        assert_eq!(fourth, 4);
        let blocks = decode(&out);
        assert_eq!(blocks.len(), 1);
        assert_eq!((blocks[0].0, blocks[0].2.len()), (GROUND_INDEX * 12, 1));
    }

    #[test]
    fn a_session_change_starts_over() {
        let (world, catalog) = built();
        let view = world_view(&world, &catalog);
        let mut sender = WorldSender::default();
        let mut out = Vec::new();
        let r = sender.write(&view, 1, Some(0), None, &mut out);
        sender.acknowledge(r);
        out.clear();
        assert_eq!(sender.write(&view, 1, Some(0), None, &mut out), 0);
        out.clear();
        let r = sender.write(&view, 2, Some(0), None, &mut out);
        assert_eq!((r, decode(&out).len()), (1, 12));
    }

    #[test]
    fn visible_blocks_come_first_and_the_budget_cuts_the_rest() {
        let catalog = catalog();
        let mut world = World::new(MapSize::new(2048, 2048).unwrap());
        // Zone, marking and area kind in a pattern of dozens of values cannot use a palette.
        for y in 0..512u16 {
            for x in 0..512u16 {
                world
                    .build_mut()
                    .grid_mut()
                    .set(
                        CellPos { x, y, floor: 0 },
                        Cell {
                            zone: u8::try_from((x + y) % 4).unwrap(),
                            cost: [10, 7, 20][usize::from((x + y * 3) % 3)],
                            ..Cell::floor(Zone::Landside)
                                .with_kind(AreaKind::ALL[usize::from((x * 5 + y) % 8)])
                        },
                    )
                    .unwrap();
            }
        }
        let view = {
            let mut v = WorldView::default();
            v.refresh(1, world.build(), &catalog);
            v
        };
        let mut sender = WorldSender::default();
        let mut out = Vec::new();
        let rect = ViewRect {
            x0: 490.0,
            y0: 490.0,
            x1: 510.0,
            y1: 510.0,
        };
        sender.write(&view, 1, Some(0), Some(rect), &mut out);
        let blocks = decode(&out);
        let per_floor = 64 * 64;
        // The raw blocks use 8 KB; the budget stops the section after about 1 MB.
        assert!(out.len() < FRAME_BUDGET + RECORD_HEADER + BLOCK_CELLS * CELL_BYTES);
        assert!(blocks.len() < per_floor);
        // The nine blocks in and around the view (a block of margin) come first.
        for (index, _, _) in &blocks[..9] {
            let (bx, by) = (index % per_floor % 64, index % per_floor / 64);
            assert!(
                (14..=16).contains(&bx) && (14..=16).contains(&by),
                "{bx}, {by}"
            );
        }
        let left = u32::from_le_bytes(out[8..12].try_into().unwrap()) as usize;
        assert_eq!(left, per_floor - blocks.len());
    }

    #[test]
    fn the_view_is_taken_again_only_for_a_change() {
        let (mut world, catalog) = built();
        let mut view = WorldView::default();
        view.refresh(1, world.build(), &catalog);
        let grid_before = view.grid().unwrap().block(GROUND_INDEX * 12).is_some();
        assert!(grid_before);
        let objects = Arc::as_ptr(&view.objects);
        view.refresh(1, world.build(), &catalog);
        assert_eq!(Arc::as_ptr(&view.objects), objects);
        // The first wall interns its type: the objects are translated again.
        world.build_wall(WALL_TYPE, rect(5, 5, 6, 6)).unwrap();
        view.refresh(1, world.build(), &catalog);
        let objects = Arc::as_ptr(&view.objects);
        // A second wall of the same type changes the grid, not the objects.
        world.build_wall(WALL_TYPE, rect(8, 5, 9, 6)).unwrap();
        view.refresh(1, world.build(), &catalog);
        assert_eq!(Arc::as_ptr(&view.objects), objects);
        assert!(view.objects().is_empty());
    }
}
