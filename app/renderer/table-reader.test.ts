import { describe, expect, it } from "vitest";
import { cell, readTable, TABLE_HEADER_BYTES } from "./table-reader.js";

/** Table with columns handle, floor, x for two rows, like core/src/query.rs. */
function build(): ArrayBuffer {
  // handle 16 bytes, floor 2 + 2 padding, x 8.
  const buf = new ArrayBuffer(TABLE_HEADER_BYTES + 16 + 4 + 8);
  const v = new DataView(buf);
  v.setUint16(0, 1, true);
  v.setUint16(2, 32, true);
  v.setUint32(4, 50, true);
  v.setUint32(8, 10, true);
  v.setUint32(12, 3, true);
  v.setUint32(16, 2, true);
  v.setUint16(20, 3, true);
  v.setUint32(24, 1, true);
  let o = TABLE_HEADER_BYTES;
  v.setUint32(o, 7, true);
  v.setUint32(o + 4, 1, true);
  v.setUint32(o + 8, 9, true);
  v.setUint32(o + 12, 1, true);
  o += 16;
  v.setInt8(o, -1);
  v.setInt8(o + 1, 2);
  o += 4;
  v.setFloat32(o, 1.5, true);
  v.setFloat32(o + 4, 2.25, true);
  return buf;
}

describe("readTable", () => {
  it("reads header and columns with padding", () => {
    const result = readTable(build(), ["handle", "floor", "x"]);
    expect(result.ok).toBe(true);
    if (!result.ok) return;
    const t = result.table;
    expect(t.header).toEqual({ tick: 50, total: 10, offset: 3, rows: 2, dictionaryVersion: 1 });
    expect([cell(t, "handle", 0), cell(t, "handle", 1)]).toEqual([7, 9]);
    expect([cell(t, "floor", 0), cell(t, "floor", 1)]).toEqual([-1, 2]);
    expect([cell(t, "x", 0), cell(t, "x", 1)]).toEqual([1.5, 2.25]);
    expect(cell(t, "x", 2)).toBeUndefined();
    expect(cell(t, "y", 0)).toBeUndefined();
  });

  it("reads all column kinds", () => {
    const buf = new ArrayBuffer(TABLE_HEADER_BYTES + 4 + 4 + 2);
    const v = new DataView(buf);
    v.setUint16(0, 1, true);
    v.setUint16(2, 32, true);
    v.setUint32(16, 1, true);
    v.setUint16(20, 3, true);
    v.setFloat32(32, 4.5, true);
    v.setUint8(36, 254);
    v.setUint16(40, 3, true);
    const result = readTable(buf, ["y", "heading", "state"]);
    expect(result.ok).toBe(true);
    if (!result.ok) return;
    expect(cell(result.table, "y", 0)).toBe(4.5);
    expect(cell(result.table, "heading", 0)).toBe(254);
    expect(cell(result.table, "state", 0)).toBe(3);
  });

  it.each([
    ["tooShort", new ArrayBuffer(8), ["x"]],
    [
      "layoutVersion",
      (() => {
        const b = build();
        new DataView(b).setUint16(0, 9, true);
        return b;
      })(),
      ["handle", "floor", "x"],
    ],
    ["columns", build(), ["handle"]],
    ["length", build(), ["handle", "x", "x"]],
    ["length", build(), ["floor", "floor", "floor"]],
    [
      "length",
      (() => {
        const b = build();
        new DataView(b).setUint32(16, 100_000, true);
        return b;
      })(),
      ["handle", "floor", "x"],
    ],
  ] as const)("rejects %s", (error, buf, columns) => {
    expect(readTable(buf, columns)).toEqual({ ok: false, error });
  });
});
