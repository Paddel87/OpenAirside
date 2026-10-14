/**
 * Preload of the game window: the platform API S-6 as one object
 * `window.openairside` with named asynchronous functions (ADR-021). Runs
 * sandboxed, hence CommonJS (`.cts`).
 *
 * MessagePorts cannot cross the context bridge as arguments; the port to
 * the core arrives via `window.postMessage` with the tag `CORE_PORT_TAG`
 * and is taken by `onCorePort` in app/renderer/platform.ts.
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

contextBridge.exposeInMainWorld("openairside", {
  apiVersion: API_VERSION,
  app: {
    info: (): Promise<unknown> => ipcRenderer.invoke("app:info"),
    quit: (): Promise<unknown> => ipcRenderer.invoke("app:quit"),
  },
  onCoreExit: (callback: (code: number) => void): void => {
    ipcRenderer.on("core:exit", (_event, code: unknown) => {
      callback(typeof code === "number" ? code : -1);
    });
  },
});

ipcRenderer.on("core:port", (event) => {
  window.postMessage(CORE_PORT_TAG, "*", event.ports);
});
