import { describe, expect, it } from "vitest";
import { DEFAULT_COLOUR, renderSheet, type SheetContext, slotTile } from "./tile-sheet.js";
import { DEFAULT_SLOT, MISSING_SLOT, SlotTable } from "./world-texels.js";

/** Context that remembers fills and answers every read with one colour per tile. */
function fakeContext(px: number) {
  const fills: { colour: string; x: number; y: number }[] = [];
  const painted = new Map<string, [number, number, number, number]>();
  const key = (x: number, y: number): string => `${String(x)},${String(y)}`;
  const ctx: SheetContext = {
    fillStyle: "",
    fillRect(x, y) {
      fills.push({ colour: ctx.fillStyle, x, y });
    },
    clearRect: () => undefined,
    getImageData(x, y, w, h) {
      const colour = painted.get(key(x, y)) ?? [0, 0, 0, 0];
      const data = new Uint8ClampedArray(w * h * 4);
      for (let i = 0; i < w * h; i += 1) data.set(colour, i * 4);
      return { data };
    },
  };
  return { ctx, fills, painted, key, px };
}

describe("renderSheet", () => {
  it("draws the images of the slots and reports the mean colours", () => {
    const px = 8;
    const { ctx, painted, key } = fakeContext(px);
    const slots = new SlotTable();
    slots.slotOf(11);
    slots.slotOf(22);
    const names = new Map([
      [11, "base:floor_concrete"],
      [22, "base:wall_concrete"],
    ]);
    const drawn: string[] = [];
    const result = renderSheet(
      ctx,
      px,
      slots,
      (v) => names.get(v),
      (name, dx, dy) => {
        drawn.push(name);
        painted.set(key(dx, dy), name.includes("floor") ? [100, 110, 120, 255] : [10, 20, 30, 255]);
        return true;
      },
    );
    expect(drawn).toEqual(["base:floor_concrete", "base:wall_concrete"]);
    expect(result.missing).toEqual([]);
    expect([...result.means.slice(4, 8)]).toEqual([100, 110, 120, 255]);
    expect([...result.means.slice(8, 12)]).toEqual([10, 20, 30, 255]);
  });

  it("shows the missing tile for an unknown value or a missing image", () => {
    const px = 8;
    const { ctx, fills } = fakeContext(px);
    const slots = new SlotTable();
    slots.slotOf(11);
    slots.slotOf(12);
    const result = renderSheet(
      ctx,
      px,
      slots,
      (v) => (v === 11 ? "base:gone" : undefined),
      () => false,
    );
    expect(result.missing).toEqual([1, 2]);
    expect(fills.some((f) => f.colour === DEFAULT_COLOUR)).toBe(true);
    // The plain and the missing tile are always drawn, in their places.
    const plain = slotTile(DEFAULT_SLOT);
    expect(fills.some((f) => f.x === plain.tx * px && f.y === plain.ty * px)).toBe(true);
    const missing = slotTile(MISSING_SLOT);
    expect(fills.filter((f) => f.x >= missing.tx * px && f.y === missing.ty * px)).not.toHaveLength(
      0,
    );
  });

  it("takes the colour of a transparent tile from its opaque pixels, grey when there are none", () => {
    const px = 4;
    const { ctx, painted, key } = fakeContext(px);
    const slots = new SlotTable();
    slots.slotOf(1);
    slots.slotOf(2);
    const result = renderSheet(
      ctx,
      px,
      slots,
      (v) => `n${String(v)}`,
      (name, dx, dy) => {
        painted.set(key(dx, dy), name === "n1" ? [200, 100, 50, 128] : [9, 9, 9, 0]);
        return true;
      },
    );
    expect([...result.means.slice(4, 8)]).toEqual([200, 100, 50, 255]);
    expect([...result.means.slice(8, 12)]).toEqual([138, 146, 158, 255]);
  });
});

describe("slotTile", () => {
  it("lays the slots out in rows of 16", () => {
    expect(slotTile(0)).toEqual({ tx: 0, ty: 0 });
    expect(slotTile(17)).toEqual({ tx: 1, ty: 1 });
    expect(slotTile(255)).toEqual({ tx: 15, ty: 15 });
  });
});
