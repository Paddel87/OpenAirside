import { mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  createFolder,
  errorCode,
  nodeLogFs,
  tryCreateFolder,
  writeFileAtomic,
  writeNewFile,
} from "./files.js";

let dir: string;

beforeEach(() => {
  dir = mkdtempSync(path.join(os.tmpdir(), "oa-files-"));
});

afterEach(() => {
  rmSync(dir, { recursive: true, force: true });
});

describe("files", () => {
  it("reads the code of a Node.js error without its message", () => {
    expect(errorCode(Object.assign(new Error("open '/secret/path'"), { code: "ENOENT" }))).toBe(
      "ENOENT",
    );
    expect(errorCode(new Error("no code"))).toBe("unknown");
    expect(errorCode("text")).toBe("unknown");
    expect(errorCode(null)).toBe("unknown");
  });

  it("creates folders and reports a failure as code", async () => {
    const nested = path.join(dir, "a", "b");
    expect(tryCreateFolder(nested)).toBeUndefined();
    expect(tryCreateFolder(nested)).toBeUndefined();
    await createFolder(path.join(dir, "c", "d"));
    const file = path.join(dir, "file");
    writeFileSync(file, "x");
    expect(tryCreateFolder(path.join(file, "below"))).toMatch(/^E/);
    await expect(createFolder(path.join(file, "below"))).rejects.toMatchObject({
      code: expect.stringMatching(/^E/) as unknown,
    });
  });

  it("writes a file by write and rename and leaves no temporary file", async () => {
    const file = path.join(dir, "sub", "a.json");
    await writeFileAtomic(file, "one");
    await writeFileAtomic(file, "two");
    expect(readFileSync(file, "utf8")).toBe("two");
    expect(readdirSync(path.join(dir, "sub"))).toEqual(["a.json"]);
  });

  it("removes the temporary file when the rename fails", async () => {
    // A folder in the place of the target makes the rename fail.
    const file = path.join(dir, "target");
    mkdirSync(path.join(file, "inner"), { recursive: true });
    await expect(writeFileAtomic(file, "x")).rejects.toBeDefined();
    expect(readdirSync(dir)).toEqual(["target"]);
  });

  it("writes a new file and never overwrites one", async () => {
    const file = await writeNewFile(path.join(dir, "out"), "r.json", "one");
    expect(file).toBe(path.join(dir, "out", "r.json"));
    expect(readFileSync(file, "utf8")).toBe("one");
    await expect(writeNewFile(path.join(dir, "out"), "r.json", "two")).rejects.toMatchObject({
      code: "EEXIST",
    });
    expect(readFileSync(file, "utf8")).toBe("one");
  });

  it("serves the log file operations", () => {
    const file = path.join(dir, "log.txt");
    expect(nodeLogFs.size(file)).toBe(0);
    nodeLogFs.append(file, "abc");
    nodeLogFs.append(file, "de");
    expect(nodeLogFs.size(file)).toBe(5);
    const moved = path.join(dir, "log.1");
    nodeLogFs.rename(file, moved);
    expect(nodeLogFs.size(file)).toBe(0);
    expect(nodeLogFs.size(moved)).toBe(5);
    // A missing file is no error.
    nodeLogFs.rename(file, path.join(dir, "log.2"));
    nodeLogFs.remove(moved);
    nodeLogFs.remove(moved);
    expect(nodeLogFs.size(moved)).toBe(0);
  });
});
