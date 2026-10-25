import { describe, expect, it } from "vitest";
import {
  blockCounts,
  blockInView,
  DEFAULT_SLOT,
  LAST_TYPE_SLOT,
  MISSING_SLOT,
  placeOf,
  SlotTable,
  tileKey,
  TILE,
  writeBlockTexels,
} from "./world-texels.js";
import { BLOCK_CELLS, MISSING, WALL, type WorldBlock } from "./world-reader.js";

function block(fill: (i: number) => [flags: number, zone: number, floor: number, wall: number]) {
  const b: WorldBlock = {
    index: 0,
    flags: new Uint8Array(BLOCK_CELLS),
    zone: new Uint8Array(BLOCK_CELLS),
    marking: new Uint8Array(BLOCK_CELLS),
    floor: new Uint16Array(BLOCK_CELLS),
    wall: new Uint16Array(BLOCK_CELLS),
    objects: [],
  };
  for (let i = 0; i < BLOCK_CELLS; i += 1) {
    const [flags, zone, floor, wall] = fill(i);
    b.flags[i] = flags;
    b.zone[i] = zone;
    b.floor[i] = floor;
    b.wall[i] = wall;
  }
  return b;
}

describe("SlotTable", () => {
  it("gives the first sight of a type the next slot and remembers it", () => {
    const slots = new SlotTable();
    expect(slots.slotOf(0)).toBe(0);
    expect(slots.slotOf(12)).toBe(1);
    expect(slots.slotOf(40)).toBe(2);
    expect(slots.slotOf(12)).toBe(1);
    expect(slots.valueOf(2)).toBe(40);
    expect(slots.valueOf(9)).toBeUndefined();
    expect(slots.size).toBe(2);
    slots.clear();
    expect(slots.size).toBe(0);
    expect(slots.slotOf(40)).toBe(1);
  });

  it("shows missing types in the missing slot, also when the table is full", () => {
    const slots = new SlotTable();
    expect(slots.slotOf(MISSING)).toBe(MISSING_SLOT);
    for (let v = 1; v <= LAST_TYPE_SLOT; v += 1) expect(slots.slotOf(v)).toBe(v);
    expect(slots.slotOf(1000)).toBe(MISSING_SLOT);
    expect(slots.slotOf(3)).toBe(3);
  });
});

describe("places", () => {
  it("finds block, floor and tile of a table index", () => {
    // 100 × 70 cells: 4 × 3 blocks per floor.
    expect(blockCounts(100, 70)).toEqual({ x: 4, y: 3 });
    expect(placeOf(0, 100, 70)).toEqual({ floorIndex: 0, x: 0, y: 0 });
    expect(placeOf(2 * 12 + 5, 100, 70)).toEqual({ floorIndex: 2, x: 32, y: 32 });
    expect(placeOf(60, 100, 70)).toBeUndefined();
    expect(placeOf(-1, 100, 70)).toBeUndefined();
    expect(placeOf(1.5, 100, 70)).toBeUndefined();
    expect(placeOf(0, 0, 0)).toBeUndefined();
    expect(tileKey(2, 0, 0)).toBe(2048);
    expect(tileKey(2, 511, 1023)).toBe(2048 + 32);
    expect(tileKey(0, 512, 0)).toBe(1);
  });

  it("checks a block against the view of a floor with a margin", () => {
    const view = { x0: 40, y0: 40, x1: 60, y1: 60 };
    const ground = 2 * 12;
    expect(blockInView(ground + 5, 100, 70, 0, view, 0)).toBe(true);
    expect(blockInView(ground + 0, 100, 70, 0, view, 0)).toBe(false);
    expect(blockInView(ground + 0, 100, 70, 0, view, 40)).toBe(true);
    expect(blockInView(ground + 5, 100, 70, 1, view, 0)).toBe(false);
  });
});

describe("writeBlockTexels", () => {
  it("writes slots, zone and opacity into the data texture at the place of the block", () => {
    const slots = new SlotTable();
    const data = new Uint8Array(TILE * TILE * 4);
    const b = block((i) => {
      if (i === 0) return [WALL, 0, 0, 0]; // void
      if (i === 1) return [0, 2, 0, 0]; // walkable, no covering
      if (i === 2) return [0, 1, 30, 0]; // floor type 30
      if (i === 3) return [WALL, 3, 30, 31]; // wall type 31 over floor 30
      if (i === 4) return [WALL, 0, 30, 0]; // blocked by an object, floor stays
      if (i === 5) return [0, 0, MISSING, 0];
      return [WALL, 0, 0, 0];
    });
    writeBlockTexels(data, 64, 96, b, slots);
    const texel = (cx: number, cy: number): number[] => {
      const o = (cy * TILE + cx) * 4;
      return [...data.slice(o, o + 4)];
    };
    expect(texel(64, 96)).toEqual([0, 0, 0, 255]);
    expect(texel(65, 96)).toEqual([DEFAULT_SLOT, 0, 2, 255]);
    expect(texel(66, 96)).toEqual([1, 0, 1, 255]);
    expect(texel(67, 96)).toEqual([1, 2, 3, 255]);
    expect(texel(68, 96)).toEqual([1, 0, 0, 255]);
    expect(texel(69, 96)).toEqual([MISSING_SLOT, 0, 0, 255]);
    expect(texel(64, 97)).toEqual([0, 0, 0, 255]);
    // Outside the block nothing was written.
    expect(texel(63, 96)).toEqual([0, 0, 0, 0]);
    expect(texel(64, 128)).toEqual([0, 0, 0, 0]);
  });

  it("places a block of a later texture at the offset inside it", () => {
    const data = new Uint8Array(TILE * TILE * 4);
    writeBlockTexels(
      data,
      TILE + 32,
      TILE,
      block(() => [0, 0, 5, 0]),
      new SlotTable(),
    );
    expect(data[(0 * TILE + 32) * 4]).toBe(1);
    expect(data[(31 * TILE + 63) * 4]).toBe(1);
    expect(data[(0 * TILE + 31) * 4]).toBe(0);
  });
});
