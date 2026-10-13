// Spike 1.5 – utility process hosting the native core (ADR-009), from spike 1.4;
// passes the scenario options (floors, gates, full recomputation, background threads).
// The simulation runs on Rust threads; this JS thread answers frame requests,
// forwards build commands (S-2) and, new in 1.4, encodes the flight table for
// the panel on request (panel-core) at the tick of the newest frame.
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { Simulation } = require("./native/sim_bridge.node");

let sim = null;
let scratch = null;
let scratchFormat = -1;
let tableScratch = null;
let lastTick = 0; // tick of the newest frame handed to the renderer

function handle(port, m) {
  if (m.type === "init") {
    if (sim) sim.stop();
    const t0 = performance.now();
    sim = new Simulation({
      persons: m.persons,
      threads: m.threads,
      hz: m.hz,
      size: m.size,
      floors: m.floors,
      gates: m.gates,
      incremental: m.incremental,
      bgThreads: m.bgThreads,
    });
    scratch = null;
    scratchFormat = -1;
    const grid = sim.grid();
    port.postMessage({
      type: "ready",
      initMs: performance.now() - t0,
      grid: grid.buffer.slice(
        grid.byteOffset,
        grid.byteOffset + grid.byteLength,
      ),
      dict: JSON.parse(sim.tableDictionary()),
    });
  } else if (m.type === "req") {
    if (scratchFormat !== m.format) {
      scratch = Buffer.allocUnsafeSlow(sim.maxFrameBytes(m.format));
      scratchFormat = m.format;
    }
    const t0 = performance.now();
    const n = sim.readFrame(
      scratch,
      m.format,
      m.floor,
      m.x0,
      m.y0,
      m.x1,
      m.y1,
      m.gridVersion,
    );
    lastTick = scratch.readDoubleLE(0);
    // Copy exactly the frame; posting across processes copies again (structured clone).
    const buf = scratch.buffer.slice(
      scratch.byteOffset,
      scratch.byteOffset + n,
    );
    port.postMessage({
      type: "frame",
      id: m.id,
      hostMs: performance.now() - t0,
      buf,
    });
  } else if (m.type === "table") {
    const need = sim.maxTableBytes(m.rows);
    if (!tableScratch || tableScratch.length < need) {
      tableScratch = Buffer.allocUnsafeSlow(need);
    }
    const t0 = performance.now();
    const n = sim.readTable(tableScratch, m.rows, lastTick);
    const buf = tableScratch.buffer.slice(
      tableScratch.byteOffset,
      tableScratch.byteOffset + n,
    );
    port.postMessage({
      type: "table",
      id: m.id,
      hostMs: performance.now() - t0,
      buf,
    });
  } else if (m.type === "build") {
    sim.build(m.id, m.x, m.y, m.floor, m.size, m.wall);
  } else if (m.type === "hz") {
    sim.setHz(m.hz);
  } else if (m.type === "stats") {
    port.postMessage({ type: "stats", stats: sim.takeStats() });
  }
}

process.parentPort.once("message", (e) => {
  const [port] = e.ports;
  port.on("message", (msg) => handle(port, msg.data));
  port.start();
});
