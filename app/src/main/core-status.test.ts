import { describe, expect, it } from "vitest";
import { exitedCoreStatus, nextCoreStatus, parseCoreHostMessage } from "./core-status.js";

const info = { name: "openairside-core", version: "0.0.0" };

describe("parseCoreHostMessage", () => {
  it("accepts a ready message with core info", () => {
    expect(parseCoreHostMessage({ type: "ready", info })).toEqual({ type: "ready", info });
  });

  it("drops unknown extra fields from core info", () => {
    expect(parseCoreHostMessage({ type: "ready", info: { ...info, extra: 1 } })).toEqual({
      type: "ready",
      info,
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
    { type: "ready", info: { name: "", version: "0.0.0" } },
    { type: "ready", info: { name: "core", version: 1 } },
    { type: "error", message: 3 },
    { type: "other" },
  ])("rejects malformed message %j", (value) => {
    expect(parseCoreHostMessage(value)).toBeUndefined();
  });
});

describe("nextCoreStatus", () => {
  it("becomes ready on a ready message", () => {
    expect(nextCoreStatus({ type: "ready", info })).toEqual({ state: "ready", info });
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
