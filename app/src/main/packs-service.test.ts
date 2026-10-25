import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { PackReport } from "../generated/PackReport.js";
import type { PackRequest } from "./core-status.js";
import { OperationLock } from "./operation-lock.js";
import { createPacksService, PACKS_TIMEOUT_MS } from "./packs-service.js";

const report = (id: string, source: PackReport["source"] = "base"): PackReport => ({
  folder: id,
  source,
  id,
  name: id,
  version: "1.0.0",
  state: "active",
  entries: 0,
  errors: [],
  errorsOmitted: 0,
});

let dir: string;
let sent: PackRequest[];
let lines: string[];

beforeEach(() => {
  dir = mkdtempSync(path.join(os.tmpdir(), "oa-packs-"));
  sent = [];
  lines = [];
});

afterEach(() => {
  vi.useRealTimers();
  rmSync(dir, { recursive: true, force: true });
});

function setup(ready = true, file = path.join(dir, "packs-active.json")) {
  const lock = new OperationLock();
  const service = createPacksService({
    lock,
    file: () => file,
    send: (message) => {
      sent.push(message);
    },
    coreReady: () => ready,
    log: (level, message) => {
      lines.push(`${level}: ${message}`);
    },
  });
  return { service, lock, file };
}

/** Waits until the service posted `count` requests to the core (the file is written by then). */
const posted = (count: number): Promise<void> =>
  vi.waitFor(() => {
    expect(sent).toHaveLength(count);
  });

describe("packs service", () => {
  it("holds the reports and languages of the core", () => {
    const { service } = setup();
    expect(service.reports()).toEqual([]);
    expect(service.languages()).toEqual([]);
    service.setReports([report("base")], [{ code: "en", name: "English", keys: 1 }]);
    expect(service.reports()).toHaveLength(1);
    expect(service.languages()).toEqual([{ code: "en", name: "English", keys: 1 }]);
  });

  it("refuses bad ids and packs it does not know", async () => {
    const { service } = setup();
    service.setReports([report("base"), report("extra", "user")], []);
    await expect(service.setActive("base")).rejects.toMatchObject({
      reason: { code: "invalidArgument" },
    });
    await expect(service.setActive(["Bad Id"])).rejects.toMatchObject({
      reason: { code: "invalidArgument" },
    });
    await expect(service.setActive(["ghost"])).rejects.toMatchObject({
      reason: { code: "unknownPack" },
    });
    expect(sent).toEqual([]);
  });

  it("is refused while the core is not ready", async () => {
    const { service } = setup(false);
    service.setReports([report("extra", "user")], []);
    await expect(service.setActive(["extra"])).rejects.toMatchObject({
      reason: { code: "notReady" },
    });
  });

  it("saves the choice, asks the core and returns its answer", async () => {
    const { service, file } = setup();
    service.setReports([report("extra", "user")], []);
    const call = service.setActive(["extra"]);
    await posted(1);
    expect(JSON.parse(readFileSync(file, "utf8"))).toEqual({ active: ["extra"] });
    expect(sent).toEqual([{ type: "packs:setActive", request: 1, ids: ["extra"] }]);
    expect(service.pendingRequest()).toBe(1);
    const reports = [report("base"), report("extra", "user")];
    service.settle({
      type: "packs",
      request: 1,
      packs: reports,
      languages: [{ code: "de", name: "Deutsch", keys: 1 }],
    });
    await expect(call).resolves.toEqual(reports);
    expect(service.reports()).toEqual(reports);
    expect(service.languages()).toHaveLength(1);
    expect(service.pendingRequest()).toBeUndefined();
  });

  it("refuses a second call while one is open", async () => {
    const { service, lock } = setup();
    service.setReports([report("extra", "user")], []);
    const first = service.setActive(["extra"]);
    await posted(1);
    // The operation lock is the first guard, the open request the second.
    await expect(service.setActive(["extra"])).rejects.toMatchObject({ reason: { code: "busy" } });
    expect(lock.busy).toBe(true);
    service.settle({ type: "packsError", request: 1, message: "coreExited" });
    await expect(first).rejects.toMatchObject({ reason: { code: "coreExited" } });
  });

  it("passes on only known error codes of the core", async () => {
    const { service } = setup();
    service.setReports([report("extra", "user")], []);
    const known = service.setActive(["extra"]);
    await posted(1);
    service.settle({ type: "packsError", request: 1, message: "invalidId" });
    await expect(known).rejects.toMatchObject({ reason: { code: "invalidId" } });

    const unknown = service.setActive(["extra"]);
    await posted(2);
    service.settle({ type: "packsError", request: 2, message: "stack trace with /secret/path" });
    await expect(unknown).rejects.toMatchObject({ reason: { code: "internal" } });
  });

  it("ignores a late answer to an earlier request", async () => {
    const { service } = setup();
    service.setReports([report("extra", "user")], []);
    const call = service.setActive(["extra"]);
    await posted(1);
    service.settle({ type: "packs", request: 99, packs: [report("x")], languages: [] });
    expect(service.pendingRequest()).toBe(1);
    expect(service.reports().map((r) => r.id)).toEqual(["extra"]);
    service.settle({ type: "packsError", request: 1, message: "busy" });
    await expect(call).rejects.toBeDefined();
  });

  it("fails with ioError and a warning if the choice cannot be saved", async () => {
    const blocker = path.join(dir, "blocker");
    writeFileSync(blocker, "a file where a folder should be");
    const { service } = setup(true, path.join(blocker, "packs-active.json"));
    service.setReports([report("extra", "user")], []);
    await expect(service.setActive(["extra"])).rejects.toMatchObject({
      reason: { code: "ioError" },
    });
    expect(sent).toEqual([]);
    expect(lines).toHaveLength(1);
    expect(lines[0]).toMatch(/^warn: cannot save the chosen packs: /);
  });

  it("gives up with timeout when the core does not answer", async () => {
    const { service } = setup();
    service.setReports([report("extra", "user")], []);
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    const call = service.setActive(["extra"]);
    const assertion = expect(call).rejects.toMatchObject({ reason: { code: "timeout" } });
    // Real file I/O runs in between; wait for the request to be posted.
    await vi.waitFor(() => {
      expect(sent).toHaveLength(1);
    });
    await vi.advanceTimersByTimeAsync(PACKS_TIMEOUT_MS);
    await assertion;
  });
});
