/**
 * Measurement tool of step 2.7 (module "Benchmark & Leistung"): the long
 * run of spike 1.5 on production code. Loads the furnished reference
 * terminal (floors, walls and 20,000 objects of the base pack, step 3.1b)
 * with 10,000 persons, runs it at the highest game speed and records per
 * window of 30 s the frame rate and frame times, the round trip of frame
 * requests, ticks per second, tick times and lag (from `coreStats`, S-3),
 * a build action every 10 s (a 3 × 3 wall built and demolished again with
 * the real commands `buildArea` and `demolish`, with the provisional
 * drawing as the build tool does it), a table query 4 times per second
 * (S-7) and long tasks of the renderer. Before the series it records the
 * time the resting world needs to arrive in the playfield (first transfer,
 * ADR-038). The camera alternates per window between the whole ground
 * floor and circling its densest spot.
 *
 * Display and measurement only (rule 002): the core decides every effect;
 * the series only sends commands and queries. Timers and the camera come
 * from a host, so that the series is testable without a browser.
 */
import type { CellRect } from "../src/generated/CellRect.js";
import type { Command } from "../src/generated/Command.js";
import type { EventRecord } from "../src/generated/EventRecord.js";
import type { Query } from "../src/generated/Query.js";
import type { Scenario } from "../src/generated/Scenario.js";
import type { TickStats } from "../src/generated/TickStats.js";
import type { WorldFigures } from "./world-view.js";

/** Version of the result format; raised by every non-additive change. */
export const RESULT_VERSION = 2;
/** Edge of the reference terminal in cells (core/src/scenario.rs `REFERENCE_EDGE`). */
export const REFERENCE_EDGE = 768;
/** Panel id of the measurement's table query. */
export const MEASURE_PANEL = 2;
/** Most minutes of one run. */
export const MAX_MINUTES = 60;

/** Settings of a run. */
export interface MeasureConfig {
  readonly minutes: number;
  readonly persons: number;
  readonly windowSeconds: number;
  readonly warmupSeconds: number;
  readonly buildEverySeconds: number;
  readonly queriesPerSecond: number;
  readonly tableRows: number;
  /** Longest wait for the scenario and its world, milliseconds. */
  readonly loadTimeoutMs: number;
  /** The scenario of the run. */
  readonly scenario: Scenario;
}

/**
 * Settings of spike 1.5 (fahrplan.md step 2.7) with the furnished terminal
 * of step 3.1b (20,000 objects: estimate of step 2.10, re-measured in 4.1).
 */
export function defaultConfig(minutes: number): MeasureConfig {
  return {
    minutes,
    persons: 10_000,
    windowSeconds: 30,
    warmupSeconds: 10,
    buildEverySeconds: 10,
    queriesPerSecond: 4,
    tableRows: 1000,
    loadTimeoutMs: 180_000,
    scenario: "furnishedTerminal",
  };
}

/** What the camera shows. */
export type View =
  | { readonly kind: "overview"; readonly floor: number }
  | {
      readonly kind: "circle";
      readonly floor: number;
      readonly x: number;
      readonly y: number;
      readonly radius: number;
      readonly zoom: number;
      /** Angle in radians. */
      readonly angle: number;
    };

/** What the series needs from the renderer. */
export interface SeriesHost {
  now(): number;
  sleep(ms: number): Promise<void>;
  /** Sends a command; returns its id, 0 if not connected. */
  command(command: Command): number;
  /** Sends a query; returns its id, 0 if not possible right now. */
  query(query: Query): number;
  /** Points the camera. */
  view(view: View): void;
  /** Densest spot among the persons drawn right now, in cells. */
  hotspot(): { readonly x: number; readonly y: number };
  /** Progress line for the player. */
  status(text: string): void;
  /** Resolution of the atlas drawn with now in pixels per cell, 0 without atlas. */
  tierPx(): number;
  /** The atlas of the build, for the result. */
  atlas(): AtlasInfo;
  /**
   * Sends a build command and draws its provisional drawing over `area`
   * like the build tool (S-2); returns the command id, 0 if not connected.
   */
  build(command: Command, area: CellRect): number;
  /** Resolves after the next frame was rendered. */
  nextFrame(): Promise<void>;
  /** The resting world as the playfield holds it. */
  world(): WorldReadout;
}

