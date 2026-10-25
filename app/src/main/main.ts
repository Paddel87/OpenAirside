/**
 * Electron main process of OpenAirside (module "Plattform").
 *
 * Starts the simulation core in a utility process (ADR-009), restarts it
 * after a crash (at most 3 times in 60 s), hands a fresh `MessagePort` to
 * every loaded page of the hardened game window and answers the platform
 * API S-6 after checking the sender (ADR-021). Since step 3.10 this file
 * only wires: the file access and the handlers live in `save-files.ts`,
 * `saves-service.ts`, `settings-file.ts`, `settings-service.ts`,
 * `packs-file.ts`, `packs-service.ts` and `files.ts`. What stays here is
 * what needs Electron: the core process and its requests, the window, the
 * sender check, the measurement tool (step 2.7) and the log file (`logs.*`).
 */
import {
  app,
  BrowserWindow,
  ipcMain,
  type IpcMainInvokeEvent,
  Menu,
  MessageChannelMain,
  session,
  shell,
  utilityProcess,
  type UtilityProcess,
} from "electron";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseCoreArgs, withoutPackOptions } from "./core-port.js";
import {
  type CoreStatus,
  exitedCoreStatus,
  nextCoreStatus,
  isPackAnswer,
  isSavesAnswer,
  parseCoreHostMessage,
  restartDecision,
  type SavesAnswer,
  type SavesRequest,
} from "./core-status.js";
import { createFolder, errorCode, nodeLogFs, tryCreateFolder, writeNewFile } from "./files.js";
import { LOG_FILE, LogFile, type LogLevel, LOGS_DIR, parseLogArgs, RateGate } from "./logs.js";
import { MeasureRun, MEMORY_INTERVAL_MS, reportFileName } from "./measure.js";
import { OperationLock } from "./operation-lock.js";
import { ACTIVE_FILE, packArgs, USER_PACKS_DIR } from "./packs.js";
import { readActivePacksFile } from "./packs-file.js";
import { createPacksService } from "./packs-service.js";
import { answer, fail, type IpcAnswer, platformReason, PlatformFailure } from "./platform-error.js";
import { createSaveFiles } from "./save-files.js";
import { createSavesService } from "./saves-service.js";
import { previewSize, SAVES_DIR } from "./saves.js";
import { createSettingsService } from "./settings-service.js";
import { SETTINGS_FILE } from "./settings.js";
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

/** Log file; set once the app is ready. */
let logFile: LogFile | undefined;

/** Writes an entry of the main process to the console and the log file. */
function log(level: LogLevel, message: string): void {
  if (level === "warn" || level === "error") console.warn(message);
  else console.log(message);
  logFile?.write(level, "main", message);
}

/** A path in the user data folder. */
function userData(...parts: string[]): string {
  return path.join(app.getPath("userData"), ...parts);
}

/** True while the core process is ready for requests. */
function coreReady(): boolean {
  return core !== undefined && globalThis.openairsideCoreStatus.state === "ready";
}

/** Base pack shipped with the game (`content/` in the app folder, step 2.4). */
function basePackDir(): string {
  return path.join(app.getAppPath(), "content", "base");
}

/** Save game or pack operation in progress (S-6 "Gleichzeitigkeit"). */
const lock = new OperationLock();

const packs = createPacksService({
  lock,
  file: () => userData(ACTIVE_FILE),
  send: (message) => {
    core?.postMessage(message);
  },
  coreReady,
  log,
});

/** Tells the core process the autosave interval of the settings. */
function sendAutosaveInterval(): void {
  core?.postMessage({ type: "autosave:interval", minutes: settings.get().autosaveMinutes });
}

const settings = createSettingsService({
  file: () => userData(SETTINGS_FILE),
  languages: () => packs.languages(),
  request: (make) => requestSaves(make),
  onAutosaveInterval: sendAutosaveInterval,
  log,
});

/** Longest wait for the core's answer to a save game request (loading computes flow fields). */
const SAVES_TIMEOUT_MS = 120_000;
/** Open save game requests to the core process by request number. */
const savesPending = new Map<
  number,
  { resolve: (value: unknown) => void; reject: (error: Error) => void; timer: NodeJS.Timeout }
>();
let savesRequest = 0;

/** Settles an open save game request; a late or unknown answer is ignored. */
function settleSaves(answer: SavesAnswer): void {
  const pending = savesPending.get(answer.request);
  if (pending === undefined) return;
  savesPending.delete(answer.request);
  clearTimeout(pending.timer);
  if (answer.ok) pending.resolve(answer.value);
  else pending.reject(new PlatformFailure(answer.error));
}

