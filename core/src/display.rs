//! Spike 3.8 (throw-away code on branch `erkundung/3-8-messaufbau`, never
//! merged; `docs/research/3.8-weltdarstellung.md`): display data of the
//! resting world and its two ways to the renderer.
//!
//! - W-A (`WorldMode::Inline`): every frame carries the blocks that changed
//!   since the blocks handed out on this port, visible ones first, up to
//!   [`FRAME_BUDGET`] bytes.
//! - W-B (`WorldMode::Announce`): the frame only lists changed block
//!   numbers; the renderer fetches them with the query `blocks` (S-7).
//!
//! The core has no floor coverings, wall kinds or objects yet (3.1), so the
//! display data are derived synthetically from flags, zone and cost of the
//! cells. Cell: 8 bytes `[flags, zone, cost, 0, covering u16, wall u16]`.
//!
//! Section in the frame (after the persons, header byte 26 = 1):
//! `u32 bytes, u32 blocks, u32 pending, u32 mode`, then block records
//! (mode 1) or `u32` table indices (mode 2). Block record: `u32 table
//! index, u8 encoding (0 uniform, 1 palette 4 bit, 3 raw), u8 palette
//! length, u16 objects, u32 payload bytes`, then the payload padded to 4
//! bytes: palette (8 bytes per value) and indices, or raw cells; then the
//! objects, 6 bytes each: `u16 graphic, u8 x, u8 y, u8 rotation, u8 state`.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::frame::ViewRect;
use crate::grid::{BLOCK, BLOCK_CELLS, Block, COST_NORMAL, GROUND_INDEX, GridSnapshot, WALL};

/// Largest world section per frame in W-A, bytes (proposal of the template).
pub const FRAME_BUDGET: usize = 1 << 20;
/// Most block numbers per frame in W-B.
pub const MAX_ANNOUNCED: usize = 8192;
/// Most blocks per query `blocks`.
pub const MAX_QUERY_BLOCKS: usize = 256;
/// Bytes of the section header.
pub const SECTION_HEADER: usize = 16;
/// Bytes of a block record header.
pub const RECORD_HEADER: usize = 12;
/// Objects placed per ground floor block of the terminal (20,000 in total).
pub const OBJECTS_PER_BLOCK: usize = 35;
/// Generated graphics of the spike atlas.
pub const GRAPHICS: u32 = 512;
/// Covering of the open terrain (4 variants from here).
pub const TERRAIN: u16 = 200;
/// Covering of a cell without floor on an upper floor (transparent).
pub const EMPTY: u16 = 0xFFFF;

/// Way of the resting world to the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum WorldMode {
    /// W-A: blocks in the frame.
    Inline,
    /// W-B: block numbers in the frame, data over S-7.
    Announce,
}

fn mix(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb_352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846c_a68b);
    h ^ (h >> 16)
}

/// Display value of cell `i` of `block` at table index `index`.
fn cell_value(block: &Block, index: usize, i: usize, ground: bool) -> u64 {
    let flags = block.flags()[i];
    let zone = block.zones()[i];
    let cost = block.costs()[i];
    let (x, y) = (i % BLOCK, i / BLOCK);
    let (covering, wall) = if flags & WALL == 0 {
        let variant = u16::try_from((x / 8 + y / 8) % 3).unwrap_or(0);
        (u16::from(zone) * 8 + variant, 0)
    } else {
        let walkable = |cx: usize, cy: usize| block.flags()[cy * BLOCK + cx] & WALL == 0;
        let wall_cell = (x > 0 && walkable(x - 1, y))
            || (x + 1 < BLOCK && walkable(x + 1, y))
            || (y > 0 && walkable(x, y - 1))
            || (y + 1 < BLOCK && walkable(x, y + 1));
        if wall_cell {
            let kind = u16::try_from(mix(u32::try_from(index).unwrap_or(0)) % 16).unwrap_or(0);
            (u16::from(zone) * 8, 128 + kind)
        } else if ground && zone == 0 && cost == COST_NORMAL {
            let h = mix(u32::try_from(index * BLOCK_CELLS + i).unwrap_or(0));
            (TERRAIN + u16::try_from(h % 4).unwrap_or(0), 0)
        } else {
            (EMPTY, 0)
        }
    };
    u64::from(flags)
        | u64::from(zone) << 8
        | u64::from(cost) << 16
        | u64::from(covering) << 32
        | u64::from(wall) << 48
}

