import { execFileSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import os from "node:os";
import path from "node:path";
import { _electron as electron, expect, test } from "@playwright/test";

// Acceptance criterion of roadmap step 2.1: an empty Electron window starts and
// the utility process reports a loaded napi module (ADR-009, ADR-018). Since
// step 2.7 also: handshake over the MessagePort, frames (S-1), commands (S-2)
// with answers (S-3) and the platform API S-6; since step 2.4 the base data
// pack (S-4) and `packs.*` (S-6); since step 2.5 save games (S-5, `saves.*`),
// the log file (`logs.*`) and the error form `{ code, textKey, params }`.
const packaged = process.env.OPENAIRSIDE_E2E_EXECUTABLE;
// Linux runners and containers cannot start the Chromium OS sandbox without a
// SUID helper; same pattern as the spike builds. Linux sandbox: roadmap step 7.2.
const sandboxArgs = process.platform === "linux" ? ["--no-sandbox"] : [];

/** Entries of the base pack: its 3 objects plus every text of `lang/` (S-4, ADR-031). */
function baseEntries(): number {
  const dir = path.join(import.meta.dirname, "..", "content", "base", "lang");
  let texts = 0;
  for (const file of readdirSync(dir).filter((name) => name.endsWith(".json"))) {
    const parsed = JSON.parse(readFileSync(path.join(dir, file), "utf8")) as {
      languages: { texts: Record<string, string> }[];
    };
    for (const language of parsed.languages) texts += Object.keys(language.texts).length;
  }
  return 3 + texts;
}

/** A user pack "kiosks" with one object type in the pack folder of `userData` (FR-044). */
function writeUserPack(userData: string): void {
  const pack = path.join(userData, "packs", "kiosks");
  mkdirSync(path.join(pack, "objects"), { recursive: true });
  writeFileSync(
    path.join(pack, "manifest.json"),
    JSON.stringify({
      id: "kiosks",
      name: "Kiosks",
      version: "1.0.0",
      format: 1,
      game: "*",
      dependencies: [{ id: "base", version: "*" }],
    }),
  );
  writeFileSync(
    path.join(pack, "objects", "kiosks.json"),
    JSON.stringify({
      objects: [
        { id: "kiosk", name: "object.kiosk", placement: { kind: "single", width: 2, depth: 2 } },
      ],
    }),
  );
}

test("window opens and core reports ready", async () => {
  // A fresh user data folder: saved pack choices of a developer do not interfere.
  const userData = mkdtempSync(path.join(os.tmpdir(), "openairside-e2e-"));
  writeUserPack(userData);
  const app = await electron.launch({
    // OPENAIRSIDE_E2E_EXECUTABLE points to a packaged build (release.yml); without
    // it the test starts the development build from the repository root.
    ...(packaged === undefined ? {} : { executablePath: packaged }),
    args: [...(packaged === undefined ? ["."] : []), ...sandboxArgs, `--user-data-dir=${userData}`],
    env: { ...process.env, ELECTRON_ENABLE_LOGGING: "1" },
  });
  try {
    const window = await app.firstWindow();
    await expect(window).toHaveTitle("OpenAirside");
    await expect(window.locator("#root")).toHaveAttribute("data-ready", "true");

    await expect
      .poll(async () => app.evaluate(() => globalThis.openairsideCoreStatus), { timeout: 20_000 })
      .toEqual({ state: "ready", info: { name: "openairside-core", version: "0.0.0" } });

    // Hardening is effective in the running renderer, not only configured.
    const nodeVisible = await window.evaluate(
      () => typeof (globalThis as { process?: unknown }).process,
    );
    expect(nodeVisible).toBe("undefined");

    // Handshake and frames over the port (S-1, ADR-021).
    const root = window.locator("#root");
    await expect(root).toHaveAttribute("data-session", "1", { timeout: 20_000 });
    await expect(root).toHaveAttribute("data-tick", "0");

    // Platform API S-6 through the preload.
    const info = await window.evaluate(async () => {
      const api = (
        globalThis as {
          openairside?: { apiVersion: number; app: { info(): Promise<{ version: string }> } };
        }
      ).openairside;
      return api === undefined
        ? undefined
        : { apiVersion: api.apiVersion, info: await api.app.info() };
    });
    expect(info?.apiVersion).toBe(1);
    expect(info?.info.version).toBe("0.0.0");
    // Measurement tool (step 2.7): no report without a started run.
    const refused = await window.evaluate(async () => {
      const api = (
        globalThis as { openairside?: { measure: { report(r: unknown): Promise<string> } } }
      ).openairside;
      try {
        await api?.measure.report({});
        return "accepted";
      } catch (error) {
        return error;
      }
    });
    // Error form of S-6: a plain object with code, text key and parameters.
    expect(refused).toEqual({
      code: "notStarted",
      textKey: "platform.error.notStarted",
      params: {},
    });

    // Data packs (step 2.4): the base pack is part of the build and loads.
    await expect(root).toHaveAttribute("data-base-pack", "loaded", { timeout: 20_000 });
    const packs = await window.evaluate(async () => {
      const api = (
        globalThis as {
          openairside?: {
            packs: {
              list(): Promise<{ id: string | null; state: string; entries: number }[]>;
              setActive(ids: string[]): Promise<unknown>;
            };
          };
        }
      ).openairside;
      if (api === undefined) return undefined;
      const list = await api.packs.list();
      let refused: unknown = "accepted";
      try {
        await api.packs.setActive(["not-installed"]);
      } catch (error) {
        refused = (error as { code?: unknown }).code;
      }
      return { base: list.find((p) => p.id === "base"), refused };
    });
    expect(packs?.base).toMatchObject({ state: "active", entries: baseEntries() });
    expect(packs?.refused).toBe("unknownPack");

    // Console commands reach the core and take effect in the frames (S-2, S-3, FR-009).
    const input = window.locator("#console-input");
    await input.fill("spawn 500");
    await input.press("Enter");
    await expect(window.locator("#log")).toContainText("#1 ok");
    await expect(window.locator("#hud")).toContainText("/500");
    await input.fill("speed x3");
    await input.press("Enter");
    await expect(window.locator("#log")).toContainText("#2 ok");
    await expect
      .poll(async () => Number(await root.getAttribute("data-tick")), { timeout: 10_000 })
      .toBeGreaterThan(30);
    // Query service S-7 with the test table.
    await input.fill("table 3");
    await input.press("Enter");
    await expect(window.locator("#log")).toContainText("table: 3 of 500 rows");
    // Figures of the tick arrive about once per second (coreStats).
    await expect(window.locator("#hud")).toContainText("ticks/s");
    await input.fill("packs");
    await input.press("Enter");
    await expect(window.locator("#log")).toContainText(
      `active base base 0.0.0 – ${String(baseEntries())} entries`,
    );
    await input.fill("speed x4");
    await input.press("Enter");
    await expect(window.locator("#log")).toContainText("speed paused|x1|x2|x3");

    // Save games (S-5, step 2.5): save, list with preview, load into a new session.
    const consoleLog = window.locator("#log");
    await input.fill("save Rauchtest");
    await input.press("Enter");
    await expect(consoleLog).toContainText(/saved m-[0-9a-f]{16}/, { timeout: 20_000 });
    await window.keyboard.press("F5");
    await expect(consoleLog).toContainText("saved quick", { timeout: 20_000 });
    const saves = await window.evaluate(async () => {
      interface Entry {
        id: string;
        state: string;
        meta?: { name: string; kind: string; tick: number };
      }
      const api = (
        globalThis as {
          openairside?: {
            saves: {
              list(): Promise<Entry[]>;
              save(r: { name: string; kind: string }): Promise<{ id: string }>;
              preview(id: string): Promise<ArrayBuffer>;
              load(id: string): Promise<unknown>;
              delete(id: string): Promise<null>;
            };
            logs: { write(level: string, message: string): Promise<null> };
          };
        }
      ).openairside;
      if (api === undefined) return undefined;
      const auto = await api.saves.save({ name: "", kind: "auto" });
      const list = await api.saves.list();
      const manual = list.find((e) => e.meta?.name === "Rauchtest");
      const png = new Uint8Array(await api.saves.preview(manual?.id ?? ""));
      const codeOf = async (run: () => Promise<unknown>): Promise<unknown> => {
        try {
          await run();
          return "accepted";
        } catch (error) {
          return (error as { code?: unknown }).code;
        }
      };
      await api.logs.write("info", "smoke test\nsecond line");
      return {
        auto: auto.id,
        ids: list.map((e) => e.id).sort(),
        manual,
        pngSignature: [...png.subarray(0, 4)],
        unknown: await codeOf(() => api.saves.load("auto-9")),
        invalid: await codeOf(() => api.saves.load("../x")),
        badName: await codeOf(() => api.saves.save({ name: "a\u0000b", kind: "manual" })),
        badLevel: await codeOf(() => api.logs.write("loud", "x")),
      };
    });
    expect(saves?.auto).toBe("auto-1");
    expect(saves?.ids).toEqual(["auto-1", saves?.manual?.id, "quick"].sort());
    expect(saves?.manual).toMatchObject({ state: "ok", meta: { kind: "manual" } });
    // PNG signature: the preview image was taken from the window.
    expect(saves?.pngSignature).toEqual([0x89, 0x50, 0x4e, 0x47]);
    expect(saves?.unknown).toBe("unknownSave");
    expect(saves?.invalid).toBe("invalidArgument");
    expect(saves?.badName).toBe("invalidArgument");
    expect(saves?.badLevel).toBe("invalidArgument");
    // The list with previews as blob: URLs (CSP img-src blob:).
    await input.fill("saves");
    await input.press("Enter");
    await expect(consoleLog).toContainText("saves: 3");
    const panel = window.locator("#saves");
    await expect(panel).toBeVisible();
    await expect(panel.locator(`li[data-id="${saves?.manual?.id ?? ""}"] img`)).toHaveAttribute(
      "src",
      /^blob:/,
    );
    const shown = await panel
      .locator(`li[data-id="${saves?.manual?.id ?? ""}"] img`)
      .evaluate((img: HTMLImageElement) => img.complete && img.naturalWidth);
    expect(shown).toBe(480);
    await panel.getByRole("button", { name: "Close" }).click();
    await expect(panel).toBeHidden();
    // Loading starts a new session with the saved tick.
    await input.fill(`load ${saves?.manual?.id ?? ""}`);
    await input.press("Enter");
    await expect(consoleLog).toContainText(`loaded ${saves?.manual?.id ?? ""}`, {
      timeout: 30_000,
    });
    await expect(root).toHaveAttribute("data-session", "2", { timeout: 20_000 });
    await input.fill(`delete ${saves?.manual?.id ?? ""}`);
    await input.press("Enter");
    await expect(consoleLog).toContainText(`deleted ${saves?.manual?.id ?? ""}`);
    // Files: ids as names, no leftover temporary files; log lines stay single lines.
    expect(readdirSync(path.join(userData, "saves")).sort()).toEqual([
      "auto-1.oasave",
      "quick.oasave",
    ]);
    const logText = readFileSync(path.join(userData, "logs", "openairside.log"), "utf8");
    expect(logText).toContain("INFO renderer smoke test\\nsecond line");
    expect(logText).toContain("saved manual game");
    expect(existsSync(path.join(userData, "logs"))).toBe(true);
    // Special files in the save game folder are refused, not read (separate
    // review of 2.5b, finding 4): a link to /dev/zero and a FIFO.
    if (process.platform !== "win32") {
      const savesFolder = path.join(userData, "saves");
      symlinkSync("/dev/zero", path.join(savesFolder, "auto-3.oasave"));
      execFileSync("mkfifo", [path.join(savesFolder, "auto-4.oasave")]);
      const special = await window.evaluate(async () => {
        const api = (
          globalThis as {
            openairside?: {
              saves: {
                list(): Promise<{ id: string; state: string; error?: { code: string } }[]>;
                load(id: string): Promise<unknown>;
              };
            };
          }
        ).openairside;
        if (api === undefined) return undefined;
        const list = await api.saves.list();
        let load: unknown = "accepted";
        try {
          await api.saves.load("auto-3");
        } catch (error) {
          load = (error as { code?: unknown }).code;
        }
        return {
          damaged: list
            .filter((e) => e.state === "damaged")
            .map((e) => `${e.id}:${e.error?.code ?? ""}`)
            .sort(),
          load,
        };
      });
      expect(special).toEqual({
        damaged: ["auto-3:invalidState", "auto-4:invalidState"],
        load: "invalidState",
      });
      rmSync(path.join(savesFolder, "auto-3.oasave"));
      rmSync(path.join(savesFolder, "auto-4.oasave"));
    }

    // Reloading the page hands over a fresh port and starts a new session (S-6).
    await window.reload();
    await expect(window.locator("#root")).toHaveAttribute("data-session", "3", { timeout: 20_000 });

    // An object from the pack folder in the game (FR-044, step 2.4): unknown
    // until the pack is chosen, buildable from the next session on.
    const log = window.locator("#log");
    const command = window.locator("#console-input");
    await command.fill("build kiosks:kiosk 10 10");
    await command.press("Enter");
    await expect(log).toContainText("#1 rejected: unknownContent");
    await command.fill("packs set kiosks");
    await command.press("Enter");
    await expect(log).toContainText("active user kiosks 1.0.0 – 1 entries");
    await command.fill("scenario hall");
    await command.press("Enter");
    await expect(root).toHaveAttribute("data-session", "4", { timeout: 20_000 });
    await command.fill("build kiosks:kiosk 10 10");
    await command.press("Enter");
    await expect(log).toContainText("#1 ok");

    // Crash of the core (S-6, ADR-029), forced: the platform restarts it and
    // the interface offers the last autosave; after the third restart within
    // 60 s it points to the log file instead.
    const killed = new Set<number>();
    const killCore = async (): Promise<void> => {
      // Waits for a started core process that was not killed yet.
      let pid: number | undefined;
      await expect
        .poll(
          async () => {
            const found = await app.evaluate(({ app: electronApp }) => ({
              state: globalThis.openairsideCoreStatus.state,
              pid: electronApp
                .getAppMetrics()
                .find((m) => m.serviceName === "OpenAirside Core" || m.name === "OpenAirside Core")
                ?.pid,
            }));
            pid = found.pid;
            return found.state === "ready" && pid !== undefined && !killed.has(pid);
          },
          { timeout: 20_000 },
        )
        .toBe(true);
      killed.add(pid ?? 0);
      process.kill(pid ?? 0, "SIGKILL");
    };
    const notice = window.locator("#notice");
    await killCore();
    await expect(notice).toContainText("Load the last autosave?", { timeout: 20_000 });
    await expect(root).toHaveAttribute("data-session", "1", { timeout: 20_000 });
    await notice.getByRole("button", { name: "Load last autosave" }).click();
    await expect(log).toContainText("loaded auto-1", { timeout: 30_000 });
    await expect(root).toHaveAttribute("data-session", "2", { timeout: 20_000 });
    for (let n = 0; n < 3; n += 1) await killCore();
    await expect(notice).toContainText("not restarted. Details are in the log file", {
      timeout: 20_000,
    });
    await expect(notice.getByRole("button", { name: "Open log folder" })).toBeVisible();
    const crashLog = readFileSync(path.join(userData, "logs", "openairside.log"), "utf8");
    expect(crashLog).toContain("no further restart");
  } finally {
    await app.close();
    rmSync(userData, { recursive: true, force: true });
  }
});