/** Sends a save game request to the core process and waits for its answer. */
function requestSaves(make: (request: number) => SavesRequest): Promise<unknown> {
  const target = core;
  if (target === undefined || !coreReady()) {
    fail("notReady");
  }
  savesRequest += 1;
  const request = savesRequest;
  return new Promise<unknown>((resolve, reject) => {
    const timer = setTimeout(() => {
      settleSaves({ type: "savesAnswer", request, ok: false, error: platformReason("timeout") });
    }, SAVES_TIMEOUT_MS);
    savesPending.set(request, { resolve, reject, timer });
    target.postMessage(make(request));
  });
}

/** Longest wait for the screenshot of the window. */
const PREVIEW_TIMEOUT_MS = 5000;

/**
 * Screenshot of the game window as preview, at most `PREVIEW_WIDTH` pixels
 * wide and within 1920 × 1080 (S-5, see `previewSize`); empty if it fails or takes
 * longer than {@link PREVIEW_TIMEOUT_MS} – a save without preview is still a save.
 */
async function previewImage(): Promise<Uint8Array> {
  const window = gameWindow;
  if (window === undefined) return new Uint8Array(0);
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    const image = await Promise.race([
      window.webContents.capturePage(),
      new Promise<undefined>((resolve) => {
        timer = setTimeout(() => {
          resolve(undefined);
        }, PREVIEW_TIMEOUT_MS);
      }),
    ]);
    if (image === undefined) {
      log("warn", "preview image timed out");
      return new Uint8Array(0);
    }
    if (image.isEmpty()) return new Uint8Array(0);
    const size = previewSize(image.getSize());
    if (size === undefined) return new Uint8Array(0);
    return new Uint8Array(image.resize(size).toPNG());
  } catch (error) {
    log("warn", `cannot take the preview image: ${errorCode(error)}`);
    return new Uint8Array(0);
  } finally {
    clearTimeout(timer);
  }
}

const saveFiles = createSaveFiles(() => userData(SAVES_DIR), log);

const saves = createSavesService({
  files: saveFiles,
  lock,
  request: (make) => requestSaves(make),
  preview: previewImage,
  autosaveCount: () => settings.get().autosaveCount,
  coreReady,
  log,
});

/** Connects the loaded page with the core by a fresh port (S-6 "Port-Übergabe"). */
function connect(): void {
  if (core === undefined || gameWindow === undefined || !pageLoaded || !coreReady()) {
    return;
  }
  const { port1, port2 } = new MessageChannelMain();
  core.postMessage({ type: "port" }, [port1]);
  gameWindow.webContents.postMessage("core:port", null, [port2]);
}

