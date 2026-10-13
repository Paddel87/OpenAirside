import { _electron as electron, expect, test } from "@playwright/test";

// Acceptance criterion of roadmap step 2.1: an empty Electron window starts and
// the utility process reports a loaded napi module (ADR-009, ADR-018).
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
  } finally {
    await app.close();
  }
});
