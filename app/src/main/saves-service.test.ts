import { describe, expect, it } from "vitest";
import type { SavesRequest } from "./core-status.js";
import { OperationLock } from "./operation-lock.js";
import { PlatformFailure, platformReason } from "./platform-error.js";
import type { SaveFile, SaveFiles } from "./save-files.js";
import { createSavesService } from "./saves-service.js";

const metaOf = (kind: string, name = ""): Record<string, unknown> => ({
  name,
  kind,
  savedAt: "2026-10-25T10:00:00Z",
  tick: 30,
  day: 1,
  year: 1,
  airportName: "",
  map: { width: 2048, height: 2048 },
  packs: [{ id: "base", version: "0.0.0", format: 1 }],
});

const infoOk = (previewOffset = 4, previewLen = 3): Record<string, unknown> => ({
  ok: true,
  info: { gameVersion: "0.0.0", meta: metaOf("manual"), previewOffset, previewLen },
});

/** In-memory save files; `data` holds the content by id. */
function fakeFiles(initial: Record<string, string> = {}): {
  files: SaveFiles;
  data: Map<string, Uint8Array>;
  written: string[];
  times: Map<string, number>;
} {
  const enc = new TextEncoder();
  const data = new Map<string, Uint8Array>(
    Object.entries(initial).map(([id, t]) => [id, enc.encode(t)]),
  );
  const times = new Map<string, number>(Object.keys(initial).map((id, i) => [id, 1000 + i]));
  const written: string[] = [];
  const need = (id: string): Uint8Array => {
    const bytes = data.get(id);
    if (bytes === undefined) throw new PlatformFailure(platformReason("unknownSave"));
    return bytes;
  };
  const files: SaveFiles = {
    removeTempFiles: () => undefined,
    list: () =>
      Promise.resolve(
        [...data.keys()]
          .map((id): SaveFile => ({ id, mtimeMs: times.get(id) ?? 0 }))
          .sort((a, b) => b.mtimeMs - a.mtimeMs),
      ),
    readPrefix: (id, max) => {
      const bytes = need(id);
      return Promise.resolve({ prefix: bytes.slice(0, max), size: bytes.length });
    },
    readAll: (id) => Promise.resolve(need(id)),
    readRange: (id, offset, length) => {
      const bytes = need(id);
      if (offset + length > bytes.length) throw new PlatformFailure(platformReason("unknownSave"));
      return Promise.resolve(bytes.slice(offset, offset + length));
    },
    write: (id, bytes) => {
      data.set(id, bytes);
      written.push(id);
      return Promise.resolve();
    },
    remove: (id) => {
      need(id);
      data.delete(id);
      return Promise.resolve();
    },
  };
  return { files, data, written, times };
}

type Answer = (request: SavesRequest) => unknown;

function setup(
  initial: Record<string, string>,
  answer: Answer,
  options: { ready?: boolean; autosaveCount?: number } = {},
): {
  service: ReturnType<typeof createSavesService>;
  lock: OperationLock;
  fake: ReturnType<typeof fakeFiles>;
  requests: SavesRequest[];
  lines: string[];
} {
  const fake = fakeFiles(initial);
  const lock = new OperationLock();
  const requests: SavesRequest[] = [];
  const lines: string[] = [];
  const service = createSavesService({
    files: fake.files,
    lock,
    request: (make) => {
      const request = make(requests.length + 1);
      requests.push(request);
      return Promise.resolve(answer(request));
    },
    preview: () => Promise.resolve(new Uint8Array([1, 2, 3])),
    autosaveCount: () => options.autosaveCount ?? 2,
    coreReady: () => options.ready ?? true,
    log: (level, message) => {
      lines.push(`${level}: ${message}`);
    },
  });
  return { service, lock, fake, requests, lines };
}

const idle = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 5));