/** What the playfield knows of the resting world. */
export interface WorldReadout {
  figures(): WorldFigures;
  /** Every block of the shown floor in view was received. */
  visibleComplete(): boolean;
  /** The core has nothing more to send. */
  complete(): boolean;
  /** Time the block holding the cell of the shown floor was last updated, if ever. */
  updatedAt(x: number, y: number): number | undefined;
}

/** The atlas the playfield drew with, for the result. */
export interface AtlasInfo {
  /** Resolutions of the atlas in pixels per cell; empty without atlas. */
  readonly tiers: readonly number[];
  /** Atlas pages of all resolutions. */
  readonly pages: number;
  /** Texture memory of the largest resolution, MB. */
  readonly textureMbMax: number;
}

/** What the playfield held of the world, for the result. */
export interface WorldResult {
  /** From the start of the session to the last block in view (ms), `undefined` if never. */
  readonly firstVisibleMs: number | undefined;
  /** From the start of the session to the last block of the world (ms), `undefined` if never. */
  readonly firstCompleteMs: number | undefined;
  /** Figures after the transfer. */
  readonly figures: WorldFigures;
}

/** Header values of a drawn frame the series needs. */
export interface FrameInfo {
  readonly session: number;
  readonly tick: number;
  readonly personsTotal: number;
  readonly mapWidth: number;
  readonly mapHeight: number;
}

/** Figures of one window. */
export interface WindowResult {
  readonly window: number;
  readonly atS: number;
  readonly view: View["kind"];
  readonly seconds: number;
  /** Atlas resolution drawn with at the end of the window, pixels per cell; 0 without atlas. */
  readonly tierPx: number;
  readonly fps: number;
  readonly frameMsP50: number;
  readonly frameMsP99: number;
  readonly frameMsMax: number;
  readonly rttMsP50: number;
  readonly rttMsP99: number;
  readonly rttMsMax: number;
  /** Answers to frame requests drawn in the window. */
  readonly answers: number;
  /**
   * Late answers: round trip longer than one display tick (`frameMsP50`), so
   * the answer missed the next tick (step 3.15, as "late" in spike 3.14).
   */
  readonly rttLate: number;
  readonly longTasks: number;
  readonly longTaskMsMax: number;
  readonly ticksPerSecMean: number;
  readonly ticksPerSecMin: number;
  readonly stepUsMean: number;
  readonly stepUsP99Max: number;
  readonly stepUsMax: number;
  readonly behindTicksMax: number;
  readonly lagEvents: number;
  readonly lagBehindMax: number;
  readonly recomputes: number;
  readonly recomputeMsMean: number;
  readonly recomputeMsMax: number;
  /** Build actions that reached the world display (command to the frame with the changed block). */
  readonly builds: number;
  readonly buildMsP50: number;
  readonly buildMsMax: number;
  readonly buildFramesMax: number;
  /** Command to the next rendered frame with the provisional drawing (step 3.1b, NFR "Bauaktion im nächsten Bild"). */
  readonly previewMsP50: number;
  readonly previewMsMax: number;
  /** Build actions the core refused. */
  readonly buildsRejected: number;
  readonly adoptMsMax: number;
  readonly queries: number;
  readonly queryMsP50: number;
  readonly queryMsMax: number;
}

/** Result of a run, before the main process adds system and memory. */
export interface SeriesResult {
  readonly version: number;
  readonly config: MeasureConfig;
  readonly initMs: number;
  readonly threads: number;
  readonly backgroundThreads: number;
  readonly hotspot: { readonly x: number; readonly y: number };
  readonly atlas: AtlasInfo;
  readonly world: WorldResult;
  readonly windows: readonly WindowResult[];
  readonly summary: Summary;
}

/** Worst and mean values over all windows. */
export interface Summary {
  readonly fpsMin: number;
  readonly fpsMean: number;
  readonly frameMsP99Max: number;
  readonly ticksPerSecMin: number;
  readonly stepUsP99Max: number;
  readonly behindTicksMax: number;
  readonly lagEvents: number;
  readonly recomputes: number;
  readonly recomputeMsMax: number;
  readonly buildMsMax: number;
  readonly queryMsMax: number;
}

/** Value at quantile `q` (0 … 1) of ascending `sorted` (nearest rank); 0 if empty. */
export function quantile(sorted: readonly number[], q: number): number {
  if (sorted.length === 0) {
    return 0;
  }
  const rank = Math.max(1, Math.ceil(sorted.length * q));
  return sorted[Math.min(sorted.length, rank) - 1] ?? 0;
}

