/**
 * Entry point of the Electron utility process that hosts the simulation core
 * (ADR-009). Loads the native bridge module and reports the result to the main
 * process. The tick loop and the state buffer follow in roadmap steps 2.2 and 2.7.
 */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import type { CoreHostMessage, CoreInfo } from "./core-status.js";

interface BridgeModule {
  coreInfo(): CoreInfo;
}

const nativeModulePath = fileURLToPath(
  new URL("../../native/openairside-bridge.node", import.meta.url),
);

function post(message: CoreHostMessage): void {
  process.parentPort.postMessage(message);
}

try {
  const require = createRequire(import.meta.url);
  const bridge = require(nativeModulePath) as BridgeModule;
  post({ type: "ready", info: bridge.coreInfo() });
} catch (error) {
  post({ type: "error", message: error instanceof Error ? error.message : String(error) });
  process.exitCode = 1;
}
