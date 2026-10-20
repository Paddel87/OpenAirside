/**
 * Reader of the frame buffer S-1 (layout version 1, core/src/frame.rs).
 * Checks every length against `byteLength` before it creates views, because
 * the buffer crosses a process boundary (ADR-021).
 */

/** Layout version this reader understands. */
export const LAYOUT_VERSION = 1;
/** Header size of layout version 1. */
export const HEADER_BYTES = 32;
/** Bytes per person of layout version 1. */
export const PERSON_BYTES = 6;
/** Fixed-point steps per cell. */
export const SUBCELL = 32;
/** Floor value of a frame without floor filter. */
export const ALL_FLOORS = 127;

/** Header of a frame. */
export interface FrameHeader {
  readonly session: number;
  readonly tick: number;
  readonly mapWidth: number;
  readonly mapHeight: number;
  readonly personsTotal: number;
  readonly persons: number;
  /** Floor of the filter, `undefined` for all floors. */
  readonly floor: number | undefined;
  readonly fieldsRevision: number;
}

/** A checked frame: header plus the person records. */
export interface Frame {
  readonly header: FrameHeader;
  /** Person records, `PERSON_BYTES` each. */
  readonly data: DataView;
  /** Spike 3.8: offset of the world section in `buf`, if any. */
  readonly worldAt?: number;
  /** Spike 3.8: the whole frame buffer. */
  readonly buf?: ArrayBuffer;
}

/** Why a frame was rejected. */
export type FrameError = "tooShort" | "layoutVersion" | "headerSize" | "personSize" | "length";

/** Result of {@link readFrame}. */
export type FrameResult =
  { readonly ok: true; readonly frame: Frame } | { readonly ok: false; readonly error: FrameError };

/** Checks and reads a frame buffer. */
export function readFrame(buf: ArrayBuffer): FrameResult {
  if (buf.byteLength < HEADER_BYTES) {
    return { ok: false, error: "tooShort" };
  }
  const view = new DataView(buf, 0, HEADER_BYTES);
  if (view.getUint16(0, true) !== LAYOUT_VERSION) {
    return { ok: false, error: "layoutVersion" };
  }
  if (view.getUint16(2, true) !== HEADER_BYTES) {
    return { ok: false, error: "headerSize" };
  }
  if (view.getUint8(25) !== PERSON_BYTES) {
    return { ok: false, error: "personSize" };
  }
  const persons = view.getUint32(20, true);
  const end = HEADER_BYTES + persons * PERSON_BYTES;
  // Spike 3.8: header byte 26 = 1 marks a world section after the persons.
  const world = view.getUint8(26) === 1;
  const worldBytes =
    world && buf.byteLength >= end + 4 ? new DataView(buf, end, 4).getUint32(0, true) : 0;
  if (buf.byteLength !== end + worldBytes || (world && worldBytes === 0)) {
    return { ok: false, error: "length" };
  }
  const floor = view.getInt8(24);
  return {
    ok: true,
    frame: {
      header: {
        session: view.getUint32(4, true),
        tick: view.getUint32(8, true),
        mapWidth: view.getUint16(12, true),
        mapHeight: view.getUint16(14, true),
        personsTotal: view.getUint32(16, true),
        persons,
        floor: floor === ALL_FLOORS ? undefined : floor,
        fieldsRevision: view.getUint32(28, true),
      },
      data: new DataView(buf, HEADER_BYTES, persons * PERSON_BYTES),
      ...(world ? { worldAt: end, buf } : {}),
    },
  };
}

/** One person of a frame, position in cells. */
export interface PersonRecord {
  x: number;
  y: number;
  floor: number;
  heading: number;
}

/**
 * Reads person `index` into `out` (reused to avoid allocations per person
 * and frame). The caller keeps `index` below `header.persons`.
 */
export function readPerson(frame: Frame, index: number, out: PersonRecord): PersonRecord {
  const offset = index * PERSON_BYTES;
  out.x = frame.data.getUint16(offset, true) / SUBCELL;
  out.y = frame.data.getUint16(offset + 2, true) / SUBCELL;
  out.floor = frame.data.getInt8(offset + 4);
  out.heading = frame.data.getUint8(offset + 5);
  return out;
}
