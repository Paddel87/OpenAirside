// Spike 1.4 – Electron main process (from spike 1.3). Throwaway measurement harness.
//
// Usage: electron . [--series=quick|none] [--persons=N] [--hz=H] [--size=S]
//                   [--threads=N] [--phase-seconds=S] [--exit] [--out=<file>]
//   --series=quick : panel measurement series (default when packaged)
//   --series=none  : interactive only (default when started with `npm start`)
//   --rows=N, --rate=R restrict the series to one table size / update rate.
//   --draw=0 skips drawing (control measurement); --screenshot=<file> saves
//   the window after the series (interactive: after 6 s).
// The renderer runs the series and reports one JSON object. Without --out the
// packaged program writes it to the downloads folder and shows the path.
//
// Hardening as required by project-context.md section 6: contextIsolation on,
// nodeIntegration off, renderer sandboxed, no remote content, strict CSP.

import {
  app,
  BrowserWindow,
  MessageChannelMain,
  ipcMain,
  protocol,
  utilityProcess,
} from "electron";
import { readFile, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));

function arg(name, fallback) {
  const hit = process.argv.find((a) => a.startsWith(`--${name}=`));
  return hit ? hit.slice(name.length + 3) : fallback;
}

const config = {
  series: arg("series", app.isPackaged ? "quick" : "none"),
  persons: arg("persons", ""),
  format: arg("format", ""),
  render: arg("render", "particles"),
  transfer: arg("transfer", "view"),
  rows: arg("rows", ""),
  rate: arg("rate", ""),
  hz: Number(arg("hz", "60")),
  size: Number(arg("size", "512")),
  threads: Number(
    arg("threads", String(Math.min(4, os.availableParallelism()))),
  ),
  phaseSeconds: Number(arg("phase-seconds", "4")),
  // Control measurement: skip drawing (transfer and decoding only).
  draw: arg("draw", "1") !== "0",
};
const screenshot = arg("screenshot", "");
const outFile = arg("out", "");
const exitAfterRun = process.argv.includes("--exit");

const MIME = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".mjs": "text/javascript",
};

// Files served under app://spike/vendor/ (PixiJS global build, CSP-safe without eval).
const pixiDist = path.join(here, "node_modules", "pixi.js", "dist");
const VENDOR = {
  "/vendor/pixi.min.js": path.join(pixiDist, "pixi.min.js"),
  "/vendor/unsafe-eval.min.js": path.join(
    pixiDist,
    "packages",
    "unsafe-eval.min.js",
  ),
};

protocol.registerSchemesAsPrivileged([
  { scheme: "app", privileges: { standard: true, secure: true } },
]);

app.whenReady().then(async () => {
  const root = path.join(here, "renderer");
  protocol.handle("app", async (request) => {
    const url = new URL(request.url);
    const pathname = decodeURIComponent(url.pathname);
    const file = VENDOR[pathname] ?? path.normalize(path.join(root, pathname));
    if (!VENDOR[pathname] && !file.startsWith(root)) {
      return new Response("forbidden", { status: 403 });
    }
    try {
      const body = await readFile(file);
      return new Response(body, {
        headers: {
          "Content-Type":
            MIME[path.extname(file)] ?? "application/octet-stream",
        },
      });
    } catch {
      return new Response("not found", { status: 404 });
    }
  });

  const win = new BrowserWindow({
    width: 1280,
    height: 800,
    show: true,
    title: "OpenAirside – Testprogramm Spike 1.4",
    webPreferences: {
      preload: path.join(here, "preload.cjs"),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
      backgroundThrottling: false,
    },
  });

  ipcMain.handle("report", async (_event, result) => {
    const full = {
      ...config,
      platform: process.platform,
      arch: process.arch,
      electron: process.versions.electron,
      cpus: os.cpus().length,
      cpuModel: os.cpus()[0]?.model ?? "",
      memGb: Math.round(os.totalmem() / 2 ** 30),
      gpuFeatureStatus: app.getGPUFeatureStatus(),
      ...result,
    };
    const line = JSON.stringify(full);
    process.stdout.write(`${line}\n`);
    let target = outFile;
    if (!target && app.isPackaged) {
      const stamp = new Date().toISOString().replace(/[:.]/g, "-");
      target = path.join(
        app.getPath("downloads"),
        `openairside-spike-1-4-${process.platform}-${stamp}.json`,
      );
    }
    if (target) await writeFile(target, `${line}\n`);
    if (screenshot) {
      const img = await win.webContents.capturePage();
      await writeFile(screenshot, img.toPNG());
    }
    if (exitAfterRun) setTimeout(() => app.exit(0), 100);
    return target;
  });
  ipcMain.on("log", (_event, msg) =>
    process.stderr.write(`[renderer] ${msg}\n`),
  );

  const host = utilityProcess.fork(path.join(here, "simhost.mjs"), [], {
    serviceName: "sim-core",
    stdio: "inherit",
  });
  host.on("exit", (code) => process.stderr.write(`sim-core exited: ${code}\n`));

  win.webContents.once("did-finish-load", () => {
    const { port1, port2 } = new MessageChannelMain();
    host.postMessage("port", [port1]);
    win.webContents.postMessage("sim-port", null, [port2]);
    win.webContents.send("config", config);
  });

  if (screenshot && config.series === "none") {
    // Interactive mode: picture of the start view after 6 s.
    setTimeout(async () => {
      const img = await win.webContents.capturePage();
      await writeFile(screenshot, img.toPNG());
      if (exitAfterRun) app.exit(0);
    }, 6000);
  }

  if (exitAfterRun) {
    setTimeout(
      () => {
        process.stderr.write("timeout\n");
        app.exit(2);
      },
      30 * 60 * 1000,
    );
  }

  await win.loadURL("app://spike/index.html");
});

app.on("window-all-closed", () => app.quit());
