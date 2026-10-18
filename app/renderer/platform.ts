/**
 * Renderer side of the platform API S-6 (ADR-021): the object
 * `window.openairside` of the preload and the arrival of the port to the
 * core, which cannot cross the context bridge as argument and comes as a
 * window message (app/src/main/preload.cts).
 */

import type { PackReport } from "../src/generated/PackReport.js";

/** Tag of the window message with the port (same as in preload.cts). */
export const CORE_PORT_TAG = "openairside:core-port";

/** Answer of `app.info()`. */
export interface AppInfo {
  readonly version: string;
  readonly platform: string;
  readonly dataFolder: string;
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
  onCoreExit(callback: (code: number) => void): void;
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