/// Appends the record of block `index` to `out`.
pub fn encode_block(block: &Block, index: usize, ground: bool, out: &mut Vec<u8>) {
    let start = out.len();
    out.resize(start + RECORD_HEADER, 0);
    let mut cells = [0u64; BLOCK_CELLS];
    let mut palette: Vec<u64> = Vec::with_capacity(16);
    let mut fits = true;
    for (i, cell) in cells.iter_mut().enumerate() {
        *cell = cell_value(block, index, i, ground);
        if fits && !palette.contains(cell) {
            if palette.len() == 16 {
                fits = false;
            } else {
                palette.push(*cell);
            }
        }
    }
    let encoding: u8 = if !fits {
        for c in &cells {
            out.extend_from_slice(&c.to_le_bytes());
        }
        3
    } else {
        for p in &palette {
            out.extend_from_slice(&p.to_le_bytes());
        }
        if palette.len() > 1 {
            for pair in cells.as_chunks::<2>().0 {
                let lo = palette.iter().position(|p| *p == pair[0]).unwrap_or(0);
                let hi = palette.iter().position(|p| *p == pair[1]).unwrap_or(0);
                out.push(u8::try_from(lo | hi << 4).unwrap_or(0));
            }
            1
        } else {
            0
        }
    };
    while !(out.len() - start).is_multiple_of(4) {
        out.push(0);
    }
    let mut objects = 0u16;
    let walkable = block.walkable_cells();
    if ground && walkable >= 100 {
        let seed = mix(u32::try_from(index).unwrap_or(0) ^ 0x9e37_79b9);
        let mut placed = 0;
        for k in 0..BLOCK_CELLS as u32 {
            if placed == OBJECTS_PER_BLOCK {
                break;
            }
            let h = mix(seed.wrapping_add(k));
            let i = (h as usize) % BLOCK_CELLS;
            if block.flags()[i] & WALL != 0 {
                continue;
            }
            let graphic = u16::try_from(h % GRAPHICS).unwrap_or(0);
            out.extend_from_slice(&graphic.to_le_bytes());
            out.push(u8::try_from(i % BLOCK).unwrap_or(0));
            out.push(u8::try_from(i / BLOCK).unwrap_or(0));
            out.push(u8::try_from((h >> 9) % 4).unwrap_or(0));
            out.push(0);
            placed += 1;
            objects += 1;
        }
        while !(out.len() - start).is_multiple_of(4) {
            out.push(0);
        }
    }
    let payload = u32::try_from(out.len() - start - RECORD_HEADER).unwrap_or(0);
    let header = &mut out[start..start + RECORD_HEADER];
    header[0..4].copy_from_slice(&u32::try_from(index).unwrap_or(0).to_le_bytes());
    header[4] = encoding;
    header[5] = u8::try_from(palette.len().min(255)).unwrap_or(0);
    header[6..8].copy_from_slice(&objects.to_le_bytes());
    header[8..12].copy_from_slice(&payload.to_le_bytes());
}

/// Answers the query `blocks` from `grid`: `u32` count, then records.
#[must_use]
pub fn answer_blocks(grid: &GridSnapshot, blocks: &[u32]) -> Vec<u8> {
    let per_floor = per_floor(grid);
    let mut out = vec![0u8; 4];
    let mut count = 0u32;
    for &index in blocks {
        let index = index as usize;
        if let Some(block) = grid.block(index) {
            encode_block(block, index, index / per_floor == GROUND_INDEX, &mut out);
            count += 1;
        }
    }
    out[0..4].copy_from_slice(&count.to_le_bytes());
    out
}

fn per_floor(grid: &GridSnapshot) -> usize {
    let (bx, by) = grid.block_counts();
    (bx * by).max(1)
}

