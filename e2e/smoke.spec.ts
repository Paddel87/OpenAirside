import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { _electron as electron, expect, test } from "@playwright/test";

// Acceptance criterion of roadmap step 2.1: an empty Electron window starts and
// the utility process reports a loaded napi module (ADR-009, ADR-018). Since
// step 2.7 also: handshake over the MessagePort, frames (S-1), commands (S-2)
// with answers (S-3) and the platform API S-6; since step 2.4 the base data
// pack (S-4) and `packs.*` (S-6).
const packaged = process.env.OPENAIRSIDE_E2E_EXECUTABLE;
// Linux runners and containers cannot start the Chromium OS sandbox without a
// SUID helper; same pattern as the spike builds. Linux sandbox: roadmap step 7.2.
const sandboxArgs = process.platform === "linux" ? ["--no-sandbox"] : [];

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
        return String(error);
      }
    });
    expect(refused).toContain("notStarted");

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
      let refused = "accepted";
      try {
        await api.packs.setActive(["not-installed"]);
      } catch (error) {
        refused = String(error);
      }
      return { base: list.find((p) => p.id === "base"), refused };
    });
    expect(packs?.base).toMatchObject({ state: "active", entries: 3 });
    expect(packs?.refused).toContain("unknownPack");

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
    await expect(window.locator("#log")).toContainText("active base base 0.0.0 – 3 entries");
    await input.fill("speed x4");
    await input.press("Enter");
    await expect(window.locator("#log")).toContainText("speed paused|x1|x2|x3");

    // Reloading the page hands over a fresh port and starts a new session (S-6).
    await window.reload();
    await expect(window.locator("#root")).toHaveAttribute("data-session", "2", { timeout: 20_000 });

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
    await expect(root).toHaveAttribute("data-session", "3", { timeout: 20_000 });
    await command.fill("build kiosks:kiosk 10 10");
    await command.press("Enter");
    await expect(log).toContainText("#1 ok");
  } finally {
    await app.close();
    rmSync(userData, { recursive: true, force: true });
  }
});
