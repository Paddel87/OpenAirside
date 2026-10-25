/**
 * The file `packs-active.json` in the user data folder (S-6 `packs.*`,
 * module "Plattform", step 3.10): the player's saved choice of packs.
 * No Electron import; the path is given by the caller.
 */
import { readFileSync } from "node:fs";
import { writeFileAtomic } from "./files.js";
import { activeFileText, readActiveFile } from "./packs.js";

/** The player's saved choice of packs; missing or unreadable means none. */
export function readActivePacksFile(file: string): string[] {
  try {
    return readActiveFile(readFileSync(file, "utf8"));
  } catch {
    return [];
  }
}

/** Saves the choice; an interrupted write never leaves half a file. */
export function writeActivePacksFile(file: string, ids: readonly string[]): Promise<void> {
  return writeFileAtomic(file, activeFileText(ids));
}
