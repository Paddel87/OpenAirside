import { describe, expect, it } from "vitest";
import { HEADER_BYTES, PERSON_BYTES, readFrame, readPerson } from "./frame-reader.js";

/** Builds a frame like core/src/frame.rs does. */
function build(persons: [number, number, number, number][], floor = 127): ArrayBuffer {
  const buf = new ArrayBuffer(HEADER_BYTES + persons.length * PERSON_BYTES);
  const v = new DataView(buf);
  v.setUint16(0, 1, true);
  v.setUint16(2, 32, true);
  v.setUint32(4, 3, true);
  v.setUint32(8, 99, true);
  v.setUint16(12, 2048, true);
  v.setUint16(14, 1536, true);
  v.setUint32(16, 500, true);
  v.setUint32(20, persons.length, true);
  v.setInt8(24, floor);
  v.setUint8(25, PERSON_BYTES);
  v.setUint32(28, 7, true);
  persons.forEach(([x, y, f, h], i) => {
    const o = HEADER_BYTES + i * PERSON_BYTES;
    v.setUint16(o, x, true);
    v.setUint16(o + 2, y, true);
    v.setInt8(o + 4, f);
    v.setUint8(o + 5, h);
  });
  return buf;
}

describe("readFrame", () => {
  it("reads header and persons", () => {
    const result = readFrame(
      build([
        [48, 1616, 0, 2],
        [32, 64, -1, 254],
      ]),
    );
    expect(result.ok).toBe(true);
    if (!result.ok) return;
    expect(result.frame.header).toEqual({
      session: 3,
      tick: 99,
      mapWidth: 2048,
      mapHeight: 1536,
      personsTotal: 500,
      persons: 2,
      floor: undefined,
      fieldsRevision: 7,
    });
    const p = { x: 0, y: 0, floor: 0, heading: 0 };
    expect(readPerson(result.frame, 0, p)).toEqual({ x: 1.5, y: 50.5, floor: 0, heading: 2 });
    expect(readPerson(result.frame, 1, p)).toEqual({ x: 1, y: 2, floor: -1, heading: 254 });
  });

  it("reports the floor of the filter", () => {
    const result = readFrame(build([], -2));
    expect(result.ok && result.frame.header.floor).toBe(-2);
  });

  it.each([
    ["tooShort", () => new ArrayBuffer(10)],
    [
      "layoutVersion",
      () => {
        const b = build([]);
        new DataView(b).setUint16(0, 2, true);
        return b;
      },
    ],
    [
      "headerSize",
      () => {
        const b = build([]);
        new DataView(b).setUint16(2, 40, true);
        return b;
      },
    ],
    [
      "personSize",
      () => {
        const b = build([]);
        new DataView(b).setUint8(25, 12);
        return b;
      },
    ],
    [
      "length",
      () => {
        const b = build([[1, 1, 0, 0]]);
        new DataView(b).setUint32(20, 1000, true);
        return b;
      },
    ],
  ] as const)("rejects %s", (error, make) => {
    expect(readFrame(make())).toEqual({ ok: false, error });
  });
});
