import { describe, expect, it } from "vitest";
import {
  AIRCRAFT_BYTES,
  BAGGAGE_BYTES,
  HEADER_BYTES,
  PERSON_BYTES,
  readFrame,
  readPerson,
  VEHICLE_BYTES,
  WORLD_HEADER_BYTES,
} from "./frame-reader.js";

interface Counts {
  vehicles?: number;
  aircraft?: number;
  baggage?: number;
  /** Bytes of the block records after the world section header. */
  records?: number;
  blocks?: number;
}

/** Builds a frame like core/src/frame.rs does. */
function build(
  persons: [number, number, number, number][],
  floor = 127,
  counts: Counts = {},
): ArrayBuffer {
  const vehicles = counts.vehicles ?? 0;
  const aircraft = counts.aircraft ?? 0;
  const baggage = counts.baggage ?? 0;
  const moving =
    persons.length * PERSON_BYTES +
    vehicles * VEHICLE_BYTES +
    aircraft * AIRCRAFT_BYTES +
    baggage * BAGGAGE_BYTES;
  const records = counts.records ?? 0;
  const buf = new ArrayBuffer(HEADER_BYTES + moving + WORLD_HEADER_BYTES + records);
  const v = new DataView(buf);
  v.setUint16(0, 2, true);
  v.setUint16(2, HEADER_BYTES, true);
  v.setUint32(4, 3, true);
  v.setUint32(8, 99, true);
  v.setUint16(12, 2048, true);
  v.setUint16(14, 1536, true);
  v.setUint32(16, 500, true);
  v.setUint32(20, persons.length, true);
  v.setInt8(24, floor);
  v.setUint8(25, PERSON_BYTES);
  v.setUint32(28, 7, true);
  v.setUint32(32, 5, true);
  v.setUint32(36, vehicles, true);
  v.setUint32(40, aircraft, true);
  v.setUint32(44, baggage, true);
  persons.forEach(([x, y, f, h], i) => {
    const o = HEADER_BYTES + i * PERSON_BYTES;
    v.setUint16(o, x, true);
    v.setUint16(o + 2, y, true);
    v.setInt8(o + 4, f);
    v.setUint8(o + 5, h);
    v.setUint8(o + 6, 3);
    v.setUint8(o + 7, 2);
  });
  const section = HEADER_BYTES + moving;
  v.setUint32(section, WORLD_HEADER_BYTES + records, true);
  v.setUint32(section + 4, counts.blocks ?? 0, true);
  v.setUint32(section + 8, 9, true);
  return buf;
}

describe("readFrame", () => {
  it("reads header, persons and the world section", () => {
    const result = readFrame(
      build(
        [
          [48, 1616, 0, 2],
          [32, 64, -1, 254],
        ],
        127,
        { blocks: 1, records: 12 },
      ),
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
      worldRevision: 5,
      vehicles: 0,
      aircraft: 0,
      baggage: 0,
    });
    const p = { x: 0, y: 0, floor: 0, heading: 0, group: 0, state: 0 };
    expect(readPerson(result.frame, 0, p)).toEqual({
      x: 1.5,
      y: 50.5,
      floor: 0,
      heading: 2,
      group: 3,
      state: 2,
    });
    expect(readPerson(result.frame, 1, p)).toMatchObject({ x: 1, y: 2, floor: -1, heading: 254 });
    expect(result.frame.world).toMatchObject({ blocks: 1, pending: 9, bytes: 28 });
    expect(result.frame.world.records.byteLength).toBe(12);
  });

  it("finds the world section behind the other sections", () => {
    const result = readFrame(build([[1, 1, 0, 0]], 127, { vehicles: 2, aircraft: 1, baggage: 3 }));
    expect(result.ok && result.frame.world.bytes).toBe(WORLD_HEADER_BYTES);
    expect(result.ok && result.frame.header.vehicles).toBe(2);
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
        new DataView(b).setUint16(0, 1, true);
        return b;
      },
    ],
    [
      "headerSize",
      () => {
        const b = build([]);
        new DataView(b).setUint16(2, 32, true);
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
    [
      "length",
      () => {
        const b = build([]);
        new DataView(b).setUint32(44, 0xffffffff, true);
        return b;
      },
    ],
    [
      "worldSection",
      () => {
        const b = build([]);
        new DataView(b).setUint32(HEADER_BYTES, WORLD_HEADER_BYTES + 4, true);
        return b;
      },
    ],
    [
      "worldSection",
      () => {
        const b = build([]);
        new DataView(b).setUint32(HEADER_BYTES, 4, true);
        return b;
      },
    ],
  ] as const)("rejects %s", (error, make) => {
    expect(readFrame(make())).toEqual({ ok: false, error });
  });
});
