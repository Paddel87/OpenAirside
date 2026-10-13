import { describe, expect, it } from "vitest";
import { findViolations, isAllowedExpression } from "./check-licenses.mjs";

describe("isAllowedExpression", () => {
  it.each([
    "MIT",
    "Apache-2.0",
    "BlueOak-1.0.0",
    "(MIT OR CC0-1.0)",
    "WTFPL OR ISC",
    "MIT AND ISC",
    "(MIT AND Zlib) OR GPL-3.0",
  ])("allows %s", (expr) => {
    expect(isAllowedExpression(expr)).toBe(true);
  });

  it.each([
    "GPL-3.0",
    "AGPL-3.0-only",
    "MIT AND GPL-2.0",
    "CC-BY-4.0",
    "Apache-2.0 WITH LLVM-exception",
    "UNLICENSED",
  ])("rejects %s", (expr) => {
    expect(isAllowedExpression(expr)).toBe(false);
  });

  it("accepts an exception term that is listed explicitly", () => {
    expect(
      isAllowedExpression(
        "Apache-2.0 WITH LLVM-exception",
        new Set(["Apache-2.0 WITH LLVM-exception"]),
      ),
    ).toBe(true);
  });

  it.each(["(MIT", "MIT OR", "AND MIT", "MIT ISC", ""])("throws on malformed %j", (expr) => {
    expect(() => isAllowedExpression(expr)).toThrow();
  });
});

describe("findViolations", () => {
  it("reports missing, malformed and forbidden licences, ignoring the root package", () => {
    const lockfile = {
      packages: {
        "": { license: "GPL-3.0" },
        "node_modules/a": { license: "MIT" },
        "node_modules/b": { license: "GPL-3.0" },
        "node_modules/c": {},
        "node_modules/a/node_modules/d": { license: "(MIT" },
      },
    };
    expect(findViolations(lockfile)).toEqual([
      { name: "b", license: "GPL-3.0" },
      { name: "c", license: "(keine Angabe)" },
      { name: "d", license: "(MIT" },
    ]);
  });

  it("accepts an empty lockfile", () => {
    expect(findViolations({})).toEqual([]);
  });
});
