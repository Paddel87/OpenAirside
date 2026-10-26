import { describe, expect, it } from "vitest";
import type { SaveMeta } from "../generated/SaveMeta.js";
import {
  AutosaveClock,
  idOfFileName,
  isSaveId,
  isSaveName,
  isTempFileName,
  previewSize,
  tempFileName,
  withinChars,
  newSaveId,
  parseInfoAnswer,
  parseNewGameArgs,
  parseSaveArgs,
  parseSaveMeta,
  type SaveEntry,
  saveFileName,
  sortEntries,
  writeErrorCode,
} from "./saves.js";

const meta = (kind: SaveMeta["kind"], savedAt: string): SaveMeta => ({
  name: "",
  kind,
  savedAt,
  tick: 30,
  day: 1,
  year: 1,
  airportName: "",
  map: { width: 2048, height: 2048 },
  packs: [{ id: "base", version: "0.0.0", format: 1 }],
});

const ok = (id: string, kind: SaveMeta["kind"], savedAt: string): SaveEntry => ({
  id,
  state: "ok",
  gameVersion: "0.0.0",
  meta: meta(kind, savedAt),
});

const damaged = (id: string): SaveEntry => ({
  id,
  state: "damaged",
  error: { code: "badMagic", textKey: "save.error.badMagic", params: {} },
});

describe("save game ids and file names", () => {
  it("accepts only ids the main process gives out", () => {
    for (const id of ["quick", "auto-1", "auto-20", "m-0123456789abcdef"]) {
      expect(isSaveId(id)).toBe(true);
    }
    for (const id of ["", "auto-0", "auto-21", "m-xyz", "../quick", "quick.oasave", 3, null]) {
      expect(isSaveId(id)).toBe(false);
    }
  });

  it("maps ids to file names and back, ignoring other files", () => {
    expect(saveFileName("auto-3")).toBe("auto-3.oasave");
    expect(idOfFileName("auto-3.oasave")).toBe("auto-3");
    expect(idOfFileName("auto-3.oasave.tmp")).toBeUndefined();
    expect(idOfFileName("my game.oasave")).toBeUndefined();
    expect(idOfFileName("notes.txt")).toBeUndefined();
    expect(isTempFileName("quick.oasave.tmp")).toBe(true);
    const temp = tempFileName("quick", Uint8Array.from([1, 2, 3, 4, 5, 6, 7, 255]));
    expect(temp).toBe("quick.oasave.01020304050607ff.tmp");
    expect(isTempFileName(temp)).toBe(true);
    expect(idOfFileName(temp)).toBeUndefined();
    expect(isTempFileName("quick.oasave")).toBe(false);
  });
});

describe("arguments of saves.save", () => {
  it("checks name and kind like the core", () => {
    expect(parseSaveArgs({ name: "Mein Flughafen", kind: "manual" })).toEqual({
      name: "Mein Flughafen",
      kind: "manual",
    });
    expect(parseSaveArgs({ name: "", kind: "quick" })).toEqual({ name: "", kind: "quick" });
    expect(parseSaveArgs({ name: "x".repeat(64), kind: "auto" })?.name).toHaveLength(64);
    expect(parseSaveArgs({ name: "x".repeat(65), kind: "manual" })).toBeUndefined();
    expect(parseSaveArgs({ name: "a\nb", kind: "manual" })).toBeUndefined();
    expect(parseSaveArgs({ name: "a\u0085b", kind: "manual" })).toBeUndefined();
    expect(parseSaveArgs({ name: "a", kind: "other" })).toBeUndefined();
    expect(parseSaveArgs(["a", "manual"])).toBeUndefined();
    expect(parseSaveArgs(null)).toBeUndefined();
  });

  it("counts characters, not UTF-16 units", () => {
    expect(isSaveName("✈".repeat(64))).toBe(true);
    expect(isSaveName("😀".repeat(64))).toBe(true);
    expect(isSaveName("😀".repeat(65))).toBe(false);
    expect(isSaveName(5)).toBe(false);
  });
});

