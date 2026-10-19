import { describe, expect, it } from "vitest";
import {
  applySettings,
  AUTOSAVE_COUNT,
  AUTOSAVE_MINUTES,
  defaultSettings,
  isLanguageCode,
  parseLanguageList,
  parseTexts,
  settingsFromFile,
  systemLanguage,
} from "./settings.js";

const languages = [
  { code: "de", name: "Deutsch", keys: 3 },
  { code: "en", name: "English", keys: 3 },
  { code: "pt-br", name: "Português", keys: 1 },
];

describe("settings", () => {
  it("takes the system language if the base pack has it", () => {
    expect(systemLanguage(["de-AT", "en-US"])).toBe("de");
    expect(systemLanguage(["DE_ch"])).toBe("de");
    expect(systemLanguage(["fr-FR", "en-GB"])).toBe("en");
    expect(systemLanguage(["fr-FR"])).toBe("en");
    expect(systemLanguage([])).toBe("en");
    expect(defaultSettings(["de"])).toEqual({
      language: "de",
      autosaveMinutes: AUTOSAVE_MINUTES.default,
      autosaveCount: AUTOSAVE_COUNT.default,
    });
  });

  it("checks language codes like the pack format", () => {
    expect(isLanguageCode("pt-br")).toBe(true);
    expect(isLanguageCode("DE")).toBe(false);
    expect(isLanguageCode("de-")).toBe(false);
    expect(isLanguageCode(7)).toBe(false);
  });

  it("reads the file field by field and falls back to defaults", () => {
    const defaults = defaultSettings(["en"]);
    expect(
      settingsFromFile({ language: "fr", autosaveMinutes: 30, autosaveCount: 20 }, defaults),
    ).toEqual({
      settings: { language: "fr", autosaveMinutes: 30, autosaveCount: 20 },
      valid: true,
    });
    expect(
      settingsFromFile({ language: "../x", autosaveMinutes: 61, autosaveCount: 0.5 }, defaults),
    ).toEqual({ settings: defaults, valid: false });
    expect(settingsFromFile({ ...defaults, extra: 1 }, defaults).valid).toBe(false);
    expect(settingsFromFile([], defaults)).toEqual({ settings: defaults, valid: false });
    expect(settingsFromFile(null, defaults).valid).toBe(false);
  });

  it("applies valid partial updates", () => {
    const current = defaultSettings(["en"]);
    expect(applySettings(current, { language: "pt-br" }, languages)).toEqual({
      ...current,
      language: "pt-br",
    });
    expect(applySettings(current, { autosaveMinutes: 1, autosaveCount: 20 }, languages)).toEqual({
      ...current,
      autosaveMinutes: 1,
      autosaveCount: 20,
    });
    expect(applySettings(current, {}, languages)).toEqual(current);
  });

  it("refuses invalid updates as a whole", () => {
    const current = defaultSettings(["en"]);
    for (const patch of [
      null,
      [],
      "de",
      { language: "fr" },
      { language: "DE" },
      { autosaveMinutes: 0 },
      { autosaveMinutes: 61 },
      { autosaveMinutes: 1.5 },
      { autosaveCount: 21 },
      { autosaveCount: "5" },
      { language: "de", theme: "dark" },
      { language: "de", autosaveCount: 0 },
    ]) {
      expect(applySettings(current, patch, languages), JSON.stringify(patch)).toBeUndefined();
    }
  });

  it("checks the language list of the core", () => {
    expect(parseLanguageList(languages)).toEqual(languages);
    expect(parseLanguageList([])).toEqual([]);
    expect(parseLanguageList({})).toBeUndefined();
    expect(parseLanguageList([{ code: "DE", name: "x", keys: 1 }])).toBeUndefined();
    expect(parseLanguageList([{ code: "de", name: 1, keys: 1 }])).toBeUndefined();
    expect(parseLanguageList([{ code: "de", name: "x", keys: -1 }])).toBeUndefined();
  });

  it("checks texts from the core", () => {
    const texts = parseTexts([
      ["save.error.ioError", "Fehler"],
      ["ui.title", "{n} Spiele"],
      ["__proto__", "p"],
      ["constructor", "c"],
    ]);
    expect(texts?.["ui.title"]).toBe("{n} Spiele");
    expect(Object.getPrototypeOf(texts)).toBeNull();
    // Finding 6 of the separate review: plain keys, no prototype change.
    expect(Object.keys(texts ?? {})).toContain("__proto__");
    expect(texts?.constructor).toBe("c");
    expect(parseTexts([["bad key", "x"]])).toBeUndefined();
    expect(parseTexts([["a", 1]])).toBeUndefined();
    expect(parseTexts([["a"]])).toBeUndefined();
    expect(parseTexts(["a"])).toBeUndefined();
    expect(parseTexts({ a: "b" })).toBeUndefined();
    expect(parseTexts(null)).toBeUndefined();
  });
});
