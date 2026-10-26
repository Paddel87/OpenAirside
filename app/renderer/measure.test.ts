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
  WALL_TYPE,
  WindowRecorder,
} from "./measure.js";
import type { CellRect } from "../src/generated/CellRect.js";

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
  it("counts answers that took longer than one display tick as late", () => {
    const rec = new WindowRecorder();
    // 60 frames 16.7 ms apart; three answers miss the next tick, one by two ticks.
    for (let i = 0; i < 60; i += 1) {
      rec.frame(i * 16.7, i === 10 || i === 20 ? 17 : i === 30 ? 34 : 1);
    }
    const w = rec.summarize(
      { window: 0, atS: 0, view: "overview", seconds: 1, tierPx: 8 },
      new Set(),
    );
    expect([w.answers, w.rttLate]).toEqual([60, 3]);
  });

  it("counts no late answers without frame intervals", () => {
    const rec = new WindowRecorder();
    rec.frame(0, 50);
    const w = rec.summarize(
      { window: 0, atS: 0, view: "overview", seconds: 1, tierPx: 8 },
      new Set(),
    );
    expect([w.answers, w.rttLate]).toEqual([1, 0]);
  });

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
    rec.preview(17, 1);
    rec.preview(9, 2);
    rec.rejected();
    rec.adopt(1300);
    rec.query(3);
    rec.query(5);
    const known = new Set<number>([1]);
    const w = rec.summarize({ window: 0, atS: 30, view: "overview", seconds: 1, tierPx: 8 }, known);
    expect(w.fps).toBe(61);
    expect(w.frameMsP50).toBe(16);
    expect(w.frameMsMax).toBe(16);
    expect(w.rttMsMax).toBe(4);
    expect([w.answers, w.rttLate]).toEqual([61, 0]);
    expect(w.longTasks).toBe(1);
    expect(w.ticksPerSecMin).toBe(170);
    expect(w.ticksPerSecMean).toBe(176.7);
    expect(w.stepUsP99Max).toBe(1200);
    expect(w.behindTicksMax).toBe(4);
    expect([w.lagEvents, w.lagBehindMax]).toEqual([1, 5]);
    expect([w.recomputes, w.recomputeMsMax]).toEqual([1, 1100]);
    expect(known.has(2)).toBe(true);
    expect([w.builds, w.buildMsP50, w.buildMsMax, w.buildFramesMax]).toEqual([2, 20, 40, 3]);
    expect([w.previewMsP50, w.previewMsMax, w.previewFramesMax, w.buildsRejected]).toEqual([
      9, 17, 2, 1,
    ]);
    expect(w.adoptMsMax).toBe(1300);
    expect([w.queries, w.queryMsMax]).toEqual([2, 5]);

    // A revision counted before is not counted again.
    const again = new WindowRecorder();
    again.stats(stats({ lastRecompute: { revision: 2, millis: 1100 } }));
    const w2 = again.summarize(
      { window: 1, atS: 60, view: "circle", seconds: 30, tierPx: 8 },
      known,
    );
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
function harness(config: MeasureConfig, options: { load?: boolean; reject?: boolean } = {}) {
  let time = 0;
  let nextId = 1;
  let nextQuery = 1;
  let tick = 0;
  let session = 1;
  const commands: Command[] = [];
  const views: View[] = [];
  const pendingQueries: number[] = [];
  const accepted: { id: number; tick: number; area?: CellRect }[] = [];
  const rejected: number[] = [];
  const blocks = new Map<string, number>();
  const previews: CellRect[] = [];
  let worldFrom = Number.POSITIVE_INFINITY;
  const persons = config.persons;
  const ref: { series?: MeasureSeries } = {};
  const info = (): FrameInfo => ({
    session,
    tick,
    personsTotal: session === 1 ? 0 : persons,
    mapWidth: 2048,
    mapHeight: 2048,
  });
  const key = (x: number, y: number): string =>
    `${String(Math.floor(x / 32))},${String(Math.floor(y / 32))}`;
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
          // The block with the change arrives in the frame of the tick of the acceptance.
          if (a.area !== undefined) blocks.set(key(a.area.x0, a.area.y0), time);
        }
        for (const id of rejected.splice(0)) {
          s.onEvents(time, session, [
            {
              seq: id,
              tick,
              event: {
                kind: "commandRejected",
                id,
                tick,
                reason: { code: "blocked", textKey: "command.rejected.blocked", params: {} },
              },
            },
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
          worldFrom = time + 300;
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
    tierPx: () => 16,
    atlas: () => ({ tiers: [8, 16, 32], pages: 3, textureMbMax: 0.1 }),
    build: (command, area) => {
      commands.push(command);
      previews.push(area);
      const id = nextId;
      nextId += 1;
      if (options.reject === true) rejected.push(id);
      else accepted.push({ id, tick: tick + 3, area });
      return id;
    },
    previewDrawn: () => host.sleep(16).then(() => ({ frames: 1, at: host.now() })),
    world: () => ({
      figures: () => ({
        blocks: 4988,
        bytes: 2_900_000,
        pending: 0,
        objects: 20_000,
        tiles: 16,
        textureMb: 17,
        cpuMb: 16,
        slots: 3,
      }),
      // Part of the world arrives after 300 ms, all of it after 800 ms.
      visibleComplete: () => time >= worldFrom,
      complete: () => time >= worldFrom + 500,
      updatedAt: (x, y) => blocks.get(key(x, y)),
    }),
  };
  const series = new MeasureSeries(host, config);
  ref.series = series;
  series.onSession(1);
  return { series, commands, views, previews };
}

describe("MeasureSeries", () => {
  it("runs the long run with alternating views, real builds and queries", async () => {
    const config = { ...defaultConfig(1), loadTimeoutMs: 5000 };
    const { series, commands, views, previews } = harness(config);
    series.onLongTask(55);
    const result = await series.run();
    expect(result.version).toBe(3);
    expect(result.windows).toHaveLength(2);
    expect(result.windows.map((w) => w.view)).toEqual(["overview", "circle"]);
    expect(result.threads).toBe(2);
    expect(result.hotspot).toEqual({ x: 1000, y: 900 });
    expect(result.atlas).toEqual({ tiers: [8, 16, 32], pages: 3, textureMbMax: 0.1 });
    // The first transfer: the visible part, then the whole world, counted from the start of the session.
    expect(result.world.firstVisibleMs).toBeGreaterThanOrEqual(300);
    expect(result.world.firstCompleteMs).toBeGreaterThanOrEqual(800);
    expect(result.world.firstCompleteMs).toBeLessThan(900);
    expect(result.world.figures).toMatchObject({ blocks: 4988, objects: 20_000 });
    expect(result.windows.map((w) => w.tierPx)).toEqual([16, 16]);
    const [first, second] = result.windows;
    expect(first?.fps).toBeGreaterThan(55);
    expect(first?.queries).toBeGreaterThan(100);
    expect(first?.builds).toBeGreaterThanOrEqual(2);
    expect(first?.buildMsMax).toBeGreaterThan(0);
    expect(first?.previewMsMax).toBeGreaterThan(0);
    expect(first?.previewFramesMax).toBe(1);
    expect(first?.buildsRejected).toBe(0);
    expect(first?.adoptMsMax).toBeGreaterThan(0);
    expect(first?.recomputes).toBeGreaterThan(0);
    expect(second?.ticksPerSecMin).toBe(180);
    expect(result.summary.recomputes).toBe(result.windows.reduce((a, w) => a + w.recomputes, 0));

    expect(commands[0]).toEqual({
      kind: "debug",
      action: { kind: "loadScenario", scenario: "furnishedTerminal", persons: 10_000 },
    });
    expect(commands[1]).toEqual({ kind: "setSpeed", speed: "x3" });
    expect(commands.at(-1)).toEqual({ kind: "setSpeed", speed: "paused" });
    // The builds are the real commands, a wall and its demolition alternating, ending with the demolition.
    const builds = commands.filter((c) => c.kind === "buildArea" || c.kind === "demolish");
    expect(builds.length).toBeGreaterThanOrEqual(2);
    expect(builds.length % 2).toBe(0);
    expect(builds[0]).toMatchObject({ kind: "buildArea", object: WALL_TYPE });
    expect(builds.at(-1)?.kind).toBe("demolish");
    expect(previews).toHaveLength(builds.length);
    expect(views.some((v) => v.kind === "circle" && v.angle > 0)).toBe(true);
    // Before the scenario the camera looks closely at the middle of the map, so that the visible part arrives first.
    expect(views[0]).toMatchObject({ kind: "circle", zoom: 4, x: 1024, y: 1024 });
    // Both views show the ground floor, where the persons are after the warm-up.
    expect(views.every((v) => v.floor === 0)).toBe(true);
    expect(views.find((v) => v.kind === "circle" && v.x === 1000)).toMatchObject({ zoom: 4 });
    expect(views.at(-1)).toEqual({ kind: "overview", floor: 0 });
  });

  it("counts refused builds and does not time them", async () => {
    const config = { ...defaultConfig(1), loadTimeoutMs: 5000 };
    const { series } = harness(config, { reject: true });
    const result = await series.run();
    const rejected = result.windows.reduce((a, w) => a + w.buildsRejected, 0);
    expect(rejected).toBeGreaterThanOrEqual(2);
    expect(result.windows.reduce((a, w) => a + w.builds, 0)).toBe(0);
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
      tierPx: () => 0,
      atlas: () => ({ tiers: [], pages: 0, textureMbMax: 0 }),
      build: () => 0,
      previewDrawn: () => Promise.resolve({ frames: 1, at: 0 }),
      world: () => ({
        figures: () => ({
          blocks: 0,
          bytes: 0,
          pending: 0,
          objects: 0,
          tiles: 0,
          textureMb: 0,
          cpuMb: 0,
          slots: 0,
        }),
        visibleComplete: () => false,
        complete: () => false,
        updatedAt: () => undefined,
      }),
    };
    await expect(new MeasureSeries(host, defaultConfig(1)).run()).rejects.toThrow("not connected");
  });
});
