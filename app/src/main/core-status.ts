/**
 * Status of the simulation core process as seen by the Electron main process.
 *
 * The utility process (core-host.ts) reports once after start-up whether the
 * native bridge module could be loaded, together with the found data packs,
 * and answers requests to change the active packs (step 2.4) and to save,
 * load and read save games (step 2.5); it also says when an autosave is
 * due. Messages cross a process boundary and are therefore validated
 * instead of trusted.
 */
import type { PackReport } from "../generated/PackReport.js";
import type { Reason } from "../generated/Reason.js";
import type { SaveKind } from "../generated/SaveKind.js";
import { parseActiveIds, parsePackReports } from "./packs.js";
import { parseReason } from "./platform-error.js";

/** Identification of the loaded core (mirrors `CoreInfo` in bridge/src/lib.rs). */
export interface CoreInfo {
  readonly name: string;
  readonly version: string;
}

/** Message of the core host as validated by the main process. */
export type CoreHostMessage =
  | { readonly type: "ready"; readonly info: CoreInfo; readonly packs: PackReport[] }
  | { readonly type: "error"; readonly message: string }
  | { readonly type: "packs"; readonly request: number; readonly packs: PackReport[] }
  | { readonly type: "packsError"; readonly request: number; readonly message: string }
  | SavesAnswer
  | { readonly type: "autosave" };

/** Answer of the core host to a {@link SavesRequest}; `value` is checked by the caller. */
export type SavesAnswer =
  | {
      readonly type: "savesAnswer";
      readonly request: number;
      readonly ok: true;
      readonly value: unknown;
    }
  | {
      readonly type: "savesAnswer";
      readonly request: number;
      readonly ok: false;
      readonly error: Reason;
    };

/** One file for the save game list: its start and its full length. */
export interface InfoFile {
  readonly prefix: Uint8Array;
  readonly size: number;
}

/** Request of the main process to the core host for save games (S-6 `saves.*`). */
export type SavesRequest =
  | {
      readonly type: "saves:save";
      readonly request: number;
      readonly name: string;
      readonly kind: SaveKind;
      readonly savedAt: string;
      readonly preview: Uint8Array;
    }
  | { readonly type: "saves:load"; readonly request: number; readonly bytes: Uint8Array }
  | { readonly type: "saves:info"; readonly request: number; readonly files: readonly InfoFile[] }
  | { readonly type: "saves:checkPreview"; readonly request: number; readonly png: Uint8Array };

/** Message as posted by the core host (reports straight from the bridge). */
export type CoreHostOutgoing =
  | { readonly type: "ready"; readonly info: CoreInfo; readonly packs: unknown }
  | { readonly type: "error"; readonly message: string }
  | { readonly type: "packs"; readonly request: number; readonly packs: unknown }
  | { readonly type: "packsError"; readonly request: number; readonly message: string }
  | SavesAnswer
  | { readonly type: "autosave" };

/**
 * Request of the main process to the core host (S-6 `packs.setActive`).
 * `request` numbers the requests, so that a late answer to an earlier one
 * (after the time limit) is not taken for the current one.
 */
export interface PackRequest {
  readonly type: "packs:setActive";
  readonly request: number;
  readonly ids: readonly string[];
}

/** Lifecycle state of the core process. */
export type CoreStatus =
  | { readonly state: "starting" }
  | { readonly state: "ready"; readonly info: CoreInfo }
  | { readonly state: "failed"; readonly reason: string };

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isCoreInfo(value: unknown): value is CoreInfo {
  return (
    isRecord(value) &&
    typeof value.name === "string" &&
    value.name.length > 0 &&
    typeof value.version === "string" &&
    value.version.length > 0
  );
}

/**
 * Validates a message from the core host.
 *
 * @returns the typed message, or `undefined` if the shape is unknown.
 */
export function parseCoreHostMessage(value: unknown): CoreHostMessage | undefined {
  if (!isRecord(value)) {
    return undefined;
  }
  if (value.type === "ready" && isCoreInfo(value.info)) {
    const packs = parsePackReports(value.packs);
    if (packs === undefined) return undefined;
    return { type: "ready", info: { name: value.info.name, version: value.info.version }, packs };
  }
  if (value.type === "packs" && isRequestNumber(value.request)) {
    const packs = parsePackReports(value.packs);
    return packs === undefined ? undefined : { type: "packs", request: value.request, packs };
  }
  if (
    value.type === "packsError" &&
    isRequestNumber(value.request) &&
    typeof value.message === "string"
  ) {
    return { type: "packsError", request: value.request, message: value.message };
  }
  if (value.type === "error" && typeof value.message === "string") {
    return { type: "error", message: value.message };
  }
  if (value.type === "autosave") {
    return { type: "autosave" };
  }
  if (value.type === "savesAnswer" && isRequestNumber(value.request)) {
    if (value.ok === true) {
      return { type: "savesAnswer", request: value.request, ok: true, value: value.value };
    }
    const error = parseReason(value.error);
    if (value.ok === false && error !== undefined) {
      return { type: "savesAnswer", request: value.request, ok: false, error };
    }
  }
  return undefined;
}