describe("ids of new save games", () => {
  const random = Uint8Array.from([0, 1, 2, 3, 250, 251, 252, 255, 9]);

  it("gives a manual save a random id and a quick save its single slot", () => {
    expect(newSaveId("manual", new Map(), random)).toBe("m-00010203fafbfcff");
    expect(newSaveId("quick", new Map([["quick", 1]]), random)).toBe("quick");
  });

  it("fills free autosave slots first, then overwrites the one modified longest ago", () => {
    expect(newSaveId("auto", new Map(), random)).toBe("auto-1");
    expect(newSaveId("auto", new Map([["auto-1", 10]]), random)).toBe("auto-2");
    const full = new Map([1, 2, 3, 4, 5].map((n) => [`auto-${String(n)}`, (n % 5) * 100]));
    expect(newSaveId("auto", full, random)).toBe("auto-5");
    expect(newSaveId("auto", full, random, 1)).toBe("auto-1");
  });

  it("rotates only over the chosen number of slots and keeps the others (step 2.6)", () => {
    // Count lowered from 5 to 2: auto-3 … auto-5 stay untouched, even if older.
    const files = new Map([
      ["auto-1", 500],
      ["auto-2", 400],
      ["auto-5", 1],
    ]);
    expect(newSaveId("auto", files, random, 2)).toBe("auto-2");
    // Count raised to 20: the first free slot comes next.
    expect(newSaveId("auto", files, random, 20)).toBe("auto-3");
    const twenty = new Map(
      Array.from({ length: 20 }, (_, i) => [`auto-${String(i + 1)}`, 100 - i] as const),
    );
    expect(newSaveId("auto", twenty, random, 20)).toBe("auto-20");
    expect(isSaveId("auto-20")).toBe(true);
    expect(isSaveId("auto-21")).toBe(false);
  });
});

describe("list order", () => {
  it("shows the newest first and damaged files last", () => {
    const sorted = sortEntries([
      damaged("m-0000000000000002"),
      ok("auto-1", "auto", "2026-10-19T10:00:00Z"),
      damaged("m-0000000000000001"),
      ok("quick", "quick", "2026-10-19T11:00:00Z"),
    ]);
    expect(sorted.map((e) => e.id)).toEqual([
      "quick",
      "auto-1",
      "m-0000000000000001",
      "m-0000000000000002",
    ]);
  });
});

describe("answers of the core process", () => {
  const info = {
    gameVersion: "0.0.0",
    meta: meta("manual", "2026-10-19T11:00:00Z"),
    previewOffset: 140,
    previewLen: 2000,
  };

  it("accepts metadata and errors of the expected shape", () => {
    expect(parseInfoAnswer({ ok: true, info })).toEqual({ ok: true, info });
    expect(
      parseInfoAnswer({
        ok: false,
        error: { code: "truncated", textKey: "save.error.truncated", params: {} },
      }),
    ).toEqual({
      ok: false,
      error: { code: "truncated", textKey: "save.error.truncated", params: {} },
    });
  });

  it("refuses unexpected shapes", () => {
    expect(parseInfoAnswer(null)).toBeUndefined();
    expect(parseInfoAnswer({ ok: false, error: "x" })).toBeUndefined();
    expect(parseInfoAnswer({ ok: true, info: { ...info, previewLen: -1 } })).toBeUndefined();
    expect(parseInfoAnswer({ ok: true, info: { ...info, previewLen: 3_000_000 } })).toBeUndefined();
    expect(parseInfoAnswer({ ok: true, info: { ...info, gameVersion: 1 } })).toBeUndefined();
    expect(parseInfoAnswer({ ok: true, info: { ...info, meta: {} } })).toBeUndefined();
    expect(parseSaveMeta({ ...info.meta, kind: "other" })).toBeUndefined();
    expect(parseSaveMeta({ ...info.meta, map: { width: "a" } })).toBeUndefined();
    expect(parseSaveMeta({ ...info.meta, packs: [{ id: "base" }] })).toBeUndefined();
    expect(parseSaveMeta({ ...info.meta, packs: "base" })).toBeUndefined();
  });
});

