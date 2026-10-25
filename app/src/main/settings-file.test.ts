import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { readActivePacksFile, writeActivePacksFile } from "./packs-file.js";
import { readSettingsFile, writeSettingsFile } from "./settings-file.js";
import { defaultSettings, MAX_SETTINGS_BYTES } from "./settings.js";

let dir: string;
let lines: string[];
const log = (level: string, message: string): void => {
  lines.push(`${level}: ${message}`);
};

beforeEach(() => {
  dir = mkdtempSync(path.join(os.tmpdir(), "oa-settings-"));
  lines = [];
});

afterEach(() => {
  rmSync(dir, { recursive: true, force: true });
});

const defaults = defaultSettings(["en"]);

describe("settings file", () => {
  it("gives the defaults of a new installation without a file or warning", () => {
    expect(readSettingsFile(path.join(dir, "settings.json"), defaults, log)).toEqual(defaults);
    expect(lines).toEqual([]);
  });

  it("reads back what it wrote", async () => {
    const file = path.join(dir, "settings.json");
    const settings = { language: "de", autosaveMinutes: 5, autosaveCount: 3 };
    await writeSettingsFile(file, settings);
    expect(readSettingsFile(file, defaults, log)).toEqual(settings);
    expect(readFileSync(file, "utf8").endsWith("}\n")).toBe(true);
    expect(lines).toEqual([]);
  });

  it("takes the valid fields of a partly broken file and warns", () => {
    const file = path.join(dir, "settings.json");
    writeFileSync(file, JSON.stringify({ language: "de", autosaveMinutes: 9999 }));
    expect(readSettingsFile(file, defaults, log)).toEqual({ ...defaults, language: "de" });
    expect(lines).toEqual(["warn: settings file partly invalid; invalid values replaced"]);
  });

  it("uses the defaults for a file that is not JSON", () => {
    const file = path.join(dir, "settings.json");
    writeFileSync(file, "{not json");
    expect(readSettingsFile(file, defaults, log)).toEqual(defaults);
    expect(lines).toEqual(["warn: settings file unreadable; using the defaults"]);
  });

  it("refuses a file that is too large", () => {
    const file = path.join(dir, "settings.json");
    writeFileSync(file, " ".repeat(MAX_SETTINGS_BYTES + 1));
    expect(readSettingsFile(file, defaults, log)).toEqual(defaults);
    expect(lines).toEqual([
      "warn: settings file not a regular file or too large; using the defaults",
    ]);
  });

  it("refuses a folder in the place of the file", () => {
    const file = path.join(dir, "settings.json");
    mkdirSync(file);
    // Windows refuses to open a folder (defaults without warning), POSIX warns.
    expect(readSettingsFile(file, defaults, log)).toEqual(defaults);
  });
});

describe("packs file", () => {
  it("reads back the saved choice", async () => {
    const file = path.join(dir, "sub", "packs-active.json");
    await writeActivePacksFile(file, ["base", "extra"]);
    expect(readActivePacksFile(file)).toEqual(["base", "extra"]);
  });

  it("treats a missing or unreadable file as no choice", () => {
    expect(readActivePacksFile(path.join(dir, "missing.json"))).toEqual([]);
    const file = path.join(dir, "broken.json");
    writeFileSync(file, "garbage");
    expect(readActivePacksFile(file)).toEqual([]);
  });
});
