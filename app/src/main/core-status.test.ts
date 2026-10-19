import { describe, expect, it } from "vitest";
import {
  exitedCoreStatus,
  isPackAnswer,
  isSavesAnswer,
  MAX_INFO_FILES,
  nextCoreStatus,
  parseAutosaveInterval,
  parseCoreHostMessage,
  parsePackRequest,
  parseSavesRequest,
  restartDecision,
  savesRequestNumber,
} from "./core-status.js";

const info = { name: "openairside-core", version: "0.0.0" };
const packs = [
  {
    folder: "base",
    source: "base",
    id: "base",
    name: "OpenAirside",
    version: "0.0.0",
    state: "active",
    entries: 3,
    errors: [],
    errorsOmitted: 0,
  },
];
const languages = [
  { code: "de", name: "Deutsch", keys: 54 },
  { code: "en", name: "English", keys: 54 },
];

describe("parseCoreHostMessage", () => {
  it("accepts a ready message with core info and packs", () => {
    expect(parseCoreHostMessage({ type: "ready", info, packs, languages })).toEqual({
      type: "ready",
      info,
      packs,
      languages,
    });
  });

  it("drops unknown extra fields from core info", () => {
    expect(
      parseCoreHostMessage({ type: "ready", info: { ...info, extra: 1 }, packs, languages }),
    ).toEqual({
      type: "ready",
      info,
      packs,
      languages,
    });
  });

  it("accepts pack reports and pack errors with their request number", () => {
    expect(parseCoreHostMessage({ type: "packs", request: 3, packs, languages })).toEqual({
      type: "packs",
      request: 3,
      packs,
      languages,
    });
    expect(parseCoreHostMessage({ type: "packsError", request: 4, message: "invalidId" })).toEqual({
      type: "packsError",
      request: 4,
      message: "invalidId",
    });
  });

  it("recognises pack answers also when they are malformed", () => {
    expect(isPackAnswer({ type: "packs", packs: "x" })).toBe(true);
    expect(isPackAnswer({ type: "packsError" })).toBe(true);
    expect(isPackAnswer({ type: "ready" })).toBe(false);
    expect(isPackAnswer(null)).toBe(false);
  });

  it("accepts an error message", () => {
    expect(parseCoreHostMessage({ type: "error", message: "boom" })).toEqual({
      type: "error",
      message: "boom",
    });
  });

  it.each([
    null,
    undefined,
    42,
    "ready",
    {},
    { type: "ready" },
    { type: "ready", info },
    { type: "ready", info: { name: "", version: "0.0.0" }, packs },
    { type: "ready", info: { name: "core", version: 1 }, packs },
    { type: "packs", request: 1, packs: [{ folder: "x" }] },
    { type: "ready", info, packs },
    { type: "ready", info, packs, languages: [{ code: "DE", name: "x", keys: 1 }] },
    { type: "packs", request: 1, packs, languages: "de" },
    { type: "packs", packs },
    { type: "packs", request: -1, packs },
    { type: "packsError", request: 1.5, message: "x" },
    { type: "packsError", request: 1 },
    { type: "error", message: 3 },
    { type: "other" },
  ])("rejects malformed message %j", (value) => {
    expect(parseCoreHostMessage(value)).toBeUndefined();
  });
});

describe("nextCoreStatus", () => {
  it("becomes ready on a ready message", () => {
    expect(nextCoreStatus({ type: "ready", info, packs, languages })).toEqual({
      state: "ready",
      info,
    });
  });

  it("fails with the reported reason on an error message", () => {
    expect(nextCoreStatus({ type: "error", message: "module not found" })).toEqual({
      state: "failed",
      reason: "module not found",
    });
  });

  it("fails on an unexpected message", () => {
    expect(nextCoreStatus({ type: "bogus" })).toEqual({
      state: "failed",
      reason: "unexpected message from core host",
    });
  });

  it("does not take pack answers as a status", () => {
    expect(nextCoreStatus({ type: "packs", request: 1, packs }).state).toBe("failed");
  });
});

describe("parsePackRequest", () => {
  it("accepts a request with valid ids", () => {
    expect(parsePackRequest({ type: "packs:setActive", request: 2, ids: ["my-mod"] })).toEqual({
      request: 2,
      ids: ["my-mod"],
    });
  });

  it.each([
    { type: "port" },
    { type: "packs:setActive" },
    { type: "packs:setActive", ids: ["my-mod"] },
    { type: "packs:setActive", request: 1, ids: ["X"] },
    null,
  ])("ignores %j", (value) => {
    expect(parsePackRequest(value)).toBeUndefined();
  });
});

describe("exitedCoreStatus", () => {
  it("turns a running core into a failed one", () => {
    expect(exitedCoreStatus({ state: "ready", info }, 3)).toEqual({
      state: "failed",
      reason: "core process exited with code 3",
    });
  });

  it("keeps an earlier failure reason", () => {
    const failed = { state: "failed", reason: "module not found" } as const;
    expect(exitedCoreStatus(failed, 1)).toBe(failed);
  });
});

