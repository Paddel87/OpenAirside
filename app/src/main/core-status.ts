/**
 * Status of the simulation core process as seen by the Electron main process.
 *
 * The utility process (core-host.ts) reports once after start-up whether the
 * native bridge module could be loaded, together with the found data packs,
 * and answers requests to change the active packs (step 2.4). Messages cross
 * a process boundary and are therefore validated instead of trusted.
 */
import type { PackReport } from "../generated/PackReport.js";
import { parseActiveIds, parsePackReports } from "./packs.js";

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
  | { readonly type: "packsError"; readonly request: number; readonly message: string };

/** Message as posted by the core host (reports straight from the bridge). */
export type CoreHostOutgoing =
  | { readonly type: "ready"; readonly info: CoreInfo; readonly packs: unknown }
  | { readonly type: "error"; readonly message: string }
  | { readonly type: "packs"; readonly request: number; readonly packs: unknown }
  | { readonly type: "packsError"; readonly request: number; readonly message: string };

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
  return undefined;
}

function isRequestNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}

/** Whether `value` claims to answer a pack request, valid or not. */
export function isPackAnswer(value: unknown): boolean {
  return isRecord(value) && (value.type === "packs" || value.type === "packsError");
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
  if (message === undefined || message.type === "packs" || message.type === "packsError") {
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
