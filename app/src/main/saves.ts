/**
 * Save games in the main process (S-5, S-6 `saves.*`; step 2.5): ids and
 * file names, checking the arguments of the renderer and the answers of
 * the core process, the autosave rotation and the autosave clock.
 *
 * The core builds and checks the files (`core/src/savegame/`); the main
 * process chooses ids and file names, writes, reads and deletes files.
 * File names come from ids the main process gives out, never from the
 * name of a save game (S-6).
 */
import type { MapSize } from "../generated/MapSize.js";
import type { Reason } from "../generated/Reason.js";
import type { SaveKind } from "../generated/SaveKind.js";
import type { SaveMeta } from "../generated/SaveMeta.js";
import type { SavedPack } from "../generated/SavedPack.js";
import { parseReason } from "./platform-error.js";

/** Folder of the save games in the user data folder. */
export const SAVES_DIR = "saves";
/** File name extension (as `EXTENSION` in core/src/savegame/mod.rs). */
export const SAVE_EXTENSION = ".oasave";
/** Suffix of a file being written; renamed when complete (S-5 "Schreiben"). */
export const TEMP_SUFFIX = ".tmp";
/** Largest save game file (as `MAX_FILE_BYTES` in the core, 512 MB, ADR-030). */
export const MAX_FILE_BYTES = 2 ** 29;
/** Bytes of a file read for the list (as `MAX_INFO_BYTES` in the core). */
export const MAX_INFO_BYTES = 12 + 1 + 64 + 16 + 64 * 1024;
/** Largest preview image (as `MAX_PREVIEW_BYTES` in the core). */
export const MAX_PREVIEW_BYTES = 2 * 1024 * 1024;
/** Longest save game name (as `MAX_NAME_CHARS` in the core). */
export const MAX_NAME_CHARS = 64;
/** Width of the preview image in pixels (step 2.5, point h). */
export const PREVIEW_WIDTH = 480;
/** Files sent to the core process in one `saves:info` request. */
export const INFO_BATCH = 100;
/** Largest preview image (as `MAX_PREVIEW_WIDTH`/`HEIGHT` in the core). */
export const MAX_PREVIEW_SIZE = { width: 1920, height: 1080 } as const;

/**
 * Size of the preview for a window of `size`: at most
 * {@link PREVIEW_WIDTH} wide and within {@link MAX_PREVIEW_SIZE}, aspect
 * kept; `undefined` for a degenerate window (finding 8 of the separate
 * review: a tall narrow window gave an image the core refuses).
 */
export function previewSize(size: {
  readonly width: number;
  readonly height: number;
}): { width: number; height: number } | undefined {
  if (size.width <= 0 || size.height <= 0) return undefined;
  const scale = Math.min(
    1,
    PREVIEW_WIDTH / size.width,
    MAX_PREVIEW_SIZE.width / size.width,
    MAX_PREVIEW_SIZE.height / size.height,
  );
  const width = Math.floor(size.width * scale);
  const height = Math.floor(size.height * scale);
  return width > 0 && height > 0 ? { width, height } : undefined;
}

/** Autosave files in rotation before the settings are read (setting `autosaveCount`, 1–20). */
export const AUTOSAVE_SLOTS = 5;
/** Running game time between two autosaves before the settings are read (setting `autosaveMinutes`). */
export const AUTOSAVE_INTERVAL_MS = 10 * 60 * 1000;
/** Id of the single quick save. */
export const QUICK_ID = "quick";
/** Most manual save games; more are refused with `limitExceeded`. */
export const MAX_MANUAL_SAVES = 200;
/** Most files read for one list; the newest by modification time count. */
export const MAX_LISTED_FILES = 1000;

const SAVE_ID = /^(?:quick|auto-(?:[1-9]|1\d|20)|m-[0-9a-f]{16})$/;
const KINDS: ReadonlySet<string> = new Set<SaveKind>(["manual", "quick", "auto"]);

/** Entry of `saves.list()`: a readable save game or a damaged file. */
export type SaveEntry =
  | {
      readonly id: string;
      readonly state: "ok";
      readonly gameVersion: string;
      readonly meta: SaveMeta;
    }
  | { readonly id: string; readonly state: "damaged"; readonly error: Reason };

/** What the core reads of a file for the list (`saveInfo` in bridge/src/lib.rs). */
export interface SaveInfo {
  readonly gameVersion: string;
  readonly meta: SaveMeta;
  readonly previewOffset: number;
  readonly previewLen: number;
}

