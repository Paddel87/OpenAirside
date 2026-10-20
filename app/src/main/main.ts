/**
 * Electron main process of OpenAirside (module "Plattform").
 *
 * Starts the simulation core in a utility process (ADR-009), restarts it
 * after a crash (at most 3 times in 60 s), hands a fresh `MessagePort` to
 * every loaded page of the hardened game window and answers the platform
 * API S-6 after checking the sender (ADR-021), including the result file
 * of the measurement tool (step 2.7) and the data packs (step 2.4): the
 * core process gets the pack folders and the player's saved choice, the
 * main process keeps its reports for `packs.list()`. Since step 2.5 also
 * save games (`saves.*`, S-5) with autosave and quick save, the log file
 * (`logs.*`) and the error form `{ code, textKey, params }` of every call.
 * Since step 2.6 the settings (`settings.*`) and the languages of the
 * loaded packs (`lang.*`, ADR-031).
 */
import {
  app,
  BrowserWindow,
  contentTracing,
  ipcMain,
  type IpcMainInvokeEvent,
  Menu,
  MessageChannelMain,
  session,
  shell,
  utilityProcess,
  type UtilityProcess,
} from "electron";
import { randomBytes } from "node:crypto";
import {
  appendFileSync,
  closeSync,
  constants as fsConstants,
  fstatSync,
  mkdirSync,
  openSync,
  readdirSync,
  readFileSync,
  renameSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import {
  type FileHandle,
  lstat,
  mkdir,
  open,
  readdir,
  rename,
  rm,
  writeFile,
} from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { pipeline } from "node:stream/promises";
import { fileURLToPath } from "node:url";
import { createGzip } from "node:zlib";
import { createReadStream, createWriteStream } from "node:fs";
import type { LanguageInfo } from "../generated/LanguageInfo.js";
import type { PackReport } from "../generated/PackReport.js";
import type { SaveMeta } from "../generated/SaveMeta.js";
import type { Settings } from "../generated/Settings.js";
import { parseCoreArgs, withoutPackOptions } from "./core-port.js";
import {
  type CoreStatus,
  exitedCoreStatus,
  nextCoreStatus,
  isPackAnswer,
  isSavesAnswer,
  type PackRequest,
  parseCoreHostMessage,
  restartDecision,
  type SavesAnswer,
  type SavesRequest,
} from "./core-status.js";
import { LOG_FILE, LogFile, type LogLevel, LOGS_DIR, parseLogArgs, RateGate } from "./logs.js";
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
import { answer, fail, type IpcAnswer, PlatformFailure, platformReason } from "./platform-error.js";
import {
  idOfFileName,
  INFO_BATCH,
  isSaveId,
  isTempFileName,
  MAX_FILE_BYTES,
  MAX_INFO_BYTES,
  MAX_LISTED_FILES,
  MAX_MANUAL_SAVES,
  newSaveId,
  parseInfoAnswer,
  parseSaveArgs,
  parseSaveMeta,
  previewSize,
  type SaveEntry,
  saveFileName,
  SAVES_DIR,
  sortEntries,
  tempFileName,
  writeErrorCode,
} from "./saves.js";
import {
  applySettings,
  defaultSettings,
  isLanguageCode,
  MAX_SETTINGS_BYTES,
  parseTexts,
  SETTINGS_FILE,
  settingsFromFile,
} from "./settings.js";
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

/**
 * Save game or pack operation in progress (S-6 "Gleichzeitigkeit"): a
 * second one is refused with `busy`; a due autosave waits for its end.
 */
let operation: string | undefined;
let autosaveDue = false;

/** Runs `run` as the only save game or pack operation; others get `busy`. */
async function exclusive<T>(name: string, run: () => Promise<T>): Promise<T> {
  if (operation !== undefined) fail("busy");
  operation = name;
  try {
    return await run();
  } finally {
    operation = undefined;
    if (autosaveDue) setImmediate(runAutosave);
  }
}

/** Latest pack reports of the core process (S-6 `packs.list`). */
let packReports: PackReport[] = [];
/** Longest wait for the core's answer to `packs.setActive`. */
const PACKS_TIMEOUT_MS = 60_000;
/** Open `packs.setActive` call; a second one is refused with `busy` (S-6). */
let packsPending:
  | {
      readonly request: number;
      resolve: (reports: PackReport[]) => void;
      reject: (error: Error) => void;
    }
  | undefined;
/** Number of the last `packs.setActive` request to the core process. */
let packsRequest = 0;

/** Code of a Node.js error (`ENOENT` …) without its message, which names paths. */
function errorCode(error: unknown): string {
  const code =
    typeof error === "object" && error !== null && "code" in error ? error.code : undefined;
  return typeof code === "string" ? code : "unknown";
}

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

/** Languages of the last loaded packs (S-6 `lang.list`, ADR-031). */
let languageList: LanguageInfo[] = [];
/** The player's settings (S-6 `settings.*`); read once the app is ready. */
let settings: Settings = defaultSettings([]);
/** Changes of `settings.json` one after the other, each on the result of the previous one. */
let settingsWrite: Promise<unknown> = Promise.resolve();

function settingsPath(): string {
  return path.join(app.getPath("userData"), SETTINGS_FILE);
}

/**
 * Reads `settings.json`; a missing file gives the defaults of a new
 * installation, a broken one its valid fields plus defaults and a warning.
 */
function readSettings(): Settings {
  const defaults = defaultSettings(app.getPreferredSystemLanguages());
  let fd: number;
  try {
    // One handle for check and read; never blocks on a FIFO (finding 4 of
    // the separate review of 2.6).
    fd = openSync(settingsPath(), OPEN_FLAGS);
  } catch {
    return defaults;
  }
  let text: string;
  try {
    const info = fstatSync(fd);
    if (!info.isFile() || info.size > MAX_SETTINGS_BYTES) {
      log("warn", "settings file not a regular file or too large; using the defaults");
      return defaults;
    }
    text = readFileSync(fd, "utf8");
  } catch {
    log("warn", "settings file unreadable; using the defaults");
    return defaults;
  } finally {
    closeSync(fd);
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch {
    log("warn", "settings file unreadable; using the defaults");
    return defaults;
  }
  const result = settingsFromFile(parsed, defaults);
  if (!result.valid) log("warn", "settings file partly invalid; invalid values replaced");
  return result.settings;
}

/** Tells the core process the autosave interval of the settings. */
function sendAutosaveInterval(): void {
  core?.postMessage({ type: "autosave:interval", minutes: settings.autosaveMinutes });
}

/**
 * S-6 `settings.set`: checks the update, writes the file, then applies it.
 * Calls run one after the other, each on the result of the previous one,
 * so that overlapping calls lose no change (finding 3 of the separate
 * review of 2.6).
 */
function setSettings(patch: unknown): Promise<Settings> {
  const change = settingsWrite.then(async () => {
    const next = applySettings(settings, patch, languageList) ?? fail("invalidArgument");
    // Write and rename: an interrupted write never leaves half a file.
    const file = settingsPath();
    const temp = `${file}.tmp`;
    try {
      await mkdir(path.dirname(file), { recursive: true });
      await writeFile(temp, `${JSON.stringify(next, null, 2)}\n`);
      await rename(temp, file);
    } catch (error) {
      await rm(temp, { force: true }).catch(() => undefined);
      log("warn", `cannot save the settings: ${errorCode(error)}`);
      fail("ioError");
    }
    const intervalChanged = next.autosaveMinutes !== settings.autosaveMinutes;
    settings = next;
    if (intervalChanged) sendAutosaveInterval();
    log("info", `settings changed: ${JSON.stringify(next)}`);
    return next;
  });
  settingsWrite = change.catch(() => undefined);
  return change;
}

/**
 * Open `lang.texts` requests by language: a second call for the same
 * language shares the first one (finding 2 of the separate review of 2.6),
 * so at most one request per listed language is open.
 */
const textsPending = new Map<string, Promise<Record<string, string>>>();

/** S-6 `lang.texts`: the texts of a listed language from the core process. */
function languageTexts(arg: unknown): Promise<Record<string, string>> {
  if (!isLanguageCode(arg) || !languageList.some((l) => l.code === arg)) fail("invalidArgument");
  const open = textsPending.get(arg);
  if (open !== undefined) return open;
  const request = (async (): Promise<Record<string, string>> => {
    const value = await requestSaves((n) => ({ type: "lang:texts", request: n, code: arg }));
    // `null`: the language vanished with a pack change since the list.
    if (value === null) fail("invalidArgument");
    return parseTexts(value) ?? fail("invalidAnswer");
  })();
  textsPending.set(arg, request);
  void request
    .finally(() => {
      textsPending.delete(arg);
    })
    .catch(() => undefined);
  return request;
}

/** Settles the open `packs.setActive` call if `answer` belongs to it. */
function settlePacks(
  answer:
    | { type: "packs"; request: number; packs: PackReport[]; languages: LanguageInfo[] }
    | { type: "packsError"; request: number; message: string },
): void {
  const pending = packsPending;
  if (pending?.request !== answer.request) {
    // Late answer to an earlier request (after its time limit).
    return;
  }
  packsPending = undefined;
  if (answer.type === "packs") {
    packReports = answer.packs;
    languageList = answer.languages;
    pending.resolve(answer.packs);
  } else {
    pending.reject(new Error(answer.message));
  }
}

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
  if (target === undefined || globalThis.openairsideCoreStatus.state !== "ready") {
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
    log("warn", `cannot create the pack folder: ${errorCode(error)}`);
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
    if (message?.type === "savesAnswer") {
      settleSaves(message);
      return;
    }
    if (message?.type === "autosave") {
      autosaveDue = true;
      runAutosave();
      return;
    }
    if (isSavesAnswer(raw)) {
      log("warn", "malformed save game answer from the core process");
      return;
    }
    if (isPackAnswer(raw)) {
      // A malformed pack answer says nothing about the state of the core.
      log("warn", "malformed pack answer from the core process");
      if (packsPending !== undefined) {
        settlePacks({
          type: "packsError",
          request: packsPending.request,
          message: "invalidAnswer",
        });
      }
      return;
    }
    if (message?.type === "ready") {
      packReports = message.packs;
      languageList = message.languages;
      sendAutosaveInterval();
    }
    globalThis.openairsideCoreStatus = nextCoreStatus(raw);
    connect();
  });
  process.on("exit", (code: number) => {
    globalThis.openairsideCoreStatus = exitedCoreStatus(globalThis.openairsideCoreStatus, code);
    core = undefined;
    if (packsPending !== undefined) {
      settlePacks({ type: "packsError", request: packsPending.request, message: "coreExited" });
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

/**
 * Registers an S-6 handler: checks the sender and answers with the
 * envelope of the error form (app/src/main/platform-error.ts).
 */
/** Lines about refused calls per second; the renderer can cause them at will. */
const refusedGate = new RateGate(20);

function logRefused(level: LogLevel, message: string): void {
  const { pass, dropped } = refusedGate.pass(performance.now());
  if (!pass) return;
  if (dropped > 0) log("warn", `${String(dropped)} lines about refused IPC calls dropped`);
  log(level, message);
}

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
  measureStartedAt = Date.now();
  traces.length = 0;
  sampleMemory();
  measureTimer = setInterval(sampleMemory, MEMORY_INTERVAL_MS);
  return null;
}

/** Spike 3.14: ANGLE backends the console may choose (Chromium switch `--use-angle`). */
const ANGLE_VALUES = ["d3d11", "d3d11on12", "vulkan", "gl", "default"] as const;
/** Spike 3.14: file in the data folder with the chosen backend. */
const ANGLE_FILE = "spike-angle.txt";
/** Spike 3.14: trace categories (renderer, compositor, GPU process, display ticks, V8, IPC). */
const TRACE_CATEGORIES = [
  "toplevel",
  "benchmark",
  "blink",
  "blink.user_timing",
  "cc",
  "viz",
  "gpu",
  "ipc",
  "mojom",
  "v8",
  "renderer.scheduler",
  "disabled-by-default-devtools.timeline",
  "disabled-by-default-devtools.timeline.frame",
  "disabled-by-default-gpu.service",
  "disabled-by-default-viz.gpu_composite_time",
];
/** Spike 3.14: traces of the open run. */
const traces: { label: string; atS: number; file: string; bytes: number }[] = [];
let traceRunning = false;

function chosenAngle(): string {
  try {
    const value = readFileSync(path.join(app.getPath("userData"), ANGLE_FILE), "utf8").trim();
    return (ANGLE_VALUES as readonly string[]).includes(value) ? value : "default";
  } catch {
    return "default";
  }
}

/** Spike 3.14: records a Chromium trace for `ms` and saves it gzipped in the downloads folder. */
async function recordTrace(label: unknown, ms: unknown): Promise<string> {
  if (typeof label !== "string" || !/^[a-z0-9]{1,16}$/.test(label)) fail("invalidArgument");
  if (typeof ms !== "number" || !Number.isInteger(ms) || ms < 1000 || ms > 30_000) {
    fail("invalidArgument");
  }
  if (traceRunning) fail("alreadyRunning");
  traceRunning = true;
  const startedAt = Date.now();
  try {
    await contentTracing.startRecording({
      included_categories: TRACE_CATEGORIES,
      excluded_categories: ["*"],
      recording_mode: "record-as-much-as-possible",
    });
    await new Promise((resolve) => setTimeout(resolve, ms));
    const folder = app.getPath("downloads");
    await mkdir(folder, { recursive: true });
    const stamp = new Date(startedAt).toISOString().replace(/[:.]/g, "-");
    const raw = path.join(app.getPath("temp"), `openairside-trace-${label}-${stamp}.json`);
    await contentTracing.stopRecording(raw);
    const file = path.join(
      folder,
      `openairside-trace-${process.platform}-${label}-${stamp}.json.gz`,
    );
    await pipeline(createReadStream(raw), createGzip(), createWriteStream(file, { flags: "wx" }));
    await rm(raw, { force: true });
    const bytes = statSync(file).size;
    traces.push({
      label,
      atS: Math.round((startedAt - (measureStartedAt ?? startedAt)) / 1000),
      file: path.basename(file),
      bytes,
    });
    log(
      "info",
      `trace ${label} written to the downloads folder (${String(Math.round(bytes / 1e6))} MB)`,
    );
    return file;
  } finally {
    traceRunning = false;
  }
}

let measureStartedAt: number | undefined;

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
    angle: app.commandLine.getSwitchValue("use-angle") || "default",
    fullscreen: gameWindow?.isFullScreen() ?? false,
    traces: [...traces],
  });
  if (!done.ok) {
    fail(done.error);
  }
  clearInterval(measureTimer);
  measureTimer = undefined;
  const folder = app.getPath("downloads");
  await mkdir(folder, { recursive: true });
  const file = path.join(folder, reportFileName(process.platform, new Date()));
  // "wx": never overwrite an existing file.
  await writeFile(file, done.json, { flag: "wx" });
  log("info", "measurement written to the downloads folder");
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
    fail("invalidArgument");
  }
  const choosable = choosableIds(packReports);
  if (!ids.every((id) => choosable.has(id))) {
    fail("unknownPack");
  }
  return exclusive("packs", () => changePacks(ids)).catch((error: unknown) => {
    // The core host answers with a code as message; only known codes pass.
    if (error instanceof PlatformFailure) throw error;
    const code = error instanceof Error ? error.message : "";
    fail(PACK_CODES.has(code) ? code : "internal");
  });
}

