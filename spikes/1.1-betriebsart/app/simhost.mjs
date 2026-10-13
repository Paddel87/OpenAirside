// Spike 1.1 – utility process hosting the native core (variant A).
// The simulation runs on Rust threads; this JS thread only answers frame requests.
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { Simulation } = require("./native/sim_napi.node");
const config = JSON.parse(process.argv[2]);

const sim = new Simulation(config.entities, config.threads, config.work);

process.parentPort.once("message", (e) => {
  const [port] = e.ports;
  port.on("message", (msg) => {
    const data = msg.data;
    if (data.type === "start") {
      sim.start(config.hz);
    } else if (data.type === "req") {
      const t0 = performance.now();
      const buf = Buffer.allocUnsafeSlow(sim.frameBytes);
      const tick = sim.readFrame(buf);
      const copyMs = performance.now() - t0;
      // Plain ArrayBuffer; posting across processes copies (structured clone).
      port.postMessage({
        type: "frame",
        id: data.id,
        tick,
        copyMs,
        buf: buf.buffer,
      });
    } else if (data.type === "stats") {
      port.postMessage({ type: "stats", stats: sim.takeStats() });
    } else if (data.type === "stop") {
      sim.stop();
      port.postMessage({ type: "stopped" });
    }
  });
  port.start();
});
