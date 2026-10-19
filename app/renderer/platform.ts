/**
 * Renderer side of the platform API S-6 (ADR-021): the object
 * `window.openairside` of the preload and the arrival of the port to the
 * core, which cannot cross the context bridge as argument and comes as a
 * window message (app/src/main/preload.cts).
 */

import type { PackReport } from "../src/generated/PackReport.js";
import type { Reason } from "../src/generated/Reason.js";
import type { SaveKind } from "../src/generated/SaveKind.js";
import type { SaveMeta } from "../src/generated/SaveMeta.js";

/** Tag of the window message with the port (same as in preload.cts). */
export const CORE_PORT_TAG = "openairside:core-port";

/** Answer of `app.info()`. */
export interface AppInfo {
  readonly version: string;
  readonly platform: string;
  readonly dataFolder: string;
}

/** Entry of `saves.list()`: a readable save game or a damaged file (S-5). */
export type SaveEntry =
  | {
      readonly id: string;
      readonly state: "ok";
      readonly gameVersion: string;
      readonly meta: SaveMeta;
    }
  | { readonly id: string; readonly state: "damaged"; readonly error: Reason };

/** Level of a log entry (`logs.write`). */
export type LogLevel = "debug" | "info" | "warn" | "error";

/**
 * Whether `value` has the error form of S-6 (`{ code, textKey, params }`);
 * every refused call rejects with it.
 */
export function isPlatformError(value: unknown): value is Reason {
  return (
    typeof value === "object" &&
    value !== null &&
    "code" in value &&
    typeof value.code === "string" &&
    "textKey" in value &&
    typeof value.textKey === "string" &&
    "params" in value &&
    typeof value.params === "object" &&
    value.params !== null
  );
}

/**
 * Short text of a refused call for the console: the code with its
 * parameters (texts follow with the localisation in step 2.6).
 */
export function describeError(error: unknown): string {
  if (isPlatformError(error)) {
    const params = Object.entries(error.params).map(([k, v]) => `${k}=${String(v)}`);
    return params.length === 0 ? error.code : `${error.code} (${params.join(", ")})`;
  }
  return error instanceof Error ? error.message : String(error);
}

/** The platform API as exposed by the preload. */
export interface OpenAirsideApi {
  readonly apiVersion: number;
  readonly app: {
    info(): Promise<AppInfo>;
    quit(): Promise<null>;
  };
  /** Measurement tool of step 2.7: memory sampling and the result file. */
  readonly measure: {
    /** Starts sampling the memory of all processes; rejects while a run is open. */
    start(): Promise<null>;
    /** Writes the result with system data and memory; returns the file path. */
    report(result: unknown): Promise<string>;
  };
  /** Data packs (step 2.4): found packs and the choice of the active ones. */
  readonly packs: {
    /** Found packs with state and errors; the choice for the next session. */
    list(): Promise<PackReport[]>;
    /** Chooses the active user packs in load order; applies from the next session on. */
    setActive(ids: readonly string[]): Promise<PackReport[]>;
  };
  /** Save games (S-5, step 2.5); ids are given out by the main process. */
  readonly saves: {
    /** All save games, newest first; damaged files with their error. */
    list(): Promise<SaveEntry[]>;
    /** Saves the running game; resolves with the id of the new file. */
    save(request: { readonly name: string; readonly kind: SaveKind }): Promise<{ id: string }>;
    /** Loads a save game; a new session starts. */
    load(id: string): Promise<SaveMeta>;
    delete(id: string): Promise<null>;
    /** PNG bytes of the preview (empty if none, ADR-028); list first. */
    preview(id: string): Promise<ArrayBuffer>;
  };
  /** Log file (step 2.5). */
  readonly logs: {
    /** Throttled entry: at most 100 per second, 2,000 characters. */
    write(level: LogLevel, message: string): Promise<null>;
    openFolder(): Promise<null>;
  };
  /** The core process ended; `restarting` if the platform starts a new one (ADR-029). */
  onCoreExit(callback: (code: number, restarting: boolean) => void): void;
}

declare global {
  interface Window {
    readonly openairside?: OpenAirsideApi;
  }
}

/** The part of `window` needed to receive the port. */
export interface PortSource {
  addEventListener(type: "message", listener: (event: MessageEvent) => void): void;
}

/**
 * Calls `callback` with every port the main process hands over (once per
 * page load, S-6 "Port-Übergabe"). Only messages of this window with the
 * tag and a port count.
 */
export function onCorePort(
  target: PortSource,
  self: unknown,
  callback: (port: MessagePort) => void,
): void {
  target.addEventListener("message", (event) => {
    const [port] = event.ports;
    if (event.source === self && event.data === CORE_PORT_TAG && port !== undefined) {
      callback(port);
    }
  });
}
