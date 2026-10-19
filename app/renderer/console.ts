/**
 * Text console of the debug panel (FR-009, META-15): turns a typed line into
 * a command of S-2. Parsing only; whether a command is allowed decides the
 * core (rule 002).
 */
import type { ColumnId } from "../src/generated/ColumnId.js";
import type { Command } from "../src/generated/Command.js";
import type { PackReport } from "../src/generated/PackReport.js";
import type { Query } from "../src/generated/Query.js";
import type { Scenario } from "../src/generated/Scenario.js";
import type { Speed } from "../src/generated/Speed.js";
import { MAX_MINUTES } from "./measure.js";

/** Result of parsing one console line. */
export type ConsoleResult =
  | { readonly kind: "command"; readonly command: Command }
  | { readonly kind: "query"; readonly query: Query }
  | { readonly kind: "measure"; readonly minutes: number }
  | { readonly kind: "packs" }
  | { readonly kind: "setPacks"; readonly ids: readonly string[] }
  | { readonly kind: "saves" }
  | { readonly kind: "save"; readonly name: string; readonly saveKind: "manual" | "quick" }
  | { readonly kind: "load"; readonly id: string }
  | { readonly kind: "deleteSave"; readonly id: string }
  | { readonly kind: "languages" }
  | { readonly kind: "setLanguage"; readonly code: string }
  | { readonly kind: "settings" }
  | { readonly kind: "help"; readonly text: string }
  | { readonly kind: "error"; readonly text: string };

/** Help text of the console. */
export const HELP = [
  "speed paused|x1|x2|x3",
  "night",
  "spawn <1–10000>",
  "clear",
  "build <pack:entry> <x> <y> [floor] – build command with a content id (S-2, S-4)",
  "walls <floor> <x0> <y0> <x1> <y1> on|off",
  "scenario hall|reference [persons]",
  "table [rows 1–1000] – persons of the newest frame, sorted by y and x (S-7)",
  `measure [minutes 1–${String(MAX_MINUTES)}] – measurement series, result as JSON file (default 10)`,
  "packs – data packs with state and errors",
  "packs set [ids…] – choose the active user packs in load order (next session)",
  "saves – list of save games with previews",
  "save [name] – save the game (name up to 64 characters)",
  "quicksave – quick save (also F5)",
  "load <id> – load a save game (ids from saves)",
  "delete <id> – delete a save game",
  "lang [code] – languages of the loaded packs, or switch the language (no restart)",
  "settings – language and autosave settings",
].join("\n");

/** Form of a save game id (as `isSaveId` in app/src/main/saves.ts). */
const SAVE_ID = /^[a-z0-9-]{1,40}$/;

/** Panel id of the console's table query. */
export const CONSOLE_PANEL = 1;
/** Columns of the console's table query. */
export const TABLE_COLUMNS: readonly ColumnId[] = ["handle", "x", "y", "floor", "state"];

const SPEEDS: ReadonlySet<string> = new Set<Speed>(["paused", "x1", "x2", "x3"]);
const SCENARIOS: Readonly<Record<string, Scenario>> = {
  hall: "openHall",
  openhall: "openHall",
  reference: "referenceTerminal",
  referenceterminal: "referenceTerminal",
};

function integer(text: string | undefined): number | undefined {
  if (text === undefined || !/^-?\d{1,6}$/.test(text)) {
    return undefined;
  }
  return Number(text);
}

function error(text: string): ConsoleResult {
  return { kind: "error", text };
}

function command(command: Command): ConsoleResult {
  return { kind: "command", command };
}

