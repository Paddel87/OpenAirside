// Container probe of spike 3.8: starts the dev build, runs `world <mode> <draw> 1`, prints the log tail.
import { _electron as electron } from "@playwright/test";
import { mkdtempSync } from "node:fs";
import os from "node:os";
import path from "node:path";
const [mode, draw, minutes = "1"] = process.argv.slice(2);
const userData = mkdtempSync(path.join(os.tmpdir(), "oas-probe-"));
const app = await electron.launch({
  args: [".", "--no-sandbox", "--enable-unsafe-swiftshader", `--user-data-dir=${userData}`],
  env: { ...process.env, ELECTRON_ENABLE_LOGGING: "1" },
});
const win = await app.firstWindow();
win.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") console.log("console:", m.text().slice(0, 300)); });
win.on("pageerror", (e) => console.log("pageerror:", e.message));
const root = win.locator("#root");
await root.waitFor();
await win.waitForFunction(() => document.getElementById("root")?.dataset.session === "1", null, { timeout: 30000 });
await win.locator("#console-input").fill(`world ${mode} ${draw} ${minutes}`);
await win.locator("#console-input").press("Enter");
await win.waitForFunction(() => ["done", "failed"].includes(document.getElementById("root")?.dataset.measure ?? ""), null, { timeout: 15 * 60000 });
await win.screenshot({ path: path.join(process.argv[1], "..", `shot-${mode}-${draw}.png`) });
console.log((await win.locator("#log").innerText()).split("\n").slice(-12).join("\n"));
await app.close();
