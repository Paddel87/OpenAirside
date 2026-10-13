// Spike 1.1 – helper worker for the threaded WebAssembly build (wasmN).
// Instantiates the same module on the shared memory and steps its chunk per tick.
const GEN = 0;
const DONE = 1;
const PTR = 2;
const RANGES = 4;

self.onmessage = async (e) => {
  const { module, memory, barrier, index } = e.data;
  const mod = await import("./pkg-mt/sim_wasm.js");
  mod.initSync({ module, memory });
  let gen = Atomics.load(barrier, GEN);
  self.postMessage("ready");
  for (;;) {
    Atomics.wait(barrier, GEN, gen);
    gen = Atomics.load(barrier, GEN);
    const start = barrier[RANGES + 2 * index];
    const end = barrier[RANGES + 2 * index + 1];
    if (end > start) mod.step_chunk(Atomics.load(barrier, PTR), start, end);
    Atomics.add(barrier, DONE, 1);
    Atomics.notify(barrier, DONE);
  }
};
