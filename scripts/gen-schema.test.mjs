import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { OUTPUT_DIR, cargoArgs, cargoEnv, run } from "./gen-schema.mjs";

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
        const key = command === process.execPath ? "prettier" : command;
        return answers[key] ?? { status: 0, stdout: "" };
      },
      removeDir: (dir) => removed.push(dir),
      log: (line) => lines.push(line),
    }),
  };
}

describe("cargoArgs and cargoEnv", () => {
  it("run only the schema export test of the core with the feature schema", () => {
    expect(cargoArgs()).toEqual([
      "test",
      "-p",
      "openairside-core",
      "--features",
      "schema",
      "--locked",
      "--lib",
      "datapack::schema::tests::export_schemas",
    ]);
  });

  it("point the export test at the output directory", () => {
    expect(cargoEnv("/repo", { A: "1" })).toEqual({
      A: "1",
      SCHEMA_EXPORT_DIR: join("/repo", OUTPUT_DIR),
    });
  });
});

describe("run", () => {
  it("regenerates the directory from scratch and formats it", () => {
    const h = harness({});
    expect(run(h.deps(false))).toBe(0);
    expect(h.removed).toEqual([join("/repo", OUTPUT_DIR)]);
    expect(h.calls.map((c) => c.command)).toEqual(["cargo", process.execPath]);
    expect(h.calls[0].options.env.SCHEMA_EXPORT_DIR).toBe(join("/repo", OUTPUT_DIR));
    expect(h.calls[1].args).toContain(OUTPUT_DIR);
    expect(h.lines.at(-1)).toContain("written");
  });

  it("passes the check when git reports no change", () => {
    const h = harness({ git: { status: 0, stdout: "" } });
    expect(run(h.deps(true))).toBe(0);
    expect(h.calls.at(-1).args).toContain("--untracked-files=all");
    expect(h.lines.at(-1)).toContain("matches");
  });

  it("fails the check and lists every differing file", () => {
    const h = harness({ git: { status: 0, stdout: " M content/schema/objects.schema.json\n" } });
    expect(run(h.deps(true))).toBe(1);
    expect(h.lines).toContain("  content/schema/objects.schema.json");
  });

  it.each([
    ["cargo", "cargo test"],
    ["prettier", "prettier"],
    ["git", "git status"],
  ])("fails when %s fails", (step, message) => {
    const h = harness({ [step]: { status: 101, stdout: "" } });
    expect(run(h.deps(true))).toBe(1);
    expect(h.lines.at(-1)).toContain(message);
  });
});
