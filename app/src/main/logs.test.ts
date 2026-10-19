import { describe, expect, it } from "vitest";
import {
  isLogLevel,
  LOG_FILE_BYTES,
  LogFile,
  type LogFs,
  maskMessage,
  MAX_ENTRIES_PER_SECOND,
  parseLogArgs,
  RateGate,
} from "./logs.js";

/** Files in memory. */
class MemoryFs implements LogFs {
  readonly files = new Map<string, string>();
  failing = false;
  size(file: string): number {
    return Buffer.byteLength(this.files.get(file) ?? "");
  }
  append(file: string, text: string): void {
    if (this.failing) throw new Error("disk full");
    this.files.set(file, (this.files.get(file) ?? "") + text);
  }
  rename(from: string, to: string): void {
    const text = this.files.get(from);
    if (text === undefined) return;
    this.files.delete(from);
    this.files.set(to, text);
  }
  remove(file: string): void {
    this.files.delete(file);
  }
}

const at = (): Date => new Date("2026-10-19T12:00:00.000Z");

describe("arguments of logs.write", () => {
  it("accepts the four levels and messages up to 2,000 characters", () => {
    expect(["debug", "info", "warn", "error"].every(isLogLevel)).toBe(true);
    expect(isLogLevel("fatal")).toBe(false);
    expect(parseLogArgs("info", "x".repeat(2000))?.message).toHaveLength(2000);
    expect(parseLogArgs("info", "x".repeat(2001))).toBeUndefined();
    expect(parseLogArgs("info", 5)).toBeUndefined();
    expect(parseLogArgs("loud", "x")).toBeUndefined();
  });
});

describe("masking", () => {
  it("keeps one entry on one line", () => {
    expect(maskMessage("a\nb\r\tc\u0000d\u001be\u0085f ✈")).toBe(
      "a\\nb\\r\\tc\\u0000d\\u001be\\u0085f ✈",
    );
  });
});

describe("log file", () => {
  it("writes lines with time, level and source and skips lower levels", () => {
    const fs = new MemoryFs();
    const log = new LogFile(fs, "log", "info", at);
    log.write("debug", "main", "hidden");
    log.write("warn", "main", "two\nlines");
    expect(fs.files.get("log")).toBe("2026-10-19T12:00:00.000Z WARN main two\\nlines\n");
  });

  it("rotates over five files of 10 MB", () => {
    const fs = new MemoryFs();
    const log = new LogFile(fs, "log", "debug", at);
    const big = "x".repeat(LOG_FILE_BYTES - 10);
    for (let n = 0; n < 7; n += 1) {
      fs.files.set("log", `${String(n)}${big}`);
      log.write("info", "main", "next");
    }
    expect([...fs.files.keys()].sort()).toEqual(["log", "log.1", "log.2", "log.3", "log.4"]);
    expect(fs.files.get("log")).toContain("next");
    expect(fs.files.get("log.1")?.startsWith("6")).toBe(true);
    expect(fs.files.get("log.4")?.startsWith("3")).toBe(true);
  });

  it("throttles the renderer to 100 entries per second and reports the dropped ones", () => {
    const fs = new MemoryFs();
    const log = new LogFile(fs, "log", "debug", at);
    let accepted = 0;
    for (let n = 0; n < MAX_ENTRIES_PER_SECOND + 5; n += 1) {
      if (log.writeThrottled("info", `m${String(n)}`, 500)) accepted += 1;
    }
    expect(accepted).toBe(MAX_ENTRIES_PER_SECOND);
    expect(log.writeThrottled("info", "later", 1600)).toBe(true);
    const lines = fs.files.get("log")?.trimEnd().split("\n") ?? [];
    expect(lines).toHaveLength(MAX_ENTRIES_PER_SECOND + 2);
    expect(lines.at(-2)).toContain("WARN main 5 renderer log entries dropped");
    expect(lines.at(-1)).toContain("INFO renderer later");
  });

  it("never throws when the disk fails", () => {
    const fs = new MemoryFs();
    fs.failing = true;
    const log = new LogFile(fs, "log", "debug", at);
    expect(() => {
      log.write("error", "main", "lost");
    }).not.toThrow();
  });
});

describe("bounds (separate review of 2.5b)", () => {
  it("refuses a huge message before counting it", () => {
    expect(parseLogArgs("info", "x".repeat(10_000_000))).toBeUndefined();
  });

  it("does not drop entries for good when the clock steps back", () => {
    const fs = new MemoryFs();
    const log = new LogFile(fs, "log", "debug", at);
    for (let n = 0; n < MAX_ENTRIES_PER_SECOND; n += 1) log.writeThrottled("info", "x", 5000);
    expect(log.writeThrottled("info", "back", 100)).toBe(true);
  });

  it("drops entries instead of growing when rotation fails", () => {
    const fs = new MemoryFs();
    fs.rename = () => undefined;
    const log = new LogFile(fs, "log", "debug", at);
    fs.files.set("log", "x".repeat(LOG_FILE_BYTES - 10));
    log.write("info", "main", "too much");
    expect(fs.size("log")).toBe(LOG_FILE_BYTES - 10);
  });

  it("lets at most n events per second pass and counts the rest", () => {
    const gate = new RateGate(2);
    expect(gate.pass(0)).toEqual({ pass: true, dropped: 0 });
    expect(gate.pass(10)).toEqual({ pass: true, dropped: 0 });
    expect(gate.pass(20)).toEqual({ pass: false, dropped: 0 });
    expect(gate.pass(30)).toEqual({ pass: false, dropped: 0 });
    expect(gate.pass(1000)).toEqual({ pass: true, dropped: 2 });
    expect(gate.pass(500)).toEqual({ pass: true, dropped: 0 });
  });
});
