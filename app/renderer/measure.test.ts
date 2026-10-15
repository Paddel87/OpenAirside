import { describe, expect, it } from "vitest";
import type { Command } from "../src/generated/Command.js";
import type { TickStats } from "../src/generated/TickStats.js";
import {
  buildArea,
  defaultConfig,
  type FrameInfo,
  type MeasureConfig,
  MeasureSeries,
  quantile,
  REFERENCE_EDGE,
  type SeriesHost,
  summarize,
  type View,
  WindowRecorder,
} from "./measure.js";

function stats(overrides: Partial<TickStats> = {}): TickStats {
  return {
    ticksPerSecond: 180,
    stepMeanUs: 400,
    stepP99Us: 900,
    stepMaxUs: 1500,
    behindTicks: 0,
    recomputing: false,
    threads: 2,
    backgroundThreads: 5,
    ...overrides,
  };
}

describe("quantile", () => {
  it("takes the nearest rank and handles empty input", () => {
    expect(quantile([], 0.5)).toBe(0);
    expect(quantile([7], 0.99)).toBe(7);
    const values = Array.from({ length: 200 }, (_, i) => i + 1);
    expect(quantile(values, 0.5)).toBe(100);
    expect(quantile(values, 0.99)).toBe(198);
    expect(quantile(values, 1)).toBe(200);
    expect(quantile(values, 0)).toBe(1);
  });
});

describe("WindowRecorder", () => {
  it("summarizes frames, tick figures, builds and queries", () => {
    const rec = new WindowRecorder();
    for (let i = 0; i <= 60; i += 1) rec.frame(i * 16, 2 + (i % 3));
    rec.longTask(70);
    rec.stats(stats({ ticksPerSecond: 170, behindTicks: 4 }));
    rec.stats(stats({ stepP99Us: 1200, lastRecompute: { revision: 2, millis: 1100 } }));
    rec.stats(stats({ lastRecompute: { revision: 2, millis: 1100 } }));
    rec.lag(5);
    rec.build(40, 3);
    rec.build(20, 1);
    rec.adopt(1300);
    rec.query(3);
    rec.query(5);
    const known = new Set<number>([1]);
    const w = rec.summarize({ window: 0, atS: 30, view: "overview", seconds: 1 }, known);
    expect(w.fps).toBe(61);
    expect(w.frameMsP50).toBe(16);
    expect(w.frameMsMax).toBe(16);
    expect(w.rttMsMax).toBe(4);
    expect(w.longTasks).toBe(1);
    expect(w.ticksPerSecMin).toBe(170);
    expect(w.ticksPerSecMean).toBe(176.7);
    expect(w.stepUsP99Max).toBe(1200);
    expect(w.behindTicksMax).toBe(4);
    expect([w.lagEvents, w.lagBehindMax]).toEqual([1, 5]);
    expect([w.recomputes, w.recomputeMsMax]).toEqual([1, 1100]);
    expect(known.has(2)).toBe(true);
    expect([w.builds, w.buildMsP50, w.buildMsMax, w.buildFramesMax]).toEqual([2, 20, 40, 3]);
    expect(w.adoptMsMax).toBe(1300);
    expect([w.queries, w.queryMsMax]).toEqual([2, 5]);

    // A revision counted before is not counted again.
    const again = new WindowRecorder();
    again.stats(stats({ lastRecompute: { revision: 2, millis: 1100 } }));
    const w2 = again.summarize({ window: 1, atS: 60, view: "circle", seconds: 30 }, known);
    expect(w2.recomputes).toBe(0);
    expect(w2.fps).toBe(0);

    const s = summarize([w, w2]);
    expect(s.fpsMin).toBe(0);
    expect(s.recomputes).toBe(1);
    expect(s.stepUsP99Max).toBe(1200);
    expect(summarize([]).ticksPerSecMin).toBe(0);
  });
});

describe("buildArea", () => {
  it("stays in the landside hall of the centred terminal", () => {
    let seed = 12_345;
    const origin = (2048 - REFERENCE_EDGE) / 2;
    for (let i = 0; i < 500; i += 1) {
      const next = buildArea(2048, 2048, seed);
      seed = next.seed;
      const { area } = next;
      expect(area.floor).toBe(0);
      expect(area.x1 - area.x0).toBe(3);
      expect(area.y1 - area.y0).toBe(3);
      expect(area.x0).toBeGreaterThanOrEqual(origin + REFERENCE_EDGE / 8);
      expect(area.x1).toBeLessThanOrEqual(origin + REFERENCE_EDGE - REFERENCE_EDGE / 8 + 3);
      expect(area.y0).toBeGreaterThan(origin + REFERENCE_EDGE / 2);
      expect(area.y1).toBeLessThan(origin + REFERENCE_EDGE);
    }
  });
});

