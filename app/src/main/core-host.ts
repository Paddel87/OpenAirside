/**
 * Entry point of the Electron utility process that hosts the simulation core
 * (ADR-009). Loads the native bridge, starts the simulation with the options
 * from the command line and connects each `MessagePort` handed over by the
 * main process to the core (S-1, S-2, S-3; ADR-021). Reports the data packs
 * (S-4) and changes the active packs on request of the main process (S-6
 * `packs.*`, step 2.4).
 */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import type { MessagePortMain } from "electron";
import { type CoreHostOutgoing, type CoreInfo, parsePackRequest } from "./core-status.js";
import {
  type BridgeOutgoing,
  type CoreOptions,
  parseCoreArgs,
  toPortMessage,
} from "./core-port.js";

interface NativeCore {
  openPort(): void;
  receive(message: unknown): BridgeOutgoing[];
  pump(): BridgeOutgoing[];
  packs(): unknown;
  setActivePacks(ids: string[]): unknown;
}

interface BridgeModule {
  coreInfo(): CoreInfo;
  Core: new (options: CoreOptions) => NativeCore;
}

/** Interval of `pump`: session changes and events while no frame is requested. */
const PUMP_MS = 50;

const nativeModulePath = fileURLToPath(
  new URL("../../native/openairside-bridge.node", import.meta.url),
);

function post(message: CoreHostOutgoing): void {
  process.parentPort.postMessage(message);
}

function send(port: MessagePortMain, out: readonly BridgeOutgoing[]): void {
  for (const answer of out) {
    port.postMessage(toPortMessage(answer));
  }
}

try {
  const require = createRequire(import.meta.url);
  const bridge = require(nativeModulePath) as BridgeModule;
  const core = new bridge.Core(parseCoreArgs(process.argv));
  let current: MessagePortMain | undefined;

  process.parentPort.on("message", (event) => {
    const request = parsePackRequest(event.data);
    if (request !== undefined) {
      try {
        post({ type: "packs", packs: core.setActivePacks(request.ids) });
      } catch (error) {
        post({
          type: "packsError",
          message: error instanceof Error ? error.message : String(error),
        });
      }
      return;
    }
    const [port] = event.ports;
    if (port === undefined) {
      return;
    }
    current?.close();
    core.openPort();
    current = port;
    port.on("message", (message) => {
      send(port, core.receive(message.data));
    });
    port.start();
  });
  setInterval(() => {
    if (current !== undefined) {
      send(current, core.pump());
    }
  }, PUMP_MS);

  post({ type: "ready", info: bridge.coreInfo(), packs: core.packs() });
} catch (error) {
  post({ type: "error", message: error instanceof Error ? error.message : String(error) });
  process.exitCode = 1;
}
