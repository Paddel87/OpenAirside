#!/usr/bin/env node
// Writes the result of a core benchmark (core/benches/tick.rs, ADR-022, or
// core/benches/flowfield.rs, ADR-023, or core/benches/continuous_build.rs, ADR-041) as a Markdown table for the job summary of the CI,
// compared with the last result on main if one is given (trend display only). The limits
// are enforced by the benchmark programs themselves.
//
// Aufruf: node scripts/bench-report.mjs <aktuell.json> [<main.json>]
// Voraussetzungen: Node 26.11.1 (project-context.md Abschnitt 3)
// Plattformen: Linux, macOS, Windows (reines Node-Skript)
// Exit-Codes: 0 = Tabelle geschrieben (auch ohne oder mit unlesbarem Vergleichswert),
//   1 = aktuelles Ergebnis fehlt oder ist unlesbar
// Idempotenz: liest nur, schreibt auf die Standardausgabe; beliebig wiederholbar.
// Reproduzierbarkeit: Ausgabe hängt allein von den beiden Eingabedateien ab.

import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

/**
 * Table layout per benchmark: title and rows of key in the JSON, label, key of the limit
 * (or null), decimals and scale factor of the displayed value.
 */
const LAYOUTS = {
  tick: {
    title: "Kern-Benchmark",
    rows: [
      ["meanMs", "Mittelwert je Schritt (ms)", "meanMs", 3, 1],
      ["p50Ms", "Median (ms)", null, 3, 1],
      ["p99Ms", "p99 (ms)", "p99Ms", 3, 1],
      ["maxMs", "Höchstwert (ms)", null, 3, 1],
    ],
  },
  flowfield: {
    title: "Wegweiser-Benchmark",
    rows: [
      ["recomputeMs", "Neuberechnung aller Wegweiser (ms)", "recomputeMs", 0, 1],
      ["fieldBytes", "Speicher der Wegweiser (MB)", null, 1, 1e-6],
      ["meanMs", "Schritt während Neuberechnung, Mittel (ms)", null, 3, 1],
      ["p99Ms", "Schritt während Neuberechnung, p99 (ms)", "p99Ms", 3, 1],
      ["maxMs", "Schritt, Höchstwert (ms)", null, 3, 1],
      ["adoptionMaxMs", "Schritt mit Übernahme (ms)", null, 3, 1],
    ],
  },
  continuousBuild: {
    title: "Dauerbau-Benchmark",
    rows: [
      ["maxGapMs", "Längster Abstand zwischen Übernahmen (ms)", "gapMs", 0, 1],
      ["settleMs", "Letzte Bauaktion bis Übernahme ihres Stands (ms)", "settleMs", 0, 1],
      ["adoptionsWhileBuilding", "Übernahmen während des Bauens", null, 0, 1],
      ["jobsPeak", "Neuberechnungen gleichzeitig (Spitze)", "jobs", 0, 1],
      ["fieldSetsPeak", "Wegweiser-Sätze gleichzeitig (Spitze)", "fieldSets", 0, 1],
      ["snapshotsPeak", "Schnappschuss-Kopien gleichzeitig (Spitze)", "snapshots", 0, 1],
      ["peakMemoryMb", "Prozessspeicher, Spitze (MB, nicht bewertet)", null, 0, 1],
      ["stepP99Ms", "Schritt während des Bauens, p99 (ms, nicht bewertet)", null, 3, 1],
    ],
  },
};

function number(value) {
  return typeof value === "number" && Number.isFinite(value) ? value : null;
}

function format(value, digits, scale) {
  return value === null ? "–" : (value * scale).toFixed(digits);
}

function change(current, previous) {
  if (current === null || previous === null || previous === 0) {
    return "–";
  }
  const percent = ((current - previous) / previous) * 100;
  return `${percent >= 0 ? "+" : ""}${percent.toFixed(1)} %`;
}

/** Builds the Markdown summary from the parsed results; `previous` may be null. */
export function renderReport(current, previous) {
  const layout = LAYOUTS[current.benchmark] ?? LAYOUTS.tick;
  const threads =
    current.backgroundThreads === undefined
      ? `${String(current.threads)} Threads`
      : `${String(current.threads)} + ${String(current.backgroundThreads)} Threads`;
  const targets = current.targets === undefined ? "" : `${String(current.targets)} Ziele, `;
  const lines = [
    `### ${layout.title}: ${current.passed === true ? "bestanden" : "Grenze überschritten"}`,
    "",
    `${targets}${String(current.persons)} Personen, ${threads} ` +
      `(${String(current.cpus)} CPUs, ${String(current.os)}), ${String(current.ticks)} Schritte`,
    "",
    "| Kennzahl | aktuell | Grenze | letzter Lauf auf main | Änderung |",
    "|---|---|---|---|---|",
  ];
  for (const [key, label, limitKey, digits, scale] of layout.rows) {
    const now = number(current[key]);
    const before = previous === null ? null : number(previous[key]);
    const limit = limitKey === null ? null : number(current.limits?.[limitKey]);
    lines.push(
      `| ${label} | ${format(now, digits, scale)} | ${limit === null ? "–" : String(limit)} | ` +
        `${format(before, digits, scale)} | ${change(now, before)} |`,
    );
  }
  if (previous === null) {
    lines.push("", "Kein Vergleichswert von main vorhanden.");
  }
  lines.push(
    "",
    "Die Änderung ist nur eine Anzeige; rot wird der Lauf allein über die Grenzen (ADR-022, ADR-023, ADR-041).",
  );
  return `${lines.join("\n")}\n`;
}

/** Reads a JSON file; returns null if it is missing or malformed. */
export function readResult(path, read = readFileSync) {
  try {
    return JSON.parse(read(path, "utf8"));
  } catch {
    return null;
  }
}

/** Entry point with injectable reader and writer; returns the exit code. */
export function main(args, read = readFileSync, write = (text) => process.stdout.write(text)) {
  const [currentPath, previousPath] = args;
  const current = currentPath === undefined ? null : readResult(currentPath, read);
  if (current === null) {
    write("Kern-Benchmark: kein lesbares Ergebnis.\n");
    return 1;
  }
  const previous = previousPath === undefined ? null : readResult(previousPath, read);
  write(renderReport(current, previous));
  return 0;
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exitCode = main(process.argv.slice(2));
}
