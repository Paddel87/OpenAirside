#!/usr/bin/env node
// Checks the licences of all npm dependencies against the allowed list of
// docs/project-context.md section 6 (ADR-018, ADR-019). Fails with exit code 1
// if a package has no licence or a licence outside the list.
//
// Voraussetzungen: Node 26.11.1 (project-context.md Abschnitt 3), package-lock.json (lockfileVersion 3)
// Plattformen: Linux, macOS, Windows (reines Node-Skript)
// Exit-Codes: 0 = alle Lizenzen erlaubt, 1 = Verstoß oder unlesbares Lockfile
// Idempotenz: liest nur das Lockfile, schreibt nichts; beliebig wiederholbar mit gleichem Ergebnis.
// Reproduzierbarkeit: Ergebnis hängt allein von package-lock.json ab, nicht von node_modules oder Netz.

import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

/** Allowed SPDX identifiers (project-context.md section 6, ADR-019). */
export const ALLOWED_LICENSES = new Set([
  "MIT",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "Apache-2.0",
  "ISC",
  "Zlib",
  "MPL-2.0",
  "Unicode-3.0",
  "BlueOak-1.0.0",
  "0BSD",
  "Python-2.0",
  "WTFPL",
]);

/**
 * Splits an SPDX expression into tokens: parentheses, AND, OR, WITH, identifiers.
 * @param {string} expression
 * @returns {string[]}
 */
function tokenize(expression) {
  return expression
    .replace(/[()]/g, (paren) => ` ${paren} `)
    .trim()
    .split(/\s+/)
    .filter(Boolean);
}

/**
 * Evaluates an SPDX licence expression. OR needs one allowed side, AND needs
 * both; "X WITH Y" is allowed only if the full term is in the allowed set.
 * @param {string} expression
 * @param {Set<string>} allowed
 * @returns {boolean}
 */
export function isAllowedExpression(expression, allowed = ALLOWED_LICENSES) {
  const tokens = tokenize(expression);
  let pos = 0;
  /** @returns {boolean} */
  const parseOr = () => {
    let result = parseAnd();
    while (tokens[pos] === "OR") {
      pos += 1;
      const right = parseAnd();
      result = result || right;
    }
    return result;
  };
  /** @returns {boolean} */
  const parseAnd = () => {
    let result = parseTerm();
    while (tokens[pos] === "AND") {
      pos += 1;
      const right = parseTerm();
      result = result && right;
    }
    return result;
  };
  /** @returns {boolean} */
  const parseTerm = () => {
    const token = tokens[pos];
    pos += 1;
    if (token === "(") {
      const inner = parseOr();
      if (tokens[pos] !== ")") throw new Error(`unbalanced parentheses in "${expression}"`);
      pos += 1;
      return inner;
    }
    if (token === undefined || ["AND", "OR", "WITH", ")"].includes(token)) {
      throw new Error(`invalid licence expression "${expression}"`);
    }
    if (tokens[pos] === "WITH") {
      const exception = tokens[pos + 1];
      pos += 2;
      return allowed.has(`${token} WITH ${exception}`);
    }
    return allowed.has(token);
  };
  const result = parseOr();
  if (pos !== tokens.length) throw new Error(`invalid licence expression "${expression}"`);
  return result;
}

/**
 * @param {string} license
 * @param {Set<string>} allowed
 * @returns {boolean} false for malformed expressions instead of throwing
 */
function isValidAndAllowed(license, allowed) {
  try {
    return isAllowedExpression(license, allowed);
  } catch {
    return false;
  }
}

/**
 * Returns all lockfile packages whose licence is missing, invalid or not allowed.
 * @param {{ packages?: Record<string, { license?: unknown }> }} lockfile
 * @param {Set<string>} allowed
 * @returns {{ name: string; license: string }[]}
 */
export function findViolations(lockfile, allowed = ALLOWED_LICENSES) {
  const violations = [];
  for (const [path, entry] of Object.entries(lockfile.packages ?? {})) {
    if (!path.startsWith("node_modules/")) continue;
    const name = path.slice(path.lastIndexOf("node_modules/") + "node_modules/".length);
    const license = typeof entry.license === "string" ? entry.license : "";
    const ok = license !== "" && isValidAndAllowed(license, allowed);
    if (!ok) violations.push({ name, license: license || "(keine Angabe)" });
  }
  return violations;
}

function main() {
  const lockfile = JSON.parse(readFileSync("package-lock.json", "utf8"));
  const violations = findViolations(lockfile);
  if (violations.length > 0) {
    console.error("Nicht erlaubte Lizenzen (project-context.md Abschnitt 6):");
    for (const v of violations) console.error(`  ${v.name}: ${v.license}`);
    process.exitCode = 1;
    return;
  }
  const count = Object.keys(lockfile.packages ?? {}).filter((p) =>
    p.startsWith("node_modules/"),
  ).length;
  console.log(`Lizenzprüfung ok: ${String(count)} Pakete.`);
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
