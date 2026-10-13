// Spike 1.1 – shared constants for the SharedArrayBuffer triple buffer.
export const LATEST = 0; // index of the newest complete slot, -1 = none
export const READING = 1; // slot the renderer currently reads, -1 = none
export const TICK = 2; // tick of the newest slot (low 32 bits)
export const STOP = 3; // 1 = worker should stop
export const CTRL_INTS = 8;
export const HEADER = 16;
export const RECORD = 16;

export function frameBytes(n) {
  return HEADER + n * RECORD;
}

export function stats(samples) {
  if (samples.length === 0) return { mean: 0, p50: 0, p99: 0, max: 0, n: 0 };
  const s = Float64Array.from(samples).sort();
  const pick = (q) => s[Math.floor((s.length - 1) * q)];
  const mean = s.reduce((a, b) => a + b, 0) / s.length;
  const r = (v) => Math.round(v * 1000) / 1000;
  return {
    mean: r(mean),
    p50: r(pick(0.5)),
    p99: r(pick(0.99)),
    max: r(s[s.length - 1]),
    n: s.length,
  };
}

// Reads every record like a renderer would (positions + state) and returns a checksum.
export function consume(view, offset) {
  const n = view.getUint32(offset + 4, true);
  let acc = 0;
  for (let i = 0; i < n; i++) {
    const o = offset + HEADER + i * RECORD;
    acc +=
      view.getFloat32(o, true) +
      view.getFloat32(o + 4, true) +
      (view.getUint32(o + 12, true) & 0xff);
  }
  return acc;
}
