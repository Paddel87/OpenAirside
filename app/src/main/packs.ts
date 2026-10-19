/**
 * Data packs in the main process (S-4, S-6 `packs.*`; step 2.4): checking
 * the arguments of the renderer and the reports of the core process, the
 * file with the player's chosen packs and the start flags of the core.
 *
 * The core process loads and checks the packs itself; the main process only
 * passes folders and ids and keeps the latest reports for `packs.list()`.
 */
import type { PackError } from "../generated/PackError.js";
import type { PackReport } from "../generated/PackReport.js";
import { parseReason } from "./platform-error.js";

/** Most packs, base pack included (S-4, `MAX_PACKS` in core/src/datapack/mod.rs). */
export const MAX_PACKS = 256;
/** Name of the file with the chosen packs in the user data folder. */
export const ACTIVE_FILE = "packs-active.json";
/** Name of the folder with the player's packs in the user data folder. */
export const USER_PACKS_DIR = "packs";

const PACK_ID = /^[a-z0-9_-]{3,64}$/;

/** Checks the naming rule of a pack id (as `is_pack_id` in the core). */
export function isPackId(value: unknown): value is string {
  return typeof value === "string" && PACK_ID.test(value);
}

/**
 * Checks the argument of `packs.setActive`: a list of at most
 * {@link MAX_PACKS} valid, distinct pack ids.
 *
 * @returns the ids, or `undefined` if the argument is invalid.
 */
export function parseActiveIds(value: unknown): string[] | undefined {
  if (!Array.isArray(value) || value.length > MAX_PACKS) {
    return undefined;
  }
  const ids: string[] = [];
  for (const id of value) {
    if (!isPackId(id) || ids.includes(id)) {
      return undefined;
    }
    ids.push(id);
  }
  return ids;
}

/** Ids from the content of {@link ACTIVE_FILE}; anything unreadable is no choice. */
export function readActiveFile(text: string | undefined): string[] {
  if (text === undefined) {
    return [];
  }
  try {
    const parsed: unknown = JSON.parse(text);
    if (typeof parsed === "object" && parsed !== null && "active" in parsed) {
      return parseActiveIds(parsed.active) ?? [];
    }
  } catch {
    // An unreadable file loads only the base pack.
  }
  return [];
}

/** Content of {@link ACTIVE_FILE} for `ids`. */
export function activeFileText(ids: readonly string[]): string {
  return `${JSON.stringify({ active: ids }, null, 2)}\n`;
}

/** Start flags of the core process for the pack folders and the chosen ids. */
export function packArgs(basePack: string, userPacks: string, active: readonly string[]): string[] {
  return [
    `--base-pack=${basePack}`,
    `--user-packs=${userPacks}`,
    `--active-packs=${active.join(",")}`,
  ];
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function optionalString(value: unknown): value is string | null {
  return value === null || typeof value === "string";
}

function optionalCount(value: unknown): value is number | null {
  return value === null || (typeof value === "number" && Number.isInteger(value) && value >= 0);
}

function isCount(value: unknown): value is number {
  return typeof value === "number" && Number.isInteger(value) && value >= 0;
}

function parseError(value: unknown): PackError | undefined {
  if (
    !isRecord(value) ||
    !optionalString(value.file) ||
    !optionalCount(value.line) ||
    !optionalCount(value.column) ||
    !optionalString(value.path)
  ) {
    return undefined;
  }
  const reason = parseReason(value.reason);
  if (reason === undefined) return undefined;
  return { file: value.file, line: value.line, column: value.column, path: value.path, reason };
}

function parseReport(value: unknown): PackReport | undefined {
  if (
    !isRecord(value) ||
    typeof value.folder !== "string" ||
    (value.source !== "base" && value.source !== "user") ||
    !optionalString(value.id) ||
    !optionalString(value.name) ||
    !optionalString(value.version) ||
    (value.state !== "active" && value.state !== "inactive" && value.state !== "failed") ||
    !isCount(value.entries) ||
    !Array.isArray(value.errors) ||
    !isCount(value.errorsOmitted)
  ) {
    return undefined;
  }
  const errors: PackError[] = [];
  for (const raw of value.errors) {
    const parsed = parseError(raw);
    if (parsed === undefined) return undefined;
    errors.push(parsed);
  }
  return {
    folder: value.folder,
    source: value.source,
    id: value.id,
    name: value.name,
    version: value.version,
    state: value.state,
    entries: value.entries,
    errors,
    errorsOmitted: value.errorsOmitted,
  };
}

/**
 * Checks the pack reports sent by the core process (they cross a process
 * boundary and are validated instead of trusted).
 *
 * @returns the reports, or `undefined` if one has an unexpected shape.
 */
export function parsePackReports(value: unknown): PackReport[] | undefined {
  if (!Array.isArray(value) || value.length > MAX_PACKS + 1) {
    return undefined;
  }
  const reports: PackReport[] = [];
  for (const raw of value) {
    const report = parseReport(raw);
    if (report === undefined) return undefined;
    reports.push(report);
  }
  return reports;
}

/** Ids that `packs.setActive` may choose: user packs found with a valid id. */
export function choosableIds(reports: readonly PackReport[]): Set<string> {
  return new Set(reports.flatMap((r) => (r.source === "user" && r.id !== null ? [r.id] : [])));
}