function round(value: number, digits = 2): number {
  const f = 10 ** digits;
  return Math.round(value * f) / f;
}

function milliseconds(value: number | undefined): number | undefined {
  return value === undefined ? undefined : round(value, 0);
}

function ascending(values: readonly number[]): number[] {
  return [...values].sort((a, b) => a - b);
}

function mean(values: readonly number[]): number {
  return values.length === 0 ? 0 : values.reduce((a, b) => a + b, 0) / values.length;
}

function max(values: readonly number[]): number {
  return values.reduce((a, b) => Math.max(a, b), 0);
}

/** Collects the samples of one window. */
export class WindowRecorder {
  readonly #frameTimes: number[] = [];
  readonly #rtts: number[] = [];
  readonly #longTasks: number[] = [];
  readonly #stats: TickStats[] = [];
  readonly #lags: number[] = [];
  readonly #recomputes = new Map<number, number>();
  readonly #builds: { ms: number; frames: number }[] = [];
  readonly #previews: number[] = [];
  #rejected = 0;
  readonly #adopts: number[] = [];
  readonly #queries: number[] = [];
  #lastFrame: number | undefined;

  frame(now: number, rttMs: number): void {
    if (this.#lastFrame !== undefined) {
      this.#frameTimes.push(now - this.#lastFrame);
    }
    this.#lastFrame = now;
    this.#rtts.push(rttMs);
  }

  longTask(ms: number): void {
    this.#longTasks.push(ms);
  }

  stats(stats: TickStats): void {
    this.#stats.push(stats);
    const last = stats.lastRecompute;
    if (last !== undefined) {
      this.#recomputes.set(last.revision, last.millis);
    }
  }

  lag(behindTicks: number): void {
    this.#lags.push(behindTicks);
  }

  build(ms: number, frames: number): void {
    this.#builds.push({ ms, frames });
  }

  /** The provisional drawing of a build action was rendered `ms` after the command. */
  preview(ms: number): void {
    this.#previews.push(ms);
  }

  /** The core refused a build action. */
  rejected(): void {
    this.#rejected += 1;
  }

  adopt(ms: number): void {
    this.#adopts.push(ms);
  }

  query(ms: number): void {
    this.#queries.push(ms);
  }