fn same(a: Option<&Arc<Block>>, b: Option<&Arc<Block>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// Blocks handed out on one port. Holding them keeps their identity: a
/// change of the grid then always copies the block (`Arc::make_mut`).
#[derive(Default)]
pub struct WorldSender {
    session: u32,
    sent: Vec<Option<Arc<Block>>>,
    started: bool,
}

impl WorldSender {
    /// Appends the world section for `grid` to `out`; returns `true` if a
    /// section was written.
    pub fn write(
        &mut self,
        session: u32,
        grid: &GridSnapshot,
        mode: WorldMode,
        floor: Option<i8>,
        view: Option<ViewRect>,
        out: &mut Vec<u8>,
    ) -> bool {
        let entries = grid.entries();
        if !self.started || self.session != session || self.sent.len() != entries.len() {
            self.started = true;
            self.session = session;
            // Nothing sent yet: every stored block counts as changed.
            self.sent = vec![None; entries.len()];
            for (slot, entry) in self.sent.iter_mut().zip(entries) {
                if entry.is_none() {
                    *slot = None;
                }
            }
        }
        let (bx, by) = grid.block_counts();
        let per_floor = per_floor(grid);
        let visible = |index: usize| -> bool {
            let f = index / per_floor;
            let shown = floor.map_or(GROUND_INDEX, |fl| {
                usize::try_from(i32::from(fl) + 2).unwrap_or(0)
            });
            if f != shown {
                return false;
            }
            let Some(v) = view else { return true };
            let (x, y) = ((index % per_floor) % bx, (index % per_floor) / bx);
            let (x0, y0) = (x as f32 * BLOCK as f32, y as f32 * BLOCK as f32);
            x0 + BLOCK as f32 >= v.x0 && x0 <= v.x1 && y0 + BLOCK as f32 >= v.y0 && y0 <= v.y1
        };
        let mut changed: Vec<usize> = (0..entries.len())
            .filter(|&i| !same(entries[i].as_ref(), self.sent[i].as_ref()))
            .collect();
        let _ = by;
        changed.sort_by_key(|&i| !visible(i));
        let start = out.len();
        out.resize(start + SECTION_HEADER, 0);
        let mut blocks = 0u32;
        for &index in &changed {
            match mode {
                WorldMode::Inline => {
                    if out.len() - start >= FRAME_BUDGET {
                        break;
                    }
                    if let Some(block) = entries[index].as_deref() {
                        encode_block(block, index, index / per_floor == GROUND_INDEX, out);
                    } else {
                        // Block removed: record with no payload and encoding 255.
                        let mut header = [0u8; RECORD_HEADER];
                        header[0..4]
                            .copy_from_slice(&u32::try_from(index).unwrap_or(0).to_le_bytes());
                        header[4] = 255;
                        out.extend_from_slice(&header);
                    }
                }
                WorldMode::Announce => {
                    if blocks as usize == MAX_ANNOUNCED {
                        break;
                    }
                    out.extend_from_slice(&u32::try_from(index).unwrap_or(0).to_le_bytes());
                }
            }
            self.sent[index].clone_from(&entries[index]);
            blocks += 1;
        }
        let pending = u32::try_from(changed.len()).unwrap_or(u32::MAX) - blocks;
        let bytes = u32::try_from(out.len() - start).unwrap_or(u32::MAX);
        let header = &mut out[start..start + SECTION_HEADER];
        header[0..4].copy_from_slice(&bytes.to_le_bytes());
        header[4..8].copy_from_slice(&blocks.to_le_bytes());
        header[8..12].copy_from_slice(&pending.to_le_bytes());
        let code: u32 = match mode {
            WorldMode::Inline => 1,
            WorldMode::Announce => 2,
        };
        header[12..16].copy_from_slice(&code.to_le_bytes());
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{CellRect, MapSize};
    use crate::grid::{Cell, Grid, Zone};

    fn grid() -> Grid {
        let map = MapSize::new(256, 256).unwrap();
        let mut grid = Grid::new(map);
        let area = CellRect {
            floor: 0,
            x0: 10,
            y0: 10,
            x1: 90,
            y1: 90,
        };
        grid.fill(area, Cell::floor(Zone::Airside)).unwrap();
        grid
    }

    #[test]
    fn inline_sends_each_block_once_and_changes_again() {
        let mut g = grid();
        let mut sender = WorldSender::default();
        let mut out = Vec::new();
        sender.write(1, &g.snapshot(), WorldMode::Inline, Some(0), None, &mut out);
        let blocks = u32::from_le_bytes(out[4..8].try_into().unwrap());
        assert_eq!(blocks, 64);
        out.clear();
        sender.write(1, &g.snapshot(), WorldMode::Inline, Some(0), None, &mut out);
        assert_eq!(u32::from_le_bytes(out[4..8].try_into().unwrap()), 0);
        let wall = CellRect {
            floor: 0,
            x0: 40,
            y0: 40,
            x1: 42,
            y1: 42,
        };
        g.fill(wall, Cell::VOID).unwrap();
        out.clear();
        sender.write(1, &g.snapshot(), WorldMode::Inline, Some(0), None, &mut out);
        assert_eq!(u32::from_le_bytes(out[4..8].try_into().unwrap()), 1);
    }

    #[test]
    fn uniform_and_palette_blocks_are_small() {
        let g = grid();
        let snap = g.snapshot();
        let mut out = Vec::new();
        encode_block(
            snap.block(GROUND_INDEX * 64 + 63).unwrap(),
            GROUND_INDEX * 64 + 63,
            true,
            &mut out,
        );
        assert!(
            out.len() <= RECORD_HEADER + 4 * 8 + 512,
            "terrain block {}",
            out.len()
        );
        let answer = answer_blocks(&snap, &[u32::try_from(GROUND_INDEX * 64).unwrap(), 9_999]);
        assert_eq!(u32::from_le_bytes(answer[0..4].try_into().unwrap()), 1);
    }

    /// Container measurement of the first transfer (spike 3.8):
    /// `cargo test --release -p openairside-core --lib display::tests::first_transfer -- --ignored --nocapture`.
    #[test]
    #[ignore = "measurement"]
    fn first_transfer_of_the_reference_terminal() {
        use crate::command::Scenario;
        let map = MapSize::new(2048, 2048).unwrap();
        let world =
            crate::scenario::scenario_world(Scenario::ReferenceTerminal, map, 0, 1).unwrap();
        let snap = world.grid().snapshot();
        let stored = snap.entries().iter().filter(|e| e.is_some()).count();
        for mode in [WorldMode::Inline, WorldMode::Announce] {
            let mut sender = WorldSender::default();
            let (mut frames, mut bytes, mut slowest) = (0, 0usize, 0f64);
            let t = std::time::Instant::now();
            loop {
                let mut out = Vec::new();
                let f = std::time::Instant::now();
                sender.write(1, &snap, mode, Some(0), None, &mut out);
                slowest = slowest.max(f.elapsed().as_secs_f64() * 1e3);
                frames += 1;
                bytes += out.len();
                if u32::from_le_bytes(out[8..12].try_into().unwrap()) == 0 {
                    break;
                }
            }
            println!(
                "{mode:?}: {stored} blocks, {frames} frames, {:.2} MB, total {:.1} ms, slowest frame {slowest:.1} ms",
                bytes as f64 / 1_048_576.0,
                t.elapsed().as_secs_f64() * 1e3
            );
        }
        let all: Vec<u32> = (0..snap.table_len() as u32)
            .filter(|&i| snap.block(i as usize).is_some())
            .collect();
        let t = std::time::Instant::now();
        let mut bytes = 0;
        for chunk in all.chunks(MAX_QUERY_BLOCKS) {
            bytes += answer_blocks(&snap, chunk).len();
        }
        println!(
            "queries: {} answers, {:.2} MB, {:.1} ms",
            all.len().div_ceil(MAX_QUERY_BLOCKS),
            bytes as f64 / 1_048_576.0,
            t.elapsed().as_secs_f64() * 1e3
        );
    }
}
