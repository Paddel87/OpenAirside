/**
 * Electron main process of OpenAirside (module "Plattform").
 *
 * Starts the simulation core in a utility process (ADR-009) and opens the
 * hardened game window. Roadmap step 2.1 provides the empty frame only.
 */
import { app, BrowserWindow, session, utilityProcess } from "electron";
import { fileURLToPath } from "node:url";
import { type CoreStatus, exitedCoreStatus, nextCoreStatus } from "./core-status.js";
import { hardenedWebPreferences, isAllowedNavigation } from "./security.js";

declare global {
  /** Current core status; read by the end-to-end smoke test (e2e/smoke.spec.ts). */
  var openairsideCoreStatus: CoreStatus;
}

globalThis.openairsideCoreStatus = { state: "starting" };

const rendererRoot = new URL("../renderer/", import.meta.url);
const rendererEntry = new URL("index.html", rendererRoot);
const coreHostPath = fileURLToPath(new URL("core-host.js", import.meta.url));

function startCore(): void {
  const core = utilityProcess.fork(coreHostPath, [], { serviceName: "OpenAirside Core" });
  core.on("message", (message: unknown) => {
    globalThis.openairsideCoreStatus = nextCoreStatus(message);
  });
  core.on("exit", (code: number) => {
    globalThis.openairsideCoreStatus = exitedCoreStatus(globalThis.openairsideCoreStatus, code);
  });
}

function createWindow(): void {
  const window = new BrowserWindow({
    width: 1280,
    height: 800,
    title: "OpenAirside",
    show: false,
    webPreferences: hardenedWebPreferences(),
  });
  window.once("ready-to-show", () => {
    window.show();
  });
  window.webContents.on("will-navigate", (event, url) => {
    if (!isAllowedNavigation(url, rendererRoot.href)) {
      event.preventDefault();
    }
  });
  window.webContents.setWindowOpenHandler(() => ({ action: "deny" }));
  void window.loadURL(rendererEntry.href);
}

// Sandbox every renderer. Skipped only when the OS sandbox is explicitly disabled
// (`--no-sandbox` on Linux runners, see e2e/smoke.spec.ts), because enableSandbox()
// would force it back on for child processes that cannot start without it.
if (!app.commandLine.hasSwitch("no-sandbox")) {
  app.enableSandbox();
}

void app.whenReady().then(() => {
  session.defaultSession.setPermissionRequestHandler((_webContents, _permission, callback) => {
    callback(false);
  });
  startCore();
  createWindow();
});

app.on("window-all-closed", () => {
  app.quit();
});
