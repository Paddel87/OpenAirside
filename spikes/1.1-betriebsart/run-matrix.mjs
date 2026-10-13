// Spike 1.1 – runs the measurement matrix and writes results/matrix.jsonl.
// Voraussetzungen: Node 22+, gebautes Spike (siehe README.md), unter Linux ohne Bildschirm xvfb-run.
// Plattformen: Linux (gemessen); macOS/Windows ohne xvfb-run lauffähig, nicht erprobt.
// Usage: node run-matrix.mjs [repetitions]
import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const reps = Number(process.argv[2] ?? "2");
const seconds = 10;
const runs = [];
for (const work of [0, 8, 32]) {
  runs.push({ variant: "napi", threads: 1, work, hz: 0 });
  runs.push({ variant: "napi", threads: 4, work, hz: 0 });
  runs.push({ variant: "wasm1", threads: 1, work, hz: 0 });
  runs.push({ variant: "wasmN", threads: 4, work, hz: 0 });
}
for (const variant of ["napi", "wasm1", "wasmN"]) {
  runs.push({ variant, threads: variant === "wasm1" ? 1 : 4, work: 8, hz: 60 });
}
runs.push({ variant: "napi", threads: 4, work: 8, hz: 0, entities: 50000 });
runs.push({ variant: "wasmN", threads: 4, work: 8, hz: 0, entities: 50000 });

const useXvfb = process.platform === "linux" && !process.env.DISPLAY;
const electron = path.join(here, "app", "node_modules", ".bin", "electron");
const lines = [];

// Native baseline without Electron.
const bench = path.join(
  here,
  "target",
  "release",
  process.platform === "win32" ? "bench.exe" : "bench",
);
for (const work of [0, 8, 32]) {
  const out = execFileSync(
    bench,
    ["10000", "1000", String(work), "1", "2", "4"],
    { encoding: "utf8" },
  );
  for (const l of out.trim().split("\n")) lines.push(l);
}

for (let rep = 1; rep <= reps; rep++) {
  for (const r of runs) {
    const args = [
      ...(process.getuid?.() === 0 ? ["--no-sandbox"] : []),
      path.join(here, "app"),
      `--variant=${r.variant}`,
      `--threads=${r.threads}`,
      `--work=${r.work}`,
      `--hz=${r.hz}`,
      `--seconds=${seconds}`,
      `--entities=${r.entities ?? 10000}`,
    ];
    const cmd = useXvfb ? "xvfb-run" : electron;
    const full = useXvfb
      ? ["-a", "-s", "-screen 0 1280x800x24", electron, ...args]
      : args;
    const out = execFileSync(cmd, full, {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    });
    const line = out
      .trim()
      .split("\n")
      .find((l) => l.startsWith("{"));
    const parsed = JSON.parse(line);
    parsed.rep = rep;
    lines.push(JSON.stringify(parsed));
    process.stderr.write(
      `${r.variant} t=${r.threads} w=${r.work} hz=${r.hz} n=${r.entities ?? 10000} rep=${rep} done\n`,
    );
  }
}
mkdirSync(path.join(here, "results"), { recursive: true });
writeFileSync(
  path.join(here, "results", "matrix.jsonl"),
  `${lines.join("\n")}\n`,
);
