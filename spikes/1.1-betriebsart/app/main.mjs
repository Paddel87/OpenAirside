// Spike 1.1 – Electron main process. Throwaway measurement harness.
//
// Usage: electron . --variant=napi|wasm1|wasmN --threads=N --work=W --hz=H --seconds=S
//   napi  : native module in a utility process, frames to the renderer via MessagePort
//   wasm1 : single-threaded WebAssembly in a Web Worker, frames via SharedArrayBuffer
//   wasmN : multi-threaded WebAssembly (N workers on shared memory), frames via SharedArrayBuffer
// Prints one JSON result line to stdout and quits. Options for the packaged
// build (step 1.6): --out=<file> writes the result as JSON; in a packaged app
// without --exit the window stays open and shows the result.
//
// Hardening as required by project-context.md section 6: contextIsolation on,
// nodeIntegration off, renderer sandboxed, no remote content. The custom app://
// protocol sets COOP/COEP so the renderer is cross-origin isolated (SharedArrayBuffer).

import {
  app,
  BrowserWindow,
  MessageChannelMain,
  ipcMain,
  protocol,
  utilityProcess,
} from "electron";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));

function arg(name, fallback) {
  const hit = process.argv.find((a) => a.startsWith(`--${name}=`));
  return hit ? hit.slice(name.length + 3) : fallback;
}

const config = {
  variant: arg("variant", "napi"),
  threads: Number(arg("threads", "4")),
  work: Number(arg("work", "8")),
  hz: Number(arg("hz", "0")),
  seconds: Number(arg("seconds", "10")),
  entities: Number(arg("entities", "10000")),
};
const outFile = arg("out", "");
const exitAfterRun = process.argv.includes("--exit") || !app.isPackaged;

const MIME = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".mjs": "text/javascript",
  ".wasm": "application/wasm",
};

protocol.registerSchemesAsPrivileged([
  {
    scheme: "app",
    privileges: { standard: true, secure: true, supportFetchAPI: true },
  },
]);

app.whenReady().then(async () => {
  const root = path.join(here, "renderer");
  protocol.handle("app", async (request) => {
    const url = new URL(request.url);
    const file = path.normalize(
      path.join(root, decodeURIComponent(url.pathname)),
    );
    if (!file.startsWith(root)) {
      return new Response("forbidden", { status: 403 });
    }
    try {
      const body = await readFile(file);
      return new Response(body, {
        headers: {
          "Content-Type":
            MIME[path.extname(file)] ?? "application/octet-stream",
          "Cross-Origin-Opener-Policy": "same-origin",
          "Cross-Origin-Embedder-Policy": "require-corp",
          "Cross-Origin-Resource-Policy": "same-origin",
        },
      });
    } catch {
      return new Response("not found", { status: 404 });
    }
  });

  const win = new BrowserWindow({
    width: 900,
    height: 700,
    show: true,
    webPreferences: {
      preload: path.join(here, "preload.cjs"),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
      backgroundThrottling: false,
    },
  });

  ipcMain.once("report", async (_event, result) => {
    const line = JSON.stringify({
      ...config,
      platform: process.platform,
      arch: process.arch,
      electron: process.versions.electron,
      ...result,
    });
    process.stdout.write(`${line}\n`);
    if (outFile) await writeFile(outFile, `${line}\n`);
    if (exitAfterRun) app.exit(0);
  });
  ipcMain.on("log", (_event, msg) =>
    process.stderr.write(`[renderer] ${msg}\n`),
  );

  let host;
  if (config.variant === "napi") {
    host = utilityProcess.fork(
      path.join(here, "simhost.mjs"),
      [JSON.stringify(config)],
      {
        serviceName: "sim-core",
        stdio: "inherit",
      },
    );
  }

  win.webContents.once("did-finish-load", () => {
    if (host) {
      const { port1, port2 } = new MessageChannelMain();
      host.postMessage("port", [port1]);
      win.webContents.postMessage("sim-port", null, [port2]);
    }
    win.webContents.send("config", config);
  });

  if (exitAfterRun) {
    setTimeout(
      () => {
        process.stderr.write("timeout\n");
        app.exit(2);
      },
      (config.seconds + 30) * 1000,
    );
  }

  await win.loadURL("app://spike/index.html");
});