/** Codes of a refused `packs.setActive` passed on to the renderer. */
const PACK_CODES: ReadonlySet<string> = new Set([
  "invalidArgument",
  "invalidId",
  "limitExceeded",
  "unknownPack",
  "busy",
  "notReady",
  "ioError",
  "timeout",
  "coreExited",
  "invalidAnswer",
]);

async function changePacks(ids: string[]): Promise<PackReport[]> {
  const target = core;
  if (packsPending !== undefined) {
    fail("busy");
  }
  if (target === undefined || globalThis.openairsideCoreStatus.state !== "ready") {
    fail("notReady");
  }
  packsRequest += 1;
  const request = packsRequest;
  const answer = new Promise<PackReport[]>((resolve, reject) => {
    const timer = setTimeout(() => {
      settlePacks({ type: "packsError", request, message: "timeout" });
    }, PACKS_TIMEOUT_MS);
    packsPending = {
      request,
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
    settlePacks({ type: "packsError", request, message: "ioError" });
    log("warn", `cannot save the chosen packs: ${errorCode(error)}`);
    return answer;
  }
  const message: PackRequest = { type: "packs:setActive", request, ids };
  target.postMessage(message);
  return answer;
}

function savesDir(): string {
  return path.join(app.getPath("userData"), SAVES_DIR);
}

function savePath(id: string): string {
  return path.join(savesDir(), saveFileName(id));
}

/** Where the preview image of each listed save game lies (from `saves.list`). */
const previews = new Map<string, { offset: number; length: number }>();

/** Removes files of interrupted saves (S-5 "Schreiben"); runs at start. */
function removeTempFiles(): void {
  let names: string[];
  try {
    mkdirSync(savesDir(), { recursive: true });
    names = readdirSync(savesDir());
  } catch (error) {
    log("warn", `cannot read the save game folder: ${errorCode(error)}`);
    return;
  }
  for (const name of names) {
    if (!isTempFileName(name)) continue;
    try {
      rmSync(path.join(savesDir(), name), { force: true });
    } catch (error) {
      // E.g. a folder of that name: the others are still removed.
      log("warn", `cannot remove a temporary save game file: ${errorCode(error)}`);
    }
  }
}

/** Flags for opening a save game: never blocks on a FIFO (POSIX only). */
const OPEN_FLAGS =
  fsConstants.O_RDONLY | (process.platform === "win32" ? 0 : fsConstants.O_NONBLOCK);

/**
 * Opens the save game `id` for reading and checks on the open handle that
 * it is a regular file within {@link MAX_FILE_BYTES} (finding 4 of the
 * separate review: no devices, FIFOs or links to them).
 */
async function openSave(
  id: string,
): Promise<{ handle: FileHandle; size: number; mtimeMs: number }> {
  let handle: FileHandle;
  try {
    handle = await open(savePath(id), OPEN_FLAGS);
  } catch (error) {
    if (errorCode(error) === "ENOENT") fail("unknownSave");
    fail("ioError");
  }
  try {
    const info = await handle.stat();
    if (!info.isFile()) fail("invalidState", { check: "file" });
    if (info.size > MAX_FILE_BYTES) fail("limitExceeded", { limit: "file", max: MAX_FILE_BYTES });
    return { handle, size: info.size, mtimeMs: info.mtimeMs };
  } catch (error) {
    await handle.close();
    if (error instanceof PlatformFailure) throw error;
    fail("ioError");
  }
}

/** Reads `length` bytes at `offset` from an open save game. */
async function readExactly(
  handle: FileHandle,
  length: number,
  offset: number,
): Promise<Uint8Array> {
  const bytes = new Uint8Array(length);
  let done = 0;
  while (done < length) {
    const { bytesRead } = await handle.read(bytes, done, length - done, offset + done);
    // The file got shorter meanwhile.
    if (bytesRead === 0) fail("unknownSave");
    done += bytesRead;
  }
  return bytes;
}

/** Ids of the save game files with their modification times, newest first. */
async function saveFiles(): Promise<{ id: string; mtimeMs: number }[]> {
  let names: string[];
  try {
    names = await readdir(savesDir());
  } catch (error) {
    if (errorCode(error) === "ENOENT") return [];
    log("warn", `cannot read the save game folder: ${errorCode(error)}`);
    fail("ioError");
  }
  const files: { id: string; mtimeMs: number }[] = [];
  for (const name of names) {
    const id = idOfFileName(name);
    if (id === undefined) continue;
    try {
      // lstat: a link or special file is listed and refused when opened.
      files.push({ id, mtimeMs: (await lstat(path.join(savesDir(), name))).mtimeMs });
    } catch {
      // Deleted meanwhile.
    }
  }
  return files.sort((a, b) => b.mtimeMs - a.mtimeMs);
}

/** Reads the save game list; the core checks header and metadata (S-5). */
async function listSaves(): Promise<SaveEntry[]> {
  const all = await saveFiles();
  if (all.length > MAX_LISTED_FILES) {
    log(
      "warn",
      `save game folder holds ${String(all.length)} files; the newest ${String(MAX_LISTED_FILES)} are listed`,
    );
  }
  previews.clear();
  const entries: SaveEntry[] = [];
  // Batches of 100: the prefixes (up to 64 KB each) are not all held at once.
  const listed = all.slice(0, MAX_LISTED_FILES);
  for (let start = 0; start < listed.length; start += INFO_BATCH) {
    const files: { id: string; prefix: Uint8Array; size: number }[] = [];
    for (const { id } of listed.slice(start, start + INFO_BATCH)) {
      try {
        const { handle, size } = await openSave(id);
        try {
          files.push({
            id,
            prefix: await readExactly(handle, Math.min(size, MAX_INFO_BYTES), 0),
            size,
          });
        } finally {
          await handle.close();
        }
      } catch (error) {
        const reason = error instanceof PlatformFailure ? error.reason : platformReason("ioError");
        // Deleted meanwhile: not listed; otherwise shown as damaged.
        if (reason.code !== "unknownSave") entries.push({ id, state: "damaged", error: reason });
      }
    }
    const answers = await requestSaves((request) => ({
      type: "saves:info",
      request,
      files: files.map(({ prefix, size }) => ({ prefix, size })),
    }));
    if (!Array.isArray(answers) || answers.length !== files.length) fail("invalidAnswer");
    files.forEach(({ id }, i) => {
      const parsed = parseInfoAnswer(answers[i]);
      if (parsed === undefined) fail("invalidAnswer");
      if (!parsed.ok) {
        entries.push({ id, state: "damaged", error: parsed.error });
        return;
      }
      previews.set(id, { offset: parsed.info.previewOffset, length: parsed.info.previewLen });
      entries.push({
        id,
        state: "ok",
        gameVersion: parsed.info.gameVersion,
        meta: parsed.info.meta,
      });
    });
  }
  return sortEntries(entries);
}

/** Longest wait for the screenshot of the window. */
const PREVIEW_TIMEOUT_MS = 5000;

/**
 * Screenshot of the game window as preview, at most {@link PREVIEW_WIDTH}
 * pixels wide and within 1920 × 1080 (S-5); empty if it fails or takes
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

/** S-6 `saves.save({ name, kind })`: writes a new file; resolves with its id. */
async function saveGame(arg: unknown): Promise<{ id: string }> {
  const parsed = parseSaveArgs(arg);
  if (parsed === undefined) fail("invalidArgument");
  return exclusive("save", () => writeSave(parsed.name, parsed.kind));
}

async function writeSave(name: string, kind: SaveMeta["kind"]): Promise<{ id: string }> {
  const files = await saveFiles();
  if (kind === "manual" && files.filter((f) => f.id.startsWith("m-")).length >= MAX_MANUAL_SAVES) {
    fail("limitExceeded", { limit: "manualSaves", max: MAX_MANUAL_SAVES });
  }
  const id = newSaveId(
    kind,
    new Map(files.map((f) => [f.id, f.mtimeMs])),
    randomBytes(8),
    settings.autosaveCount,
  );
  const preview = await previewImage();
  const bytes = await requestSaves((request) => ({
    type: "saves:save",
    request,
    name,
    kind,
    savedAt: new Date().toISOString(),
    preview,
  }));
  if (!(bytes instanceof Uint8Array)) fail("invalidAnswer");
  const file = savePath(id);
  const temp = path.join(savesDir(), tempFileName(id, randomBytes(8)));
  try {
    await mkdir(savesDir(), { recursive: true });
    // Write, sync and rename: neither a crash nor a power loss while saving
    // destroys the previous state. "wx": never write through a placed file.
    const handle = await open(temp, "wx");
    try {
      await handle.writeFile(bytes);
      await handle.sync();
    } finally {
      await handle.close();
    }
    await rename(temp, file);
  } catch (error) {
    await rm(temp, { force: true }).catch(() => undefined);
    log("warn", `cannot write a save game: ${errorCode(error)}`);
    fail(writeErrorCode(error));
  }
  previews.delete(id);
  log("info", `saved ${kind} game ${id}`);
  return { id };
}

/** Checks the form of a save game id from the renderer. */
function saveId(arg: unknown): string {
  if (!isSaveId(arg)) fail("invalidArgument");
  return arg;
}

/** S-6 `saves.load(id)`: checks the file in the core and starts its session. */
async function loadGame(arg: unknown): Promise<SaveMeta> {
  const id = saveId(arg);
  return exclusive("load", async () => {
    const { handle, size } = await openSave(id);
    let bytes: Uint8Array;
    try {
      // Size and content from the same handle.
      bytes = await readExactly(handle, size, 0);
    } finally {
      await handle.close();
    }
    const meta = parseSaveMeta(
      await requestSaves((request) => ({ type: "saves:load", request, bytes })),
    );
    if (meta === undefined) fail("invalidAnswer");
    log("info", `loaded game ${id}`);
    return meta;
  });
}

/** S-6 `saves.delete(id)`; removes a link itself, never its target. */
async function deleteSave(arg: unknown): Promise<null> {
  const id = saveId(arg);
  return exclusive("delete", async () => {
    try {
      await rm(savePath(id));
    } catch (error) {
      if (errorCode(error) === "ENOENT") fail("unknownSave");
      fail("ioError");
    }
    previews.delete(id);
    log("info", `deleted game ${id}`);
    return null;
  });
}

/** S-6 `saves.preview(id)` (ADR-028): PNG bytes, empty if the save has none. */
async function savePreview(arg: unknown): Promise<ArrayBuffer> {
  const id = saveId(arg);
  return exclusive("preview", async () => {
    const place = previews.get(id);
    // Not listed since its last change: read the list first.
    if (place === undefined) fail("unknownSave");
    const { handle, size } = await openSave(id);
    let png: Uint8Array;
    try {
      if (place.offset + place.length > size) fail("unknownSave");
      png = await readExactly(handle, place.length, place.offset);
    } finally {
      await handle.close();
    }
    if (png.length > 0)
      await requestSaves((request) => ({ type: "saves:checkPreview", request, png }));
    return png.buffer as ArrayBuffer;
  });
}

/** Saves an autosave once no other operation runs (S-5 rotation). */
function runAutosave(): void {
  if (!autosaveDue || operation !== undefined) return;
  if (core === undefined || globalThis.openairsideCoreStatus.state !== "ready") {
    autosaveDue = false;
    return;
  }
  autosaveDue = false;
  exclusive("autosave", () => writeSave("", "auto")).catch((error: unknown) => {
    const code = error instanceof PlatformFailure ? error.reason.code : errorCode(error);
    log("warn", `autosave failed: ${code}`);
  });
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
  const folder = path.join(app.getPath("userData"), LOGS_DIR);
  try {
    await mkdir(folder, { recursive: true });
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
  const folder = path.join(app.getPath("userData"), LOGS_DIR);
  try {
    mkdirSync(folder, { recursive: true });
  } catch (error) {
    console.warn(`cannot create the log folder: ${errorCode(error)}`);
  }
  logFile = new LogFile(
    {
      size: (file) => {
        try {
          return statSync(file).size;
        } catch {
          return 0;
        }
      },
      append: (file, text) => {
        appendFileSync(file, text);
      },
      rename: (from, to) => {
        try {
          renameSync(from, to);
        } catch {
          // Missing older file.
        }
      },
      remove: (file) => {
        rmSync(file, { force: true });
      },
    },
    path.join(folder, LOG_FILE),
    app.isPackaged ? "info" : "debug",
  );
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

// Spike 3.14: graphics backend chosen with the console command `angle`.
{
  const angle = chosenAngle();
  if (angle !== "default" && !app.commandLine.hasSwitch("use-angle")) {
    app.commandLine.appendSwitch("use-angle", angle);
  }
}

void app.whenReady().then(() => {
  openLogFile();
  settings = readSettings();
  removeTempFiles();
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
  handle("measure:trace", recordTrace);
  handle("measure:setAngle", (value: unknown) => {
    if (typeof value !== "string" || !(ANGLE_VALUES as readonly string[]).includes(value)) {
      fail("invalidArgument");
    }
    writeFileSync(path.join(app.getPath("userData"), ANGLE_FILE), value);
    log("info", `spike 3.14: angle ${value}, restarting`);
    app.relaunch();
    app.exit(0);
    return null;
  });
  handle("measure:fullscreen", (on: unknown) => {
    if (typeof on !== "boolean") fail("invalidArgument");
    gameWindow?.setFullScreen(on);
    return null;
  });
  handle("packs:list", () => packReports);
  handle("packs:setActive", setActivePacks);
  handle("saves:list", () => exclusive("list", listSaves));
  handle("saves:save", saveGame);
  handle("saves:load", loadGame);
  handle("saves:delete", deleteSave);
  handle("saves:preview", savePreview);
  handle("logs:write", writeLog);
  handle("logs:openFolder", openLogFolder);
  handle("settings:get", () => settings);
  handle("settings:set", setSettings);
  handle("lang:list", () => languageList);
  handle("lang:texts", languageTexts);
  startCore();
  createWindow();
});

app.on("before-quit", () => {
  quitting = true;
});

app.on("window-all-closed", () => {
  app.quit();
});