function isBytes(value: unknown): value is Uint8Array {
  return value instanceof Uint8Array;
}

/** Most files in one `saves:info` request. */
export const MAX_INFO_FILES = 1000;

/** Validates a save game request of the main process in the core host. */
export function parseSavesRequest(value: unknown): SavesRequest | undefined {
  if (!isRecord(value) || !isRequestNumber(value.request)) return undefined;
  const request = value.request;
  switch (value.type) {
    case "saves:save":
      return typeof value.name === "string" &&
        (value.kind === "manual" || value.kind === "quick" || value.kind === "auto") &&
        typeof value.savedAt === "string" &&
        isBytes(value.preview)
        ? {
            type: "saves:save",
            request,
            name: value.name,
            kind: value.kind,
            savedAt: value.savedAt,
            preview: value.preview,
          }
        : undefined;
    case "saves:load":
      return isBytes(value.bytes) ? { type: "saves:load", request, bytes: value.bytes } : undefined;
    case "saves:checkPreview":
      return isBytes(value.png)
        ? { type: "saves:checkPreview", request, png: value.png }
        : undefined;
    case "saves:info": {
      if (!Array.isArray(value.files) || value.files.length > MAX_INFO_FILES) return undefined;
      const files: InfoFile[] = [];
      for (const file of value.files as unknown[]) {
        if (!isRecord(file) || !isBytes(file.prefix) || !isRequestNumber(file.size)) {
          return undefined;
        }
        files.push({ prefix: file.prefix, size: file.size });
      }
      return { type: "saves:info", request, files };
    }
    default:
      return undefined;
  }
}

function isRequestNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}

/** Whether `value` claims to answer a pack request, valid or not. */
export function isPackAnswer(value: unknown): boolean {
  return isRecord(value) && (value.type === "packs" || value.type === "packsError");
}

/** Whether `value` claims to answer a save game request, valid or not. */
export function isSavesAnswer(value: unknown): boolean {
  return isRecord(value) && value.type === "savesAnswer";
}

/** Validates a request of the main process in the core host. */
export function parsePackRequest(
  value: unknown,
): { readonly request: number; readonly ids: string[] } | undefined {
  if (!isRecord(value) || value.type !== "packs:setActive" || !isRequestNumber(value.request)) {
    return undefined;
  }
  const ids = parseActiveIds(value.ids);
  return ids === undefined ? undefined : { request: value.request, ids };
}

/**
 * Computes the next core status after a raw message from the core host.
 * Unknown messages mark the core as failed, because the main process cannot
 * rely on a core that speaks an unexpected protocol.
 */
export function nextCoreStatus(raw: unknown): CoreStatus {
  const message = parseCoreHostMessage(raw);
  if (message === undefined || (message.type !== "ready" && message.type !== "error")) {
    return { state: "failed", reason: "unexpected message from core host" };
  }
  if (message.type === "ready") {
    return { state: "ready", info: message.info };
  }
  return { state: "failed", reason: message.message };
}

/** Status after the core process exited with the given code. */
export function exitedCoreStatus(previous: CoreStatus, code: number): CoreStatus {
  if (previous.state === "failed") {
    return previous;
  }
  return { state: "failed", reason: `core process exited with code ${String(code)}` };
}

/** Most restarts of the core process within {@link RESTART_WINDOW_MS} (S-6). */
export const MAX_RESTARTS = 3;
/** Window of the restart limit in milliseconds. */
export const RESTART_WINDOW_MS = 60_000;

/**
 * Decides whether the core process may be restarted after an exit at
 * `now`, given the times of earlier restarts (S-6 "Absturz des Kerns").
 * Returns the restart times to keep, including `now` if allowed.
 */
export function restartDecision(
  restarts: readonly number[],
  now: number,
): { readonly allowed: boolean; readonly restarts: readonly number[] } {
  const recent = restarts.filter((t) => now - t < RESTART_WINDOW_MS);
  if (recent.length >= MAX_RESTARTS) {
    return { allowed: false, restarts: recent };
  }
  return { allowed: true, restarts: [...recent, now] };
}
