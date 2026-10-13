#!/usr/bin/env node
// Writes the result of the core benchmark (core/benches/tick.rs) as a Markdown table for
// the job summary of the CI, compared with the last result on main if one is given
// (trend display only, ADR-022). The limits are enforced by the benchmark itself.
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

/** Rows of the table: key in the JSON, label, whether a limit exists. */
const ROWS = [
  ["meanMs", "Mittelwert je Schritt (ms)", "meanMs"],
  ["p50Ms", "Median (ms)", null],
  ["p99Ms", "p99 (ms)", "p99Ms"],
  ["maxMs", "Höchstwert (ms)", null],
];

function number(value) {
  return typeof value === "number" && Number.isFinite(value) ? value : null;
}

function format(value) {
  return value === null ? "–" : value.toFixed(3);
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
  const lines = [
    `### Kern-Benchmark: ${current.passed === true ? "bestanden" : "Grenze überschritten"}`,
    "",
    `${String(current.persons)} Personen, ${String(current.threads)} Threads ` +
      `(${String(current.cpus)} CPUs, ${String(current.os)}), ${String(current.ticks)} Schritte`,
    "",
    "| Kennzahl | aktuell | Grenze | letzter Lauf auf main | Änderung |",
    "|---|---|---|---|---|",
  ];
  for (const [key, label, limitKey] of ROWS) {
    const now = number(current[key]);
    const before = previous === null ? null : number(previous[key]);
    const limit = limitKey === null ? null : number(current.limits?.[limitKey]);
    lines.push(
      `| ${label} | ${format(now)} | ${limit === null ? "–" : String(limit)} | ` +
        `${format(before)} | ${change(now, before)} |`,
    );
  }
  if (previous === null) {
    lines.push("", "Kein Vergleichswert von main vorhanden.");
  }
  lines.push(
    "",
    "Die Änderung ist nur eine Anzeige; rot wird der Lauf allein über die Grenzen (ADR-022).",
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
