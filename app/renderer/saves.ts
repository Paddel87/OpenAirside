/**
 * Save games in the renderer (S-5, S-6 `saves.*`; step 2.5): console
 * lines for the list and the choice of the autosave offered after a crash
 * of the core. Display only; files and checks belong to the platform and
 * the core.
 */
import type { SaveMeta } from "../src/generated/SaveMeta.js";
import type { SaveEntry } from "./platform.js";

/** Label of a save game: its name, or its kind for unnamed ones. */
export function saveLabel(meta: SaveMeta): string {
  if (meta.name !== "") return meta.name;
  if (meta.kind === "quick") return "quick save";
  if (meta.kind === "auto") return "autosave";
  return "(unnamed)";
}

/** Saving time as `YYYY-MM-DD HH:MM` (UTC) for the list. */
export function savedAtText(savedAt: string): string {
  const match = /^(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2})/.exec(savedAt);
  return match === null ? savedAt : `${match[1] ?? ""} ${match[2] ?? ""}`;
}

/** One line of the list for the console. */
export function formatEntry(entry: SaveEntry): string {
  if (entry.state === "damaged") return `${entry.id} damaged: ${entry.error.code}`;
  const { meta } = entry;
  return `${entry.id} ${meta.kind} "${saveLabel(meta)}" – ${savedAtText(meta.savedAt)}, year ${String(meta.year)} day ${String(meta.day)}, tick ${String(meta.tick)}`;
}

/** Console lines of `saves.list()`. */
export function formatSaves(entries: readonly SaveEntry[]): string[] {
  if (entries.length === 0) return ["saves: none"];
  return [`saves: ${String(entries.length)}`, ...entries.map((e) => `  ${formatEntry(e)}`)];
}

/** Newest readable autosave, offered after a crash of the core (S-6). */
export function latestAutosave(entries: readonly SaveEntry[]): string | undefined {
  let best: { id: string; at: string } | undefined;
  for (const entry of entries) {
    if (entry.state !== "ok" || entry.meta.kind !== "auto") continue;
    if (best === undefined || entry.meta.savedAt > best.at) {
      best = { id: entry.id, at: entry.meta.savedAt };
    }
  }
  return best?.id;
}
