import { describe, expect, it } from "vitest";
import { CONSOLE_PANEL, HELP, parseConsoleLine, TABLE_COLUMNS } from "./console.js";

describe("parseConsoleLine", () => {
  it.each([
    ["speed x3", { kind: "setSpeed", speed: "x3" }],
    ["  SPEED   Paused ", { kind: "setSpeed", speed: "paused" }],
    ["night", { kind: "skipNight" }],
    ["spawn 10000", { kind: "debug", action: { kind: "spawnPersons", count: 10_000 } }],
    ["clear", { kind: "debug", action: { kind: "clearPersons" } }],
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
  ])("parses %j", (line, command) => {
    expect(parseConsoleLine(line)).toEqual({ kind: "command", command });
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
});
