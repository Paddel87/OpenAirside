import { describe, expect, it } from "vitest";
import {
  activeFileText,
  choosableIds,
  isPackId,
  MAX_PACKS,
  packArgs,
  parseActiveIds,
  parsePackReports,
  readActiveFile,
} from "./packs.js";

const report = {
  folder: "my-mod",
  source: "user",
  id: "my-mod",
  name: "My Mod",
  version: "1.0.0",
  state: "inactive",
  entries: 0,
  errors: [
    {
      file: "manifest.json",
      line: 1,
      column: 2,
      path: "id",
      reason: {
        code: "invalidId",
        textKey: "datapack.error.invalidId",
        params: { id: "x", max: 3 },
      },
    },
  ],
  errorsOmitted: 0,
};

describe("ids", () => {
  it("follow the naming rule of the core", () => {
    expect(isPackId("base")).toBe(true);
    expect(isPackId("ab")).toBe(false);
    expect(isPackId("Base")).toBe(false);
    expect(isPackId(3)).toBe(false);
  });

  it("accept a list of distinct valid ids", () => {
    expect(parseActiveIds(["a-b", "c_d"])).toEqual(["a-b", "c_d"]);
    expect(parseActiveIds([])).toEqual([]);
  });

  it.each([
    [undefined],
    ["my-mod"],
    [["my-mod", "my-mod"]],
    [["ok-id", "X"]],
    [Array.from({ length: MAX_PACKS + 1 }, (_, i) => `p${String(i).padStart(4, "0")}`)],
  ])("refuse %j", (value) => {
    expect(parseActiveIds(value)).toBeUndefined();
  });
});

describe("active file", () => {
  it("round-trips the chosen ids", () => {
    expect(readActiveFile(activeFileText(["my-mod", "other"]))).toEqual(["my-mod", "other"]);
  });

  it.each([undefined, "", "{", "[]", '{"active": "x"}', '{"active": ["Bad"]}', "null"])(
    "reads %j as no choice",
    (text) => {
      expect(readActiveFile(text)).toEqual([]);
    },
  );
});

describe("packArgs", () => {
  it("passes folders and ids as flags", () => {
    expect(packArgs("/app/content/base", "/data/packs", ["a-b", "c_d"])).toEqual([
      "--base-pack=/app/content/base",
      "--user-packs=/data/packs",
      "--active-packs=a-b,c_d",
    ]);
  });
});

describe("parsePackReports", () => {
  it("accepts reports of the core", () => {
    expect(parsePackReports([report])).toEqual([report]);
    expect(parsePackReports([])).toEqual([]);
  });

  it.each([
    [null],
    [{}],
    [[{ ...report, source: "web" }]],
    [[{ ...report, state: "broken" }]],
    [[{ ...report, entries: -1 }]],
    [[{ ...report, id: 7 }]],
    [[{ ...report, errors: "none" }]],
    [[{ ...report, errors: [{ ...report.errors[0], line: 1.5 }] }]],
    [[{ ...report, errors: [{ ...report.errors[0], reason: { code: "x" } }] }]],
    [
      [
        {
          ...report,
          errors: [
            { ...report.errors[0], reason: { ...report.errors[0]?.reason, params: { a: {} } } },
          ],
        },
      ],
    ],
    [[{ ...report, errors: [null] }]],
    [Array.from({ length: MAX_PACKS + 2 }, () => report)],
  ])("refuses %j", (value) => {
    expect(parsePackReports(value)).toBeUndefined();
  });

  it("lists the ids a player may choose", () => {
    const base = { ...report, folder: "base", source: "base" as const, id: "base" };
    const broken = { ...report, folder: "broken", id: null };
    const reports = parsePackReports([report, base, broken]) ?? [];
    expect([...choosableIds(reports)]).toEqual(["my-mod"]);
  });
});
