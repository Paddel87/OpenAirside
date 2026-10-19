import { describe, expect, it } from "vitest";
import {
  answer,
  fail,
  parseReason,
  PlatformFailure,
  platformReason,
  reasonOfBridgeError,
} from "./platform-error.js";

describe("error form of S-6", () => {
  it("gives errors of the main process the key platform.error.<code>", () => {
    expect(platformReason("busy")).toEqual({
      code: "busy",
      textKey: "platform.error.busy",
      params: {},
    });
    expect(() => fail("unknownSave", { id: "quick" })).toThrow(PlatformFailure);
  });

  it("wraps values, refusals and unexpected errors in the envelope", async () => {
    expect(
      await answer(
        () => 5,
        () => undefined,
      ),
    ).toEqual({ ok: true, value: 5 });
    expect(
      await answer(
        () => fail("busy"),
        () => undefined,
      ),
    ).toEqual({
      ok: false,
      error: platformReason("busy"),
    });
    const seen: unknown[] = [];
    const internal = await answer(
      async () => {
        await Promise.resolve();
        throw new Error("ENOENT /home/someone/secret");
      },
      (error) => seen.push(error),
    );
    // The message (it may name paths) stays out of the answer.
    expect(internal).toEqual({ ok: false, error: platformReason("internal") });
    expect(seen).toHaveLength(1);
  });

  it("reads the reason of a bridge error from its JSON message", () => {
    const reason = { code: "checksumMismatch", textKey: "save.error.checksumMismatch", params: {} };
    expect(reasonOfBridgeError(new Error(JSON.stringify(reason)))).toEqual(reason);
    expect(reasonOfBridgeError(new Error("core stopped"))).toEqual(platformReason("internal"));
    expect(reasonOfBridgeError("{}")).toEqual(platformReason("internal"));
  });

  it("checks reasons that crossed a process boundary", () => {
    expect(parseReason({ code: "a", textKey: "b", params: { n: 1, t: "x" } })).toEqual({
      code: "a",
      textKey: "b",
      params: { n: 1, t: "x" },
    });
    expect(parseReason({ code: "a", textKey: "b", params: { n: {} } })).toBeUndefined();
    expect(parseReason({ code: "a", textKey: "b", params: { n: Infinity } })).toBeUndefined();
    expect(parseReason({ code: "a", params: {} })).toBeUndefined();
    expect(parseReason([])).toBeUndefined();
  });
});
