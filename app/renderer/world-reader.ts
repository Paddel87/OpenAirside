/**
 * Reader of the world section of a frame (S-1 layout 2, core/src/display.rs):
 * block records with floor covering, wall type, flags and objects. Every
 * length is checked against the section before a view is created (ADR-021).
 */
import type { WorldSection } from "./frame-reader.js";

/** Edge of a block in cells. */
export const BLOCK = 32;
/** Cells per block. */
export const BLOCK_CELLS = BLOCK * BLOCK;
/** Bytes of a block record header. */
export const RECORD_HEADER = 12;
/** Bytes of a cell value. */
export const CELL_BYTES = 8;
/** Bytes of an object record. */
export const OBJECT_BYTES = 6;
/** Encoding: all cells have the same value. */
export const UNIFORM = 0;
/** Encoding: palette of 2 … 16 values, 4 bits per cell. */
export const PALETTE = 1;
/** Encoding: one value per cell. */
export const RAW = 2;
/** Type value of a type the catalog does not know. */
export const MISSING = 0xffff;
/** Flag of a cell that cannot be walked on (core `grid::WALL`). */
export const WALL = 1;
/** Position of the area kind in the flags (core `grid::KIND_SHIFT`). */
export const KIND_SHIFT = 2;
/** Bits of the area kind in the flags (core `grid::KIND_MASK`). */
export const KIND_MASK = 0b111 << KIND_SHIFT;
/** Area kind "terrain": the ground of a new game, nothing built (ADR-045). */
export const TERRAIN = 7;
/** State of a cell or object: built (ADR-045). */
export const BUILT = 0;
/** State of a cell or object: planned, no material yet (ADR-045). */
export const PLANNED = 1;
/** State of a cell or object: in construction (ADR-045). */
export const BUILDING = 2;

/** An object of a block. */
export interface WorldObject {
  /** Catalog index plus 1, or {@link MISSING}. */
  readonly type: number;
  /** Cell in the block, east. */
  readonly x: number;
  /** Cell in the block, south. */
  readonly y: number;
  /** Rotation 0 … 3. */
  readonly rotation: number;
  /** {@link BUILT}, {@link PLANNED} or {@link BUILDING}. */
  readonly state: number;
}

/**
 * A block as read. The arrays are scratch space of the reader and only valid
 * during the callback; copy what is needed.
 */
export interface WorldBlock {
  /** Index in the block table: `floor · blocks per floor + by · blocks per row + bx`. */
  index: number;
  /** Flags (wall, stairs, area kind, staff zone) per cell in row order. */
  readonly flags: Uint8Array;
  /** Zone per cell. */
  readonly zone: Uint8Array;
  /** Marking per cell: 0 none, 1 preferred route, 2 avoid. */
  readonly marking: Uint8Array;
  /** State per cell: {@link BUILT}, or {@link PLANNED} / {@link BUILDING} under an open order (then `floor` or `wall` is the planned type). */
  readonly state: Uint8Array;
  /** Floor covering per cell: 0 none, else catalog index plus 1. */
  readonly floor: Uint16Array;
  /** Wall type per cell: 0 none, else catalog index plus 1. */
  readonly wall: Uint16Array;
  objects: WorldObject[];
}

/** Why a world section was rejected. */
export type WorldError = "record" | "encoding" | "palette" | "length" | "count";

/** Result of {@link readBlocks}. */
export type WorldResult =
  | { readonly ok: true; readonly blocks: number }
  | { readonly ok: false; readonly error: WorldError };

const pad4 = (n: number): number => Math.ceil(n / 4) * 4;

/** Scratch arrays reused by every {@link readBlocks} call. */
const scratch: WorldBlock = {
  index: 0,
  flags: new Uint8Array(BLOCK_CELLS),
  zone: new Uint8Array(BLOCK_CELLS),
  marking: new Uint8Array(BLOCK_CELLS),
  state: new Uint8Array(BLOCK_CELLS),
  floor: new Uint16Array(BLOCK_CELLS),
  wall: new Uint16Array(BLOCK_CELLS),
  objects: [],
};

