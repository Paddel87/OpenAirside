#!/usr/bin/env node
// Decides whether a CI run needs the code jobs or only the documentation job (ADR-043).
// Documentation means files under docs/ and Markdown files anywhere. The code jobs are
// skipped only if every changed file is documentation and, for a push onto an earlier
// commit, every check of that earlier commit passed: a documentation commit on top of a
// red commit never turns the checks green. In every doubtful case the code jobs run.
//
// Aufruf: node scripts/ci-changes.mjs (in GitHub Actions; schreibt `code=true|false` nach $GITHUB_OUTPUT)
// Voraussetzungen: Node 26.11.1 (project-context.md Abschnitt 3), git mit voller Historie
//   (actions/checkout mit fetch-depth 0); ENV: GITHUB_EVENT_NAME, GITHUB_EVENT_PATH,
//   GITHUB_REPOSITORY, GITHUB_OUTPUT, GITHUB_API_URL (Voreinstellung https://api.github.com),
//   GH_TOKEN (Recht checks: read)
// Plattformen: Linux, macOS, Windows (reines Node-Skript; in der CI auf Linux)
// Exit-Codes: 0 = Entscheidung geschrieben (auch "code" bei jedem Zweifel), 1 = Umgebung unvollständig
// Idempotenz: liest nur (git, GitHub-API) und hängt eine Zeile an $GITHUB_OUTPUT; beliebig wiederholbar.
// Reproduzierbarkeit: hängt von der Git-Historie und dem Stand der Checks des vorigen Commits ab;
//   läuft dort noch ein Check, entscheidet das Skript "code".

import { execFileSync } from "node:child_process";
import { appendFileSync, readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const ZERO_SHA = /^0+$/;
/** Conclusions of a check that count as passed. */
const PASSED = new Set(["success", "skipped", "neutral"]);

/** `true` for a documentation file: under docs/ or a Markdown file. */
export function isDocsPath(path) {
  return path.startsWith("docs/") || path.toLowerCase().endsWith(".md");
}

/** `true` if there are changed files and all of them are documentation. */
export function docsOnly(files) {
  return files.length > 0 && files.every(isDocsPath);
}

/**
 * `true` if the check runs of a commit (GitHub API, `check_runs`) exist, are all completed
 * and all passed. Only runs of GitHub Actions count.
 */
export function allPassed(checkRuns) {
  const runs = checkRuns.filter((run) => run.app?.slug === "github-actions");
  return (
    runs.length > 0 &&
    runs.every((run) => run.status === "completed" && PASSED.has(run.conclusion ?? ""))
  );
}

/**
 * Which change to look at for an event: `null` means "always run the code jobs".
 * `from`/`to` are commits; `threeDot` compares with the merge base; `previous` is the
 * commit whose checks must have passed (or `null` if none is needed).
 */
export function plan(event, payload) {
  if (event === "push") {
    const { before, after } = payload;
    if (typeof before !== "string" || typeof after !== "string" || ZERO_SHA.test(before)) {
      return null;
    }
    return { from: before, to: after, threeDot: false, previous: before };
  }
  if (event === "pull_request") {
    const { action, before, after, pull_request: pr } = payload;
    if (action === "synchronize" && typeof before === "string" && typeof after === "string") {
      return { from: before, to: after, threeDot: false, previous: before };
    }
    const base = pr?.base?.sha;
    const head = pr?.head?.sha;
    if (typeof base !== "string" || typeof head !== "string") return null;
    // The whole pull request against its base: unchanged code was checked on the base branch.
    return { from: base, to: head, threeDot: true, previous: null };
  }
  return null;
}

/**
 * The decision: `{ code, reason }`. `changedFiles(from, to, threeDot)` and
 * `checkRuns(sha)` may throw; then the code jobs run.
 */
export async function decide(event, payload, { changedFiles, checkRuns }) {
  const p = plan(event, payload);
  if (p === null) return { code: true, reason: `event ${event}: full run` };
  let files;
  try {
    files = changedFiles(p.from, p.to, p.threeDot);
  } catch (error) {
    return { code: true, reason: `diff failed (${String(error)}): full run` };
  }
  if (!docsOnly(files)) {
    return { code: true, reason: `${String(files.length)} changed files, not documentation only` };
  }
  if (p.previous !== null) {
    let runs;
    try {
      runs = await checkRuns(p.previous);
    } catch (error) {
      return { code: true, reason: `checks of ${p.previous} not readable (${String(error)})` };
    }
    if (!allPassed(runs)) {
      return { code: true, reason: `checks of ${p.previous} not all passed: full run` };
    }
  }
  return { code: false, reason: `${String(files.length)} documentation files only` };
}

function gitChangedFiles(from, to, threeDot) {
  const range = threeDot ? `${from}...${to}` : `${from}..${to}`;
  const out = execFileSync("git", ["diff", "--name-only", range], { encoding: "utf8" });
  return out.split("\n").filter((line) => line.length > 0);
}

async function apiCheckRuns(sha) {
  const api = process.env.GITHUB_API_URL ?? "https://api.github.com";
  const url = `${api}/repos/${process.env.GITHUB_REPOSITORY}/commits/${sha}/check-runs?per_page=100&filter=latest`;
  const response = await globalThis.fetch(url, {
    headers: {
      authorization: `Bearer ${process.env.GH_TOKEN ?? ""}`,
      accept: "application/vnd.github+json",
    },
  });
  if (!response.ok) throw new Error(`HTTP ${String(response.status)}`);
  const body = await response.json();
  return body.check_runs ?? [];
}

async function main() {
  const { GITHUB_EVENT_NAME: event, GITHUB_EVENT_PATH: path, GITHUB_OUTPUT: output } = process.env;
  if (event === undefined || path === undefined || output === undefined) {
    console.error("GITHUB_EVENT_NAME, GITHUB_EVENT_PATH und GITHUB_OUTPUT müssen gesetzt sein");
    process.exit(1);
  }
  const payload = JSON.parse(readFileSync(path, "utf8"));
  const { code, reason } = await decide(event, payload, {
    changedFiles: gitChangedFiles,
    checkRuns: apiCheckRuns,
  });
  console.log(`code=${String(code)}: ${reason}`);
  appendFileSync(output, `code=${String(code)}\n`);
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? "").href) {
  await main();
}
