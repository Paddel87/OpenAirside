// Spike 1.1 – renderer: consumes frames each animation frame and measures.
import {
  CTRL_INTS,
  LATEST,
  READING,
  STOP,
  TICK,
  consume,
  frameBytes,
  stats,
} from "./shared.js";

const out = document.getElementById("out");

function getConfig() {
  return new Promise((resolve) => window.spike.onConfig(resolve));
}

function getPort() {
  return new Promise((resolve) => {
    window.addEventListener("message", function handler(e) {
      if (e.data === "sim-port" && e.ports.length > 0) {
        window.removeEventListener("message", handler);
        resolve(e.ports[0]);
      }
    });
  });
}

// Listen before awaiting the config: main sends the port first.
const portPromise = getPort();
const cfg = await getConfig();
const frameLen = frameBytes(cfg.entities);
const frameIntervals = [];
const readMs = [];
const longFrames = { over20: 0, over33: 0 };
let frames = 0;
let firstTick = null;
let lastTick = 0;
let sink = 0;

function trackTick(tick) {
  if (firstTick === null) firstTick = tick;
  lastTick = tick;
}

async function runNapi() {
  const port = await portPromise;
  const rtt = [];
  const copyMs = [];
  let pending = null;
  let latest = null;
  let stopResolve;
  const stopped = new Promise((r) => (stopResolve = r));
  let simStats = null;
  port.onmessage = (e) => {
    const m = e.data;
    if (m.type === "frame") {
      rtt.push(performance.now() - pending.t);
      copyMs.push(m.copyMs);
      latest = m;
      pending = null;
    } else if (m.type === "stats") {
      simStats = m.stats;
    } else if (m.type === "stopped") {
      stopResolve();
    }
  };
  port.postMessage({ type: "start" });
  let id = 0;
  await loop(() => {
    if (pending === null) {
      pending = { id: ++id, t: performance.now() };
      port.postMessage({ type: "req", id: pending.id });
    }
    if (latest !== null) {
      const t0 = performance.now();
      sink += consume(new DataView(latest.buf), 0);
      readMs.push(performance.now() - t0);
      trackTick(latest.tick);
    }
  });
  port.postMessage({ type: "stats" });
  port.postMessage({ type: "stop" });
  await stopped;
  return {
    transfer: "MessagePort (structured clone, copy across processes)",
    rtt_ms: stats(rtt),
    host_copy_ms: stats(copyMs),
    sim: simStats,
  };
}

async function runWasm() {
  const frameSab = new SharedArrayBuffer(3 * frameLen);
  const ctrl = new Int32Array(new SharedArrayBuffer(CTRL_INTS * 4));
  Atomics.store(ctrl, LATEST, -1);
  Atomics.store(ctrl, READING, -1);
  const worker = new Worker("wasm-worker.js", { type: "module" });
  const result = new Promise((resolve, reject) => {
    worker.onmessage = (e) => {
      if (e.data.type === "log") window.spike.log(e.data.msg);
      if (e.data.type === "done") resolve(e.data);
    };
    worker.onerror = (e) => reject(new Error(e.message));
  });
  worker.postMessage({ cfg, frameSab, ctrl });
  const view = new DataView(frameSab);
  await loop(() => {
    let slot;
    for (;;) {
      slot = Atomics.load(ctrl, LATEST);
      if (slot < 0) return;
      Atomics.store(ctrl, READING, slot);
      if (Atomics.load(ctrl, LATEST) === slot) break;
    }
    const t0 = performance.now();
    sink += consume(view, slot * frameLen);
    readMs.push(performance.now() - t0);
    trackTick(view.getUint32(slot * frameLen + 8, true));
    Atomics.store(ctrl, READING, -1);
  });
  Atomics.store(ctrl, STOP, 1);
  const done = await result;
  worker.terminate();
  return {
    transfer: "SharedArrayBuffer triple buffer (zero copy for the renderer)",
    sim: done.stats,
    determinism_checksum_1000_ticks: done.checksum,
    tick_at_stop: Atomics.load(ctrl, TICK),
  };
}

function loop(onFrame) {
  return new Promise((resolve) => {
    const start = performance.now();
    let prev = start;
    function frame(now) {
      if (frames > 0) {
        const dt = now - prev;
        frameIntervals.push(dt);
        if (dt > 20) longFrames.over20++;
        if (dt > 33.4) longFrames.over33++;
      }
      prev = now;
      frames++;
      onFrame();
      if (now - start < cfg.seconds * 1000) {
        requestAnimationFrame(frame);
      } else {
        resolve();
      }
    }
    requestAnimationFrame(frame);
  });
}

try {
  const t0 = performance.now();
  const extra = cfg.variant === "napi" ? await runNapi() : await runWasm();
  const seconds = (performance.now() - t0) / 1000;
  const result = {
    crossOriginIsolated: self.crossOriginIsolated,
    fps: Math.round((frames / seconds) * 10) / 10,
    frame_interval_ms: stats(frameIntervals),
    long_frames: longFrames,
    renderer_read_ms: stats(readMs),
    ticks_seen_per_s:
      Math.round(((lastTick - (firstTick ?? 0)) / seconds) * 10) / 10,
    sink: Math.round(sink) % 1000,
    ...extra,
  };
  out.textContent = JSON.stringify(result, null, 2);
  window.spike.report(result);
} catch (err) {
  window.spike.log(err.stack ?? String(err));
  window.spike.report({ error: String(err) });
}
