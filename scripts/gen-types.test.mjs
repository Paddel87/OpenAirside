import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { OUTPUT_DIR, cargoArgs, cargoEnv, changedFiles, run } from "./gen-types.mjs";

/** Fake side effects: answers per command, records calls and log lines. */
function harness(answers) {
  const calls = [];
  const lines = [];
  const removed = [];
  return {
    calls,
    lines,
    removed,
    deps: (check) => ({
      root: "/repo",
      check,
      env: { PATH: "/bin" },
      exec: (command, args, options) => {
        calls.push({ command, args, options });
        const tool = args[0].includes("eslint") ? "eslint" : "prettier";
        const key = command === process.execPath ? tool : command;
        return answers[key] ?? { status: 0, stdout: "" };
      },
      removeDir: (dir) => removed.push(dir),
      log: (line) => lines.push(line),
    }),
  };
}

describe("cargoArgs and cargoEnv", () => {
  it("run only the ts-rs export tests of the core with the feature ts", () => {
    expect(cargoArgs()).toEqual([
      "test",
      "-p",
      "openairside-core",
      "--features",
      "ts",
      "--locked",
      "--lib",
      "export_bindings",
    ]);
  });

  it("point ts-rs at the output directory and use .js imports", () => {
    expect(cargoEnv("/repo", { A: "1" })).toEqual({
      A: "1",
      TS_RS_EXPORT_DIR: join("/repo", OUTPUT_DIR),
      TS_RS_IMPORT_EXTENSION: "js",
    });
  });
});

describe("changedFiles", () => {
  it("extracts modified, deleted and untracked paths", () => {
    const porcelain =
      " M app/src/generated/Event.ts\r\n D app/src/generated/Old.ts\n?? app/src/generated/New.ts\n";
    expect(changedFiles(porcelain)).toEqual([
      "app/src/generated/Event.ts",
      "app/src/generated/Old.ts",
      "app/src/generated/New.ts",
    ]);
    expect(changedFiles("")).toEqual([]);
  });
});

describe("run", () => {
  it("regenerates the directory from scratch and formats it", () => {
    const h = harness({});
    expect(run(h.deps(false))).toBe(0);
    expect(h.removed).toEqual([join("/repo", OUTPUT_DIR)]);
    expect(h.calls.map((c) => c.command)).toEqual(["cargo", process.execPath, process.execPath]);
    expect(h.calls[0].options.env.TS_RS_EXPORT_DIR).toBe(join("/repo", OUTPUT_DIR));
    expect(h.calls[1].args).toContain("--fix");
    expect(h.calls[2].args).toContain(OUTPUT_DIR);
    expect(h.lines.at(-1)).toContain("written");
  });

  it("passes the check when git reports no change", () => {
    const h = harness({ git: { status: 0, stdout: "" } });
    expect(run(h.deps(true))).toBe(0);
    expect(h.calls.at(-1).args).toContain("--untracked-files=all");
    expect(h.lines.at(-1)).toContain("matches");
  });

  it("fails the check and lists every differing file", () => {
    const h = harness({ git: { status: 0, stdout: "?? app/src/generated/New.ts\n" } });
    expect(run(h.deps(true))).toBe(1);
    expect(h.lines).toContain("  app/src/generated/New.ts");
  });

  it.each([
    ["cargo", "cargo test"],
    ["eslint", "eslint --fix"],
    ["prettier", "prettier"],
    ["git", "git status"],
  ])("fails when %s fails", (step, message) => {
    const h = harness({ [step]: { status: 101, stdout: "" } });
    expect(run(h.deps(true))).toBe(1);
    expect(h.lines.at(-1)).toContain(message);
  });
});
