/**
 * Helpers of the core process (utility process, ADR-009) that need no
 * Electron: start options from the command line and the conversion of the
 * bridge's answers into port messages (S-1, S-3).
 */

import type { Scenario } from "../generated/Scenario.js";

/** Start options of the native core (mirrors `CoreOptions` in bridge/src/lib.rs). */
export interface CoreOptions {
  mapSize?: number;
  threads?: number;
  backgroundThreads?: number;
  scenario?: Scenario;
  persons?: number;
}

/** Answer of the bridge (mirrors `OutgoingMessage` in bridge/src/lib.rs). */
export interface BridgeOutgoing {
  readonly message: Record<string, unknown>;
  readonly buf?: Uint8Array | null;
}

const SCENARIOS: ReadonlySet<string> = new Set<Scenario>(["openHall", "referenceTerminal"]);

function count(value: string, min: number, max: number): number | undefined {
  if (!/^\d{1,7}$/.test(value)) {
    return undefined;
  }
  const n = Number(value);
  return n >= min && n <= max ? n : undefined;
}

/**
 * Reads the core options from command-line flags such as
 * `--scenario=referenceTerminal --persons=10000 --threads=2`. Unknown or
 * invalid flags are ignored, so that a typo never stops the game.
 */
export function parseCoreArgs(args: readonly string[]): CoreOptions {
  const options: CoreOptions = {};
  for (const arg of args) {
    const match = /^--(map-size|threads|background-threads|scenario|persons)=(.*)$/.exec(arg);
    if (match === null) {
      continue;
    }
    const [, name, value = ""] = match;
    switch (name) {
      case "map-size": {
        const n = count(value, 1536, 2048);
        if (n === 1536 || n === 2048) options.mapSize = n;
        break;
      }
      case "threads": {
        const n = count(value, 1, 256);
        if (n !== undefined) options.threads = n;
        break;
      }
      case "background-threads": {
        const n = count(value, 1, 256);
        if (n !== undefined) options.backgroundThreads = n;
        break;
      }
      case "persons": {
        const n = count(value, 0, 200_000);
        if (n !== undefined) options.persons = n;
        break;
      }
      case "scenario":
        if (SCENARIOS.has(value)) options.scenario = value as Scenario;
        break;
    }
  }
  return options;
}

/** Message as posted on the port, with the frame buffer as own `ArrayBuffer`. */
export type PortMessage = Record<string, unknown>;

/**
 * Converts an answer of the bridge into a port message. The frame buffer is
 * copied into an `ArrayBuffer` of exactly its length (the native buffer may
 * be larger or shared).
 */
export function toPortMessage(out: BridgeOutgoing): PortMessage {
  if (out.buf === undefined || out.buf === null) {
    return out.message;
  }
  const buf = out.buf.buffer.slice(out.buf.byteOffset, out.buf.byteOffset + out.buf.byteLength);
  return { ...out.message, buf };
}
