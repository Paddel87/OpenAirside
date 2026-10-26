/**
 * The tile sheet of the world display (step 3.1b): 16 × 16 tiles at the
 * resolution of the atlas tier in use, one tile per slot of `SlotTable`,
 * plus the mean colour of every tile for the simplified overview. The
 * images come from the texture atlas by the name of the object type
 * (`base:floor_concrete`, ADR-032); a type without image shows the
 * neutral fallback of the atlas. Drawing goes through a small context
 * interface, so that the arrangement is testable without a canvas.
 */
import { DEFAULT_SLOT, MISSING_SLOT, SHEET_TILES, type SlotTable } from "./world-texels.js";

/** The part of a 2D canvas context the sheet needs. */
export interface SheetContext {
  fillStyle: string;
  fillRect(x: number, y: number, w: number, h: number): void;
  clearRect(x: number, y: number, w: number, h: number): void;
  getImageData(x: number, y: number, w: number, h: number): { readonly data: Uint8ClampedArray };
}

/** Colour of the plain floor of a walkable cell without covering. */
export const DEFAULT_COLOUR = "#b3b0a8";
/** Colours of the "missing" tile: a checker, so that a missing image is seen at once. */
export const MISSING_COLOURS: readonly [string, string] = ["#c8433b", "#262a30"];
/** Mean colour of a tile with no opaque pixel. */
const EMPTY_MEAN: readonly [number, number, number] = [138, 146, 158];

/** Result of {@link renderSheet}. */
export interface SheetResult {
  /** Mean colour per tile, RGBA, 16 × 16 texels in slot order. */
  readonly means: Uint8Array;
  /** Slots whose image was not found and show the missing tile. */
  readonly missing: readonly number[];
}

/** Position of `slot` in the sheet, in tiles. */
export function slotTile(slot: number): { readonly tx: number; readonly ty: number } {
  return { tx: slot % SHEET_TILES, ty: Math.floor(slot / SHEET_TILES) };
}

function meanOf(ctx: SheetContext, dx: number, dy: number, px: number): [number, number, number] {
  const { data } = ctx.getImageData(dx, dy, px, px);
  let r = 0;
  let g = 0;
  let b = 0;
  let weight = 0;
  for (let i = 0; i + 3 < data.length; i += 4) {
    const a = data[i + 3] ?? 0;
    r += (data[i] ?? 0) * a;
    g += (data[i + 1] ?? 0) * a;
    b += (data[i + 2] ?? 0) * a;
    weight += a;
  }
  return weight === 0
    ? [...EMPTY_MEAN]
    : [Math.round(r / weight), Math.round(g / weight), Math.round(b / weight)];
}

/**
 * Draws the sheet for `px` pixels per tile into `ctx` (a canvas of
 * `16 · px` pixels per edge) and returns the mean colours.
 *
 * @param nameOf image name for the catalog value of a slot, `undefined` if the catalog has none.
 * @param draw draws the image `name` into the tile at (`dx`, `dy`), `px` square; `false` if there is none.
 */
export function renderSheet(
  ctx: SheetContext,
  px: number,
  slots: SlotTable,
  nameOf: (value: number) => string | undefined,
  draw: (name: string, dx: number, dy: number) => boolean,
): SheetResult {
  const edge = SHEET_TILES * px;
  ctx.clearRect(0, 0, edge, edge);
  const means = new Uint8Array(SHEET_TILES * SHEET_TILES * 4);
  const missing: number[] = [];
  const keep = (slot: number, colour: readonly [number, number, number]): void => {
    means.set([colour[0], colour[1], colour[2], 255], slot * 4);
  };
  const fill = (slot: number, colour: string): void => {
    const { tx, ty } = slotTile(slot);
    ctx.fillStyle = colour;
    ctx.fillRect(tx * px, ty * px, px, px);
  };
  const missingTile = (slot: number): void => {
    const { tx, ty } = slotTile(slot);
    const half = Math.max(1, px / 2);
    for (let i = 0; i < 4; i += 1) {
      ctx.fillStyle = MISSING_COLOURS[(Math.floor(i / 2) + i) % 2] ?? "#000000";
      ctx.fillRect(tx * px + (i % 2) * half, ty * px + Math.floor(i / 2) * half, half, half);
    }
    keep(slot, meanOf(ctx, tx * px, ty * px, px));
  };
  fill(DEFAULT_SLOT, DEFAULT_COLOUR);
  keep(
    DEFAULT_SLOT,
    meanOf(ctx, slotTile(DEFAULT_SLOT).tx * px, slotTile(DEFAULT_SLOT).ty * px, px),
  );
  missingTile(MISSING_SLOT);
  for (let slot = 1; slot <= slots.size; slot += 1) {
    const value = slots.valueOf(slot);
    const name = value === undefined ? undefined : nameOf(value);
    const { tx, ty } = slotTile(slot);
    if (name !== undefined && draw(name, tx * px, ty * px)) {
      keep(slot, meanOf(ctx, tx * px, ty * px, px));
    } else {
      missing.push(slot);
      missingTile(slot);
    }
  }
  return { means, missing };
}
