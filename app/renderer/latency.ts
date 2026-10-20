/**
 * Spike 3.14: time stamps around every frame request, to find out where a
 * late answer waits (docs/research/3.14-spaete-bilder.md). Throw-away code
 * of the measurement branch, not for `main`.
 *
 * Per request: display tick (`requestAnimationFrame` time stamp), duration of
 * the PixiJS ticker, sending of the request, arrival of a self message
 * through an own `MessageChannel`, arrival of the answer, number of display
 * ticks in between and the stamps of the core process.
 */

/** Stamps of the core process, added to a frame in spike 3.14 (`core-host.ts`). */
export interface CoreStamps {
  /** Arrival of the request in the core process, ms since the epoch. */
  readonly recv: number;
  /** Sending of the frame, ms since the epoch. */
  readonly send: number;
  /** Longest event-loop delay of the core process since the last frame, ms. */
  readonly eld: number;
}

/** An answer this much later than its request counts as late (one tick at 60 Hz is 16.7 ms). */
export const LATE_MS = 12;
/** Most late answers kept per run. */
export const MAX_LATE = 400;

/** Figures of one measurement window. */
export interface LatencyFigures {
  readonly answers: number;
  readonly late: number;
  readonly lateTwoTicks: number;
  readonly arriveMsP50: number;
  readonly arriveMsP99: number;
  readonly arriveMsMax: number;
  /** Answers handled only after at least one more display tick. */
  readonly afterTick: number;
  readonly selfMsP50: number;
  readonly selfMsP99: number;
  readonly selfMsMax: number;
  readonly selfLate: number;
  readonly pixiMsP50: number;
  readonly pixiMsP99: number;
  readonly pixiMsMax: number;
  /** Time from the display tick to the request. */
  readonly sendAfterTickMsP50: number;
  readonly sendAfterTickMsP99: number;
  readonly tickGapMsP99: number;
  readonly tickGapMsMax: number;
  readonly coreMsP99: number;
  readonly coreMsMax: number;
  readonly coreEldMsMax: number;
  /** Renderer send → core arrival and core send → renderer arrival, on two clocks. */
  readonly toCoreMsMin: number;
  readonly toCoreMsP99: number;
  readonly toCoreMsMax: number;
  readonly fromCoreMsMin: number;
  readonly fromCoreMsP99: number;
  readonly fromCoreMsMax: number;
  readonly traced: boolean;
}

/** One late answer. */
export interface LateAnswer {
  /** Seconds since the start of the series. */
  readonly atS: number;
  readonly label: string;
  readonly arriveMs: number;
  readonly selfMs: number;
  readonly ticks: number;
  readonly sendAfterTickMs: number;
  /** Longest PixiJS ticker run between request and answer. */
  readonly pixiMs: number;
  readonly coreMs: number;
  readonly eldMs: number;
  readonly toCoreMs: number;
  readonly fromCoreMs: number;
}

function percentile(sorted: readonly number[], p: number): number {
  if (sorted.length === 0) return 0;
  return sorted[Math.min(sorted.length - 1, Math.floor(p * sorted.length))] ?? 0;
}

function round(v: number): number {
  return Math.round(v * 10) / 10;
}

class Samples {
  readonly #values: number[] = [];
  add(v: number): void {
    if (Number.isFinite(v)) this.#values.push(v);
  }
  stats(): { p50: number; p99: number; max: number; min: number } {
    const s = [...this.#values].sort((a, b) => a - b);
    return {
      p50: round(percentile(s, 0.5)),
      p99: round(percentile(s, 0.99)),
      max: round(s.at(-1) ?? 0),
      min: round(s[0] ?? 0),
    };
  }
}

class Window {
  answers = 0;
  late = 0;
  lateTwo = 0;
  afterTick = 0;
  selfLate = 0;
  traced = false;
  readonly arrive = new Samples();
  readonly self = new Samples();
  readonly pixi = new Samples();
  readonly sendAfter = new Samples();
  readonly tickGap = new Samples();
  readonly core = new Samples();
  readonly eld = new Samples();
  readonly toCore = new Samples();
  readonly fromCore = new Samples();
}

interface Pending {
  readonly sentAt: number;
  readonly sentAbs: number;
  readonly ticks: number;
  readonly sendAfterTick: number;
  selfAt: number | undefined;
  pixiMax: number;
}

/** Time stamps of the frame path in the renderer. */
export class LatencyProbe {
  #ticks = 0;
  #lastTick = 0;
  #pixiStart = 0;
  #pending: Pending | undefined;
  #window = new Window();
  #late: LateAnswer[] = [];
  #recent: number[] = [];
  #startedAt: number | undefined;
  #label = "";
  #marks = false;
  readonly #channel = new MessageChannel();

  constructor(
    private readonly now: () => number = () => performance.now(),
    private readonly origin: number = performance.timeOrigin,
    /** Called once per series at the first burst of late answers. */
    private readonly onBurst: () => void = () => undefined,
  ) {
    this.#channel.port1.onmessage = () => {
      if (this.#pending !== undefined && this.#pending.selfAt === undefined) {
        this.#pending.selfAt = this.now();
      }
    };
  }

  #burstFired = true;

  /** Starts a series: clears the late list and arms the burst trigger. */
  start(): void {
    this.#startedAt = this.now();
    this.#late = [];
    this.#window = new Window();
    this.#burstFired = false;
  }

  /** Ends a series. */
  stop(): void {
    this.#startedAt = undefined;
    this.#burstFired = true;
  }

  /** Name of the current view, kept with late answers. */
  set label(label: string) {
    this.#label = label;
  }

