// Spike 1.3 – utility process hosting the native core (ADR-009).
// The simulation runs on Rust threads; this JS thread only answers frame
// requests and forwards build commands (S-2).
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { Simulation } = require("./native/sim_bridge.node");

let sim = null;
let scratch = null;
let scratchFormat = -1;

function handle(port, m) {
  if (m.type === "init") {
    if (sim) sim.stop();
    const t0 = performance.now();
    sim = new Simulation({
      persons: m.persons,
      threads: m.threads,
      hz: m.hz,
      size: m.size,
    });
    scratch = null;
    scratchFormat = -1;
    const grid = sim.grid();
    port.postMessage({
      type: "ready",
      initMs: performance.now() - t0,
      grid: grid.buffer.slice(grid.byteOffset, grid.byteOffset + grid.byteLength),
    });
  } else if (m.type === "req") {
    if (scratchFormat !== m.format) {
      scratch = Buffer.allocUnsafeSlow(sim.maxFrameBytes(m.format));
      scratchFormat = m.format;
    }
    const t0 = performance.now();
    const n = sim.readFrame(scratch, m.format, m.floor, m.x0, m.y0, m.x1, m.y1, m.gridVersion);
    // Copy exactly the frame; posting across processes copies again (structured clone).
    const buf = scratch.buffer.slice(scratch.byteOffset, scratch.byteOffset + n);
    port.postMessage({ type: "frame", id: m.id, hostMs: performance.now() - t0, buf });
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
