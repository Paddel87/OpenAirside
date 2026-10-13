// Spike 1.1 – simulation worker (variant B). Owns the world and steps it.
// wasm1: single-threaded build (stable Rust).
// wasmN: threaded build (nightly + build-std), helpers share the wasm memory.
import { LATEST, READING, STOP, TICK, stats } from "./shared.js";

// Barrier layout (Int32Array on its own SharedArrayBuffer).
const GEN = 0;
const DONE = 1;
const PTR = 2;
const RANGES = 4; // start/end pairs per helper

function log(msg) {
  self.postMessage({ type: "log", msg });
}

self.onmessage = async (e) => {
  const { cfg, frameSab, ctrl } = e.data;
  const threaded = cfg.variant === "wasmN";
  const pkg = threaded ? "./pkg-mt/sim_wasm.js" : "./pkg-st/sim_wasm.js";
  const mod = await import(pkg);
  let memory;
  let exportsObj;
  let helpers = [];
  let barrier;
  const helperCount = threaded ? Math.max(cfg.threads - 1, 0) : 0;
  if (threaded) {
    const module = await WebAssembly.compileStreaming(
      fetch("./pkg-mt/sim_wasm_bg.wasm"),
    );
    memory = new WebAssembly.Memory({
      initial: 64,
      maximum: 16384,
      shared: true,
    });
    exportsObj = await mod.default({ module_or_path: module, memory });
    barrier = new Int32Array(
      new SharedArrayBuffer((RANGES + 2 * helperCount + 2) * 4),
    );
    helpers = await Promise.all(
      Array.from({ length: helperCount }, (_, i) => {
        const h = new Worker("./wasm-helper.js", { type: "module" });
        return new Promise((resolve, reject) => {
          h.onmessage = (m) =>
            m.data === "ready" ? resolve(h) : log(String(m.data));
          h.onerror = (err) => reject(new Error(err.message));
          h.postMessage({ module, memory, barrier, index: i });
        });
      }),
    );
  } else {
    exportsObj = await mod.default({
      module_or_path: "./pkg-st/sim_wasm_bg.wasm",
    });
  }
  const parts = helperCount + 1;

  function stepThreaded(world, ptr, n) {
    const chunk = Math.ceil(n / parts);
    Atomics.store(barrier, DONE, 0);
    Atomics.store(barrier, PTR, ptr);
    for (let i = 0; i < helperCount; i++) {
      barrier[RANGES + 2 * i] = Math.min((i + 1) * chunk, n);
      barrier[RANGES + 2 * i + 1] = Math.min((i + 2) * chunk, n);
    }
    Atomics.add(barrier, GEN, 1);
    Atomics.notify(barrier, GEN);
    mod.step_chunk(ptr, 0, Math.min(chunk, n));
    for (;;) {
      const d = Atomics.load(barrier, DONE);
      if (d >= helperCount) break;
      Atomics.wait(barrier, DONE, d, 50);
    }
    world.finish_tick();
  }

  function makeStepper(world) {
    if (!threaded) return () => world.step();
    const ptr = world.world_ptr();
    const n = world.len();
    return () => stepThreaded(world, ptr, n);
  }

  // Determinism: 10,000 persons, 1000 ticks, same work factor as the native bench.
  const det = new mod.WasmWorld(10000, cfg.work);
  const detStep = makeStepper(det);
  for (let i = 0; i < 1000; i++) detStep();
  const checksum = det.checksum();
  det.free();

  const world = new mod.WasmWorld(cfg.entities, cfg.work);
  const step = makeStepper(world);
  const frameLen = world.frame_len();

  const samples = [];
  let ticks = 0;
  let behind = 0;
  const sleeper = new Int32Array(new SharedArrayBuffer(4));
  const period = cfg.hz > 0 ? 1000 / cfg.hz : 0;
  let next = performance.now();
  const frames = new Uint8Array(frameSab);
  while (Atomics.load(ctrl, STOP) === 0) {
    const t0 = performance.now();
    step();
    const off = world.write_frame();
    const latest = Atomics.load(ctrl, LATEST);
    const reading = Atomics.load(ctrl, READING);
    let slot = 0;
    while (slot === latest || slot === reading) slot++;
    // Memory may have grown; take a fresh view every tick.
    const mem = exportsObj.memory.buffer;
    frames.set(new Uint8Array(mem, off, frameLen), slot * frameLen);
    Atomics.store(ctrl, TICK, Number(world.tick()) | 0);
    Atomics.store(ctrl, LATEST, slot);
    samples.push(performance.now() - t0);
    ticks++;
    if (period > 0) {
      next += period;
      const now = performance.now();
      if (next > now) Atomics.wait(sleeper, 0, 0, next - now);
      else {
        if (now - next > 1) behind++;
        next = now;
      }
    }
  }
  for (const h of helpers) h.terminate();
  const s = stats(samples.map((ms) => ms * 1000));
  self.postMessage({
    type: "done",
    checksum,
    stats: {
      ticks,
      step_us_mean: s.mean,
      step_us_p50: s.p50,
      step_us_p99: s.p99,
      behind_ticks: behind,
      note: "step time includes frame write + copy into the SharedArrayBuffer slot",
    },
  });
};
