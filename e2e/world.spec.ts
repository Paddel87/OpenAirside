import { mkdtempSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { _electron as electron, expect, test } from "@playwright/test";

// Acceptance of step 3.1b (ADR-038, measure (11)): the resting world reaches
// the playfield over S-1 layout 2 and survives several session changes. Each
// change starts the transfer afresh (new game, scenario); the catalog (S-7)
// is asked for again, the display of the old session is gone, and no world
// section is dropped as malformed.
const packaged = process.env.OPENAIRSIDE_E2E_EXECUTABLE;
const sandboxArgs = process.platform === "linux" ? ["--no-sandbox"] : [];

test("the world follows four session changes and a build action", async () => {
  test.setTimeout(120_000);
  const userData = mkdtempSync(path.join(os.tmpdir(), "openairside-e2e-world-"));
  const app = await electron.launch({
    ...(packaged === undefined ? {} : { executablePath: packaged }),
    args: [...(packaged === undefined ? ["."] : []), ...sandboxArgs, `--user-data-dir=${userData}`],
  });
  try {
    const window = await app.firstWindow();
    const root = window.locator("#root");
    await expect(root).toHaveAttribute("data-ready", "true");
    const input = window.locator("#console-input");
    const run = async (line: string): Promise<void> => {
      await input.fill(line);
      await input.press("Enter");
    };
    const settled = async (session: number, blocks: number, objects: number): Promise<void> => {
      await expect(root).toHaveAttribute("data-session", String(session), { timeout: 30_000 });
      await expect(root).toHaveAttribute("data-world-blocks", String(blocks), { timeout: 30_000 });
      await expect(root).toHaveAttribute("data-world-objects", String(objects), {
        timeout: 30_000,
      });
      await expect(root).toHaveAttribute("data-world-complete", "true", { timeout: 30_000 });
    };

    // The scenario the core starts with: the ground floor of the default map (2048 cells, 64 × 64 blocks).
    await settled(1, 4096, 0);
    await expect(root).toHaveAttribute("data-catalog", "3", { timeout: 30_000 });

    // 1: a new game on 1536 cells has 48 × 48 blocks and no object.
    await run("newgame 1536");
    await settled(2, 2304, 0);
    await expect(root).toHaveAttribute("data-catalog", "3", { timeout: 30_000 });

    // 2: the furnished terminal on that map: floors, walls and about 20,000 objects of the base pack.
    await run("scenario furnished 100");
    await expect(root).toHaveAttribute("data-session", "3", { timeout: 30_000 });
    await expect
      .poll(async () => Number(await root.getAttribute("data-world-objects")), { timeout: 60_000 })
      .toBeGreaterThan(19_000);
    await expect(root).toHaveAttribute("data-world-complete", "true", { timeout: 30_000 });

    // 3: a new game on 2048 cells: the objects of the old session are gone, the blocks are those of the map.
    await run("newgame 2048");
    await settled(4, 4096, 0);

    // 4: and back; a build action of the console reaches the world in the same way.
    await run("newgame 1536");
    await settled(5, 2304, 0);
    await run("build base:bench 100 100");
    await expect(window.locator("#log")).toContainText("rejected: needsFloor");
    await expect(window.locator("#log")).not.toContainText("dropped");
    // The build tool answers the refusal with a hint that names the object.
    await expect(window.locator("#build .note")).toContainText("Bench needs floor");
  } finally {
    await app.close();
  }
});
