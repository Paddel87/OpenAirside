/**
 * Electron main process of OpenAirside (module "Plattform").
 *
 * Starts the simulation core in a utility process (ADR-009), restarts it
 * after a crash (at most 3 times in 60 s), hands a fresh `MessagePort` to
 * every loaded page of the hardened game window and answers the platform
 * API S-6 after checking the sender (ADR-021).
 */
import {
  app,
  BrowserWindow,
  ipcMain,
  type IpcMainInvokeEvent,
  Menu,
  MessageChannelMain,
  session,
  utilityProcess,
  type UtilityProcess,
} from "electron";
import { fileURLToPath } from "node:url";
import {
  type CoreStatus,
  exitedCoreStatus,
  nextCoreStatus,
  restartDecision,
} from "./core-status.js";
import {
  hardenedWebPreferences,
  isAllowedNavigation,
  isTrustedSender,
  menuTemplate,
} from "./security.js";

declare global {
  /** Current core status; read by the end-to-end smoke test (e2e/smoke.spec.ts). */
  var openairsideCoreStatus: CoreStatus;
}

globalThis.openairsideCoreStatus = { state: "starting" };

const rendererRoot = new URL("../renderer/", import.meta.url);
const rendererEntry = new URL("index.html", rendererRoot);
const coreHostPath = fileURLToPath(new URL("core-host.js", import.meta.url));
const preloadPath = fileURLToPath(new URL("preload.cjs", import.meta.url));

let core: UtilityProcess | undefined;
let gameWindow: BrowserWindow | undefined;
let pageLoaded = false;
let restarts: readonly number[] = [];

/** Connects the loaded page with the core by a fresh port (S-6 "Port-Übergabe"). */
function connect(): void {
  if (
    core === undefined ||
    gameWindow === undefined ||
    !pageLoaded ||
    globalThis.openairsideCoreStatus.state !== "ready"
  ) {
    return;
  }
  const { port1, port2 } = new MessageChannelMain();
  core.postMessage({ type: "port" }, [port1]);
  gameWindow.webContents.postMessage("core:port", null, [port2]);
}

function startCore(): void {
  globalThis.openairsideCoreStatus = { state: "starting" };
  // Flags such as --scenario= reach the core; it ignores everything else.
  const process = utilityProcess.fork(coreHostPath, globalThis.process.argv.slice(1), {
    serviceName: "OpenAirside Core",
  });
  core = process;
  process.on("message", (message: unknown) => {
    globalThis.openairsideCoreStatus = nextCoreStatus(message);
    connect();
  });
  process.on("exit", (code: number) => {
    globalThis.openairsideCoreStatus = exitedCoreStatus(globalThis.openairsideCoreStatus, code);
    core = undefined;
    gameWindow?.webContents.send("core:exit", code);
    const decision = restartDecision(restarts, Date.now());
    restarts = decision.restarts;
    if (decision.allowed && !quitting) {
      startCore();
    }
  });
}

let quitting = false;

function trusted(event: IpcMainInvokeEvent): boolean {
  const frame = event.senderFrame;
  return (
    gameWindow !== undefined &&
    isTrustedSender(
      {
        frameUrl: frame?.url ?? "",
        isMainFrame: frame !== null && frame.parent === null,
        webContentsId: event.sender.id,
      },
      gameWindow.webContents.id,
      rendererEntry.href,
    )
  );
}

function handle(channel: string, answer: () => unknown): void {
  ipcMain.handle(channel, (event) => {
    if (!trusted(event)) {
      console.warn(`rejected IPC call ${channel} from an untrusted sender`);
      throw new Error("untrusted sender");
    }
    return answer();
  });
}

function createWindow(): void {
  const window = new BrowserWindow({
    width: 1280,
    height: 800,
    title: "OpenAirside",
    show: false,
    webPreferences: hardenedWebPreferences(preloadPath, !app.isPackaged),
  });
  gameWindow = window;
  window.once("ready-to-show", () => {
    window.show();
  });
  window.webContents.on("did-start-loading", () => {
    pageLoaded = false;
  });
  window.webContents.on("did-finish-load", () => {
    pageLoaded = true;
    connect();
  });
  window.webContents.on("will-navigate", (event, url) => {
    if (!isAllowedNavigation(url, rendererRoot.href)) {
      event.preventDefault();
    }
  });
  window.webContents.setWindowOpenHandler(() => ({ action: "deny" }));
  window.on("closed", () => {
    gameWindow = undefined;
  });
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
  const template = menuTemplate(process.platform, app.isPackaged);
  if (template !== undefined) {
    Menu.setApplicationMenu(template === null ? null : Menu.buildFromTemplate([...template]));
  }
  handle("app:info", () => ({
    version: app.getVersion(),
    platform: process.platform,
    dataFolder: app.getPath("userData"),
  }));
  handle("app:quit", () => {
    app.quit();
    return null;
  });
  startCore();
  createWindow();
});

app.on("before-quit", () => {
  quitting = true;
});

app.on("window-all-closed", () => {
  app.quit();
});