/** Parses one console line. */
export function parseConsoleLine(line: string): ConsoleResult {
  const [name = "", ...args] = line.trim().toLowerCase().split(/\s+/);
  switch (name) {
    case "":
    case "help":
      return { kind: "help", text: HELP };
    case "speed": {
      const speed = args[0];
      return speed !== undefined && SPEEDS.has(speed)
        ? command({ kind: "setSpeed", speed: speed as Speed })
        : error("speed paused|x1|x2|x3");
    }
    case "night":
      return command({ kind: "skipNight" });
    case "spawn": {
      const count = integer(args[0]);
      return count !== undefined && count > 0
        ? command({ kind: "debug", action: { kind: "spawnPersons", count } })
        : error("spawn <1–10000>");
    }
    case "clear":
      return command({ kind: "debug", action: { kind: "clearPersons" } });
    case "build": {
      const [object = "", ...rest] = args;
      const [x, y] = rest.slice(0, 2).map(integer);
      const floor = rest[2] === undefined ? 0 : integer(rest[2]);
      if (
        !/^[a-z0-9_-]{3,64}:[a-z0-9_-]{1,64}$/.test(object) ||
        x === undefined ||
        y === undefined ||
        floor === undefined ||
        rest.length > 3
      ) {
        return error("build <pack:entry> <x> <y> [floor]");
      }
      return command({ kind: "build", object, at: { x, y, floor }, rotation: 0 });
    }
    case "walls": {
      const [floor, x0, y0, x1, y1] = args.slice(0, 5).map(integer);
      const mode = args[5];
      if (
        floor === undefined ||
        x0 === undefined ||
        y0 === undefined ||
        x1 === undefined ||
        y1 === undefined ||
        (mode !== "on" && mode !== "off")
      ) {
        return error("walls <floor> <x0> <y0> <x1> <y1> on|off");
      }
      return command({
        kind: "debug",
        action: { kind: "setWalls", area: { floor, x0, y0, x1, y1 }, wall: mode === "on" },
      });
    }
    case "scenario": {
      const scenario = SCENARIOS[args[0] ?? ""];
      const persons = args[1] === undefined ? 0 : integer(args[1]);
      if (scenario === undefined || persons === undefined || persons < 0) {
        return error("scenario hall|reference [persons]");
      }
      return command({ kind: "debug", action: { kind: "loadScenario", scenario, persons } });
    }
    case "table": {
      const limit = args[0] === undefined ? 5 : integer(args[0]);
      if (limit === undefined || limit < 1 || limit > 1000) {
        return error("table [rows 1–1000]");
      }
      return {
        kind: "query",
        query: {
          kind: "table",
          panel: CONSOLE_PANEL,
          table: "debugPersons",
          columns: [...TABLE_COLUMNS],
          filter: [],
          sort: [
            { column: "y", descending: false },
            { column: "x", descending: false },
          ],
          window: { offset: 0, limit },
        },
      };
    }
    case "packs": {
      if (args.length === 0) return { kind: "packs" };
      const [verb, ...ids] = args;
      if (verb !== "set" || !ids.every((id) => /^[a-z0-9_-]{3,64}$/.test(id))) {
        return error("packs | packs set [ids…]");
      }
      return { kind: "setPacks", ids };
    }
    case "saves":
      return args.length === 0 ? { kind: "saves" } : error("saves");
    case "save": {
      const name = line.trim().slice("save".length).trim();
      return Array.from(name).length > 64
        ? error("save [name up to 64 characters]")
        : { kind: "save", name, saveKind: "manual" };
    }
    case "quicksave":
      return { kind: "save", name: "", saveKind: "quick" };
    case "load":
    case "delete": {
      const [id] = args;
      if (args.length !== 1 || id === undefined || !SAVE_ID.test(id)) return error(`${name} <id>`);
      return name === "load" ? { kind: "load", id } : { kind: "deleteSave", id };
    }
    case "lang": {
      const [code] = args;
      if (args.length === 0) return { kind: "languages" };
      return args.length === 1 && code !== undefined && /^[a-z]{2,3}(?:-[a-z0-9]{2,8})?$/.test(code)
        ? { kind: "setLanguage", code }
        : error("lang [code]");
    }
    case "settings":
      return args.length === 0 ? { kind: "settings" } : error("settings");
    case "measure": {
      const minutes = args[0] === undefined ? 10 : integer(args[0]);
      if (minutes === undefined || minutes < 1 || minutes > MAX_MINUTES) {
        return error(`measure [minutes 1–${String(MAX_MINUTES)}]`);
      }
      return { kind: "measure", minutes };
    }
    default:
      return error(`unknown command "${name}" – try help`);
  }
}

/** Place of a pack error: file, line and column, JSON path. */
function place(error: PackReport["errors"][number]): string {
  const at =
    error.line === null
      ? ""
      : `:${String(error.line)}${error.column === null ? "" : `:${String(error.column)}`}`;
  const file = error.file === null ? "" : ` ${error.file}${at}`;
  return `${file}${error.path === null ? "" : ` ${error.path}`}`;
}

/** Console lines for the pack reports of `packs.list()` (step 2.4). */
export function formatPacks(reports: readonly PackReport[]): string[] {
  const lines: string[] = [];
  for (const r of reports) {
    const name = r.id ?? r.folder;
    const version = r.version === null ? "" : ` ${r.version}`;
    const entries = r.state === "active" ? ` – ${String(r.entries)} entries` : "";
    lines.push(`${r.state} ${r.source} ${name}${version}${entries}`);
    for (const error of r.errors) {
      const detail = error.reason.params.detail;
      lines.push(
        `  ${error.reason.code}${place(error)}${typeof detail === "string" ? ` – ${detail}` : ""}`,
      );
    }
    if (r.errorsOmitted > 0) lines.push(`  … ${String(r.errorsOmitted)} more errors`);
  }
  return lines;
}

/** Whether every base pack loaded (S-4: a faulty base pack is reported at start). */
export function basePackLoaded(reports: readonly PackReport[]): boolean {
  return (
    reports.some((r) => r.source === "base") &&
    reports.every((r) => r.source !== "base" || r.state === "active")
  );
}
