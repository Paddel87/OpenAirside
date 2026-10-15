/**
 * Performance display of the debug panel: frames per second, round trip of
 * frame requests and values of the newest frame. Display only.
 */

/** Values shown in the performance display. */
export interface PerfSnapshot {
  /** Frames drawn in the last second. */
  readonly fps: number;
  /** Median round trip request → frame in the last second, milliseconds. */
  readonly rttMs: number;
  /** Largest round trip in the last second, milliseconds. */
  readonly rttMaxMs: number;
}

/** Collects frame times over a sliding window of one second. */
export class PerfMeter {
  readonly #frames: number[] = [];
  readonly #rtts: { at: number; ms: number }[] = [];

  /** Records a drawn frame at `now` with its round trip `rttMs`. */
  frame(now: number, rttMs: number): void {
    this.#frames.push(now);
    this.#rtts.push({ at: now, ms: rttMs });
    this.#trim(now);
  }

  /** Values of the last second before `now`. */
  snapshot(now: number): PerfSnapshot {
    this.#trim(now);
    const sorted = this.#rtts.map((r) => r.ms).sort((a, b) => a - b);
    return {
      fps: this.#frames.length,
      rttMs: sorted[Math.floor((sorted.length - 1) / 2)] ?? 0,
      rttMaxMs: sorted.at(-1) ?? 0,
    };
  }

  #trim(now: number): void {
    while (this.#frames.length > 0 && now - (this.#frames[0] ?? now) > 1000) {
      this.#frames.shift();
    }
    while (this.#rtts.length > 0 && now - (this.#rtts[0]?.at ?? now) > 1000) {
      this.#rtts.shift();
    }
  }
}
