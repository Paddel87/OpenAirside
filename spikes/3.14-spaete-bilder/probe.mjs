// Container probe of spike 3.14: starts the dev build and runs the given console lines one after
// another (each a measurement with a new session), then prints the log tail.
// Usage: node probe.mjs <angle|-> "<line>" ["<line>" ...]
import { _electron as electron } from "@playwright/test";
import { mkdtempSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
const [angle, ...lines] = process.argv.slice(2);
const userData = mkdtempSync(path.join(os.tmpdir(), "oas-probe-"));
if (angle !== "-") writeFileSync(path.join(userData, "spike-angle.txt"), angle);
const app = await electron.launch({
  args: [".", "--no-sandbox", "--enable-unsafe-swiftshader", `--user-data-dir=${userData}`],
  env: { ...process.env, ELECTRON_ENABLE_LOGGING: "1" },
});
const win = await app.firstWindow();
win.on("pageerror", (e) => console.log("pageerror:", e.message));
const root = win.locator("#root");
await root.waitFor();
await win.waitForFunction(() => document.getElementById("root")?.dataset.session === "1", null, { timeout: 30000 });
for (const line of lines) {
  await win.evaluate(() => { delete document.getElementById("root").dataset.measure; });
  await win.locator("#console-input").fill(line);
  await win.locator("#console-input").press("Enter");
  if (line.startsWith("world") || line.startsWith("measure")) {
    await win.waitForFunction(() => ["done", "failed"].includes(document.getElementById("root")?.dataset.measure ?? ""), null, { timeout: 20 * 60000 });
  } else {
    await win.waitForTimeout(2000);
  }
  console.log(`--- ${line}`);
  console.log((await win.locator("#log").innerText()).split("\n").slice(-6).join("\n"));
}
await app.close();
