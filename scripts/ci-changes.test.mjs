import { describe, expect, it } from "vitest";
import { allPassed, decide, docsOnly, isDocsPath, plan } from "./ci-changes.mjs";

const actions = { slug: "github-actions" };
const run = (conclusion, status = "completed", app = actions) => ({ status, conclusion, app });
const green = [run("success"), run("skipped"), run("success")];

const push = { before: "a".repeat(40), after: "b".repeat(40) };
const sync = { action: "synchronize", before: "c".repeat(40), after: "d".repeat(40) };
const opened = {
  action: "opened",
  pull_request: { base: { sha: "e".repeat(40) }, head: { sha: "f".repeat(40) } },
};

/** Decides with fixed changed files and check runs. */
function decideWith(event, payload, files, runs) {
  return decide(event, payload, {
    changedFiles: () => {
      if (files instanceof Error) throw files;
      return files;
    },
    checkRuns: () => (runs instanceof Error ? Promise.reject(runs) : Promise.resolve(runs)),
  });
}

describe("isDocsPath and docsOnly", () => {
  it("counts docs/ and Markdown files as documentation, nothing else", () => {
    expect(isDocsPath("docs/fahrplan.md")).toBe(true);
    expect(isDocsPath("docs/research/3.1b-ergebnisse/mac-measure.json")).toBe(true);
    expect(isDocsPath("README.md")).toBe(true);
    expect(isDocsPath("templates/docs/README.MD")).toBe(true);
    expect(isDocsPath("app/renderer/measure.ts")).toBe(false);
    expect(isDocsPath("content/base/lang/de.json")).toBe(false);
    expect(isDocsPath(".github/workflows/ci.yml")).toBe(false);
    expect(isDocsPath("mydocs/x.txt")).toBe(false);
  });

  it("needs at least one file, all of them documentation", () => {
    expect(docsOnly(["docs/a.md", "README.md"])).toBe(true);
    expect(docsOnly(["docs/a.md", "core/src/lib.rs"])).toBe(false);
    expect(docsOnly([])).toBe(false);
  });
});

describe("allPassed", () => {
  it("accepts completed runs of GitHub Actions that passed or were skipped", () => {
    expect(allPassed(green)).toBe(true);
    expect(allPassed([...green, run("failure", "completed", { slug: "other-app" })])).toBe(true);
  });

  it("refuses failed, cancelled, running or missing runs", () => {
    expect(allPassed([...green, run("failure")])).toBe(false);
    expect(allPassed([...green, run("cancelled")])).toBe(false);
    expect(allPassed([...green, run(null, "in_progress")])).toBe(false);
    expect(allPassed([])).toBe(false);
    expect(allPassed([run("success", "completed", { slug: "other-app" })])).toBe(false);
  });
});

describe("plan", () => {
  it("compares a push with the commit before, a new pull request with its base", () => {
    expect(plan("push", push)).toEqual({
      from: push.before,
      to: push.after,
      threeDot: false,
      previous: push.before,
    });
    expect(plan("pull_request", sync)).toEqual({
      from: sync.before,
      to: sync.after,
      threeDot: false,
      previous: sync.before,
    });
    expect(plan("pull_request", opened)).toEqual({
      from: "e".repeat(40),
      to: "f".repeat(40),
      threeDot: true,
      previous: null,
    });
  });

  it("runs everything for a new branch, a manual run or an unknown payload", () => {
    expect(plan("push", { before: "0".repeat(40), after: push.after })).toBeNull();
    expect(plan("workflow_dispatch", {})).toBeNull();
    expect(plan("pull_request", { action: "opened" })).toBeNull();
    expect(plan("push", {})).toBeNull();
  });
});

describe("decide", () => {
  it("skips the code jobs for documentation on top of passed checks", async () => {
    const result = await decideWith("pull_request", sync, ["docs/logbuch.md"], green);
    expect(result.code).toBe(false);
  });

  it("skips them for a new pull request with documentation only, without earlier checks", async () => {
    const result = await decideWith("pull_request", opened, ["README.md"], new Error("unused"));
    expect(result.code).toBe(false);
  });

  it("runs them for documentation on top of a red, running or unreadable commit", async () => {
    // The forced failure: a documentation commit must not turn a red commit green.
    const red = await decideWith("pull_request", sync, ["docs/a.md"], [...green, run("failure")]);
    expect(red.code).toBe(true);
    const running = await decideWith("push", push, ["docs/a.md"], [run(null, "queued")]);
    expect(running.code).toBe(true);
    const unreadable = await decideWith("push", push, ["docs/a.md"], new Error("HTTP 403"));
    expect(unreadable).toMatchObject({ code: true });
    expect(unreadable.reason).toContain("HTTP 403");
  });

  it("runs them for code, for an empty or failed diff and for manual runs", async () => {
    expect((await decideWith("push", push, ["docs/a.md", "app/x.ts"], green)).code).toBe(true);
    expect((await decideWith("push", push, [], green)).code).toBe(true);
    expect((await decideWith("push", push, new Error("bad object"), green)).code).toBe(true);
    expect((await decideWith("workflow_dispatch", {}, ["docs/a.md"], green)).code).toBe(true);
  });
});
