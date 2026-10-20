/**
 * Entry point of the Electron utility process that hosts the simulation core
 * (ADR-009). Loads the native bridge, starts the simulation with the options
 * from the command line and connects each `MessagePort` handed over by the
 * main process to the core (S-1, S-2, S-3; ADR-021). Reports the data packs
 * (S-4) and changes the active packs on request of the main process (S-6
 * `packs.*`, step 2.4). Saves, loads and reads save games for the main
 * process and tells it when an autosave is due (S-5, S-6 `saves.*`, step
 * 2.5); encoding and checking run off this thread in the bridge. Hands out
 * the languages and texts of the loaded packs (S-6 `lang.*`, step 2.6,
 * ADR-031) and takes the autosave interval from the settings.
 */
import { createRequire } from "node:module";
import { monitorEventLoopDelay } from "node:perf_hooks";
import { fileURLToPath } from "node:url";
import type { MessagePortMain } from "electron";
import type { Reason } from "../generated/Reason.js";
import {
  type CoreHostOutgoing,
  type CoreInfo,
  parseAutosaveInterval,
  parsePackRequest,
  parseSavesRequest,
  type SavesRequest,
  savesRequestNumber,
} from "./core-status.js";
import {
  type BridgeOutgoing,
  type CoreOptions,
  parseCoreArgs,
  toPortMessage,
} from "./core-port.js";
import { reasonOfBridgeError } from "./platform-error.js";
import { AutosaveClock } from "./saves.js";

interface NativeCore {
  openPort(): void;
  receive(message: unknown): BridgeOutgoing[];
  pump(): BridgeOutgoing[];
  packs(): unknown;
  setActivePacks(ids: string[]): unknown;
  languages(): unknown;
  texts(code: string): unknown;
  status(): { session: number; running: boolean };
  saveGame(name: string, kind: string, savedAt: string, preview: Buffer): Promise<Buffer>;
  loadGame(bytes: Buffer): Promise<string>;
}

interface BridgeModule {
  coreInfo(): CoreInfo;
  saveInfo(prefix: Buffer, fileLen: number): unknown;
  checkPreview(png: Buffer): void;
  Core: new (options: CoreOptions) => NativeCore;
}

/** Interval of the autosave clock. */
const AUTOSAVE_TICK_MS = 1000;

/** Interval of `pump`: session changes and events while no frame is requested. */
const PUMP_MS = 50;

const nativeModulePath = fileURLToPath(
  new URL("../../native/openairside-bridge.node", import.meta.url),
);

function post(message: CoreHostOutgoing): void {
  process.parentPort.postMessage(message);
}

function asBuffer(bytes: Uint8Array): Buffer {
  return Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

/** Answers a save game request; the bridge's errors carry the reason as JSON. */
async function answerSaves(
  bridge: BridgeModule,
  core: NativeCore,
  request: SavesRequest,
): Promise<unknown> {
  switch (request.type) {
    case "saves:save":
      return new Uint8Array(
        await core.saveGame(request.name, request.kind, request.savedAt, asBuffer(request.preview)),
      );
    case "saves:load":
      return JSON.parse(await core.loadGame(asBuffer(request.bytes))) as unknown;
    case "saves:checkPreview":
      bridge.checkPreview(asBuffer(request.png));
      return null;
    case "lang:texts":
      return core.texts(request.code);
    case "saves:info":
      return request.files.map((file) => {
        try {
          return { ok: true, info: bridge.saveInfo(asBuffer(file.prefix), file.size) };
        } catch (error) {
          return { ok: false, error: reasonOfBridgeError(error) };
        }
      });
  }
}

function send(port: MessagePortMain, out: readonly BridgeOutgoing[]): void {
  for (const answer of out) {
    port.postMessage(toPortMessage(answer));
  }
}

// Spike 3.14: event-loop delay of the core process, reported with every frame.
const loopDelay = monitorEventLoopDelay({ resolution: 1 });
loopDelay.enable();

/** Spike 3.14: like `send`, with arrival and sending stamps on frames (ms since the epoch). */
function sendStamped(port: MessagePortMain, out: readonly BridgeOutgoing[], recv: number): void {
  for (const answer of out) {
    const message = toPortMessage(answer) as unknown as Record<string, unknown>;
    if (message.type === "frame") {
      const eld = loopDelay.max / 1e6;
      loopDelay.reset();
      message.stamps = { recv, send: performance.timeOrigin + performance.now(), eld };
    }
    port.postMessage(message);
  }
}

try {
  const require = createRequire(import.meta.url);
  const bridge = require(nativeModulePath) as BridgeModule;
  const core = new bridge.Core(parseCoreArgs(process.argv));
  let current: MessagePortMain | undefined;
  const autosave = new AutosaveClock();

  process.parentPort.on("message", (event) => {
    const saves = parseSavesRequest(event.data);
    if (saves !== undefined) {
      answerSaves(bridge, core, saves).then(
        (value) => {
          post({ type: "savesAnswer", request: saves.request, ok: true, value });
        },
        (error: unknown) => {
          const reason: Reason = reasonOfBridgeError(error);
          post({ type: "savesAnswer", request: saves.request, ok: false, error: reason });
        },
      );
      return;
    }
    const invalid = savesRequestNumber(event.data);
    if (invalid !== undefined) {
      post({
        type: "savesAnswer",
        request: invalid,
        ok: false,
        error: { code: "invalidArgument", textKey: "save.error.invalidArgument", params: {} },
      });
      return;
    }
    const interval = parseAutosaveInterval(event.data);
    if (interval !== undefined) {
      autosave.setInterval(interval.minutes * 60_000);
      return;
    }
    const request = parsePackRequest(event.data);
    if (request !== undefined) {
      try {
        const packs = core.setActivePacks(request.ids);
        post({ type: "packs", request: request.request, packs, languages: core.languages() });
      } catch (error) {
        post({
          type: "packsError",
          request: request.request,
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
      const recv = performance.timeOrigin + performance.now();
      sendStamped(port, core.receive(message.data), recv);
    });
    port.start();
  });
  setInterval(() => {
    if (current !== undefined) {
      send(current, core.pump());
    }
  }, PUMP_MS);
  setInterval(() => {
    try {
      const status = core.status();
      if (autosave.observe(Date.now(), status.session, status.running)) {
        post({ type: "autosave" });
      }
    } catch {
      // A stopped core has nothing to save.
    }
  }, AUTOSAVE_TICK_MS);

  post({
    type: "ready",
    info: bridge.coreInfo(),
    packs: core.packs(),
    languages: core.languages(),
  });
} catch (error) {
  post({ type: "error", message: error instanceof Error ? error.message : String(error) });
  process.exitCode = 1;
}
