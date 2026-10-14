import { describe, expect, it } from "vitest";
import { main, readResult, renderReport } from "./bench-report.mjs";

const current = {
  persons: 10000,
  threads: 4,
  cpus: 4,
  os: "linux",
  ticks: 2000,
  meanMs: 0.2,
  p50Ms: 0.18,
  p99Ms: 0.5,
  maxMs: 1.2,
  limits: { meanMs: 1, p99Ms: 4 },
  passed: true,
};

describe("renderReport", () => {
  it("shows values, limits and the change against main", () => {
    const text = renderReport(current, { ...current, meanMs: 0.25, p99Ms: 0.4 });
    expect(text).toContain("### Kern-Benchmark: bestanden");
    expect(text).toContain("| Mittelwert je Schritt (ms) | 0.200 | 1 | 0.250 | -20.0 % |");
    expect(text).toContain("| p99 (ms) | 0.500 | 4 | 0.400 | +25.0 % |");
    expect(text).toContain("| Median (ms) | 0.180 | – | 0.180 | +0.0 % |");
    expect(text).not.toContain("Kein Vergleichswert");
  });

  it("works without a previous result and marks a failed run", () => {
    const text = renderReport({ ...current, passed: false, maxMs: "x" }, null);
    expect(text).toContain("Grenze überschritten");
    expect(text).toContain("| Höchstwert (ms) | – | – | – | – |");
    expect(text).toContain("Kein Vergleichswert von main vorhanden.");
  });

  it("does not divide by a zero previous value", () => {
    const text = renderReport(current, { ...current, meanMs: 0 });
    expect(text).toContain("| 0.200 | 1 | 0.000 | – |");
  });
});

describe("renderReport for the flow field benchmark", () => {
  const flowfield = {
    benchmark: "flowfield",
    targets: 64,
    persons: 10000,
    threads: 2,
    backgroundThreads: 2,
    cpus: 4,
    os: "linux",
    ticks: 2000,
    recomputeMs: 1646.3,
    fieldBytes: 100716544,
    meanMs: 0.29,
    p99Ms: 0.9,
    maxMs: 4.8,
    adoptionMaxMs: 0.3,
    limits: { recomputeMs: 4000, p99Ms: 4 },
    passed: true,
  };

  it("shows recomputation, memory and step times with their limits", () => {
    const text = renderReport(flowfield, { ...flowfield, recomputeMs: 1500 });
    expect(text).toContain("### Wegweiser-Benchmark: bestanden");
    expect(text).toContain("64 Ziele, 10000 Personen, 2 + 2 Threads (4 CPUs, linux)");
    expect(text).toContain("| Neuberechnung aller Wegweiser (ms) | 1646 | 4000 | 1500 | +9.8 % |");
    expect(text).toContain("| Speicher der Wegweiser (MB) | 100.7 | – | 100.7 | +0.0 % |");
    expect(text).toContain("| Schritt während Neuberechnung, p99 (ms) | 0.900 | 4 |");
  });

  it("falls back to the tick layout for unknown benchmarks", () => {
    expect(renderReport({ ...current, benchmark: "other" }, null)).toContain("### Kern-Benchmark");
  });
});

describe("readResult and main", () => {
  const files = { "now.json": JSON.stringify(current), "bad.json": "{" };
  const read = (path) => {
    if (!(path in files)) {
      throw new Error("ENOENT");
    }
    return files[path];
  };

  it("returns null for missing or malformed files", () => {
    expect(readResult("missing.json", read)).toBeNull();
    expect(readResult("bad.json", read)).toBeNull();
    expect(readResult("now.json", read)).toEqual(current);
  });

  it("writes the table and tolerates a missing comparison", () => {
    let out = "";
    expect(main(["now.json", "missing.json"], read, (t) => (out += t))).toBe(0);
    expect(out).toContain("Kein Vergleichswert");
  });

  it("fails without a readable current result", () => {
    let out = "";
    expect(main([], read, (t) => (out += t))).toBe(1);
    expect(main(["bad.json"], read, (t) => (out += t))).toBe(1);
    expect(out).toContain("kein lesbares Ergebnis");
  });
});
