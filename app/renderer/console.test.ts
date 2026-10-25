import { describe, expect, it } from "vitest";
import type { PackReport } from "../src/generated/PackReport.js";
import {
  basePackLoaded,
  CONSOLE_PANEL,
  formatPacks,
  HELP,
  parseConsoleLine,
  TABLE_COLUMNS,
} from "./console.js";

describe("parseConsoleLine", () => {
  it.each([
    ["speed x3", { kind: "setSpeed", speed: "x3" }],
    ["  SPEED   Paused ", { kind: "setSpeed", speed: "paused" }],
    ["night", { kind: "skipNight" }],
    ["spawn 10000", { kind: "debug", action: { kind: "spawnPersons", count: 10_000 } }],
    ["clear", { kind: "debug", action: { kind: "clearPersons" } }],
    [
      "build base:bench 10 20",
      { kind: "build", object: "base:bench", at: { x: 10, y: 20, floor: 0 }, rotation: 0 },
    ],
    [
      "build my-mod:kiosk 1 2 -1",
      { kind: "build", object: "my-mod:kiosk", at: { x: 1, y: 2, floor: -1 }, rotation: 0 },
    ],
    [
      "walls 0 10 20 30 40 on",
      {
        kind: "debug",
        action: {
          kind: "setWalls",
          area: { floor: 0, x0: 10, y0: 20, x1: 30, y1: 40 },
          wall: true,
        },
      },
    ],
    [
      "walls -1 0 0 2 2 off",
      {
        kind: "debug",
        action: { kind: "setWalls", area: { floor: -1, x0: 0, y0: 0, x1: 2, y1: 2 }, wall: false },
      },
    ],
    [
      "scenario reference 10000",
      {
        kind: "debug",
        action: { kind: "loadScenario", scenario: "referenceTerminal", persons: 10_000 },
      },
    ],
    [
      "scenario hall",
      { kind: "debug", action: { kind: "loadScenario", scenario: "openHall", persons: 0 } },
    ],
    [
      "scenario furnished 10000",
      {
        kind: "debug",
        action: { kind: "loadScenario", scenario: "furnishedTerminal", persons: 10_000 },
      },
    ],
  ])("parses %j", (line, command) => {
    expect(parseConsoleLine(line)).toEqual({ kind: "command", command });
  });

  it("parses a new game and the detail switch", () => {
    expect(parseConsoleLine("newgame")).toEqual({ kind: "newGame", map: 1536 });
    expect(parseConsoleLine("newgame 2048")).toEqual({ kind: "newGame", map: 2048 });
    expect(parseConsoleLine("details on")).toEqual({ kind: "details", all: true });
    expect(parseConsoleLine("details off")).toEqual({ kind: "details", all: false });
    for (const line of ["newgame 64", "newgame 1536 1", "details", "details maybe"]) {
      expect(parseConsoleLine(line).kind, line).toBe("error");
    }
  });

  it("shows the help for an empty line", () => {
    expect(parseConsoleLine("")).toEqual({ kind: "help", text: HELP });
    expect(parseConsoleLine("help")).toEqual({ kind: "help", text: HELP });
  });

  it.each([
    "speed x4",
    "speed",
    "spawn 0",
    "spawn many",
    "spawn 1e3",
    "walls 0 1 2 3",
    "walls 0 1 2 3 4 maybe",
    "scenario castle",
    "scenario hall -5",
    "table 0",
    "table 1001",
    "measure 0",
    "measure 61",
    "measure soon",
    "packs list",
    "build bench 1 2",
    "build base:bench 1",
    "build base:bench 1 2 0 9",
    "build base:bench x 2",
    "build base:bench 1 2 up",
    "packs set ab",
    "fly",
  ])("rejects %j with a hint", (line) => {
    expect(parseConsoleLine(line).kind).toBe("error");
  });

  it("builds the table query of S-7", () => {
    const result = parseConsoleLine("table 20");
    expect(result).toEqual({
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
        window: { offset: 0, limit: 20 },
      },
    });
    const short = parseConsoleLine("table");
    expect(short.kind === "query" && short.query.kind === "table" && short.query.window.limit).toBe(
      5,
    );
  });

  it("starts the measurement series", () => {
    expect(parseConsoleLine("measure")).toEqual({ kind: "measure", minutes: 10 });
    expect(parseConsoleLine("MEASURE 3")).toEqual({ kind: "measure", minutes: 3 });
    expect(parseConsoleLine("measure 60")).toEqual({ kind: "measure", minutes: 60 });
  });
});

