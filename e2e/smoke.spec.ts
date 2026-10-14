import { _electron as electron, expect, test } from "@playwright/test";

// Acceptance criterion of roadmap step 2.1: an empty Electron window starts and
// the utility process reports a loaded napi module (ADR-009, ADR-018). Since
// step 2.7 also: handshake over the MessagePort, frames (S-1), commands (S-2)
// with answers (S-3) and the platform API S-6.
const packaged = process.env.OPENAIRSIDE_E2E_EXECUTABLE;
// Linux runners and containers cannot start the Chromium OS sandbox without a
// SUID helper; same pattern as the spike builds. Linux sandbox: roadmap step 7.2.
const sandboxArgs = process.platform === "linux" ? ["--no-sandbox"] : [];

test("window opens and core reports ready", async () => {
  const app = await electron.launch({
    // OPENAIRSIDE_E2E_EXECUTABLE points to a packaged build (release.yml); without
    // it the test starts the development build from the repository root.
    ...(packaged === undefined ? {} : { executablePath: packaged }),
    args: [...(packaged === undefined ? ["."] : []), ...sandboxArgs],
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
    await input.fill("speed x4");
    await input.press("Enter");
    await expect(window.locator("#log")).toContainText("speed paused|x1|x2|x3");

    // Reloading the page hands over a fresh port and starts a new session (S-6).
    await window.reload();
    await expect(window.locator("#root")).toHaveAttribute("data-session", "2", { timeout: 20_000 });
  } finally {
    await app.close();
  }
});
