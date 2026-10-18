import { describe, expect, it } from "vitest";
import {
  exitedCoreStatus,
  nextCoreStatus,
  parseCoreHostMessage,
  parsePackRequest,
  restartDecision,
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

describe("parseCoreHostMessage", () => {
  it("accepts a ready message with core info and packs", () => {
    expect(parseCoreHostMessage({ type: "ready", info, packs })).toEqual({
      type: "ready",
      info,
      packs,
    });
  });

  it("drops unknown extra fields from core info", () => {
    expect(parseCoreHostMessage({ type: "ready", info: { ...info, extra: 1 }, packs })).toEqual({
      type: "ready",
      info,
      packs,
    });
  });

  it("accepts pack reports and pack errors", () => {
    expect(parseCoreHostMessage({ type: "packs", packs })).toEqual({ type: "packs", packs });
    expect(parseCoreHostMessage({ type: "packsError", message: "invalidId" })).toEqual({
      type: "packsError",
      message: "invalidId",
    });
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
    { type: "packs", packs: [{ folder: "x" }] },
    { type: "error", message: 3 },
    { type: "other" },
  ])("rejects malformed message %j", (value) => {
    expect(parseCoreHostMessage(value)).toBeUndefined();
  });
});

describe("nextCoreStatus", () => {
  it("becomes ready on a ready message", () => {
    expect(nextCoreStatus({ type: "ready", info, packs })).toEqual({ state: "ready", info });
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
    expect(nextCoreStatus({ type: "packs", packs }).state).toBe("failed");
  });
});

describe("parsePackRequest", () => {
  it("accepts a request with valid ids", () => {
    expect(parsePackRequest({ type: "packs:setActive", ids: ["my-mod"] })).toEqual({
      ids: ["my-mod"],
    });
  });

  it.each([
    { type: "port" },
    { type: "packs:setActive" },
    { type: "packs:setActive", ids: ["X"] },
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