/** Checks the form of a save game id; whether it exists the caller checks. */
export function isSaveId(value: unknown): value is string {
  return typeof value === "string" && SAVE_ID.test(value);
}

/** File name of the save game `id`. */
export function saveFileName(id: string): string {
  return `${id}${SAVE_EXTENSION}`;
}

/** Id of a save game file name, or `undefined` for other files. */
export function idOfFileName(name: string): string | undefined {
  if (!name.endsWith(SAVE_EXTENSION)) return undefined;
  const id = name.slice(0, -SAVE_EXTENSION.length);
  return isSaveId(id) ? id : undefined;
}

/** Whether `name` is a file left over by an interrupted save. */
export function isTempFileName(name: string): boolean {
  return /\.oasave(?:\.[0-9a-f]{16})?\.tmp$/.test(name);
}

/**
 * Temporary file name of a save in progress: random, so that no file
 * placed in advance under a predictable name is written through.
 *
 * @param random 8 random bytes.
 */
export function tempFileName(id: string, random: Uint8Array): string {
  return `${saveFileName(id)}.${hex(random)}${TEMP_SUFFIX}`;
}

function hex(random: Uint8Array): string {
  return [...random.subarray(0, 8)].map((b) => b.toString(16).padStart(2, "0")).join("");
}

/**
 * Whether `text` may have at most `max` characters: a cheap upper bound
 * before counting code points (at most two UTF-16 units each), so that a
 * huge string from the renderer is refused without walking through it.
 */
export function withinChars(text: string, max: number): boolean {
  return text.length <= max || (text.length <= 2 * max && Array.from(text).length <= max);
}

/** Checks a name as the core does: at most 64 characters, no control characters. */
export function isSaveName(value: unknown): value is string {
  if (typeof value !== "string" || !withinChars(value, MAX_NAME_CHARS)) return false;
  return !Array.from(value).some(isControl);
}

/** Unicode category Cc, as `char::is_control` in Rust. */
function isControl(char: string): boolean {
  const code = char.codePointAt(0) ?? 0;
  return code <= 0x1f || (code >= 0x7f && code <= 0x9f);
}

/**
 * Checks the argument of `saves.save({ name, kind })`.
 *
 * @returns name and kind, or `undefined` if the argument is invalid.
 */
export function parseSaveArgs(value: unknown): { name: string; kind: SaveKind } | undefined {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return undefined;
  const { name, kind } = value as Record<string, unknown>;
  if (!isSaveName(name) || typeof kind !== "string" || !KINDS.has(kind)) return undefined;
  return { name, kind: kind as SaveKind };
}

/** Map edges of a new game in cells (S-6 `game.new`, core `savegame::MAP_EDGES`). */
export const MAP_EDGES: readonly number[] = [1536, 2048];

/**
 * Checks the argument of `game.new({ map })` (ADR-032, L-3): exactly the
 * field `map`, one of {@link MAP_EDGES}.
 *
 * @returns the edge, or `undefined` if the argument is invalid.
 */
export function parseNewGameArgs(value: unknown): { map: number } | undefined {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return undefined;
  const keys = Object.keys(value);
  const { map } = value as Record<string, unknown>;
  if (keys.length !== 1 || typeof map !== "number" || !MAP_EDGES.includes(map)) return undefined;
  return { map };
}

/**
 * Id for a new save game of `kind`: a fresh random id for a manual save,
 * the single slot for a quick save, and for an autosave the first free
 * slot or else the one modified longest ago.
 *
 * @param modified modification times (ms) of the existing files by id.
 * @param random 8 random bytes for a manual id.
 */
export function newSaveId(
  kind: SaveKind,
  modified: ReadonlyMap<string, number>,
  random: Uint8Array,
  slots: number = AUTOSAVE_SLOTS,
): string {
  if (kind === "quick") return QUICK_ID;
  if (kind === "manual") return `m-${hex(random)}`;
  // The modification time decides, not `savedAt` from the file content
  // (finding 11 of the separate review: a placed file could pin a slot).
  let oldest: { id: string; at: number } | undefined;
  for (let slot = 1; slot <= slots; slot += 1) {
    const id = `auto-${String(slot)}`;
    const at = modified.get(id);
    if (at === undefined) return id;
    if (oldest === undefined || at < oldest.at) oldest = { id, at };
  }
  return oldest?.id ?? "auto-1";
}

