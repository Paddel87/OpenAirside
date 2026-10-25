/**
 * Files of the save games in the user data folder (S-5 "Schreiben", S-6
 * `saves.*`; module "Plattform", step 3.10): listing, reading, writing and
 * deleting. File names come from ids, never from a save game name (S-6).
 * No Electron import; the folder is given by the caller.
 */
import { randomBytes } from "node:crypto";
import { readdirSync, rmSync } from "node:fs";
import { type FileHandle, lstat, mkdir, open, readdir, rename, rm } from "node:fs/promises";
import path from "node:path";
import { errorCode, type Log, OPEN_FLAGS, tryCreateFolder } from "./files.js";
import { fail, PlatformFailure } from "./platform-error.js";
import {
  idOfFileName,
  isTempFileName,
  MAX_FILE_BYTES,
  saveFileName,
  tempFileName,
  writeErrorCode,
} from "./saves.js";

/** A save game file with its modification time. */
export interface SaveFile {
  readonly id: string;
  readonly mtimeMs: number;
}

/** The first bytes of a save game file and its whole size. */
export interface SavePrefix {
  readonly prefix: Uint8Array;
  readonly size: number;
}

/** File operations on the save game folder. */
export interface SaveFiles {
  /** Removes files of interrupted saves; runs at start. */
  removeTempFiles(): void;
  /** Ids of the save game files with their modification times, newest first. */
  list(): Promise<SaveFile[]>;
  /** The first `max` bytes of the save `id` (all if it is shorter). */
  readPrefix(id: string, max: number): Promise<SavePrefix>;
  /** The whole save `id`. */
  readAll(id: string): Promise<Uint8Array>;
  /** `length` bytes at `offset`; `unknownSave` if the save is shorter. */
  readRange(id: string, offset: number, length: number): Promise<Uint8Array>;
  /** Writes the save `id` by write, sync and rename. */
  write(id: string, bytes: Uint8Array): Promise<void>;
  /** Deletes the save `id`; removes a link itself, never its target. */
  remove(id: string): Promise<void>;
}

/**
 * Opens a save game for reading and checks on the open handle that it is a
 * regular file within {@link MAX_FILE_BYTES} (finding 4 of the separate
 * review: no devices, FIFOs or links to them).
 */
async function openChecked(file: string): Promise<{ handle: FileHandle; size: number }> {
  let handle: FileHandle;
  try {
    handle = await open(file, OPEN_FLAGS);
  } catch (error) {
    if (errorCode(error) === "ENOENT") fail("unknownSave");
    fail("ioError");
  }
  try {
    const info = await handle.stat();
    if (!info.isFile()) fail("invalidState", { check: "file" });
    if (info.size > MAX_FILE_BYTES) fail("limitExceeded", { limit: "file", max: MAX_FILE_BYTES });
    return { handle, size: info.size };
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

/** Save game files in the folder given by `dir` (asked on every call). */
export function createSaveFiles(dir: () => string, log: Log): SaveFiles {
  const filePath = (id: string): string => path.join(dir(), saveFileName(id));

  /** Runs `use` on the opened save; the handle is closed in any case. */
  const withSave = async <T>(
    id: string,
    use: (handle: FileHandle, size: number) => Promise<T>,
  ): Promise<T> => {
    const { handle, size } = await openChecked(filePath(id));
    try {
      return await use(handle, size);
    } finally {
      await handle.close();
    }
  };

  return {
    removeTempFiles() {
      const folder = dir();
      const failure = tryCreateFolder(folder);
      if (failure !== undefined) {
        log("warn", `cannot read the save game folder: ${failure}`);
        return;
      }
      let names: string[];
      try {
        names = readdirSync(folder);
      } catch (error) {
        log("warn", `cannot read the save game folder: ${errorCode(error)}`);
        return;
      }
      for (const name of names) {
        if (!isTempFileName(name)) continue;
        try {
          rmSync(path.join(dir(), name), { force: true });
        } catch (error) {
          // E.g. a folder of that name: the others are still removed.
          log("warn", `cannot remove a temporary save game file: ${errorCode(error)}`);
        }
      }
    },

    async list() {
      let names: string[];
      try {
        names = await readdir(dir());
      } catch (error) {
        if (errorCode(error) === "ENOENT") return [];
        log("warn", `cannot read the save game folder: ${errorCode(error)}`);
        fail("ioError");
      }
      const files: SaveFile[] = [];
      for (const name of names) {
        const id = idOfFileName(name);
        if (id === undefined) continue;
        try {
          // lstat: a link or special file is listed and refused when opened.
          files.push({ id, mtimeMs: (await lstat(path.join(dir(), name))).mtimeMs });
        } catch {
          // Deleted meanwhile.
        }
      }
      return files.sort((a, b) => b.mtimeMs - a.mtimeMs);
    },

    readPrefix: (id, max) =>
      withSave(id, async (handle, size) => ({
        prefix: await readExactly(handle, Math.min(size, max), 0),
        size,
      })),

    // Size and content from the same handle.
    readAll: (id) => withSave(id, (handle, size) => readExactly(handle, size, 0)),

    readRange: (id, offset, length) =>
      withSave(id, (handle, size) => {
        if (offset + length > size) fail("unknownSave");
        return readExactly(handle, length, offset);
      }),

    async write(id, bytes) {
      const file = filePath(id);
      const temp = path.join(dir(), tempFileName(id, randomBytes(8)));
      try {
        await mkdir(dir(), { recursive: true });
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
    },

    async remove(id) {
      try {
        await rm(filePath(id));
      } catch (error) {
        if (errorCode(error) === "ENOENT") fail("unknownSave");
        fail("ioError");
      }
    },
  };
}