describe("restartDecision", () => {
  it("allows three restarts within 60 seconds", () => {
    let restarts: readonly number[] = [];
    for (const now of [0, 1000, 2000]) {
      const decision = restartDecision(restarts, now);
      expect(decision.allowed).toBe(true);
      restarts = decision.restarts;
    }
    expect(restartDecision(restarts, 3000).allowed).toBe(false);
  });

  it("forgets restarts older than 60 seconds", () => {
    const decision = restartDecision([0, 1000, 2000], 62_001);
    expect(decision).toEqual({ allowed: true, restarts: [62_001] });
  });
});

describe("save game messages (step 2.5)", () => {
  const reason = { code: "checksumMismatch", textKey: "save.error.checksumMismatch", params: {} };
  const bytes = new Uint8Array([1, 2, 3]);

  it("reads answers and the autosave signal of the core host", () => {
    expect(parseCoreHostMessage({ type: "savesAnswer", request: 4, ok: true, value: 7 })).toEqual({
      type: "savesAnswer",
      request: 4,
      ok: true,
      value: 7,
    });
    expect(
      parseCoreHostMessage({ type: "savesAnswer", request: 4, ok: false, error: reason }),
    ).toEqual({ type: "savesAnswer", request: 4, ok: false, error: reason });
    expect(parseCoreHostMessage({ type: "autosave" })).toEqual({ type: "autosave" });
    expect(
      parseCoreHostMessage({ type: "savesAnswer", request: 4, ok: false, error: "x" }),
    ).toBeUndefined();
    expect(parseCoreHostMessage({ type: "savesAnswer", request: -1, ok: true })).toBeUndefined();
    expect(isSavesAnswer({ type: "savesAnswer" })).toBe(true);
    expect(isSavesAnswer({ type: "packs" })).toBe(false);
  });

  it("never takes a save game message for a status", () => {
    expect(nextCoreStatus({ type: "autosave" })).toEqual({
      state: "failed",
      reason: "unexpected message from core host",
    });
  });

  it("checks the requests of the main process in the core host", () => {
    expect(
      parseSavesRequest({
        type: "saves:save",
        request: 1,
        name: "a",
        kind: "quick",
        savedAt: "2026-10-19T12:00:00Z",
        preview: bytes,
      }),
    ).toMatchObject({ type: "saves:save", kind: "quick" });
    expect(
      parseSavesRequest({
        type: "saves:save",
        request: 1,
        name: "a",
        kind: "x",
        savedAt: "",
        preview: bytes,
      }),
    ).toBeUndefined();
    expect(parseSavesRequest({ type: "saves:load", request: 2, bytes })).toEqual({
      type: "saves:load",
      request: 2,
      bytes,
    });
    expect(parseSavesRequest({ type: "saves:load", request: 2, bytes: [1] })).toBeUndefined();
    expect(parseSavesRequest({ type: "saves:checkPreview", request: 3, png: bytes })).toEqual({
      type: "saves:checkPreview",
      request: 3,
      png: bytes,
    });
    expect(
      parseSavesRequest({ type: "saves:info", request: 5, files: [{ prefix: bytes, size: 3 }] }),
    ).toEqual({ type: "saves:info", request: 5, files: [{ prefix: bytes, size: 3 }] });
    expect(
      parseSavesRequest({ type: "saves:info", request: 5, files: [{ prefix: bytes, size: -3 }] }),
    ).toBeUndefined();
    const many = Array.from({ length: MAX_INFO_FILES + 1 }, () => ({ prefix: bytes, size: 3 }));
    expect(parseSavesRequest({ type: "saves:info", request: 5, files: many })).toBeUndefined();
    expect(parseSavesRequest({ type: "saves:delete", request: 6 })).toBeUndefined();
    expect(parseSavesRequest({ type: "saves:load", bytes })).toBeUndefined();
  });
});

describe("invalid save game requests", () => {
  it("still name their request number, so that they can be answered", () => {
    expect(savesRequestNumber({ type: "saves:info", request: 9, files: "x" })).toBe(9);
    expect(savesRequestNumber({ type: "saves:info", files: [] })).toBeUndefined();
    expect(savesRequestNumber({ type: "packs:setActive", request: 9 })).toBeUndefined();
    expect(savesRequestNumber("saves:info")).toBeUndefined();
  });
});

describe("language messages (step 2.6)", () => {
  it("parses a request for the texts of a language", () => {
    expect(parseSavesRequest({ type: "lang:texts", request: 7, code: "pt-br" })).toEqual({
      type: "lang:texts",
      request: 7,
      code: "pt-br",
    });
    expect(parseSavesRequest({ type: "lang:texts", request: 7, code: "../x" })).toBeUndefined();
    expect(savesRequestNumber({ type: "lang:texts", request: 8 })).toBe(8);
    expect(savesRequestNumber({ type: "other:texts", request: 8 })).toBeUndefined();
  });

  it("parses the autosave interval within its bounds", () => {
    expect(parseAutosaveInterval({ type: "autosave:interval", minutes: 60 })).toEqual({
      type: "autosave:interval",
      minutes: 60,
    });
    for (const minutes of [0, 61, 1.5, "10"]) {
      expect(parseAutosaveInterval({ type: "autosave:interval", minutes })).toBeUndefined();
    }
    expect(parseAutosaveInterval({ type: "port" })).toBeUndefined();
  });
});
