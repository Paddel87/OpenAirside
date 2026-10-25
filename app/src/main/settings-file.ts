/**
 * The file `settings.json` in the user data folder (S-6 `settings.*`,
 * module "Plattform", step 3.10): reading at start and writing a change.
 * No Electron import; the path is given by the caller.
 */
import { closeSync, fstatSync, openSync, readFileSync } from "node:fs";
import { OPEN_FLAGS, type Log, writeFileAtomic } from "./files.js";
import type { Settings } from "../generated/Settings.js";
import { MAX_SETTINGS_BYTES, settingsFromFile } from "./settings.js";

/**
 * Reads the settings file; a missing file gives the defaults of a new
 * installation, a broken one its valid fields plus defaults and a warning.
 */
export function readSettingsFile(file: string, defaults: Settings, log: Log): Settings {
  let fd: number;
  try {
    // One handle for check and read; never blocks on a FIFO (finding 4 of
    // the separate review of 2.6).
    fd = openSync(file, OPEN_FLAGS);
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

/** Writes the settings file; an interrupted write never leaves half a file. */
export function writeSettingsFile(file: string, settings: Settings): Promise<void> {
  return writeFileAtomic(file, `${JSON.stringify(settings, null, 2)}\n`);
}