describe("saves service: list", () => {
  it("lists the saves the core accepts and shows the others as damaged", async () => {
    const { service, requests } = setup(
      { "m-00000000000000aa": "file one", quick: "file two" },
      (request) =>
        request.type === "saves:info"
          ? [infoOk(), { ok: false, error: platformReason("badHeader") }]
          : undefined,
    );
    const entries = await service.list();
    expect(entries.map((e) => `${e.id}:${e.state}`).sort()).toEqual([
      "m-00000000000000aa:damaged",
      "quick:ok",
    ]);
    expect(requests).toHaveLength(1);
    const first = requests[0];
    expect(first?.type === "saves:info" && first.files.map((f) => f.size)).toEqual([8, 8]);
  });

  it("shows a file it cannot read as damaged and skips a vanished one", async () => {
    const { service, fake } = setup({ quick: "abcdefgh", "auto-1": "abcdefgh" }, (request) =>
      request.type === "saves:info" ? request.files.map(() => infoOk()) : undefined,
    );
    const real = fake.files.readPrefix.bind(fake.files);
    fake.files.readPrefix = (id, max) =>
      id === "auto-1"
        ? Promise.reject(new PlatformFailure(platformReason("unknownSave")))
        : real(id, max);
    const entries = await service.list();
    expect(entries.map((e) => e.id)).toEqual(["quick"]);

    fake.files.readPrefix = () => Promise.reject(new Error("boom"));
    const damaged = await service.list();
    expect(damaged.map((e) => e.state)).toEqual(["damaged", "damaged"]);
  });

  it("refuses an answer of the wrong length or form", async () => {
    const short = setup({ quick: "abcdefgh" }, () => []);
    await expect(short.service.list()).rejects.toMatchObject({ reason: { code: "invalidAnswer" } });
    const bad = setup({ quick: "abcdefgh" }, () => [{ ok: "maybe" }]);
    await expect(bad.service.list()).rejects.toMatchObject({ reason: { code: "invalidAnswer" } });
  });

  it("lists an empty folder without asking the core", async () => {
    const { service, requests } = setup({}, () => []);
    expect(await service.list()).toEqual([]);
    expect(requests).toHaveLength(0);
  });

  it("is refused while another operation runs", async () => {
    const { service, lock } = setup({}, () => []);
    let release: () => void = () => undefined;
    const running = lock.exclusive(
      "other",
      () =>
        new Promise<void>((resolve) => {
          release = resolve;
        }),
    );
    await expect(service.list()).rejects.toMatchObject({ reason: { code: "busy" } });
    release();
    await running;
  });
});

describe("saves service: save", () => {
  const saved = (request: SavesRequest): unknown =>
    request.type === "saves:save" ? new Uint8Array([9, 9]) : undefined;

  it("writes a manual save under a fresh id and reports it", async () => {
    const { service, fake, requests, lines } = setup({}, saved);
    const { id } = await service.save({ name: "Hello", kind: "manual" });
    expect(id).toMatch(/^m-[0-9a-f]{16}$/);
    expect(fake.written).toEqual([id]);
    const request = requests[0];
    expect(request?.type === "saves:save" && request.name).toBe("Hello");
    expect(request?.type === "saves:save" && [...request.preview]).toEqual([1, 2, 3]);
    expect(lines).toEqual([`info: saved manual game ${id}`]);
  });

  it("uses the single slot for a quick save", async () => {
    const { service } = setup({ quick: "old" }, saved);
    expect(await service.save({ name: "", kind: "quick" })).toEqual({ id: "quick" });
  });

  it("refuses a manual save beyond the limit", async () => {
    const initial = Object.fromEntries(
      Array.from({ length: 200 }, (_, i) => [`m-${i.toString(16).padStart(16, "0")}`, "x"]),
    );
    const { service, fake } = setup(initial, saved);
    await expect(service.save({ name: "", kind: "manual" })).rejects.toMatchObject({
      reason: { code: "limitExceeded" },
    });
    expect(fake.written).toEqual([]);
  });

  it("refuses bad arguments and a bad answer of the core", async () => {
    const { service } = setup({}, () => "not bytes");
    await expect(service.save({ name: 3, kind: "manual" })).rejects.toMatchObject({
      reason: { code: "invalidArgument" },
    });
    await expect(service.save({ name: "", kind: "manual" })).rejects.toMatchObject({
      reason: { code: "invalidAnswer" },
    });
  });
});