describe("write errors", () => {
  it("tells a full disk from other errors", () => {
    expect(writeErrorCode({ code: "ENOSPC" })).toBe("diskFull");
    expect(writeErrorCode({ code: "EDQUOT" })).toBe("diskFull");
    expect(writeErrorCode({ code: "EACCES" })).toBe("ioError");
    expect(writeErrorCode(new Error("x"))).toBe("ioError");
  });
});

describe("autosave clock", () => {
  it("counts only running time and starts again with every session", () => {
    const clock = new AutosaveClock(10_000);
    expect(clock.observe(0, 1, true)).toBe(false);
    expect(clock.observe(6_000, 1, true)).toBe(false);
    // Paused: no time counts.
    expect(clock.observe(60_000, 1, false)).toBe(false);
    expect(clock.observe(63_000, 1, true)).toBe(false);
    expect(clock.observe(64_000, 1, true)).toBe(true);
    expect(clock.observe(65_000, 1, true)).toBe(false);
    // A new session (loaded game) resets the clock.
    expect(clock.observe(74_500, 2, true)).toBe(false);
    expect(clock.observe(84_000, 2, true)).toBe(false);
    expect(clock.observe(84_600, 2, true)).toBe(true);
  });

  it("takes a new interval at once and keeps the time counted so far (step 2.6)", () => {
    const clock = new AutosaveClock(10_000);
    expect(clock.observe(0, 1, true)).toBe(false);
    expect(clock.observe(4_000, 1, true)).toBe(false);
    clock.setInterval(3_000);
    expect(clock.observe(4_001, 1, true)).toBe(true);
    clock.setInterval(60_000);
    expect(clock.observe(50_000, 1, true)).toBe(false);
    expect(clock.observe(64_002, 1, true)).toBe(true);
  });
});

describe("bounds against huge or odd input (separate review of 2.5b)", () => {
  it("refuses a huge string before counting its characters", () => {
    expect(withinChars("x".repeat(64), 64)).toBe(true);
    expect(withinChars("😀".repeat(64), 64)).toBe(true);
    expect(withinChars("😀".repeat(65), 64)).toBe(false);
    expect(withinChars("x".repeat(10_000_000), 64)).toBe(false);
    expect(isSaveName("x".repeat(10_000_000))).toBe(false);
  });

  it("fits the preview into 480 pixels width and 1920 × 1080", () => {
    expect(previewSize({ width: 1280, height: 800 })).toEqual({ width: 480, height: 300 });
    expect(previewSize({ width: 400, height: 300 })).toEqual({ width: 400, height: 300 });
    expect(previewSize({ width: 600, height: 1400 })).toEqual({ width: 462, height: 1080 });
    expect(previewSize({ width: 1, height: 100_000 })).toBeUndefined();
    expect(previewSize({ width: 0, height: 10 })).toBeUndefined();
  });
});

describe("parseNewGameArgs (S-6 game.new)", () => {
  it("accepts exactly the two map sizes", () => {
    expect(parseNewGameArgs({ map: 1536 })).toEqual({ map: 1536 });
    expect(parseNewGameArgs({ map: 2048 })).toEqual({ map: 2048 });
    expect(parseNewGameArgs({ map: 1024 })).toBeUndefined();
    expect(parseNewGameArgs({ map: "2048" })).toBeUndefined();
    expect(parseNewGameArgs({})).toBeUndefined();
    expect(parseNewGameArgs({ map: 2048, name: "x" })).toBeUndefined();
    expect(parseNewGameArgs([2048])).toBeUndefined();
    expect(parseNewGameArgs(null)).toBeUndefined();
    expect(parseNewGameArgs(2048)).toBeUndefined();
  });
});
