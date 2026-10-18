/**
 * Electron main process of OpenAirside (module "Plattform").
 *
 * Starts the simulation core in a utility process (ADR-009), restarts it
 * after a crash (at most 3 times in 60 s), hands a fresh `MessagePort` to
 * every loaded page of the hardened game window and answers the platform
 * API S-6 after checking the sender (ADR-021), including the result file
 * of the measurement tool (step 2.7) and the data packs (step 2.4): the
 * core process gets the pack folders and the player's saved choice, the
 * main process keeps its reports for `packs.list()`.
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
import { mkdirSync, readFileSync } from "node:fs";
import { mkdir, rename, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import type { PackReport } from "../generated/PackReport.js";
import { parseCoreArgs } from "./core-port.js";
import {
  type CoreStatus,
  exitedCoreStatus,
  nextCoreStatus,
  type PackRequest,
  parseCoreHostMessage,
  restartDecision,
} from "./core-status.js";
import { MeasureRun, MEMORY_INTERVAL_MS, reportFileName } from "./measure.js";
import {
  ACTIVE_FILE,
  activeFileText,
  choosableIds,
  packArgs,
  parseActiveIds,
  readActiveFile,
  USER_PACKS_DIR,
} from "./packs.js";
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

/** Service name of the core process; also names it in the memory samples. */
const CORE_SERVICE = "OpenAirside Core";

let core: UtilityProcess | undefined;
let gameWindow: BrowserWindow | undefined;
let pageLoaded = false;
let restarts: readonly number[] = [];

/** Latest pack reports of the core process (S-6 `packs.list`). */
let packReports: PackReport[] = [];
/** Longest wait for the core's answer to `packs.setActive`. */
const PACKS_TIMEOUT_MS = 60_000;
/** Open `packs.setActive` call; a second one is refused with `busy` (S-6). */
let packsPending:
  { resolve: (reports: PackReport[]) => void; reject: (error: Error) => void } | undefined;

/** Base pack shipped with the game (`content/` in the app folder, step 2.4). */
function basePackDir(): string {
  return path.join(app.getAppPath(), "content", "base");
}

function activeFilePath(): string {
  return path.join(app.getPath("userData"), ACTIVE_FILE);
}

/** The player's saved choice of packs; missing or unreadable means none. */
function savedActivePacks(): string[] {
  try {
    return readActiveFile(readFileSync(activeFilePath(), "utf8"));
  } catch {
    return [];
  }
}

