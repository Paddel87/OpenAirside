import { describe, expect, it } from "vitest";
import { parseCoreArgs, toPortMessage } from "./core-port.js";

describe("parseCoreArgs", () => {
  it("reads valid flags", () => {
    expect(
      parseCoreArgs([
        "--scenario=referenceTerminal",
        "--persons=10000",
        "--threads=2",
        "--background-threads=3",
        "--map-size=1536",
      ]),
    ).toEqual({
      scenario: "referenceTerminal",
      persons: 10_000,
      threads: 2,
      backgroundThreads: 3,
      mapSize: 1536,
    });
  });

  it("reads the pack folders and the active ids", () => {
    expect(
      parseCoreArgs([
        "--base-pack=/opt/OpenAirside/content/base",
        "--user-packs=C:\\Users\\Me Too\\packs",
        "--active-packs=my-mod,other_pack",
      ]),
    ).toEqual({
      basePack: "/opt/OpenAirside/content/base",
      userPacks: "C:\\Users\\Me Too\\packs",
      activePacks: ["my-mod", "other_pack"],
    });
    expect(parseCoreArgs(["--active-packs="])).toEqual({ activePacks: [] });
  });

  it("lets later flags win", () => {
    expect(parseCoreArgs(["--base-pack=/a", "--base-pack=/b"])).toEqual({ basePack: "/b" });
  });

  it.each([
    ["--scenario=castle"],
    ["--base-pack="],
    [`--user-packs=${"x".repeat(4097)}`],
    ["--user-packs=a\0b"],
    ["--active-packs=Bad"],
    ["--active-packs=ok-id,,x"],
    [
      `--active-packs=${Array.from({ length: 257 }, (_, i) => `p${String(i).padStart(3, "0")}`).join(",")}`,
    ],
    ["--persons=-1"],
    ["--persons=200001"],
    ["--threads=0"],
    ["--threads=1e3"],
    ["--map-size=1800"],
    ["--other=1"],
    ["."],
  ])("ignores %s", (arg) => {
    expect(parseCoreArgs([arg])).toEqual({});
  });
});

describe("toPortMessage", () => {
  it("passes messages without buffer unchanged", () => {
    const message = { type: "welcome", session: 1 };
    expect(toPortMessage({ message })).toBe(message);
    expect(toPortMessage({ message, buf: null })).toBe(message);
  });

  it("copies exactly the frame bytes into an own ArrayBuffer", () => {
    const backing = new Uint8Array([9, 1, 2, 3, 9]);
    const view = backing.subarray(1, 4);
    const out = toPortMessage({ message: { type: "frame" }, buf: view });
    expect(out.type).toBe("frame");
    const buf = out.buf as ArrayBuffer;
    expect(buf).toBeInstanceOf(ArrayBuffer);
    expect([...new Uint8Array(buf)]).toEqual([1, 2, 3]);
  });
});