  /** Writes `performance.mark` entries while a trace records (they show in the trace). */
  set marks(on: boolean) {
    this.#marks = on;
    if (on) this.#window.traced = true;
  }

  #mark(name: string): void {
    if (!this.#marks) return;
    performance.mark(name);
    performance.clearMarks(name);
  }

  /** A display tick with the `requestAnimationFrame` time stamp `ts`. */
  onTick(ts: number): void {
    if (this.#lastTick > 0) this.#window.tickGap.add(ts - this.#lastTick);
    this.#lastTick = ts;
    this.#ticks += 1;
  }

  /** Before the PixiJS ticker. */
  onPixiStart(): void {
    this.#pixiStart = this.now();
  }

  /** After the PixiJS ticker (render included). */
  onPixiEnd(): void {
    const ms = this.now() - this.#pixiStart;
    this.#window.pixi.add(ms);
    if (this.#pending !== undefined) this.#pending.pixiMax = Math.max(this.#pending.pixiMax, ms);
  }

  /** A frame request was sent now. */
  onRequest(): void {
    const at = this.now();
    this.#pending = {
      sentAt: at,
      sentAbs: this.origin + at,
      ticks: this.#ticks,
      sendAfterTick: at - this.#lastTick,
      selfAt: undefined,
      pixiMax: 0,
    };
    this.#channel.port2.postMessage(0);
    this.#mark("oas:request");
  }

  /** The answer to the pending request arrived at `at` (before any handling). */
  onAnswer(at: number, stamps: CoreStamps | undefined): void {
    this.#mark("oas:answer");
    const p = this.#pending;
    this.#pending = undefined;
    if (p === undefined) return;
    const w = this.#window;
    const arrive = at - p.sentAt;
    const self = (p.selfAt ?? at) - p.sentAt;
    const ticks = this.#ticks - p.ticks;
    const arrAbs = this.origin + at;
    const core = stamps === undefined ? Number.NaN : stamps.send - stamps.recv;
    const toCore = stamps === undefined ? Number.NaN : stamps.recv - p.sentAbs;
    const fromCore = stamps === undefined ? Number.NaN : arrAbs - stamps.send;
    w.answers += 1;
    w.arrive.add(arrive);
    w.self.add(self);
    w.sendAfter.add(p.sendAfterTick);
    w.core.add(core);
    w.eld.add(stamps?.eld ?? Number.NaN);
    w.toCore.add(toCore);
    w.fromCore.add(fromCore);
    if (ticks > 0) w.afterTick += 1;
    if (self > LATE_MS) w.selfLate += 1;
    if (arrive > LATE_MS) {
      w.late += 1;
      if (arrive > 2 * LATE_MS + 4) w.lateTwo += 1;
      if (this.#startedAt !== undefined && this.#late.length < MAX_LATE) {
        this.#late.push({
          atS: round((at - this.#startedAt) / 1000),
          label: this.#label,
          arriveMs: round(arrive),
          selfMs: round(self),
          ticks,
          sendAfterTickMs: round(p.sendAfterTick),
          pixiMs: round(p.pixiMax),
          coreMs: round(core),
          eldMs: round(stamps?.eld ?? Number.NaN),
          toCoreMs: round(toCore),
          fromCoreMs: round(fromCore),
        });
      }
      this.#recent.push(at);
      while (this.#recent.length > 0 && at - (this.#recent[0] ?? at) > 2000) this.#recent.shift();
      if (!this.#burstFired && this.#recent.length >= 5) {
        this.#burstFired = true;
        this.onBurst();
      }
    }
  }

  /** Figures since the last call; starts a new window. */
  take(): LatencyFigures {
    const w = this.#window;
    this.#window = new Window();
    this.#window.traced = this.#marks;
    const a = w.arrive.stats();
    const s = w.self.stats();
    const px = w.pixi.stats();
    const sa = w.sendAfter.stats();
    const tg = w.tickGap.stats();
    const c = w.core.stats();
    const e = w.eld.stats();
    const tc = w.toCore.stats();
    const fc = w.fromCore.stats();
    return {
      answers: w.answers,
      late: w.late,
      lateTwoTicks: w.lateTwo,
      arriveMsP50: a.p50,
      arriveMsP99: a.p99,
      arriveMsMax: a.max,
      afterTick: w.afterTick,
      selfMsP50: s.p50,
      selfMsP99: s.p99,
      selfMsMax: s.max,
      selfLate: w.selfLate,
      pixiMsP50: px.p50,
      pixiMsP99: px.p99,
      pixiMsMax: px.max,
      sendAfterTickMsP50: sa.p50,
      sendAfterTickMsP99: sa.p99,
      tickGapMsP99: tg.p99,
      tickGapMsMax: tg.max,
      coreMsP99: c.p99,
      coreMsMax: c.max,
      coreEldMsMax: e.max,
      toCoreMsMin: tc.min,
      toCoreMsP99: tc.p99,
      toCoreMsMax: tc.max,
      fromCoreMsMin: fc.min,
      fromCoreMsP99: fc.p99,
      fromCoreMsMax: fc.max,
      traced: w.traced,
    };
  }

  /** Late answers of the series so far. */
  get late(): readonly LateAnswer[] {
    return this.#late;
  }
}

/** Reads the stamps of a frame message; `undefined` if absent or malformed. */
export function coreStamps(data: unknown): CoreStamps | undefined {
  if (typeof data !== "object" || data === null) return undefined;
  const s = (data as { stamps?: unknown }).stamps;
  if (typeof s !== "object" || s === null) return undefined;
  const { recv, send, eld } = s as Record<string, unknown>;
  return typeof recv === "number" && typeof send === "number" && typeof eld === "number"
    ? { recv, send, eld }
    : undefined;
}