function startCore(): void {
  globalThis.openairsideCoreStatus = { state: "starting" };
  const userPacks = userData(USER_PACKS_DIR);
  // Created so that players find the folder for their packs.
  const folderError = tryCreateFolder(userPacks);
  if (folderError !== undefined) log("warn", `cannot create the pack folder: ${folderError}`);
  // Flags such as --scenario= reach the core; it ignores everything else.
  // The pack flags come last and win over flags of the command line.
  const args = [
    ...globalThis.process.argv.slice(1),
    ...packArgs(basePackDir(), userPacks, readActivePacksFile(userData(ACTIVE_FILE))),
  ];
  const process = utilityProcess.fork(coreHostPath, args, {
    serviceName: CORE_SERVICE,
  });
  core = process;
  process.on("message", (raw: unknown) => {
    const message = parseCoreHostMessage(raw);
    if (message?.type === "packs" || message?.type === "packsError") {
      packs.settle(message);
      return;
    }
    if (message?.type === "savesAnswer") {
      settleSaves(message);
      return;
    }
    if (message?.type === "autosave") {
      saves.autosaveRequested();
      return;
    }
    if (isSavesAnswer(raw)) {
      log("warn", "malformed save game answer from the core process");
      return;
    }
    if (isPackAnswer(raw)) {
      // A malformed pack answer says nothing about the state of the core.
      log("warn", "malformed pack answer from the core process");
      const open = packs.pendingRequest();
      if (open !== undefined) {
        packs.settle({ type: "packsError", request: open, message: "invalidAnswer" });
      }
      return;
    }
    if (message?.type === "ready") {
      packs.setReports(message.packs, message.languages);
      sendAutosaveInterval();
    }
    globalThis.openairsideCoreStatus = nextCoreStatus(raw);
    connect();
  });
  process.on("exit", (code: number) => {
    globalThis.openairsideCoreStatus = exitedCoreStatus(globalThis.openairsideCoreStatus, code);
    core = undefined;
    const open = packs.pendingRequest();
    if (open !== undefined) {
      packs.settle({ type: "packsError", request: open, message: "coreExited" });
    }
    for (const request of [...savesPending.keys()]) {
      settleSaves({ type: "savesAnswer", request, ok: false, error: platformReason("coreExited") });
    }
    const decision = restartDecision(restarts, Date.now());
    restarts = decision.restarts;
    const restarting = decision.allowed && !quitting;
    log(
      restarting ? "warn" : "error",
      `core process exited with code ${String(code)}${restarting ? "; restarting" : "; no further restart"}`,
    );
    // S-6 `onCoreExit(code, restarting)` (ADR-029).
    gameWindow?.webContents.send("core:exit", code, restarting);
    if (restarting) {
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

/** Lines about refused calls per second; the renderer can cause them at will. */
const refusedGate = new RateGate(20);

function logRefused(level: LogLevel, message: string): void {
  const { pass, dropped } = refusedGate.pass(performance.now());
  if (!pass) return;
  if (dropped > 0) log("warn", `${String(dropped)} lines about refused IPC calls dropped`);
  log(level, message);
}

/**
 * Registers an S-6 handler: checks the sender and answers with the
 * envelope of the error form (app/src/main/platform-error.ts).
 */
function handle(channel: string, run: (...args: unknown[]) => unknown): void {
  ipcMain.handle(channel, (event, ...args: unknown[]): Promise<IpcAnswer> | IpcAnswer => {
    if (!trusted(event)) {
      logRefused("warn", `rejected IPC call ${channel} from an untrusted sender`);
      return { ok: false, error: platformReason("untrustedSender") };
    }
    return answer(
      () => run(...args),
      (error) => {
        log("error", `IPC call ${channel} failed: ${errorCode(error)}`);
      },
    ).then((result) => {
      if (!result.ok) logRefused("info", `IPC call ${channel} refused: ${result.error.code}`);
      return result;
    });
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
    fail("alreadyRunning");
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
    coreOptions: withoutPackOptions(parseCoreArgs(process.argv)),
    finishedAt: new Date().toISOString(),
  });
  if (!done.ok) {
    fail(done.error);
  }
  clearInterval(measureTimer);
  measureTimer = undefined;
  const file = await writeNewFile(
    app.getPath("downloads"),
    reportFileName(process.platform, new Date()),
    done.json,
  );
  log("info", "measurement written to the downloads folder");
  return file;
}

/** S-6 `logs.write(level, message)`: throttled entry of the renderer. */
function writeLog(level: unknown, message: unknown): null {
  const parsed = parseLogArgs(level, message);
  if (parsed === undefined) fail("invalidArgument");
  logFile?.writeThrottled(parsed.level, parsed.message, Date.now());
  return null;
}

/** S-6 `logs.openFolder()`: shows the folder of the log files. */
async function openLogFolder(): Promise<null> {
  const folder = userData(LOGS_DIR);
  try {
    await createFolder(folder);
  } catch {
    fail("ioError");
  }
  const problem = await shell.openPath(folder);
  if (problem !== "") {
    log("warn", "cannot open the log folder");
    fail("ioError");
  }
  return null;
}

/** Opens the log file in `<userData>/logs` (release builds from level `info`). */
function openLogFile(): void {
  const folder = userData(LOGS_DIR);
  const folderError = tryCreateFolder(folder);
  if (folderError !== undefined) console.warn(`cannot create the log folder: ${folderError}`);
  logFile = new LogFile(nodeLogFs, path.join(folder, LOG_FILE), app.isPackaged ? "info" : "debug");
  log("info", `OpenAirside ${app.getVersion()} started on ${process.platform} ${process.arch}`);
}

function createWindow(): void {
  const window = new BrowserWindow({
    width: 1280,
    height: 800,
    title: "OpenAirside",
    // Windows and macOS take the icon of the program package; Linux needs it on the window (step 2.10).
    ...(process.platform === "linux"
      ? { icon: fileURLToPath(new URL("icon.png", rendererRoot)) }
      : {}),
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
  openLogFile();
  settings.load(app.getPreferredSystemLanguages());
  saveFiles.removeTempFiles();
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
  handle("packs:list", () => packs.reports());
  handle("packs:setActive", (arg) => packs.setActive(arg));
  handle("saves:list", () => saves.list());
  handle("saves:save", (arg) => saves.save(arg));
  handle("saves:load", (arg) => saves.load(arg));
  handle("game:new", (arg) => saves.newGame(arg));
  handle("saves:delete", (arg) => saves.delete(arg));
  handle("saves:preview", (arg) => saves.preview(arg));
  handle("logs:write", writeLog);
  handle("logs:openFolder", openLogFolder);
  handle("settings:get", () => settings.get());
  handle("settings:set", (patch) => settings.set(patch));
  handle("lang:list", () => packs.languages());
  handle("lang:texts", (arg) => settings.texts(arg));
  startCore();
  createWindow();
});

app.on("before-quit", () => {
  quitting = true;
});

app.on("window-all-closed", () => {
  app.quit();
});
