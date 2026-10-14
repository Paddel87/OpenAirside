/**
 * Entry point of the Electron utility process that hosts the simulation core
 * (ADR-009). Loads the native bridge, starts the simulation with the options
 * from the command line and connects each `MessagePort` handed over by the
 * main process to the core (S-1, S-2, S-3; ADR-021).
 */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import type { MessagePortMain } from "electron";
import type { CoreHostMessage, CoreInfo } from "./core-status.js";
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

function post(message: CoreHostMessage): void {
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

  post({ type: "ready", info: bridge.coreInfo() });
} catch (error) {
  post({ type: "error", message: error instanceof Error ? error.message : String(error) });
  process.exitCode = 1;
}