/** A simulated core and renderer around the series, on a fake clock. */
function harness(config: MeasureConfig, options: { load?: boolean } = {}) {
  let time = 0;
  let nextId = 1;
  let nextQuery = 1;
  let tick = 0;
  let session = 1;
  const commands: Command[] = [];
  const views: View[] = [];
  const pendingQueries: number[] = [];
  const accepted: { id: number; tick: number }[] = [];
  const persons = config.persons;
  const ref: { series?: MeasureSeries } = {};
  const info = (): FrameInfo => ({
    session,
    tick,
    personsTotal: session === 1 ? 0 : persons,
    mapWidth: 2048,
    mapHeight: 2048,
  });
  const host: SeriesHost = {
    now: () => time,
    sleep: (ms) => {
      // Advance in frames of 16 ms; the core answers on the way.
      const end = time + ms;
      while (time < end) {
        time = Math.min(end, time + 16);
        tick += 3;
        const s = ref.series;
        if (s === undefined) continue;
        for (const a of accepted.splice(0)) {
          s.onEvents(time, session, [
            { seq: a.id, tick: a.tick, event: { kind: "commandAccepted", id: a.id, tick: a.tick } },
          ]);
        }
        for (const id of pendingQueries.splice(0)) s.onQuery(time, id);
        s.onFrame(time, info(), 2);
        if (time % 1000 < 16) {
          s.onStats(stats({ lastRecompute: { revision: Math.floor(time / 10_000), millis: 900 } }));
          s.onEvents(time, session, [
            { seq: 0, tick, event: { kind: "waypointsAdopted", revision: 1 } },
          ]);
        }
      }
      return Promise.resolve();
    },
    command: (command) => {
      commands.push(command);
      const id = nextId;
      nextId += 1;
      if (command.kind === "debug" && command.action.kind === "loadScenario") {
        if (options.load !== false) {
          session = 2;
          nextId = 1;
          ref.series?.onSession(2);
        }
      } else {
        accepted.push({ id, tick: tick + 3 });
      }
      return id;
    },
    query: () => {
      const id = nextQuery;
      nextQuery += 1;
      pendingQueries.push(id);
      return id;
    },
    view: (view) => {
      views.push(view);
    },
    hotspot: () => ({ x: 1000, y: 900 }),
    status: () => undefined,
  };
  const series = new MeasureSeries(host, config);
  ref.series = series;
  series.onSession(1);
  return { series, commands, views };
}

describe("MeasureSeries", () => {
  it("runs the long run with alternating views, builds and queries", async () => {
    const config = { ...defaultConfig(1), loadTimeoutMs: 1000 };
    const { series, commands, views } = harness(config);
    series.onLongTask(55);
    const result = await series.run();
    expect(result.version).toBe(1);
    expect(result.windows).toHaveLength(2);
    expect(result.windows.map((w) => w.view)).toEqual(["overview", "circle"]);
    expect(result.threads).toBe(2);
    expect(result.hotspot).toEqual({ x: 1000, y: 900 });
    const [first, second] = result.windows;
    expect(first?.fps).toBeGreaterThan(55);
    expect(first?.queries).toBeGreaterThan(100);
    expect(first?.builds).toBeGreaterThanOrEqual(2);
    expect(first?.buildMsMax).toBeGreaterThan(0);
    expect(first?.adoptMsMax).toBeGreaterThan(0);
    expect(first?.recomputes).toBeGreaterThan(0);
    expect(second?.ticksPerSecMin).toBe(180);
    expect(result.summary.recomputes).toBe(result.windows.reduce((a, w) => a + w.recomputes, 0));

    expect(commands[0]).toEqual({
      kind: "debug",
      action: { kind: "loadScenario", scenario: "referenceTerminal", persons: 10_000 },
    });
    expect(commands[1]).toEqual({ kind: "setSpeed", speed: "x3" });
    expect(commands.at(-1)).toEqual({ kind: "setSpeed", speed: "paused" });
    const walls = commands.filter((c) => c.kind === "debug" && c.action.kind === "setWalls");
    // Every wall is removed again: set and remove alternate and end with a removal.
    expect(walls.length % 2).toBe(0);
    expect(walls.at(-1)).toMatchObject({ action: { wall: false } });
    expect(views.some((v) => v.kind === "circle" && v.angle > 0)).toBe(true);
    // Both views show the ground floor, where the persons are after the warm-up.
    expect(views.every((v) => v.floor === 0)).toBe(true);
    expect(views.find((v) => v.kind === "circle")).toMatchObject({ x: 1000, y: 900, zoom: 4 });
    expect(views.at(-1)).toEqual({ kind: "overview", floor: 0 });
  });

  it("gives up when the scenario does not load", async () => {
    const config = { ...defaultConfig(1), loadTimeoutMs: 500 };
    const { series } = harness(config, { load: false });
    await expect(series.run()).rejects.toThrow("scenario did not load");
  });

  it("stops when the core is not connected", async () => {
    const host: SeriesHost = {
      now: () => 0,
      sleep: () => Promise.resolve(),
      command: () => 0,
      query: () => 0,
      view: () => undefined,
      hotspot: () => ({ x: 0, y: 0 }),
      status: () => undefined,
    };
    await expect(new MeasureSeries(host, defaultConfig(1)).run()).rejects.toThrow("not connected");
  });
});
