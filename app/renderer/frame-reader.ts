/**
 * Reader of the frame buffer S-1 (layout version 2, core/src/frame.rs).
 * Checks every length against `byteLength` before it creates views, because
 * the buffer crosses a process boundary (ADR-021). The blocks of the world
 * section are read by `world-reader.ts`.
 */

/** Layout version this reader understands. */
export const LAYOUT_VERSION = 2;
/** Header size of layout version 2. */
export const HEADER_BYTES = 48;
/** Bytes per person. */
export const PERSON_BYTES = 8;
/** Bytes per vehicle (section empty until step 3.4). */
export const VEHICLE_BYTES = 12;
/** Bytes per aircraft (section empty until step 3.4). */
export const AIRCRAFT_BYTES = 16;
/** Bytes per baggage item (section empty until step 3.5). */
export const BAGGAGE_BYTES = 8;
/** Bytes of the header of the world section (core/src/display.rs). */
export const WORLD_HEADER_BYTES = 16;
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
  /** World revision of the frame, 0 if it carries no block; acknowledged with the next request. */
  readonly worldRevision: number;
  readonly vehicles: number;
  readonly aircraft: number;
  readonly baggage: number;
}

/** The world section of a frame: how many blocks it carries and where the records start. */
export interface WorldSection {
  /** Blocks in this frame. */
  readonly blocks: number;
  /** Changed blocks that did not fit and follow in later frames. */
  readonly pending: number;
  /** Bytes of the section including its header. */
  readonly bytes: number;
  /** The block records, after the section header. */
  readonly records: DataView;
}

/** A checked frame: header plus the person records and the world section. */
export interface Frame {
  readonly header: FrameHeader;
  /** Person records, `PERSON_BYTES` each. */
  readonly data: DataView;
  readonly world: WorldSection;
}

/** Why a frame was rejected. */
export type FrameError =
  "tooShort" | "layoutVersion" | "headerSize" | "personSize" | "length" | "worldSection";

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
  const vehicles = view.getUint32(36, true);
  const aircraft = view.getUint32(40, true);
  const baggage = view.getUint32(44, true);
  const movingBytes =
    persons * PERSON_BYTES +
    vehicles * VEHICLE_BYTES +
    aircraft * AIRCRAFT_BYTES +
    baggage * BAGGAGE_BYTES;
  const worldAt = HEADER_BYTES + movingBytes;
  if (!Number.isSafeInteger(worldAt) || buf.byteLength < worldAt + WORLD_HEADER_BYTES) {
    return { ok: false, error: "length" };
  }
  const section = new DataView(buf, worldAt);
  const bytes = section.getUint32(0, true);
  if (bytes !== section.byteLength || bytes < WORLD_HEADER_BYTES) {
    return { ok: false, error: "worldSection" };
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
        worldRevision: view.getUint32(32, true),
        vehicles,
        aircraft,
        baggage,
      },
      data: new DataView(buf, HEADER_BYTES, persons * PERSON_BYTES),
      world: {
        blocks: section.getUint32(4, true),
        pending: section.getUint32(8, true),
        bytes,
        records: new DataView(buf, worldAt + WORLD_HEADER_BYTES, bytes - WORLD_HEADER_BYTES),
      },
    },
  };
}

/** One person of a frame, position in cells. */
export interface PersonRecord {
  x: number;
  y: number;
  floor: number;
  heading: number;
  /** Group (ADR-037); 0 until step 3.2. */
  group: number;
  /** State; 0 until step 3.2. */
  state: number;
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
  out.group = frame.data.getUint8(offset + 6);
  out.state = frame.data.getUint8(offset + 7);
  return out;
}
