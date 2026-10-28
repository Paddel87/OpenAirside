/**
 * Data texels of the resting world (module "Spielfeld", step 3.1b, ADR-038
 * Z-1): one texel per cell in data textures of 512 × 512 cells; the shader
 * of `world-view.ts` reads the texel and picks the tile of floor covering or
 * wall type from a tile sheet. This module is the arithmetic without WebGL,
 * so that it is testable.
 *
 * Texel (RGBA8): R = slot of the floor covering, G = slot of the wall type,
 * B = zone (0 … 3), A = opacity by the state of the cell ({@link STATE_ALPHA}:
 * built 255, planned and in construction less, ADR-045). Slot 0 means
 * "none"; a cell with neither is void or terrain and not drawn. A slot is the
 * place of a type in the 16 × 16 tile sheet; types get their slot when a
 * block first shows them.
 */
import {
  BLOCK,
  BLOCK_CELLS,
  KIND_MASK,
  KIND_SHIFT,
  MISSING,
  TERRAIN,
  WALL,
  type WorldBlock,
} from "./world-reader.js";

/** Edge of a data texture in cells. */
export const TILE = 512;
/** Blocks per edge of a data texture. */
export const TILE_BLOCKS = TILE / BLOCK;
/** Floors of the world (−2 … +2). */
export const FLOORS = 5;
/** Slot of "none". */
export const NO_SLOT = 0;
/** Slot of the plain floor of a walkable cell without any covering. */
export const DEFAULT_SLOT = 255;
/** Slot of a type that has no slot or no catalog entry. */
export const MISSING_SLOT = 254;
/** Highest slot a type can get. */
export const LAST_TYPE_SLOT = 253;
/** Tiles per edge of the tile sheet. */
export const SHEET_TILES = 16;
/** Opacity (0 … 255) of a cell or object by its state: built, planned, in construction (ADR-045). */
export const STATE_ALPHA: readonly number[] = [255, 102, 166];

/** Opacity of the state `state`; unknown states count as built. */
export function stateAlpha(state: number): number {
  return STATE_ALPHA[state] ?? 255;
}

/** The slots of the types shown, by catalog value (catalog index plus 1). */
export class SlotTable {
  readonly #slots = new Map<number, number>();
  readonly #values: number[] = [];

  /** Slot of `value` (0 = none, {@link MISSING} = missing); allocates one on first sight. */
  slotOf(value: number): number {
    if (value === 0) return NO_SLOT;
    if (value === MISSING) return MISSING_SLOT;
    const known = this.#slots.get(value);
    if (known !== undefined) return known;
    if (this.#values.length >= LAST_TYPE_SLOT) return MISSING_SLOT;
    this.#values.push(value);
    const slot = this.#values.length;
    this.#slots.set(value, slot);
    return slot;
  }

  /** Catalog value of the type in `slot`, `undefined` for a free or special slot. */
  valueOf(slot: number): number | undefined {
    return this.#values[slot - 1];
  }

  /** Number of slots in use. */
  get size(): number {
    return this.#values.length;
  }

  /** Forgets every slot (new session). */
  clear(): void {
    this.#slots.clear();
    this.#values.length = 0;
  }
}

/** Position of a block in the world. */
export interface BlockPlace {
  readonly floorIndex: number;
  /** Cell of the north-west corner of the block. */
  readonly x: number;
  readonly y: number;
}

/** Blocks per row and per column of a map. */
export function blockCounts(
  width: number,
  height: number,
): { readonly x: number; readonly y: number } {
  return { x: Math.ceil(width / BLOCK), y: Math.ceil(height / BLOCK) };
}

/** Where table index `index` lies; `undefined` if it is outside the table. */
export function placeOf(index: number, width: number, height: number): BlockPlace | undefined {
  const counts = blockCounts(width, height);
  const perFloor = counts.x * counts.y;
  if (perFloor === 0 || !Number.isInteger(index) || index < 0 || index >= perFloor * FLOORS) {
    return undefined;
  }
  const inFloor = index % perFloor;
  return {
    floorIndex: Math.floor(index / perFloor),
    x: (inFloor % counts.x) * BLOCK,
    y: Math.floor(inFloor / counts.x) * BLOCK,
  };
}

/** Key of the data texture holding cell (`x`, `y`) of a floor. */
export function tileKey(floorIndex: number, x: number, y: number): number {
  return floorIndex * 1024 + Math.floor(y / TILE) * 32 + Math.floor(x / TILE);
}

/** Writes the cells of `block` (north-west corner at cell (`x`, `y`)) into `data`, a data texture of 512 × 512 texels. */
export function writeBlockTexels(
  data: Uint8Array,
  x: number,
  y: number,
  block: WorldBlock,
  slots: SlotTable,
): void {
  const x0 = x % TILE;
  const y0 = y % TILE;
  for (let i = 0; i < BLOCK_CELLS; i += 1) {
    const o = ((y0 + Math.floor(i / BLOCK)) * TILE + x0 + (i % BLOCK)) * 4;
    const wall = slots.slotOf(block.wall[i] ?? 0);
    const covering = block.floor[i] ?? 0;
    const flags = block.flags[i] ?? WALL;
    const walkable = (flags & WALL) === 0;
    const terrain = (flags & KIND_MASK) >> KIND_SHIFT === TERRAIN;
    data[o] =
      covering !== 0
        ? slots.slotOf(covering)
        : wall === NO_SLOT && walkable && !terrain
          ? DEFAULT_SLOT
          : NO_SLOT;
    data[o + 1] = wall;
    data[o + 2] = (block.zone[i] ?? 0) & 3;
    data[o + 3] = stateAlpha(block.state[i] ?? 0);
  }
}

/** `true` if block `index` of `floor` lies inside the rectangle (cells), `margin` cells around it included. */
export function blockInView(
  index: number,
  width: number,
  height: number,
  floor: number,
  view: { readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number },
  margin: number,
): boolean {
  const place = placeOf(index, width, height);
  if (place?.floorIndex !== floor + 2) return false;
  return (
    place.x + BLOCK + margin >= view.x0 &&
    place.x - margin <= view.x1 &&
    place.y + BLOCK + margin >= view.y0 &&
    place.y - margin <= view.y1
  );
}
