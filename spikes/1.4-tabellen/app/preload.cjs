// Spike 1.4 – narrow preload API (sandboxed preload, CommonJS), unchanged from spike 1.3.
const { contextBridge, ipcRenderer } = require("electron");

contextBridge.exposeInMainWorld("spike", {
  onConfig: (cb) => ipcRenderer.once("config", (_e, cfg) => cb(cfg)),
  // Resolves with the path of the written result file ("" if none).
  report: (result) => ipcRenderer.invoke("report", result),
  log: (msg) => ipcRenderer.send("log", String(msg)),
});

// MessagePorts cannot cross the context bridge as arguments; forward via window.postMessage.
ipcRenderer.on("sim-port", (event) => {
  window.postMessage("sim-port", "*", event.ports);
});
