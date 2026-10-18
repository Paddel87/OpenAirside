#!/usr/bin/env node
// Generates the JSON Schema files of the data pack format (S-4) from the Rust types of the core
// (schemars behind the feature `schema`, ADR-025) into content/schema/ and formats them with
// Prettier. With --check it fails if the checked-in files differ from the generated ones,
// including missing or surplus files (CI step, ADR-025).
//
// Voraussetzungen: Node 26.11.1 (project-context.md Abschnitt 3), Rust 1.98.1 mit cargo
//   (rust-toolchain.toml), git, installierte npm-Abhängigkeiten (Prettier 3.9.10)
// Plattformen: Linux, macOS, Windows (reines Node-Skript)
// Exit-Codes: 0 = erzeugt bzw. unverändert, 1 = Abweichung (--check) oder Fehler eines Teilschritts
// Idempotenz: löscht content/schema/ und erzeugt es vollständig neu; wiederholte Aufrufe
//   ergeben dieselben Dateien.
// Reproduzierbarkeit: Ergebnis hängt nur von den Rust-Typen, schemars 1.2.2 und der
//   Prettier-Konfiguration ab, nicht von früheren Läufen.

import { spawnSync } from "node:child_process";
import { rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { changedFiles } from "./gen-types.mjs";

/** Target directory of the generated schemas, relative to the repository root. */
export const OUTPUT_DIR = "content/schema";

/** Arguments of `cargo` that run only the schema export test of the core. */
export function cargoArgs() {
  return [
    "test",
    "-p",
    "openairside-core",
    "--features",
    "schema",
    "--locked",
    "--lib",
    "datapack::schema::tests::export_schemas",
  ];
}

/** Environment for `cargo`: export directory of the test. */
export function cargoEnv(root, base) {
  return { ...base, SCHEMA_EXPORT_DIR: join(root, OUTPUT_DIR) };
}

/**
 * Runs generation (and with `check` the comparison) with injectable side effects.
 * `exec(command, args, options)` returns `{ status, stdout }`.
 */
export function run({ root, check, exec, removeDir, log, env }) {
  removeDir(join(root, OUTPUT_DIR));
  const cargo = exec("cargo", cargoArgs(), { cwd: root, env: cargoEnv(root, env) });
  if (cargo.status !== 0) {
    log("gen-schema: cargo test --features schema failed");
    return 1;
  }
  const prettier = join(root, "node_modules", "prettier", "bin", "prettier.cjs");
  const format = exec(process.execPath, [prettier, "--write", "--log-level", "warn", OUTPUT_DIR], {
    cwd: root,
    env,
  });
  if (format.status !== 0) {
    log("gen-schema: prettier failed");
    return 1;
  }
  if (!check) {
    log(`gen-schema: schemas written to ${OUTPUT_DIR}`);
    return 0;
  }
  const status = exec("git", ["status", "--porcelain", "--untracked-files=all", "--", OUTPUT_DIR], {
    cwd: root,
    env,
  });
  if (status.status !== 0) {
    log("gen-schema: git status failed");
    return 1;
  }
  const changed = changedFiles(status.stdout);
  if (changed.length > 0) {
    log(
      `gen-schema: ${OUTPUT_DIR} differs from the Rust types; run "npm run gen:schema" and commit:`,
    );
    for (const path of changed) {
      log(`  ${path}`);
    }
    return 1;
  }
  log(`gen-schema: ${OUTPUT_DIR} matches the Rust types`);
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
