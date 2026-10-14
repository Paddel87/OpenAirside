/**
 * Densest spot among drawn persons, for the camera of the measurement series
 * (step 2.7, as in spike 1.5). Display only (rule 002).
 */

/** Edge of the counting blocks in cells. */
export const HOTSPOT_BLOCK = 16;

/**
 * Centre of the 16 × 16 block with the most `points` on a map of
 * `width` × `height` cells; the centre of the map if there are no points.
 */
export function densestSpot(
  points: readonly { readonly x: number; readonly y: number }[],
  width: number,
  height: number,
): { x: number; y: number } {
  const bw = Math.max(1, Math.ceil(width / HOTSPOT_BLOCK));
  const bh = Math.max(1, Math.ceil(height / HOTSPOT_BLOCK));
  const counts = new Uint32Array(bw * bh);
  let best = -1;
  let bestCount = 0;
  for (const { x, y } of points) {
    const bx = Math.floor(x / HOTSPOT_BLOCK);
    const by = Math.floor(y / HOTSPOT_BLOCK);
    if (bx < 0 || by < 0 || bx >= bw || by >= bh) {
      continue;
    }
    const i = by * bw + bx;
    const count = (counts[i] ?? 0) + 1;
    counts[i] = count;
    if (count > bestCount) {
      bestCount = count;
      best = i;
    }
  }
  if (best < 0) {
    return { x: width / 2, y: height / 2 };
  }
  return {
    x: (best % bw) * HOTSPOT_BLOCK + HOTSPOT_BLOCK / 2,
    y: Math.floor(best / bw) * HOTSPOT_BLOCK + HOTSPOT_BLOCK / 2,
  };
}