/** Entries newest first; damaged files last, by id. */
export function sortEntries(entries: readonly SaveEntry[]): SaveEntry[] {
  const at = (e: SaveEntry): string => (e.state === "ok" ? e.meta.savedAt : "");
  return [...entries].sort((a, b) =>
    at(a) === at(b) ? a.id.localeCompare(b.id) : at(a) < at(b) ? 1 : -1,
  );
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isCount(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}

function parseMap(value: unknown): MapSize | undefined {
  if (!isRecord(value) || !isCount(value.width) || !isCount(value.height)) return undefined;
  return { width: value.width, height: value.height };
}

function parsePacks(value: unknown): SavedPack[] | undefined {
  if (!Array.isArray(value)) return undefined;
  const packs: SavedPack[] = [];
  for (const p of value) {
    if (
      !isRecord(p) ||
      typeof p.id !== "string" ||
      typeof p.version !== "string" ||
      !isCount(p.format)
    ) {
      return undefined;
    }
    packs.push({ id: p.id, version: p.version, format: p.format });
  }
  return packs;
}

/**
 * Checks metadata sent by the core process.
 *
 * @returns the metadata, or `undefined` for an unexpected shape.
 */
export function parseSaveMeta(value: unknown): SaveMeta | undefined {
  if (
    !isRecord(value) ||
    typeof value.name !== "string" ||
    typeof value.kind !== "string" ||
    !KINDS.has(value.kind) ||
    typeof value.savedAt !== "string" ||
    !isCount(value.tick) ||
    !isCount(value.day) ||
    !isCount(value.year) ||
    typeof value.airportName !== "string"
  ) {
    return undefined;
  }
  const map = parseMap(value.map);
  const packs = parsePacks(value.packs);
  if (map === undefined || packs === undefined) return undefined;
  return {
    name: value.name,
    kind: value.kind as SaveKind,
    savedAt: value.savedAt,
    tick: value.tick,
    day: value.day,
    year: value.year,
    airportName: value.airportName,
    map,
    packs,
  };
}

/**
 * Checks the answer of the core process for one file of the list:
 * `{ ok: true, info }` or `{ ok: false, error }`.
 */
export function parseInfoAnswer(
  value: unknown,
): { ok: true; info: SaveInfo } | { ok: false; error: Reason } | undefined {
  if (!isRecord(value)) return undefined;
  if (value.ok === false) {
    const error = parseReason(value.error);
    return error === undefined ? undefined : { ok: false, error };
  }
  const info = value.info;
  if (value.ok !== true || !isRecord(info) || typeof info.gameVersion !== "string") {
    return undefined;
  }
  const meta = parseSaveMeta(info.meta);
  if (
    meta === undefined ||
    !isCount(info.previewOffset) ||
    !isCount(info.previewLen) ||
    info.previewLen > MAX_PREVIEW_BYTES
  ) {
    return undefined;
  }
  return {
    ok: true,
    info: {
      gameVersion: info.gameVersion,
      meta,
      previewOffset: info.previewOffset,
      previewLen: info.previewLen,
    },
  };
}

/** Node.js error code of a failed write as S-5 code (`diskFull` or `ioError`). */
export function writeErrorCode(error: unknown): "diskFull" | "ioError" {
  const code =
    typeof error === "object" && error !== null && "code" in error ? error.code : undefined;
  return code === "ENOSPC" || code === "EDQUOT" ? "diskFull" : "ioError";
}

/**
 * Real time of running play towards the next autosave (step 2.5, point b):
 * counts only while the game runs, starts again with every session.
 */
export class AutosaveClock {
  private session = -1;
  private runningMs = 0;
  private last: number | undefined;

  constructor(private intervalMs: number = AUTOSAVE_INTERVAL_MS) {}

  /**
   * Takes a new interval from the settings at once; the running time
   * counted so far stays, so a shorter interval may be due at the next
   * observation.
   */
  setInterval(intervalMs: number): void {
    this.intervalMs = intervalMs;
  }

  /**
   * Observes the core at `now`; returns `true` once when an autosave is due.
   */
  observe(now: number, session: number, running: boolean): boolean {
    if (session !== this.session) {
      this.session = session;
      this.runningMs = 0;
      this.last = undefined;
    }
    if (running && this.last !== undefined) {
      this.runningMs += Math.max(0, now - this.last);
    }
    this.last = now;
    if (this.runningMs >= this.intervalMs) {
      this.runningMs = 0;
      return true;
    }
    return false;
  }
}
