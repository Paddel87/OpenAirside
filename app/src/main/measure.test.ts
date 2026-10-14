import { describe, expect, it } from "vitest";
import {
  MAX_MEMORY_SAMPLES,
  MAX_REPORT_BYTES,
  MeasureRun,
  memorySample,
  reportFileName,
} from "./measure.js";

const metrics = [
  { type: "Browser", memory: { workingSetSize: 200 * 1024 } },
  { type: "Tab", memory: { workingSetSize: 300 * 1024 } },
  { type: "GPU", memory: { workingSetSize: 100 * 1024 } },
  { type: "Utility", serviceName: "OpenAirside Core", memory: { workingSetSize: 400 * 1024 } },
  { type: "Utility", name: "Network Service", memory: { workingSetSize: 10 * 1024 } },
];

describe("memorySample", () => {
  it("sums the working sets per type and names the core process", () => {
    expect(memorySample(metrics, "OpenAirside Core", 5.4)).toEqual({
      s: 5,
      totalMb: 1010,
      byTypeMb: { Browser: 200, Tab: 300, GPU: 100, core: 400, Utility: 10 },
    });
  });
});

describe("reportFileName", () => {
  it("has no characters that are invalid in file names", () => {
    const name = reportFileName("win32", new Date("2026-10-14T07:05:00.000Z"));
    expect(name).toBe("openairside-measure-win32-2026-10-14T07-05-00-000Z.json");
    expect(reportFileName("../x/", new Date(0))).toMatch(/^openairside-measure-x-/);
  });
});

describe("MeasureRun", () => {
  it("accepts one report per started run with memory and system data", () => {
    const run = new MeasureRun();
    expect(run.finish({}, {})).toEqual({ ok: false, error: "notStarted" });
    run.sample(metrics, "OpenAirside Core", 0);
    expect(run.samples).toHaveLength(0);
    expect(run.start(1000)).toBe(true);
    expect(run.running).toBe(true);
    expect(run.start(2000)).toBe(false);
    run.sample(metrics, "OpenAirside Core", 6000);
    const done = run.finish({ series: { windows: [] } }, { platform: "linux" });
    expect(done.ok).toBe(true);
    if (!done.ok) return;
    const file = JSON.parse(done.json) as Record<string, unknown>;
    expect(file).toEqual({
      tool: "openairside-measure",
      system: { platform: "linux" },
      series: { windows: [] },
      memory: [memorySample(metrics, "OpenAirside Core", 5)],
    });
    expect(run.running).toBe(false);
    expect(run.finish({}, {})).toEqual({ ok: false, error: "notStarted" });
  });

  it("refuses results that are not plain objects or too large", () => {
    const run = new MeasureRun();
    run.start(0);
    for (const bad of [null, [], "text", 5, new Date(), new Map()]) {
      expect(run.finish(bad, {})).toEqual({ ok: false, error: "notAnObject" });
    }
    const cyclic: Record<string, unknown> = {};
    cyclic.self = cyclic;
    expect(run.finish(cyclic, {})).toEqual({ ok: false, error: "notAnObject" });
    expect(run.finish({ big: "x".repeat(MAX_REPORT_BYTES) }, {})).toEqual({
      ok: false,
      error: "tooLarge",
    });
    expect(run.running).toBe(true);
    // The renderer cannot override the main process's fields.
    const done = run.finish({ tool: "fake", system: "fake", memory: [1] }, { cpus: 8 });
    expect(done.ok && JSON.parse(done.json)).toEqual({
      tool: "openairside-measure",
      system: { cpus: 8 },
      memory: [],
    });
  });

  it("keeps a bounded number of memory samples", () => {
    const run = new MeasureRun();
    run.start(0);
    for (let i = 0; i < MAX_MEMORY_SAMPLES + 10; i += 1) run.sample([], "x", i * 5000);
    expect(run.samples).toHaveLength(MAX_MEMORY_SAMPLES);
  });
});
