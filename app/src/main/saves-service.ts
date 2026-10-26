/**
 * The handlers of `saves.*` of the platform API S-6 and the autosave
 * (module "Plattform", step 3.10): list, save, load, delete and preview of
 * save games. The core process builds and checks the files; this service
 * chooses ids, reads and writes through {@link SaveFiles} and passes bytes
 * to the core by `request`. No Electron import.
 */
import { randomBytes } from "node:crypto";
import type { SaveMeta } from "../generated/SaveMeta.js";
import type { SavesRequest } from "./core-status.js";
import type { Log } from "./files.js";
import { errorCode } from "./files.js";
import type { OperationLock } from "./operation-lock.js";
import { fail, PlatformFailure, platformReason } from "./platform-error.js";
import type { SaveFiles } from "./save-files.js";
import {
  INFO_BATCH,
  isSaveId,
  MAX_INFO_BYTES,
  MAX_LISTED_FILES,
  MAX_MANUAL_SAVES,
  newSaveId,
  parseInfoAnswer,
  parseNewGameArgs,
  parseSaveArgs,
  parseSaveMeta,
  type SaveEntry,
  sortEntries,
} from "./saves.js";

/** What the service needs from the main process. */
export interface SavesDeps {
  readonly files: SaveFiles;
  readonly lock: OperationLock;
  /** Sends a request to the core process and waits for its answer. */
  readonly request: (make: (request: number) => SavesRequest) => Promise<unknown>;
  /** Screenshot of the game window as PNG, empty if there is none. */
  readonly preview: () => Promise<Uint8Array>;
  /** Number of autosave slots from the settings. */
  readonly autosaveCount: () => number;
  /** True while the core process is ready for requests. */
  readonly coreReady: () => boolean;
  readonly log: Log;
}

/** Handlers of `saves.*`, the autosave and the quick save. */
export interface SavesService {
  list(): Promise<SaveEntry[]>;
  save(arg: unknown): Promise<{ id: string }>;
  load(arg: unknown): Promise<SaveMeta>;
  /** S-6 `game.new({ map })`: starts a new game; a new session begins. */
  newGame(arg: unknown): Promise<null>;
  delete(arg: unknown): Promise<null>;
  preview(arg: unknown): Promise<ArrayBuffer>;
  /** The core process asked for an autosave: runs it once no other operation runs. */
  autosaveRequested(): void;
}

/** Checks the form of a save game id from the renderer. */
function saveId(arg: unknown): string {
  if (!isSaveId(arg)) fail("invalidArgument");
  return arg;
}

export function createSavesService(deps: SavesDeps): SavesService {
  const { files, lock, request, log } = deps;
  /** Where the preview image of each listed save game lies (from `saves.list`). */
  const previews = new Map<string, { offset: number; length: number }>();

  async function listSaves(): Promise<SaveEntry[]> {
    const all = await files.list();
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
      const infos: { id: string; prefix: Uint8Array; size: number }[] = [];
      for (const { id } of listed.slice(start, start + INFO_BATCH)) {
        try {
          infos.push({ id, ...(await files.readPrefix(id, MAX_INFO_BYTES)) });
        } catch (error) {
          const reason =
            error instanceof PlatformFailure ? error.reason : platformReason("ioError");
          // Deleted meanwhile: not listed; otherwise shown as damaged.
          if (reason.code !== "unknownSave") entries.push({ id, state: "damaged", error: reason });
        }
      }
      const answers = await request((n) => ({
        type: "saves:info",
        request: n,
        files: infos.map(({ prefix, size }) => ({ prefix, size })),
      }));
      if (!Array.isArray(answers) || answers.length !== infos.length) fail("invalidAnswer");
      infos.forEach(({ id }, i) => {
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

  async function writeSave(name: string, kind: SaveMeta["kind"]): Promise<{ id: string }> {
    const existing = await files.list();
    if (
      kind === "manual" &&
      existing.filter((f) => f.id.startsWith("m-")).length >= MAX_MANUAL_SAVES
    ) {
      fail("limitExceeded", { limit: "manualSaves", max: MAX_MANUAL_SAVES });
    }
    const id = newSaveId(
      kind,
      new Map(existing.map((f) => [f.id, f.mtimeMs])),
      randomBytes(8),
      deps.autosaveCount(),
    );
    const preview = await deps.preview();
    const bytes = await request((n) => ({
      type: "saves:save",
      request: n,
      name,
      kind,
      savedAt: new Date().toISOString(),
      preview,
    }));
    if (!(bytes instanceof Uint8Array)) fail("invalidAnswer");
    await files.write(id, bytes);
    previews.delete(id);
    log("info", `saved ${kind} game ${id}`);
    return { id };
  }

  /** Saves an autosave once no other operation runs (S-5 rotation). */
  function runAutosave(): void {
    if (!lock.autosaveDue || lock.busy) return;
    lock.autosaveDue = false;
    if (!deps.coreReady()) return;
    lock
      .exclusive("autosave", () => writeSave("", "auto"))
      .catch((error: unknown) => {
        const code = error instanceof PlatformFailure ? error.reason.code : errorCode(error);
        log("warn", `autosave failed: ${code}`);
      });
  }
  lock.onIdle = runAutosave;

  return {
    list: () => lock.exclusive("list", listSaves),

    // S-6 `saves.save({ name, kind })`: writes a new file; resolves with its id.
    async save(arg) {
      const parsed = parseSaveArgs(arg);
      if (parsed === undefined) fail("invalidArgument");
      return lock.exclusive("save", () => writeSave(parsed.name, parsed.kind));
    },

    // S-6 `saves.load(id)`: checks the file in the core and starts its session.
    async load(arg) {
      const id = saveId(arg);
      return lock.exclusive("load", async () => {
        const bytes = await files.readAll(id);
        const meta = parseSaveMeta(
          await request((n) => ({ type: "saves:load", request: n, bytes })),
        );
        if (meta === undefined) fail("invalidAnswer");
        log("info", `loaded game ${id}`);
        return meta;
      });
    },

    // S-6 `game.new({ map })` (ADR-032, L-3): an empty world on a square map; like loading,
    // one operation at a time with the save games and packs.
    async newGame(arg) {
      const parsed = parseNewGameArgs(arg);
      if (parsed === undefined) fail("invalidArgument");
      return lock.exclusive("newGame", async () => {
        const answer = await request((n) => ({ type: "game:new", request: n, map: parsed.map }));
        if (answer !== null) fail("invalidAnswer");
        log("info", `new game on a map of ${String(parsed.map)} cells`);
        return null;
      });
    },

    // S-6 `saves.delete(id)`; removes a link itself, never its target.
    async delete(arg) {
      const id = saveId(arg);
      return lock.exclusive("delete", async () => {
        await files.remove(id);
        previews.delete(id);
        log("info", `deleted game ${id}`);
        return null;
      });
    },

    // S-6 `saves.preview(id)` (ADR-028): PNG bytes, empty if the save has none.
    async preview(arg) {
      const id = saveId(arg);
      return lock.exclusive("preview", async () => {
        const place = previews.get(id);
        // Not listed since its last change: read the list first.
        if (place === undefined) fail("unknownSave");
        const png = await files.readRange(id, place.offset, place.length);
        if (png.length > 0) {
          await request((n) => ({ type: "saves:checkPreview", request: n, png }));
        }
        return png.buffer as ArrayBuffer;
      });
    },

    autosaveRequested() {
      lock.autosaveDue = true;
      runAutosave();
    },
  };
}
