/**
 * Status of the simulation core process as seen by the Electron main process.
 *
 * The utility process (core-host.ts) reports once after start-up whether the
 * native bridge module could be loaded. Messages cross a process boundary and
 * are therefore validated instead of trusted.
 */

/** Identification of the loaded core (mirrors `CoreInfo` in bridge/src/lib.rs). */
export interface CoreInfo {
  readonly name: string;
  readonly version: string;
}

/** Message sent by the core host after loading the native module. */
export type CoreHostMessage =
  | { readonly type: "ready"; readonly info: CoreInfo }
  | { readonly type: "error"; readonly message: string };

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
    return { type: "ready", info: { name: value.info.name, version: value.info.version } };
  }
  if (value.type === "error" && typeof value.message === "string") {
    return { type: "error", message: value.message };
  }
  return undefined;
}

/**
 * Computes the next core status after a raw message from the core host.
 * Unknown messages mark the core as failed, because the main process cannot
 * rely on a core that speaks an unexpected protocol.
 */
export function nextCoreStatus(raw: unknown): CoreStatus {
  const message = parseCoreHostMessage(raw);
  if (message === undefined) {
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