  /**
   * Figures of the window. `known` holds recompute revisions already
   * counted in earlier windows; new ones are added to it.
   */
  summarize(
    head: { window: number; atS: number; view: View["kind"]; seconds: number; tierPx: number },
    known: Set<number>,
  ): WindowResult {
    const frames = ascending(this.#frameTimes);
    const rtts = ascending(this.#rtts);
    // One display tick: the median interval between drawn frames.
    const tick = quantile(frames, 0.5);
    const builds = ascending(this.#builds.map((b) => b.ms));
    const previews = ascending(this.#previews);
    const queries = ascending(this.#queries);
    const tps = this.#stats.map((s) => s.ticksPerSecond);
    const recomputes: number[] = [];
    for (const [revision, millis] of this.#recomputes) {
      if (!known.has(revision)) {
        known.add(revision);
        recomputes.push(millis);
      }
    }
    return {
      ...head,
      seconds: round(head.seconds),
      fps: round(this.#rtts.length / Math.max(head.seconds, 1e-9), 1),
      frameMsP50: round(tick),
      frameMsP99: round(quantile(frames, 0.99)),
      frameMsMax: round(max(frames)),
      rttMsP50: round(quantile(rtts, 0.5)),
      rttMsP99: round(quantile(rtts, 0.99)),
      rttMsMax: round(max(rtts)),
      answers: rtts.length,
      rttLate: frames.length === 0 ? 0 : rtts.filter((ms) => ms > tick).length,
      longTasks: this.#longTasks.length,
      longTaskMsMax: round(max(this.#longTasks)),
      ticksPerSecMean: round(mean(tps), 1),
      ticksPerSecMin: tps.length === 0 ? 0 : Math.min(...tps),
      stepUsMean: round(mean(this.#stats.map((s) => s.stepMeanUs)), 1),
      stepUsP99Max: max(this.#stats.map((s) => s.stepP99Us)),
      stepUsMax: max(this.#stats.map((s) => s.stepMaxUs)),
      behindTicksMax: max(this.#stats.map((s) => s.behindTicks)),
      lagEvents: this.#lags.length,
      lagBehindMax: max(this.#lags),
      recomputes: recomputes.length,
      recomputeMsMean: round(mean(recomputes), 1),
      recomputeMsMax: max(recomputes),
      builds: builds.length,
      buildMsP50: round(quantile(builds, 0.5)),
      buildMsMax: round(max(builds)),
      buildFramesMax: max(this.#builds.map((b) => b.frames)),
      previewMsP50: round(quantile(previews, 0.5)),
      previewMsMax: round(max(previews)),
      buildsRejected: this.#rejected,
      adoptMsMax: round(max(this.#adopts)),
      queries: queries.length,
      queryMsP50: round(quantile(queries, 0.5)),
      queryMsMax: round(max(queries)),
    };
  }
}

/** Worst and mean values over `windows`. */
export function summarize(windows: readonly WindowResult[]): Summary {
  const fps = windows.map((w) => w.fps);
  return {
    fpsMin: fps.length === 0 ? 0 : Math.min(...fps),
    fpsMean: round(mean(fps), 1),
    frameMsP99Max: max(windows.map((w) => w.frameMsP99)),
    ticksPerSecMin: windows.length === 0 ? 0 : Math.min(...windows.map((w) => w.ticksPerSecMin)),
    stepUsP99Max: max(windows.map((w) => w.stepUsP99Max)),
    behindTicksMax: max(windows.map((w) => w.behindTicksMax)),
    lagEvents: windows.reduce((a, w) => a + w.lagEvents, 0),
    recomputes: windows.reduce((a, w) => a + w.recomputes, 0),
    recomputeMsMax: max(windows.map((w) => w.recomputeMsMax)),
    buildMsMax: max(windows.map((w) => w.buildMsMax)),
    queryMsMax: max(windows.map((w) => w.queryMsMax)),
  };
}

/**
 * A 3 × 3 wall area in the landside hall of the ground floor of the
 * reference terminal, centred on a map of `mapWidth` × `mapHeight` cells
 * (core/src/scenario.rs: hall below the security line in the middle row).
 * `seed` is advanced by a linear congruential generator.
 */
export function buildArea(
  mapWidth: number,
  mapHeight: number,
  seed: number,
): { area: CellRect; seed: number } {
  const next = (Math.imul(seed, 1_103_515_245) + 12_345) >>> 0;
  const edge = REFERENCE_EDGE;
  const ox = Math.floor((mapWidth - edge) / 2);
  const oy = Math.floor((mapHeight - edge) / 2);
  const x0 = ox + Math.floor(edge / 8) + (next % Math.floor((edge * 3) / 4));
  const y0 = oy + edge / 2 + 15 + ((next >>> 12) % Math.floor(edge / 2 - 90));
  return { area: { floor: 0, x0, y0, x1: x0 + 3, y1: y0 + 3 }, seed: next };
}

interface PendingBuild {
  readonly id: number;
  readonly sentAt: number;
  /** Cell whose block shows the change. */
  readonly cell: { readonly x: number; readonly y: number };
  acceptedTick?: number;
  frames: number;
  /** Frames and time until a frame with the changed block was drawn. */
  worldFrames?: number;
  worldMs?: number;
}

/** Wall type the series builds (base pack, step 3.1a). */
export const WALL_TYPE = "base:wall_concrete";

/** One run of the measurement series. */
export class MeasureSeries {
  readonly #host: SeriesHost;
  readonly #config: MeasureConfig;
  #recorder = new WindowRecorder();
  #session = 0;
  #frame: FrameInfo | undefined;
  #stats: TickStats | undefined;
  #view: View = { kind: "overview", floor: 0 };
  #circleStart = 0;
  #build: PendingBuild | undefined;
  #lastBuildAt: number | undefined;
  #walls: CellRect | undefined;
  #seed = 12_345;
  /** Start of the session of the scenario, for the first transfer of the world. */
  #sessionAt = 0;
  /** The first transfer of the world is being timed. */
  #transfer: { visibleMs?: number; completeMs?: number } | undefined;
  readonly #queriesSent = new Map<number, number>();

  constructor(host: SeriesHost, config: MeasureConfig) {
    this.#host = host;
    this.#config = config;
  }

  /** A new session began (scenario loaded). */
  onSession(session: number): void {
    this.#session = session;
    this.#sessionAt = this.#host.now();
    // Query ids start anew with every session.
    this.#queriesSent.clear();
  }

  /** A frame of `info` was drawn at `now`, `rttMs` after its request. */
  onFrame(now: number, info: FrameInfo, rttMs: number): void {
    this.#frame = info;
    this.#recorder.frame(now, rttMs);
    const transfer = this.#transfer;
    if (transfer !== undefined && info.session === this.#session) {
      const world = this.#host.world();
      if (transfer.visibleMs === undefined && world.visibleComplete()) {
        transfer.visibleMs = now - this.#sessionAt;
      }
      if (transfer.completeMs === undefined && world.complete()) {
        transfer.completeMs = now - this.#sessionAt;
      }
    }
    const build = this.#build;
    if (build !== undefined && info.session === this.#session) {
      build.frames += 1;
      if (build.worldMs === undefined) {
        const changed = this.#host.world().updatedAt(build.cell.x, build.cell.y);
        if (changed !== undefined && changed >= build.sentAt) {
          build.worldMs = now - build.sentAt;
          build.worldFrames = build.frames;
        }
      }
      if (
        build.worldMs !== undefined &&
        build.acceptedTick !== undefined &&
        info.tick >= build.acceptedTick
      ) {
        this.#recorder.build(build.worldMs, build.worldFrames ?? build.frames);
        this.#build = undefined;
      }
    }
    if (this.#view.kind === "circle") {
      const angle = ((now - this.#circleStart) / 1000) * 0.6;
      this.#view = { ...this.#view, angle };
      this.#host.view(this.#view);
    }
  }

  /** Figures of the tick arrived. */
  onStats(stats: TickStats): void {
    this.#stats = stats;
    this.#recorder.stats(stats);
  }

  /** Events of the current session arrived at `now`. */
  onEvents(now: number, session: number, events: readonly EventRecord[]): void {
    if (session !== this.#session) {
      return;
    }
    for (const { event } of events) {
      if (event.kind === "simulationLag") {
        this.#recorder.lag(event.behindTicks);
      } else if (event.kind === "commandAccepted" && event.id === this.#build?.id) {
        this.#build.acceptedTick = event.tick;
      } else if (event.kind === "commandRejected" && event.id === this.#build?.id) {
        this.#recorder.rejected();
        this.#build = undefined;
        this.#lastBuildAt = undefined;
      } else if (event.kind === "waypointsAdopted" && this.#lastBuildAt !== undefined) {
        this.#recorder.adopt(now - this.#lastBuildAt);
        this.#lastBuildAt = undefined;
      }
    }
  }

  /** Answer of query `id` arrived at `now`. */
  onQuery(now: number, id: number): void {
    const sentAt = this.#queriesSent.get(id);
    if (sentAt !== undefined) {
      this.#queriesSent.delete(id);
      this.#recorder.query(now - sentAt);
    }
  }

  /** A long task of `ms` milliseconds blocked the renderer. */
  onLongTask(ms: number): void {
    this.#recorder.longTask(ms);
  }

  /** Runs the series; rejects if the scenario does not load in time. */
  async run(): Promise<SeriesResult> {
    const host = this.#host;
    const c = this.#config;
    const t0 = host.now();
    const startSession = this.#session;
    host.status(`measure: loading the furnished terminal with ${String(c.persons)} persons`);
    // A close view over the middle of the map: the visible part of the world must arrive first.
    this.#show({ kind: "circle", floor: 0, x: 1024, y: 1024, radius: 0, zoom: 4, angle: 0 });
    this.#transfer = {};
    this.#send({
      kind: "debug",
      action: { kind: "loadScenario", scenario: c.scenario, persons: c.persons },
    });
    await this.#until(
      () =>
        this.#session !== startSession &&
        this.#frame?.session === this.#session &&
        this.#frame.personsTotal === c.persons,
      c.loadTimeoutMs,
      "scenario did not load",
    );
    const initMs = round(host.now() - t0, 0);
    host.status("measure: waiting for the world");
    await this.#until(
      () => this.#transfer?.completeMs !== undefined,
      c.loadTimeoutMs,
      "the world did not arrive",
    );
    const world: WorldResult = {
      firstVisibleMs: milliseconds(this.#transfer.visibleMs),
      firstCompleteMs: milliseconds(this.#transfer.completeMs),
      figures: host.world().figures(),
    };
    this.#transfer = undefined;

    this.#send({ kind: "setSpeed", speed: "x3" });
    this.#show({ kind: "overview", floor: 0 });
    host.status(`measure: warm-up ${String(c.warmupSeconds)} s`);
    await host.sleep(c.warmupSeconds * 1000);
    const hotspot = host.hotspot();

    const windows: WindowResult[] = [];
    const known = new Set<number>();
    const started = host.now();
    const end = started + c.minutes * 60_000;
    let nextBuild = started + 2000;
    for (let k = 0; host.now() < end; k += 1) {
      const view: View =
        k % 2 === 0
          ? { kind: "overview", floor: 0 }
          : { kind: "circle", floor: 0, ...hotspot, radius: 115, zoom: 4, angle: 0 };
      this.#show(view);
      const left = Math.max(0, Math.round((end - host.now()) / 60_000));
      host.status(
        `measure: window ${String(k + 1)}, about ${String(left)} min left – do not touch, keep the window visible`,
      );
      this.#recorder = new WindowRecorder();
      const w0 = host.now();
      while (host.now() - w0 < c.windowSeconds * 1000) {
        const now = host.now();
        if (now >= nextBuild) {
          nextBuild += c.buildEverySeconds * 1000;
          this.#toggleWalls(now);
        }
        this.#sendQuery(now);
        await host.sleep(1000 / c.queriesPerSecond);
      }
      const seconds = (host.now() - w0) / 1000;
      const result = this.#recorder.summarize(
        {
          window: k,
          atS: Math.round((host.now() - started) / 1000),
          view: view.kind,
          seconds,
          tierPx: host.tierPx(),
        },
        known,
      );
      windows.push(result);
    }
    if (this.#walls !== undefined) {
      this.#toggleWalls(host.now());
    }
    this.#send({ kind: "setSpeed", speed: "paused" });
    this.#show({ kind: "overview", floor: 0 });
    return {
      version: RESULT_VERSION,
      config: c,
      initMs,
      threads: this.#stats?.threads ?? 0,
      backgroundThreads: this.#stats?.backgroundThreads ?? 0,
      hotspot,
      atlas: host.atlas(),
      world,
      windows,
      summary: summarize(windows),
    };
  }

  #send(command: Command): number {
    const id = this.#host.command(command);
    if (id === 0) {
      throw new Error("not connected");
    }
    return id;
  }

  #show(view: View): void {
    this.#view = view;
    this.#circleStart = this.#host.now();
    this.#host.view(view);
  }

  /** Builds a 3 × 3 wall, or demolishes it again, as the build tool does (command and provisional drawing). */
  #toggleWalls(now: number): void {
    const frame = this.#frame;
    if (frame === undefined) {
      return;
    }
    const host = this.#host;
    let area = this.#walls;
    const wall = area === undefined;
    if (area === undefined) {
      const next = buildArea(frame.mapWidth, frame.mapHeight, this.#seed);
      this.#seed = next.seed;
      area = next.area;
    }
    this.#walls = wall ? area : undefined;
    const command: Command = wall
      ? { kind: "buildArea", object: WALL_TYPE, area }
      : { kind: "demolish", area };
    const id = host.build(command, area);
    if (id === 0) {
      throw new Error("not connected");
    }
    this.#build = { id, sentAt: now, frames: 0, cell: { x: area.x0, y: area.y0 } };
    this.#lastBuildAt = now;
    const recorder = this.#recorder;
    void host.nextFrame().then(() => {
      recorder.preview(host.now() - now);
    });
  }

  #sendQuery(now: number): void {
    const id = this.#host.query({
      kind: "table",
      panel: MEASURE_PANEL,
      table: "debugPersons",
      columns: ["handle", "x", "y", "floor", "state"],
      filter: [],
      sort: [
        { column: "y", descending: false },
        { column: "x", descending: false },
      ],
      window: { offset: 0, limit: this.#config.tableRows },
    });
    if (id !== 0) {
      this.#queriesSent.set(id, now);
    }
  }

  async #until(done: () => boolean, timeoutMs: number, error: string): Promise<void> {
    const deadline = this.#host.now() + timeoutMs;
    while (!done()) {
      if (this.#host.now() >= deadline) {
        throw new Error(error);
      }
      await this.#host.sleep(50);
    }
  }
}
