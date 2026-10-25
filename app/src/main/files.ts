/**
 * File access helpers of the main process (module "Plattform", step 3.10):
 * everything that touches the file system for settings, packs, logs and the
 * measurement result lives in this module and the modules next to it, so
 * that `main.ts` only wires. No Electron import: tested on real folders.
 */
import {
  appendFileSync,
  constants as fsConstants,
  mkdirSync,
  renameSync,
  rmSync,
  statSync,
} from "node:fs";
import { mkdir, rename, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import type { LogFs, LogLevel } from "./logs.js";

/** Writes an entry of the main process to the console and the log file. */
export type Log = (level: LogLevel, message: string) => void;

/** Code of a Node.js error (`ENOENT` …) without its message, which names paths. */
export function errorCode(error: unknown): string {
  const code =
    typeof error === "object" && error !== null && "code" in error ? error.code : undefined;
  return typeof code === "string" ? code : "unknown";
}

/** Flags for opening a user file: never blocks on a FIFO (POSIX only). */
export const OPEN_FLAGS =
  fsConstants.O_RDONLY | (process.platform === "win32" ? 0 : fsConstants.O_NONBLOCK);

/**
 * Creates `folder` if needed; returns the error code of a failure, never
 * throws (the callers go on and log a warning).
 */
export function tryCreateFolder(folder: string): string | undefined {
  try {
    mkdirSync(folder, { recursive: true });
    return undefined;
  } catch (error) {
    return errorCode(error);
  }
}

/** Creates `folder` if needed; rejects with the Node.js error. */
export async function createFolder(folder: string): Promise<void> {
  await mkdir(folder, { recursive: true });
}

/**
 * Writes `text` to `file` by write and rename: an interrupted write never
 * leaves half a file. Creates the folder; rejects with the Node.js error
 * (the temporary file is removed).
 */
export async function writeFileAtomic(file: string, text: string): Promise<void> {
  const temp = `${file}.tmp`;
  try {
    await mkdir(path.dirname(file), { recursive: true });
    await writeFile(temp, text);
    await rename(temp, file);
  } catch (error) {
    await rm(temp, { force: true }).catch(() => undefined);
    throw error;
  }
}

/**
 * Writes a new file `name` in `folder` (created if needed) and returns its
 * path. "wx": never overwrites an existing file.
 */
export async function writeNewFile(folder: string, name: string, text: string): Promise<string> {
  await mkdir(folder, { recursive: true });
  const file = path.join(folder, name);
  await writeFile(file, text, { flag: "wx" });
  return file;
}

/** File operations of the log file on the real file system. */
export const nodeLogFs: LogFs = {
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
};
