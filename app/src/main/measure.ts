/**
 * Main-process side of the measurement tool of step 2.7 (S-6 `measure.*`,
 * additive): samples the memory of all processes while a run is open and
 * turns the renderer's result into the JSON file of the measurement
 * protocol. No Electron imports, so that the rules are testable.
 *
 * The renderer only triggers: the main process chooses folder and file name,
 * accepts one report per started run and limits its size.
 */

/** Largest accepted result of the renderer, in bytes of its JSON form. */
export const MAX_REPORT_BYTES = 1_000_000;
/** Value of the field `tool` of a result file. */
export const TOOL = "openairside-measure";
/** Interval of the memory samples in milliseconds. */
export const MEMORY_INTERVAL_MS = 5000;
/** Most memory samples kept (two hours at 5 s). */
export const MAX_MEMORY_SAMPLES = 1440;

/** The part of `app.getAppMetrics()` entries the sampler reads. */
export interface ProcessMetric {
  readonly type: string;
  readonly serviceName?: string | undefined;
  readonly name?: string | undefined;
  readonly memory: { readonly workingSetSize: number };
}

/** Memory of all processes at one moment, in MB (working set). */
export interface MemorySample {
  /** Seconds since the start of the run. */
  readonly s: number;
  readonly totalMb: number;
  /** Per process type; the core process is listed as `core`. */
  readonly byTypeMb: Readonly<Record<string, number>>;
}

/** Turns process metrics into one sample; `coreService` names the core process. */
export function memorySample(
  metrics: readonly ProcessMetric[],
  coreService: string,
  seconds: number,
): MemorySample {
  const byType: Record<string, number> = {};
  for (const m of metrics) {
    const key = m.serviceName === coreService || m.name === coreService ? "core" : m.type;
    // workingSetSize is in kilobytes.
    byType[key] = (byType[key] ?? 0) + m.memory.workingSetSize / 1024;
  }
  const total = Object.values(byType).reduce((a, b) => a + b, 0);
  return {
    s: Math.round(seconds),
    totalMb: Math.round(total),
    byTypeMb: Object.fromEntries(Object.entries(byType).map(([k, v]) => [k, Math.round(v)])),
  };
}

/** File name of a result, e.g. `openairside-measure-win32-2026-10-14T07-05-00-000Z.json`. */
export function reportFileName(platform: string, at: Date): string {
  const safePlatform = platform.replace(/[^a-z0-9]/gi, "");
  return `openairside-measure-${safePlatform}-${at.toISOString().replace(/[:.]/g, "-")}.json`;
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const proto = Object.getPrototypeOf(value) as unknown;
  return proto === Object.prototype || proto === null;
}

/** Why a report was refused. */
export type ReportError = "notStarted" | "notAnObject" | "tooLarge";

/** One measurement run as seen by the main process. */
export class MeasureRun {
  #startedAt: number | undefined;
  #samples: MemorySample[] = [];

  /** `true` between `start` and the accepted report. */
  get running(): boolean {
    return this.#startedAt !== undefined;
  }

  /** Starts a run at `now` (ms); `false` if one is already open. */
  start(now: number): boolean {
    if (this.#startedAt !== undefined) {
      return false;
    }
    this.#startedAt = now;
    this.#samples = [];
    return true;
  }

  /** Records the memory at `now` while a run is open. */
  sample(metrics: readonly ProcessMetric[], coreService: string, now: number): void {
    const startedAt = this.#startedAt;
    if (startedAt === undefined || this.#samples.length >= MAX_MEMORY_SAMPLES) {
      return;
    }
    this.#samples.push(memorySample(metrics, coreService, (now - startedAt) / 1000));
  }

  /** Samples taken so far. */
  get samples(): readonly MemorySample[] {
    return this.#samples;
  }

  /**
   * Checks the renderer's `result` and returns the JSON text of the file
   * with `system` data and the memory samples; ends the run on success.
   */
  finish(
    result: unknown,
    system: Readonly<Record<string, unknown>>,
  ): { ok: true; json: string } | { ok: false; error: ReportError } {
    if (this.#startedAt === undefined) {
      return { ok: false, error: "notStarted" };
    }
    if (!isPlainObject(result)) {
      return { ok: false, error: "notAnObject" };
    }
    let body: string;
    try {
      body = JSON.stringify(result);
    } catch {
      return { ok: false, error: "notAnObject" };
    }
    if (body.length > MAX_REPORT_BYTES) {
      return { ok: false, error: "tooLarge" };
    }
    // The fields of the main process come first and cannot be overridden.
    const file: Record<string, unknown> = {
      tool: TOOL,
      system,
      ...(JSON.parse(body) as Record<string, unknown>),
    };
    file.tool = TOOL;
    file.system = system;
    file.memory = this.#samples;
    const json = JSON.stringify(file, null, 2);
    this.#startedAt = undefined;
    return { ok: true, json: `${json}\n` };
  }
}
