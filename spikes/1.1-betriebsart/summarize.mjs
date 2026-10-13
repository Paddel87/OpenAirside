// Spike 1.1 – condenses results/matrix.jsonl into a Markdown table (stdout).
// Voraussetzungen: Node 22+. Plattformen: alle.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const rows = readFileSync(path.join(here, "results", "matrix.jsonl"), "utf8")
  .trim()
  .split("\n")
  .map((l) => JSON.parse(l));

const f = (v, d = 0) =>
  v === undefined || v === null ? "–" : Number(v).toFixed(d);

console.log("# Messergebnisse Spike 1.1\n");
console.log("## Native Grundlinie (ohne Electron, 1000 Ticks)\n");
console.log("| Arbeit | Threads | Schritt Mittel µs | p99 µs | Prüfsumme |");
console.log("|---|---|---|---|---|");
for (const r of rows.filter((r) => r.variant === "native")) {
  console.log(
    `| ${r.work} | ${r.threads} | ${f(r.step_us_mean)} | ${f(r.step_us_p99)} | \`${r.checksum}\` |`,
  );
}

console.log("\n## Electron (10 s je Lauf, Mittel über Wiederholungen)\n");
console.log(
  "| Variante | Threads | Arbeit | Hz | Personen | Ticks/s | Schritt Mittel µs | p99 µs | Rückstand | FPS | Bild p99 ms | Lesen Renderer µs | Übergabe RTT ms (p99) | Prüfsumme |",
);
console.log("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
const groups = new Map();
for (const r of rows.filter((r) => r.variant !== "native")) {
  const key = [r.variant, r.threads, r.work, r.hz, r.entities].join("|");
  if (!groups.has(key)) groups.set(key, []);
  groups.get(key).push(r);
}
const avg = (xs) => xs.reduce((a, b) => a + b, 0) / xs.length;
for (const [, rs] of groups) {
  const r0 = rs[0];
  const sim = (r, k, alt) => r.sim?.[k] ?? r.sim?.[alt];
  const ticksPerS = avg(rs.map((r) => sim(r, "ticks", "ticks") / r.seconds));
  const mean = avg(rs.map((r) => sim(r, "step_us_mean", "stepUsMean")));
  const p99 = avg(rs.map((r) => sim(r, "step_us_p99", "stepUsP99")));
  const behind = rs.map((r) => sim(r, "behind_ticks", "behindTicks")).join("/");
  const fps = avg(rs.map((r) => r.fps));
  const frameP99 = avg(rs.map((r) => r.frame_interval_ms.p99));
  const read = avg(rs.map((r) => r.renderer_read_ms.mean * 1000));
  const rtt = r0.rtt_ms
    ? `${f(avg(rs.map((r) => r.rtt_ms.mean)), 2)} (${f(avg(rs.map((r) => r.rtt_ms.p99)), 2)})`
    : "0 (gemeinsamer Speicher)";
  const checksum = r0.determinism_checksum_1000_ticks
    ? `\`${r0.determinism_checksum_1000_ticks}\``
    : "= nativ";
  console.log(
    `| ${r0.variant} | ${r0.threads} | ${r0.work} | ${r0.hz || "max"} | ${r0.entities} | ${f(ticksPerS)} | ${f(mean)} | ${f(p99)} | ${behind} | ${f(fps, 1)} | ${f(frameP99, 2)} | ${f(read)} | ${rtt} | ${checksum} |`,
  );
}