describe("saves service: load, delete, preview", () => {
  it("loads a save through the core and returns its meta", async () => {
    const { service, requests } = setup({ quick: "content" }, (request) =>
      request.type === "saves:load" ? metaOf("quick") : undefined,
    );
    const meta = await service.load("quick");
    expect(meta.kind).toBe("quick");
    const request = requests[0];
    expect(request?.type === "saves:load" && new TextDecoder().decode(request.bytes)).toBe(
      "content",
    );
  });

  it("refuses a bad id, an unknown save and a bad answer", async () => {
    const { service } = setup({ quick: "content" }, () => "nonsense");
    await expect(service.load("../x")).rejects.toMatchObject({
      reason: { code: "invalidArgument" },
    });
    await expect(service.load("auto-3")).rejects.toMatchObject({ reason: { code: "unknownSave" } });
    await expect(service.load("quick")).rejects.toMatchObject({
      reason: { code: "invalidAnswer" },
    });
  });

  it("deletes a save and forgets its preview", async () => {
    const { service, fake, lines } = setup({ quick: "abcdefgh" }, () => [infoOk()]);
    await service.list();
    expect(await service.delete("quick")).toBeNull();
    expect(fake.data.has("quick")).toBe(false);
    expect(lines).toContain("info: deleted game quick");
    await expect(service.preview("quick")).rejects.toMatchObject({
      reason: { code: "unknownSave" },
    });
    await expect(service.delete("quick")).rejects.toMatchObject({
      reason: { code: "unknownSave" },
    });
    await expect(service.delete(7)).rejects.toMatchObject({ reason: { code: "invalidArgument" } });
  });

  it("returns the preview after the core checked it", async () => {
    const { service, requests } = setup({ quick: "abcdPNGefgh" }, (request) =>
      request.type === "saves:info" ? [infoOk(4, 3)] : undefined,
    );
    await expect(service.preview("quick")).rejects.toMatchObject({
      reason: { code: "unknownSave" },
    });
    await service.list();
    const png = await service.preview("quick");
    expect(new TextDecoder().decode(png)).toBe("PNG");
    expect(requests.at(-1)?.type).toBe("saves:checkPreview");
  });

  it("returns an empty preview without asking the core", async () => {
    const { service, requests } = setup({ quick: "abcdefgh" }, (request) =>
      request.type === "saves:info" ? [infoOk(0, 0)] : undefined,
    );
    await service.list();
    const before = requests.length;
    expect((await service.preview("quick")).byteLength).toBe(0);
    expect(requests).toHaveLength(before);
  });

  it("forgets a preview that lies past the end of a changed file", async () => {
    const { service, fake } = setup({ quick: "abcdefgh" }, (request) =>
      request.type === "saves:info" ? [infoOk(4, 3)] : undefined,
    );
    await service.list();
    fake.data.set("quick", new TextEncoder().encode("abc"));
    await expect(service.preview("quick")).rejects.toMatchObject({
      reason: { code: "unknownSave" },
    });
  });
});

describe("saves service: autosave", () => {
  const saved = (request: SavesRequest): unknown =>
    request.type === "saves:save" ? new Uint8Array([1]) : undefined;

  it("saves into the first free slot when the core asks", async () => {
    const { service, fake } = setup({}, saved);
    service.autosaveRequested();
    await idle();
    expect(fake.written).toEqual(["auto-1"]);
  });

  it("reuses the oldest slot once all are taken", async () => {
    const { service, fake } = setup({ "auto-1": "x", "auto-2": "y" }, saved, { autosaveCount: 2 });
    fake.times.set("auto-1", 5000);
    fake.times.set("auto-2", 1000);
    service.autosaveRequested();
    await idle();
    expect(fake.written).toEqual(["auto-2"]);
  });

  it("waits for a running operation and then saves once", async () => {
    const { service, lock, fake } = setup({}, saved);
    let release: () => void = () => undefined;
    const running = lock.exclusive(
      "other",
      () =>
        new Promise<void>((resolve) => {
          release = resolve;
        }),
    );
    service.autosaveRequested();
    service.autosaveRequested();
    await idle();
    expect(fake.written).toEqual([]);
    expect(lock.autosaveDue).toBe(true);
    release();
    await running;
    await idle();
    expect(fake.written).toEqual(["auto-1"]);
    expect(lock.autosaveDue).toBe(false);
  });

  it("drops a due autosave if the core is not ready", async () => {
    const { service, lock, fake } = setup({}, saved, { ready: false });
    service.autosaveRequested();
    await idle();
    expect(fake.written).toEqual([]);
    expect(lock.autosaveDue).toBe(false);
  });

  it("logs a failed autosave and goes on", async () => {
    const { service, lines } = setup({}, () => "no bytes");
    service.autosaveRequested();
    await idle();
    expect(lines).toEqual(["warn: autosave failed: invalidAnswer"]);
    expect(lines).toHaveLength(1);
  });
});
