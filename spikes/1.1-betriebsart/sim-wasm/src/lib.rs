//! Variant B: simulation core as WebAssembly in a Web Worker (wasm-bindgen).
//!
//! Single-threaded build (stable Rust): one worker owns the world and steps it.
//! Multi-threaded build (`--cfg sim_threads`, requires atomics + shared memory):
//! several workers share one `WebAssembly.Memory`; each steps a chunk of persons
//! via [`step_chunk`], coordinated from JavaScript with `Atomics`.

use sim_core::{World, frame_bytes, step_persons};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmWorld {
    world: World,
    frame: Vec<u8>,
}

#[wasm_bindgen]
impl WasmWorld {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(entities: u32, work: u32) -> Self {
        let mut world = World::new(entities as usize, 42);
        world.work = work;
        Self {
            frame: vec![0; frame_bytes(entities as usize)],
            world,
        }
    }

    /// One tick on the calling thread.
    pub fn step(&mut self) {
        self.world.step_serial();
    }

    /// Raw pointer to this world inside linear memory, for helper workers.
    #[must_use]
    pub fn world_ptr(&mut self) -> usize {
        std::ptr::from_mut(&mut self.world) as usize
    }

    /// Persons count.
    #[must_use]
    pub fn len(&self) -> u32 {
        self.world.persons.len() as u32
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.world.persons.is_empty()
    }

    /// Marks the end of a tick stepped via [`step_chunk`].
    pub fn finish_tick(&mut self) {
        self.world.tick += 1;
    }

    /// Writes the frame into wasm memory and returns its offset.
    #[must_use]
    pub fn write_frame(&mut self) -> usize {
        self.world.write_frame(&mut self.frame);
        self.frame.as_ptr() as usize
    }

    #[must_use]
    pub fn frame_len(&self) -> u32 {
        self.frame.len() as u32
    }

    #[must_use]
    pub fn tick(&self) -> f64 {
        self.world.tick as f64
    }

    #[must_use]
    pub fn checksum(&self) -> String {
        format!("{:016x}", self.world.checksum())
    }
}

/// Steps persons `start..end` of the world at `world_ptr` (multi-threaded build).
///
/// # Safety
/// `world_ptr` must come from [`WasmWorld::world_ptr`] of a live world in the
/// shared memory, ranges of concurrent calls must not overlap, and no other
/// access to the persons may happen until all chunks of the tick are done.
#[wasm_bindgen]
pub unsafe fn step_chunk(world_ptr: usize, start: u32, end: u32) {
    // SAFETY: guaranteed by the caller contract above (spike 1.1, coordinated by
    // the JS barrier in app/renderer/wasm-worker.js).
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let (start, end) = (start as usize, end as usize);
    let persons = &mut world.persons[start..end];
    // Fields are read-only during the tick; split borrow via raw pointer.
    // SAFETY: `fields` is never mutated after construction.
    let fields = unsafe { &*std::ptr::from_ref(&world.fields) };
    step_persons(fields, persons, world.work);
}
