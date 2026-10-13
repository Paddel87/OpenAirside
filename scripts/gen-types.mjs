#!/usr/bin/env node
// Generates the TypeScript types of the interface contracts from the Rust types of the
// core (ts-rs behind the feature `ts`, ADR-021) into app/src/generated/, applies the
// auto-fixable ESLint style rules (type → interface, index signature → Record) and formats
// them with Prettier. With --check it fails if the checked-in files differ from the generated
// ones, including missing or surplus files (CI step, ADR-022).
//
// Voraussetzungen: Node 26.11.1 (project-context.md Abschnitt 3), Rust 1.98.1 mit cargo
//   (rust-toolchain.toml), git, installierte npm-Abhängigkeiten (ESLint 10.9.1, Prettier 3.9.10)
// Plattformen: Linux, macOS, Windows (reines Node-Skript)
// Exit-Codes: 0 = erzeugt bzw. unverändert, 1 = Abweichung (--check) oder Fehler eines Teilschritts
// Idempotenz: löscht app/src/generated/ und erzeugt es vollständig neu; wiederholte Aufrufe
//   ergeben dieselben Dateien.
// Reproduzierbarkeit: Ergebnis hängt nur von den Rust-Typen, ts-rs 12.0.1 und der
//   ESLint- und Prettier-Konfiguration ab, nicht von früheren Läufen.

import { spawnSync } from "node:child_process";
import { rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

/** Target directory of the generated types, relative to the repository root. */
export const OUTPUT_DIR = "app/src/generated";

/** Arguments of `cargo` that run only the export tests of ts-rs. */
export function cargoArgs() {
  return [
    "test",
    "-p",
    "openairside-core",
    "--features",
    "ts",
    "--locked",
    "--lib",
    "export_bindings",
  ];
}

/** Environment for `cargo`: export directory and `.js` import extension (NodeNext). */
export function cargoEnv(root, base) {
  return { ...base, TS_RS_EXPORT_DIR: join(root, OUTPUT_DIR), TS_RS_IMPORT_EXTENSION: "js" };
}

/** Paths reported by `git status --porcelain`, one per non-empty line. */
export function changedFiles(porcelain) {
  return porcelain
    .split(/\r?\n/)
    .map((line) => line.slice(3).trim())
    .filter((path) => path.length > 0);
}

/**
 * Runs generation (and with `check` the comparison) with injectable side effects.
 * `exec(command, args, options)` returns `{ status, stdout }`.
 */
export function run({ root, check, exec, removeDir, log, env }) {
  removeDir(join(root, OUTPUT_DIR));
  const cargo = exec("cargo", cargoArgs(), { cwd: root, env: cargoEnv(root, env) });
  if (cargo.status !== 0) {
    log("gen-types: cargo test --features ts failed");
    return 1;
  }
  const eslint = join(root, "node_modules", "eslint", "bin", "eslint.js");
  const fix = exec(process.execPath, [eslint, "--fix", "--max-warnings", "0", OUTPUT_DIR], {
    cwd: root,
    env,
  });
  if (fix.status !== 0) {
    log("gen-types: eslint --fix left problems in the generated types");
    return 1;
  }
  const prettier = join(root, "node_modules", "prettier", "bin", "prettier.cjs");
  const format = exec(process.execPath, [prettier, "--write", "--log-level", "warn", OUTPUT_DIR], {
    cwd: root,
    env,
  });
  if (format.status !== 0) {
    log("gen-types: prettier failed");
    return 1;
  }
  if (!check) {
    log(`gen-types: types written to ${OUTPUT_DIR}`);
    return 0;
  }
  const status = exec("git", ["status", "--porcelain", "--untracked-files=all", "--", OUTPUT_DIR], {
    cwd: root,
    env,
  });
  if (status.status !== 0) {
    log("gen-types: git status failed");
    return 1;
  }
  const changed = changedFiles(status.stdout);
  if (changed.length > 0) {
    log(
      `gen-types: ${OUTPUT_DIR} differs from the Rust types; run "npm run gen:types" and commit:`,
    );
    for (const path of changed) {
      log(`  ${path}`);
    }
    return 1;
  }
  log(`gen-types: ${OUTPUT_DIR} matches the Rust types`);
  return 0;
}

/** Runs a command; stdout is captured (and echoed except for git). */
function execSync(command, args, options) {
  const result = spawnSync(command, args, {
    ...options,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "inherit"],
  });
  if (result.stdout && command !== "git") {
    process.stdout.write(result.stdout);
  }
  return { status: result.status ?? 1, stdout: result.stdout ?? "" };
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const root = join(dirname(fileURLToPath(import.meta.url)), "..");
  process.exitCode = run({
    root,
    check: process.argv.includes("--check"),
    exec: execSync,
    removeDir: (dir) => rmSync(dir, { recursive: true, force: true }),
    log: (line) => console.log(line),
    env: process.env,
  });
}
