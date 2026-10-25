import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { LanguageInfo } from "../generated/LanguageInfo.js";
import type { SavesRequest } from "./core-status.js";
import { createSettingsService } from "./settings-service.js";

const languages: LanguageInfo[] = [
  { code: "de", name: "Deutsch", keys: 2 },
  { code: "en", name: "English", keys: 2 },
];

let dir: string;
let lines: string[];
let intervalCalls: number;

beforeEach(() => {
  dir = mkdtempSync(path.join(os.tmpdir(), "oa-settings-service-"));
  lines = [];
  intervalCalls = 0;
});

afterEach(() => {
  rmSync(dir, { recursive: true, force: true });
});

function setup(
  answer: (request: SavesRequest) => Promise<unknown> = () => Promise.resolve(undefined),
  folder: () => string = () => path.join(dir, "settings.json"),
): { service: ReturnType<typeof createSettingsService>; requests: SavesRequest[] } {
  const requests: SavesRequest[] = [];
  const service = createSettingsService({
    file: folder,
    languages: () => languages,
    request: (make) => {
      const request = make(requests.length + 1);
      requests.push(request);
      return answer(request);
    },
    onAutosaveInterval: () => {
      intervalCalls += 1;
    },
    log: (level, message) => {
      lines.push(`${level}: ${message}`);
    },
  });
  return { service, requests };
}

describe("settings service: settings", () => {
  it("starts with the defaults of the system language", () => {
    const { service } = setup();
    service.load(["de-DE"]);
    expect(service.get()).toEqual({ language: "de", autosaveMinutes: 10, autosaveCount: 5 });
  });

  it("loads the file of an earlier session", () => {
    writeFileSync(
      path.join(dir, "settings.json"),
      JSON.stringify({ language: "en", autosaveMinutes: 3, autosaveCount: 2 }),
    );
    const { service } = setup();
    service.load(["de"]);
    expect(service.get()).toEqual({ language: "en", autosaveMinutes: 3, autosaveCount: 2 });
  });

  it("saves a change to the file, applies it and logs it", async () => {
    const { service } = setup();
    service.load([]);
    const next = await service.set({ language: "de", autosaveCount: 7 });
    expect(next).toEqual({ language: "de", autosaveMinutes: 10, autosaveCount: 7 });
    expect(service.get()).toEqual(next);
    expect(JSON.parse(readFileSync(path.join(dir, "settings.json"), "utf8"))).toEqual(next);
    expect(lines).toEqual([`info: settings changed: ${JSON.stringify(next)}`]);
    // The interval did not change.
    expect(intervalCalls).toBe(0);
  });

  it("tells the core only when the autosave interval changed", async () => {
    const { service } = setup();
    service.load([]);
    await service.set({ autosaveMinutes: 20 });
    await service.set({ autosaveMinutes: 20 });
    expect(intervalCalls).toBe(1);
  });

  it("refuses an invalid change and keeps the settings", async () => {
    const { service } = setup();
    service.load([]);
    const before = service.get();
    await expect(service.set({ autosaveMinutes: 0 })).rejects.toMatchObject({
      reason: { code: "invalidArgument" },
    });
    await expect(service.set({ language: "xx" })).rejects.toMatchObject({
      reason: { code: "invalidArgument" },
    });
    expect(service.get()).toEqual(before);
  });

  it("keeps the settings if the file cannot be written", async () => {
    const blocker = path.join(dir, "blocker");
    writeFileSync(blocker, "a file where a folder should be");
    const { service } = setup(undefined, () => path.join(blocker, "settings.json"));
    service.load([]);
    const before = service.get();
    await expect(service.set({ autosaveCount: 9 })).rejects.toMatchObject({
      reason: { code: "ioError" },
    });
    expect(service.get()).toEqual(before);
    expect(lines).toHaveLength(1);
    expect(lines[0]).toMatch(/^warn: cannot save the settings: /);
    expect(intervalCalls).toBe(0);
  });

  it("loses no change when calls overlap", async () => {
    const { service } = setup();
    service.load([]);
    const calls = [
      service.set({ autosaveMinutes: 11 }),
      service.set({ autosaveCount: 12 }),
      service.set({ language: "de" }),
    ];
    await Promise.all(calls);
    expect(service.get()).toEqual({ language: "de", autosaveMinutes: 11, autosaveCount: 12 });
  });

  it("goes on after a refused change", async () => {
    const { service } = setup();
    service.load([]);
    const refused = service.set({ nonsense: 1 });
    const accepted = service.set({ autosaveCount: 4 });
    await expect(refused).rejects.toBeDefined();
    await expect(accepted).resolves.toMatchObject({ autosaveCount: 4 });
  });
});

describe("settings service: lang.texts", () => {
  it("returns the texts of a listed language from the core", async () => {
    const { service, requests } = setup(() => Promise.resolve([["menu.start", "Start"]]));
    await expect(service.texts("de")).resolves.toEqual({ "menu.start": "Start" });
    expect(requests).toEqual([{ type: "lang:texts", request: 1, code: "de" }]);
  });

  it("refuses an unlisted language or a bad code", () => {
    const { service } = setup();
    expect(() => service.texts("fr")).toThrow();
    expect(() => service.texts(5)).toThrow();
  });

  it("shares the open request of the same language", async () => {
    let release: (value: unknown) => void = () => undefined;
    const { service, requests } = setup(
      () =>
        new Promise((resolve) => {
          release = resolve;
        }),
    );
    const first = service.texts("en");
    const second = service.texts("en");
    expect(second).toBe(first);
    release([["a", "b"]]);
    await expect(first).resolves.toEqual({ a: "b" });
    expect(requests).toHaveLength(1);
    // Asked again once the first is settled.
    void service.texts("en");
    expect(requests).toHaveLength(2);
  });

  it("refuses when the language vanished or the answer is malformed", async () => {
    const vanished = setup(() => Promise.resolve(null));
    await expect(vanished.service.texts("de")).rejects.toMatchObject({
      reason: { code: "invalidArgument" },
    });
    const malformed = setup(() => Promise.resolve([["a", 1]]));
    await expect(malformed.service.texts("de")).rejects.toMatchObject({
      reason: { code: "invalidAnswer" },
    });
  });
});
