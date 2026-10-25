import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  rmSync,
  symlinkSync,
  utimesSync,
  writeFileSync,
} from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { createSaveFiles, type SaveFiles } from "./save-files.js";
import { saveFileName } from "./saves.js";

let root: string;
let dir: string;
let lines: string[];
let files: SaveFiles;

beforeEach(() => {
  root = mkdtempSync(path.join(os.tmpdir(), "oa-saves-"));
  dir = path.join(root, "saves");
  lines = [];
  files = createSaveFiles(
    () => dir,
    (level, message) => {
      lines.push(`${level}: ${message}`);
    },
  );
});

afterEach(() => {
  rmSync(root, { recursive: true, force: true });
});

const put = (id: string, content: string, mtime?: number): void => {
  mkdirSync(dir, { recursive: true });
  const file = path.join(dir, saveFileName(id));
  writeFileSync(file, content);
  if (mtime !== undefined) utimesSync(file, mtime, mtime);
};

const bytes = (text: string): Uint8Array => new TextEncoder().encode(text);
const text = (data: Uint8Array): string => new TextDecoder().decode(data);

describe("save files", () => {
  it("writes a save and reads it back whole, as prefix and as range", async () => {
    await files.write("m-0123456789abcdef", bytes("0123456789"));
    expect(readdirSync(dir)).toEqual(["m-0123456789abcdef.oasave"]);
    expect(text(await files.readAll("m-0123456789abcdef"))).toBe("0123456789");
    const prefix = await files.readPrefix("m-0123456789abcdef", 4);
    expect(text(prefix.prefix)).toBe("0123");
    expect(prefix.size).toBe(10);
    expect((await files.readPrefix("m-0123456789abcdef", 100)).prefix.length).toBe(10);
    expect(text(await files.readRange("m-0123456789abcdef", 3, 4))).toBe("3456");
    expect((await files.readRange("m-0123456789abcdef", 10, 0)).length).toBe(0);
  });

  it("replaces an existing save without leaving a temporary file", async () => {
    await files.write("quick", bytes("old"));
    await files.write("quick", bytes("newer"));
    expect(text(await files.readAll("quick"))).toBe("newer");
    expect(readdirSync(dir)).toEqual(["quick.oasave"]);
  });

  it("answers unknownSave for a missing save and a range past its end", async () => {
    await expect(files.readAll("quick")).rejects.toMatchObject({ reason: { code: "unknownSave" } });
    put("quick", "abc");
    await expect(files.readRange("quick", 2, 5)).rejects.toMatchObject({
      reason: { code: "unknownSave" },
    });
  });

  it("refuses a folder in the place of a save", async () => {
    mkdirSync(path.join(dir, saveFileName("quick")), { recursive: true });
    // POSIX opens the folder and sees it is no file; Windows cannot open it.
    await expect(files.readAll("quick")).rejects.toMatchObject({
      reason: { code: expect.stringMatching(/^(invalidState|ioError)$/) as unknown },
    });
  });

  it("lists the saves newest first and ignores other files", async () => {
    put("auto-1", "a", 1_000);
    put("quick", "b", 3_000);
    put("m-00000000000000aa", "c", 2_000);
    writeFileSync(path.join(dir, "notes.txt"), "x");
    const listed = await files.list();
    expect(listed.map((f) => f.id)).toEqual(["quick", "m-00000000000000aa", "auto-1"]);
    expect(listed[0]?.mtimeMs).toBe(3_000_000);
  });

  it("lists nothing for a missing folder", async () => {
    expect(await files.list()).toEqual([]);
    expect(lines).toEqual([]);
  });

  it("fails with ioError and a warning if the folder cannot be read", async () => {
    writeFileSync(path.join(root, "saves"), "a file where the folder should be");
    await expect(files.list()).rejects.toMatchObject({ reason: { code: "ioError" } });
    expect(lines.length).toBe(1);
    expect(lines[0]).toMatch(/^warn: cannot read the save game folder: /);
  });

  it("deletes a save and answers unknownSave for a missing one", async () => {
    put("quick", "abc");
    await files.remove("quick");
    expect(existsSync(path.join(dir, "quick.oasave"))).toBe(false);
    await expect(files.remove("quick")).rejects.toMatchObject({ reason: { code: "unknownSave" } });
  });

  it("removes a link itself and never its target", async () => {
    const target = path.join(root, "target.bin");
    writeFileSync(target, "keep");
    mkdirSync(dir, { recursive: true });
    try {
      symlinkSync(target, path.join(dir, "quick.oasave"));
    } catch {
      // Creating links needs rights on Windows; nothing to check there.
      return;
    }
    await files.remove("quick");
    expect(existsSync(target)).toBe(true);
    expect(existsSync(path.join(dir, "quick.oasave"))).toBe(false);
  });

  it("removes files of interrupted saves at start and keeps the saves", () => {
    put("quick", "abc");
    writeFileSync(path.join(dir, "quick.oasave.0123456789abcdef.tmp"), "half");
    writeFileSync(path.join(dir, "quick.oasave.tmp"), "half");
    writeFileSync(path.join(dir, "other.txt"), "x");
    files.removeTempFiles();
    expect(readdirSync(dir).sort()).toEqual(["other.txt", "quick.oasave"]);
    expect(lines).toEqual([]);
  });

  it("creates the folder at start and goes on if a temporary name is a folder", () => {
    files.removeTempFiles();
    expect(existsSync(dir)).toBe(true);
    mkdirSync(path.join(dir, "quick.oasave.tmp", "inner"), { recursive: true });
    writeFileSync(path.join(dir, "auto-1.oasave.tmp"), "half");
    files.removeTempFiles();
    expect(existsSync(path.join(dir, "auto-1.oasave.tmp"))).toBe(false);
    // `rmSync` with `force` and no `recursive` refuses the folder: a warning, no stop.
    expect(lines.every((l) => l.startsWith("warn: "))).toBe(true);
  });

  it("warns if the folder cannot be created", () => {
    writeFileSync(path.join(root, "saves"), "a file where the folder should be");
    files.removeTempFiles();
    expect(lines.length).toBe(1);
    expect(lines[0]).toMatch(/^warn: cannot read the save game folder: /);
  });

  it("fails with ioError and a warning if a save cannot be written", async () => {
    writeFileSync(path.join(root, "saves"), "a file where the folder should be");
    await expect(files.write("quick", bytes("x"))).rejects.toMatchObject({
      reason: { code: "ioError" },
    });
    expect(lines.length).toBe(1);
    expect(lines[0]).toMatch(/^warn: cannot write a save game: /);
  });
});