function cell(view: DataView, at: number, i: number): void {
  scratch.flags[i] = view.getUint8(at);
  scratch.zone[i] = view.getUint8(at + 1);
  scratch.marking[i] = view.getUint8(at + 2);
  scratch.state[i] = view.getUint8(at + 3);
  scratch.floor[i] = view.getUint16(at + 4, true);
  scratch.wall[i] = view.getUint16(at + 6, true);
}

/**
 * Reads the `section.blocks` records of a world section and calls
 * `onBlock` for each. Nothing is called for a section that is malformed
 * from the start; a record that breaks a length stops the reading (the
 * blocks before it were delivered).
 */
export function readBlocks(
  section: WorldSection,
  onBlock: (block: WorldBlock) => void,
): WorldResult {
  const view = section.records;
  let at = 0;
  for (let k = 0; k < section.blocks; k += 1) {
    if (at + RECORD_HEADER > view.byteLength) {
      return { ok: false, error: "count" };
    }
    const index = view.getUint32(at, true);
    const encoding = view.getUint8(at + 4);
    const palette = view.getUint8(at + 5);
    const objects = view.getUint16(at + 6, true);
    const payload = view.getUint32(at + 8, true);
    const body = at + RECORD_HEADER;
    let cellsBytes: number;
    if (encoding === UNIFORM) {
      if (palette !== 1) return { ok: false, error: "palette" };
      cellsBytes = CELL_BYTES;
    } else if (encoding === PALETTE) {
      if (palette < 2 || palette > 16) return { ok: false, error: "palette" };
      cellsBytes = palette * CELL_BYTES + BLOCK_CELLS / 2;
    } else if (encoding === RAW) {
      if (palette !== 0) return { ok: false, error: "palette" };
      cellsBytes = BLOCK_CELLS * CELL_BYTES;
    } else {
      return { ok: false, error: "encoding" };
    }
    if (payload !== cellsBytes + pad4(objects * OBJECT_BYTES) || body + payload > view.byteLength) {
      return { ok: false, error: "length" };
    }
    if (encoding === RAW) {
      for (let i = 0; i < BLOCK_CELLS; i += 1) cell(view, body + i * CELL_BYTES, i);
    } else if (encoding === UNIFORM) {
      cell(view, body, 0);
      scratch.flags.fill(scratch.flags[0] ?? 0);
      scratch.zone.fill(scratch.zone[0] ?? 0);
      scratch.marking.fill(scratch.marking[0] ?? 0);
      scratch.state.fill(scratch.state[0] ?? 0);
      scratch.floor.fill(scratch.floor[0] ?? 0);
      scratch.wall.fill(scratch.wall[0] ?? 0);
    } else {
      const indices = body + palette * CELL_BYTES;
      for (let i = 0; i < BLOCK_CELLS; i += 1) {
        const p = (view.getUint8(indices + (i >> 1)) >> ((i & 1) * 4)) & 15;
        if (p >= palette) return { ok: false, error: "palette" };
        cell(view, body + p * CELL_BYTES, i);
      }
    }
    const list: WorldObject[] = [];
    const first = body + cellsBytes;
    for (let o = 0; o < objects; o += 1) {
      const r = first + o * OBJECT_BYTES;
      list.push({
        type: view.getUint16(r, true),
        x: view.getUint8(r + 2),
        y: view.getUint8(r + 3),
        rotation: view.getUint8(r + 4) & 3,
        state: view.getUint8(r + 5),
      });
    }
    scratch.index = index;
    scratch.objects = list;
    onBlock(scratch);
    at = body + payload;
  }
  return at === view.byteLength
    ? { ok: true, blocks: section.blocks }
    : { ok: false, error: "count" };
}
