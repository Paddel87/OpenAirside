import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { PLATFORM_CODES, PLATFORM_TEXT_PREFIX } from "../src/main/platform-error.js";
import { BUNDLED, format, loadTexts, Localizer, textMap, textsOfFile } from "./i18n.js";

/** Names of the parameters `{name}` in a text. */
function placeholders(text: string): string[] {
  return [...text.matchAll(/\{([A-Za-z0-9_]+)\}/g)].map((m) => m[1] ?? "").sort();
}

describe("format", () => {
  it("inserts text and numbers formatted for the language", () => {
    expect(format("{n} Spiele, {name}", { n: 1234.5, name: "Hub" }, "de")).toBe(
      "1.234,5 Spiele, Hub",
    );
    expect(format("{n} games", { n: 1234.5 }, "en")).toBe("1,234.5 games");
  });

  it("keeps unknown placeholders and never interprets markup", () => {
    expect(format("{a} {missing}", { a: "<b>x</b>" }, "en")).toBe("<b>x</b> {missing}");
    expect(format("{toString}", {}, "en")).toBe("{toString}");
  });

  it("falls back to English number formats for an unknown language tag", () => {
    expect(format("{n}", { n: 1000 }, "x-invalid-tag-!")).toBe("1,000");
  });
});

describe("Localizer", () => {
  const de = new Map([
    ["a", "A-de {n}"],
    ["only.de", "nur deutsch"],
  ]);
  const en = new Map([
    ["a", "A-en {n}"],
    ["b", "B-en"],
  ]);

  it("looks up the chosen language, then English, then the key", () => {
    const missing: string[] = [];
    const i18n = new Localizer("de", de, en, (key) => missing.push(key));
    expect(i18n.t("a", { n: 2 })).toBe("A-de 2");
    expect(i18n.t("b")).toBe("B-en");
    expect(i18n.t("c")).toBe("c");
    expect(i18n.t("c")).toBe("c");
    expect(missing).toEqual(["c"]);
    expect(i18n.reason({ code: "x", textKey: "a", params: { n: 3 } })).toBe("A-de 3");
  });

  it("switches the language without restart and tells the views", () => {
    const i18n = new Localizer("en", en, en);
    let renders = 0;
    i18n.onChange(() => {
      renders += 1;
    });
    i18n.use("de", de);
    expect(i18n.code).toBe("de");
    expect(i18n.t("only.de")).toBe("nur deutsch");
    expect(renders).toBe(1);
  });

  it("starts with the bundled texts of the base pack", () => {
    const i18n = new Localizer("de");
    expect(i18n.t("ui.saves.load")).toBe("Laden");
    expect(new Localizer("xx").t("ui.saves.load")).toBe("Load");
  });
});

describe("bundled texts of the base pack (FR-006)", () => {
  const de = BUNDLED.get("de") ?? new Map<string, string>();
  const en = BUNDLED.get("en") ?? new Map<string, string>();

  it("has the same keys and parameters in German and English", () => {
    expect(de.size).toBeGreaterThan(90);
    expect([...de.keys()].sort()).toEqual([...en.keys()].sort());
    for (const [key, text] of de) {
      expect(placeholders(text), key).toEqual(placeholders(en.get(key) ?? ""));
    }
  });

  it("covers every key of the interface and every error of the main process", () => {
    const dir = import.meta.dirname;
    const keys = new Set<string>();
    for (const file of readdirSync(dir).filter(
      (f) => f.endsWith(".ts") && !f.endsWith(".test.ts"),
    )) {
      const text = readFileSync(path.join(dir, file), "utf8");
      for (const match of text.matchAll(/"(ui\.[A-Za-z0-9_.-]+)"/g)) keys.add(match[1] ?? "");
    }
    expect(keys.size).toBeGreaterThan(20);
    for (const code of PLATFORM_CODES) keys.add(`${PLATFORM_TEXT_PREFIX}.${code}`);
    for (const [language, texts] of [
      ["de", de],
      ["en", en],
    ] as const) {
      expect(
        [...keys].filter((key) => !texts.has(key)),
        language,
      ).toEqual([]);
    }
  });

  it("reads only the language asked for from a file", () => {
    const file = {
      languages: [
        { id: "fr", texts: { a: "x" } },
        { id: "de", texts: { a: "y" } },
      ],
    };
    expect([...textsOfFile(file, "de")]).toEqual([["a", "y"]]);
  });
});

describe("loadTexts", () => {
  it("takes the texts of the packs from the platform", async () => {
    const asked: string[] = [];
    const source = {
      texts: (code: string): Promise<Record<string, string>> => {
        asked.push(code);
        return Promise.resolve({ k: `text-${code}` });
      },
    };
    const { primary, fallback } = await loadTexts(source, "pt-br");
    expect(primary.get("k")).toBe("text-pt-br");
    expect(fallback.get("k")).toBe("text-en");
    expect(asked).toEqual(["en", "pt-br"]);
  });

  it("asks once for English", async () => {
    let calls = 0;
    const source = {
      texts: (): Promise<Record<string, string>> => {
        calls += 1;
        return Promise.resolve({ k: "v" });
      },
    };
    const { primary, fallback } = await loadTexts(source, "en");
    expect(primary).toBe(fallback);
    expect(calls).toBe(1);
  });

  it("falls back to the bundled texts if the core does not answer", async () => {
    const source = {
      texts: (): Promise<Record<string, string>> => Promise.reject(new Error("notReady")),
    };
    const { primary, fallback } = await loadTexts(source, "de");
    expect(primary.get("ui.saves.load")).toBe("Laden");
    expect(fallback.get("ui.saves.load")).toBe("Load");
    const unknown = await loadTexts(undefined, "fr");
    expect(unknown.primary.size).toBe(0);
    expect(textMap({ a: "b" }).get("a")).toBe("b");
  });
});
