/**
 * Text console of the debug panel (FR-009, META-15): turns a typed line into
 * a command of S-2. Parsing only; whether a command is allowed decides the
 * core (rule 002).
 */
import type { Command } from "../src/generated/Command.js";
import type { Scenario } from "../src/generated/Scenario.js";
import type { Speed } from "../src/generated/Speed.js";

/** Result of parsing one console line. */
export type ConsoleResult =
  | { readonly kind: "command"; readonly command: Command }
  | { readonly kind: "help"; readonly text: string }
  | { readonly kind: "error"; readonly text: string };

/** Help text of the console. */
export const HELP = [
  "speed paused|x1|x2|x3",
  "night",
  "spawn <1–10000>",
  "clear",
  "walls <floor> <x0> <y0> <x1> <y1> on|off",
  "scenario hall|reference [persons]",
].join("\n");

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
    default:
      return error(`unknown command "${name}" – try help`);
  }
}
