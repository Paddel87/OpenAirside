import { describe, expect, it } from "vitest";
import { PerfMeter } from "./perf.js";

describe("PerfMeter", () => {
  it("is empty at the start", () => {
    expect(new PerfMeter().snapshot(0)).toEqual({ fps: 0, rttMs: 0, rttMaxMs: 0 });
  });

  it("counts the frames of the last second with median and maximum round trip", () => {
    const meter = new PerfMeter();
    meter.frame(0, 9);
    for (let i = 1; i <= 60; i += 1) {
      meter.frame(1000 + i * 10, i % 3);
    }
    const snap = meter.snapshot(1600);
    expect(snap.fps).toBe(60);
    expect(snap.rttMs).toBe(1);
    expect(snap.rttMaxMs).toBe(2);
    expect(meter.snapshot(5000).fps).toBe(0);
  });
});
