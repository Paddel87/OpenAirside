import { describe, expect, it } from "vitest";
import { WORLD_HEADER_BYTES, type WorldSection } from "./frame-reader.js";
import {
  BLOCK_CELLS,
  CELL_BYTES,
  MISSING,
  OBJECT_BYTES,
  PALETTE,
  RAW,
  readBlocks,
  RECORD_HEADER,
  UNIFORM,
  type WorldBlock,
} from "./world-reader.js";

type Cell = [flags: number, zone: number, marking: number, floor: number, wall: number];

function put(view: DataView, at: number, [flags, zone, marking, floor, wall]: Cell): void {
  view.setUint8(at, flags);
  view.setUint8(at + 1, zone);
  view.setUint8(at + 2, marking);
  view.setUint16(at + 4, floor, true);
  view.setUint16(at + 6, wall, true);
}

interface Spec {
  index: number;
  encoding: number;
  palette: Cell[];
  cells?: Cell[];
  objects?: [number, number, number, number][];
}

/** Encodes records like core/src/display.rs. */
function section(specs: Spec[]): WorldSection {
  const parts: Uint8Array[] = [];
  for (const spec of specs) {
    const raw = spec.encoding === RAW;
    const indices = spec.encoding === PALETTE ? BLOCK_CELLS / 2 : 0;
    const cellsBytes = raw ? BLOCK_CELLS * CELL_BYTES : spec.palette.length * CELL_BYTES + indices;
    const objects = spec.objects ?? [];
    const objectBytes = Math.ceil((objects.length * OBJECT_BYTES) / 4) * 4;
    const buf = new Uint8Array(RECORD_HEADER + cellsBytes + objectBytes);
    const v = new DataView(buf.buffer);
    v.setUint32(0, spec.index, true);
    v.setUint8(4, spec.encoding);
    v.setUint8(5, raw ? 0 : spec.palette.length);
    v.setUint16(6, objects.length, true);
    v.setUint32(8, cellsBytes + objectBytes, true);
    if (raw) {
      (spec.cells ?? []).forEach((c, i) => {
        put(v, RECORD_HEADER + i * CELL_BYTES, c);
      });
    } else {
      spec.palette.forEach((c, i) => {
        put(v, RECORD_HEADER + i * CELL_BYTES, c);
      });
      const at = RECORD_HEADER + spec.palette.length * CELL_BYTES;
      (spec.cells ?? []).forEach((c, i) => {
        const p = spec.palette.findIndex((q) => q.join() === c.join());
        const old = v.getUint8(at + (i >> 1));
        v.setUint8(at + (i >> 1), old | (p << ((i & 1) * 4)));
      });
    }
    objects.forEach(([type, x, y, rotation], i) => {
      const o = RECORD_HEADER + cellsBytes + i * OBJECT_BYTES;
      v.setUint16(o, type, true);
      v.setUint8(o + 2, x);
      v.setUint8(o + 3, y);
      v.setUint8(o + 4, rotation);
    });
    parts.push(buf);
  }
  const total = parts.reduce((n, p) => n + p.length, 0);
  const records = new Uint8Array(total);
  let at = 0;
  for (const p of parts) {
    records.set(p, at);
    at += p.length;
  }
  return {
    blocks: specs.length,
    pending: 0,
    bytes: WORLD_HEADER_BYTES + total,
    records: new DataView(records.buffer),
  };
}

function collect(s: WorldSection) {
  const seen: {
    index: number;
    floor: number[];
    wall: number[];
    flags: number[];
    objects: WorldBlock["objects"];
  }[] = [];
  const result = readBlocks(s, (b) => {
    seen.push({
      index: b.index,
      floor: [b.floor[0] ?? -1, b.floor[33] ?? -1, b.floor[BLOCK_CELLS - 1] ?? -1],
      wall: [b.wall[0] ?? -1, b.wall[33] ?? -1],
      flags: [b.flags[0] ?? -1, b.flags[33] ?? -1],
      objects: b.objects,
    });
  });
  return { result, seen };
}

const void_: Cell = [1, 0, 0, 0, 0];
const concrete: Cell = [0, 1, 0, 3, 0];
const wall: Cell = [1, 1, 0, 3, 4];

describe("readBlocks", () => {
  it("reads uniform, palette and raw blocks with their objects", () => {
    const cells: Cell[] = Array.from({ length: BLOCK_CELLS }, (_, i) =>
      i === 33 ? wall : i === 0 ? void_ : concrete,
    );
    const { result, seen } = collect(
      section([
        { index: 7, encoding: UNIFORM, palette: [void_] },
        {
          index: 8,
          encoding: PALETTE,
          palette: [void_, concrete, wall],
          cells,
          objects: [[5, 10, 3, 1]],
        },
        { index: 9, encoding: RAW, palette: [], cells, objects: [[MISSING, 0, 31, 3]] },
      ]),
    );
    expect(result).toEqual({ ok: true, blocks: 3 });
    expect(seen.map((b) => b.index)).toEqual([7, 8, 9]);
    expect(seen[0]).toMatchObject({ floor: [0, 0, 0], wall: [0, 0], flags: [1, 1], objects: [] });
    expect(seen[1]).toMatchObject({ floor: [0, 3, 3], wall: [0, 4], flags: [1, 1] });
    expect(seen[1]?.objects).toEqual([{ type: 5, x: 10, y: 3, rotation: 1 }]);
    expect(seen[2]).toMatchObject({ floor: [0, 3, 3], wall: [0, 4] });
    expect(seen[2]?.objects).toEqual([{ type: MISSING, x: 0, y: 31, rotation: 3 }]);
  });

  it("accepts a section without blocks", () => {
    expect(collect(section([])).result).toEqual({ ok: true, blocks: 0 });
  });

  it.each([
    [
      "encoding",
      (v: DataView) => {
        v.setUint8(4, 9);
      },
    ],
    [
      "palette",
      (v: DataView) => {
        v.setUint8(5, 2);
      },
    ],
    [
      "length",
      (v: DataView) => {
        v.setUint32(8, 4, true);
      },
    ],
    [
      "length",
      (v: DataView) => {
        v.setUint16(6, 2, true);
      },
    ],
  ] as const)("rejects a record with a broken %s", (error, damage) => {
    const s = section([{ index: 1, encoding: UNIFORM, palette: [void_] }]);
    damage(s.records);
    expect(collect(s).result).toEqual({ ok: false, error });
  });

  it("rejects a palette index beyond the palette and a wrong count", () => {
    const s = section([
      { index: 1, encoding: PALETTE, palette: [void_, concrete], cells: [concrete] },
    ]);
    s.records.setUint8(RECORD_HEADER + 2 * CELL_BYTES, 0x0f);
    expect(collect(s).result).toEqual({ ok: false, error: "palette" });
    const two = section([{ index: 1, encoding: UNIFORM, palette: [void_] }]);
    expect(collect({ ...two, blocks: 2 }).result).toEqual({ ok: false, error: "count" });
    expect(collect({ ...two, blocks: 0 }).result).toEqual({ ok: false, error: "count" });
  });
});
