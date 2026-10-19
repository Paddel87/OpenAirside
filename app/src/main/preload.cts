/**
 * Preload of the game window: the platform API S-6 as one object
 * `window.openairside` with named asynchronous functions (ADR-021). Runs
 * sandboxed, hence CommonJS (`.cts`).
 *
 * MessagePorts cannot cross the context bridge as arguments; the port to
 * the core arrives via `window.postMessage` with the tag `CORE_PORT_TAG`
 * and is taken by `onCorePort` in app/renderer/platform.ts.
 *
 * Every call answers with an envelope (app/src/main/platform-error.ts);
 * a refused call rejects with `{ code, textKey, params }` (S-6 "Fehler").
 * The sandboxed preload cannot import local modules, hence the unwrapping
 * is written out here.
 */
import electron = require("electron");

const { contextBridge, ipcRenderer } = electron;

/** The page window; the main-process tsconfig has no DOM types. */
declare const window: {
  postMessage(message: string, targetOrigin: string, transfer: readonly unknown[]): void;
};

/** Version of the platform API S-6. */
const API_VERSION = 1;
/** Tag of the window message that carries the port to the core. */
const CORE_PORT_TAG = "openairside:core-port";

/** Reason of a broken answer (a programming error between preload and main process). */
const INVALID_ANSWER = {
  code: "invalidAnswer",
  textKey: "platform.error.invalidAnswer",
  params: {},
};

/** Calls `channel` and unwraps the envelope of the main process. */
async function call(channel: string, ...args: unknown[]): Promise<unknown> {
  const answer: unknown = await ipcRenderer.invoke(channel, ...args).catch(() => undefined);
  if (typeof answer === "object" && answer !== null && "ok" in answer) {
    if (answer.ok === true && "value" in answer) return answer.value;
  }
  const error =
    typeof answer === "object" && answer !== null && "error" in answer
      ? answer.error
      : INVALID_ANSWER;
  throw error;
}

contextBridge.exposeInMainWorld("openairside", {
  apiVersion: API_VERSION,
  app: {
    info: (): Promise<unknown> => call("app:info"),
    quit: (): Promise<unknown> => call("app:quit"),
  },
  measure: {
    start: (): Promise<unknown> => call("measure:start"),
    report: (result: unknown): Promise<unknown> => call("measure:report", result),
  },
  packs: {
    list: (): Promise<unknown> => call("packs:list"),
    setActive: (ids: unknown): Promise<unknown> => call("packs:setActive", ids),
  },
  saves: {
    list: (): Promise<unknown> => call("saves:list"),
    save: (request: unknown): Promise<unknown> => call("saves:save", request),
    load: (id: unknown): Promise<unknown> => call("saves:load", id),
    delete: (id: unknown): Promise<unknown> => call("saves:delete", id),
    preview: (id: unknown): Promise<unknown> => call("saves:preview", id),
  },
  logs: {
    write: (level: unknown, message: unknown): Promise<unknown> =>
      call("logs:write", level, message),
    openFolder: (): Promise<unknown> => call("logs:openFolder"),
  },
  onCoreExit: (callback: (code: number, restarting: boolean) => void): void => {
    ipcRenderer.on("core:exit", (_event, code: unknown, restarting: unknown) => {
      callback(typeof code === "number" ? code : -1, restarting === true);
    });
  },
});

ipcRenderer.on("core:port", (event) => {
  window.postMessage(CORE_PORT_TAG, "*", event.ports);
});