describe("packs", () => {
  const base: PackReport = {
    folder: "base",
    source: "base",
    id: "base",
    name: "OpenAirside",
    version: "0.0.0",
    state: "active",
    entries: 3,
    errors: [],
    errorsOmitted: 0,
  };
  const broken: PackReport = {
    folder: "My Mod",
    source: "user",
    id: null,
    name: null,
    version: null,
    state: "failed",
    entries: 0,
    errors: [
      {
        file: "objects/a.json",
        line: 3,
        column: 12,
        path: null,
        reason: {
          code: "syntaxError",
          textKey: "datapack.error.syntaxError",
          params: { detail: "expected value" },
        },
      },
      {
        file: null,
        line: null,
        column: null,
        path: null,
        reason: { code: "ioError", textKey: "datapack.error.ioError", params: {} },
      },
    ],
    errorsOmitted: 2,
  };

  it("parses the pack commands", () => {
    expect(parseConsoleLine("packs")).toEqual({ kind: "packs" });
    expect(parseConsoleLine("packs set my-mod other")).toEqual({
      kind: "setPacks",
      ids: ["my-mod", "other"],
    });
    expect(parseConsoleLine("packs set")).toEqual({ kind: "setPacks", ids: [] });
  });

  it("formats reports with errors and their places", () => {
    expect(formatPacks([base, broken])).toEqual([
      "active base base 0.0.0 – 3 entries",
      "failed user My Mod",
      "  syntaxError objects/a.json:3:12 – expected value",
      "  ioError",
      "  … 2 more errors",
    ]);
    const withPath: PackReport = {
      ...broken,
      errors: [
        {
          file: "manifest.json",
          line: 2,
          column: null,
          path: "id",
          reason: { code: "invalidId", textKey: "datapack.error.invalidId", params: {} },
        },
      ],
      errorsOmitted: 0,
    };
    expect(formatPacks([withPath])[1]).toBe("  invalidId manifest.json:2 id");
  });

  it("tells whether the base pack loaded", () => {
    expect(basePackLoaded([base, broken])).toBe(true);
    expect(basePackLoaded([{ ...base, state: "failed" }])).toBe(false);
    expect(basePackLoaded([broken])).toBe(false);
    expect(basePackLoaded([])).toBe(false);
  });
});

describe("save game commands (step 2.5)", () => {
  it.each([
    ["saves", { kind: "saves" }],
    ["save", { kind: "save", name: "", saveKind: "manual" }],
    ["save  Mein Flughafen ", { kind: "save", name: "Mein Flughafen", saveKind: "manual" }],
    ["quicksave", { kind: "save", name: "", saveKind: "quick" }],
    ["load auto-1", { kind: "load", id: "auto-1" }],
    ["delete m-0011223344556677", { kind: "deleteSave", id: "m-0011223344556677" }],
  ])("%s", (line, expected) => {
    expect(parseConsoleLine(line)).toEqual(expected);
  });

  it.each([
    ["saves x", "saves"],
    [`save ${"x".repeat(65)}`, "save [name up to 64 characters]"],
    ["load", "load <id>"],
    ["load ../x", "load <id>"],
    ["delete a b", "delete <id>"],
  ])("refuses %s", (line, text) => {
    expect(parseConsoleLine(line)).toEqual({ kind: "error", text });
  });
});

describe("language and settings commands (step 2.6)", () => {
  it.each([
    ["lang", { kind: "languages" }],
    ["lang DE", { kind: "setLanguage", code: "de" }],
    ["lang pt-br", { kind: "setLanguage", code: "pt-br" }],
    ["settings", { kind: "settings" }],
  ])("parses %s", (line, expected) => {
    expect(parseConsoleLine(line)).toEqual(expected);
  });

  it.each(["lang de en", "lang ../x", "lang d", "settings now"])("refuses %s", (line) => {
    expect(parseConsoleLine(line).kind).toBe("error");
  });

  it("lists both commands in the help", () => {
    expect(HELP).toContain("lang [code]");
    expect(HELP).toContain("settings –");
  });
});