function settlePacks(
  answer: { type: "packs"; packs: PackReport[] } | { type: "packsError"; message: string },
): void {
  const pending = packsPending;
  packsPending = undefined;
  if (answer.type === "packs") {
    packReports = answer.packs;
    pending?.resolve(answer.packs);
  } else {
    pending?.reject(new Error(answer.message));
  }
}

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
  const userPacks = path.join(app.getPath("userData"), USER_PACKS_DIR);
  try {
    // Created so that players find the folder for their packs.
    mkdirSync(userPacks, { recursive: true });
  } catch (error) {
    console.warn(`cannot create the pack folder: ${String(error)}`);
  }
  // Flags such as --scenario= reach the core; it ignores everything else.
  // The pack flags come last and win over flags of the command line.
  const args = [
    ...globalThis.process.argv.slice(1),
    ...packArgs(basePackDir(), userPacks, savedActivePacks()),
  ];
  const process = utilityProcess.fork(coreHostPath, args, {
    serviceName: CORE_SERVICE,
  });
  core = process;
  process.on("message", (raw: unknown) => {
    const message = parseCoreHostMessage(raw);
    if (message?.type === "packs" || message?.type === "packsError") {
      settlePacks(message);
      return;
    }
    if (message?.type === "ready") {
      packReports = message.packs;
    }
    globalThis.openairsideCoreStatus = nextCoreStatus(raw);
    connect();
  });
  process.on("exit", (code: number) => {
    globalThis.openairsideCoreStatus = exitedCoreStatus(globalThis.openairsideCoreStatus, code);
    core = undefined;
    settlePacks({ type: "packsError", message: "coreExited" });
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

function handle(channel: string, answer: (arg: unknown) => unknown): void {
  ipcMain.handle(channel, (event, arg: unknown) => {
    if (!trusted(event)) {
      console.warn(`rejected IPC call ${channel} from an untrusted sender`);
      throw new Error("untrusted sender");
    }
    return answer(arg);
  });
}

const measureRun = new MeasureRun();
let measureTimer: NodeJS.Timeout | undefined;

function sampleMemory(): void {
  measureRun.sample(app.getAppMetrics(), CORE_SERVICE, Date.now());
}

/** S-6 `measure.start`: begins sampling the memory of all processes. */
function startMeasure(): null {
  if (!measureRun.start(Date.now())) {
    throw new Error("measurement already running");
  }
  sampleMemory();
  measureTimer = setInterval(sampleMemory, MEMORY_INTERVAL_MS);
  return null;
}

/** S-6 `measure.report`: writes the result file into the downloads folder. */
async function reportMeasure(result: unknown): Promise<string> {
  sampleMemory();
  const cpus = os.cpus();
  const done = measureRun.finish(result, {
    appVersion: app.getVersion(),
    packaged: app.isPackaged,
    platform: process.platform,
    arch: process.arch,
    osRelease: os.release(),
    electron: process.versions.electron,
    chrome: process.versions.chrome,
    cpus: cpus.length,
    cpuModel: cpus[0]?.model ?? "",
    memGb: Math.round(os.totalmem() / 2 ** 30),
    gpuFeatureStatus: app.getGPUFeatureStatus(),
    coreOptions: parseCoreArgs(process.argv),
    finishedAt: new Date().toISOString(),
  });
  if (!done.ok) {
    throw new Error(`measurement report refused: ${done.error}`);
  }
  clearInterval(measureTimer);
  measureTimer = undefined;
  const folder = app.getPath("downloads");
  await mkdir(folder, { recursive: true });
  const file = path.join(folder, reportFileName(process.platform, new Date()));
  // "wx": never overwrite an existing file.
  await writeFile(file, done.json, { flag: "wx" });
  console.log(`measurement written to ${file}`);
  return file;
}

/**
 * S-6 `packs.setActive`: checks the ids, saves the choice (a restarted
 * core process starts with it) and lets the core reload the packs; the
 * choice applies from the next session on. Rejects with a code as message.
 */
async function setActivePacks(arg: unknown): Promise<PackReport[]> {
  const ids = parseActiveIds(arg);
  if (ids === undefined) {
    throw new Error("invalidArgument");
  }
  const choosable = choosableIds(packReports);
  if (!ids.every((id) => choosable.has(id))) {
    throw new Error("unknownPack");
  }
  const target = core;
  if (packsPending !== undefined) {
    throw new Error("busy");
  }
  if (target === undefined || globalThis.openairsideCoreStatus.state !== "ready") {
    throw new Error("notReady");
  }
  const answer = new Promise<PackReport[]>((resolve, reject) => {
    const timer = setTimeout(() => {
      settlePacks({ type: "packsError", message: "timeout" });
    }, PACKS_TIMEOUT_MS);
    packsPending = {
      resolve: (reports) => {
        clearTimeout(timer);
        resolve(reports);
      },
      reject: (error) => {
        clearTimeout(timer);
        reject(error);
      },
    };
  });
  try {
    // Write and rename: an interrupted write never leaves half a file.
    const file = activeFilePath();
    await mkdir(path.dirname(file), { recursive: true });
    await writeFile(`${file}.tmp`, activeFileText(ids));
    await rename(`${file}.tmp`, file);
  } catch (error) {
    settlePacks({ type: "packsError", message: "ioError" });
    console.warn(`cannot save the chosen packs: ${String(error)}`);
    return answer;
  }
  const request: PackRequest = { type: "packs:setActive", ids };
  target.postMessage(request);
  return answer;
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
  handle("measure:start", startMeasure);
  handle("measure:report", reportMeasure);
  handle("packs:list", () => packReports);
  handle("packs:setActive", setActivePacks);
  startCore();
  createWindow();
});

app.on("before-quit", () => {
  quitting = true;
});

app.on("window-all-closed", () => {
  app.quit();
});
