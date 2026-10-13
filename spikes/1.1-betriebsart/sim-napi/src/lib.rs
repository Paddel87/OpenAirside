//! Variant A: simulation core as native Node module (napi-rs).
//!
//! The simulation runs on its own Rust thread with a dedicated rayon pool, so the
//! hosting JavaScript thread (Electron utility process) is never blocked. Finished
//! frames are published by swapping buffers under a short lock; JavaScript copies
//! the latest frame into its own buffer on demand.

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use sim_core::{World, frame_bytes};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

struct Published {
    bytes: Vec<u8>,
    tick: u64,
}

#[derive(Default)]
struct StepStats {
    samples_us: Vec<f64>,
    ticks: u64,
    behind_ticks: u64,
}

struct Shared {
    latest: Mutex<Published>,
    stats: Mutex<StepStats>,
    running: AtomicBool,
    tick_hz: AtomicU32,
}

/// Step statistics since the last call to `takeStats`.
#[napi(object)]
pub struct Stats {
    pub ticks: f64,
    pub step_us_mean: f64,
    pub step_us_p50: f64,
    pub step_us_p99: f64,
    pub behind_ticks: f64,
}

#[napi]
pub struct Simulation {
    entities: usize,
    threads: usize,
    work: u32,
    shared: Arc<Shared>,
    handle: Option<JoinHandle<()>>,
}

#[napi]
impl Simulation {
    #[napi(constructor)]
    #[must_use]
    pub fn new(entities: u32, threads: u32, work: u32) -> Self {
        let entities = entities as usize;
        Self {
            entities,
            threads: threads.max(1) as usize,
            work,
            shared: Arc::new(Shared {
                latest: Mutex::new(Published {
                    bytes: vec![0; frame_bytes(entities)],
                    tick: 0,
                }),
                stats: Mutex::new(StepStats::default()),
                running: AtomicBool::new(false),
                tick_hz: AtomicU32::new(0),
            }),
            handle: None,
        }
    }

    /// Size of one frame in bytes.
    #[napi(getter)]
    #[must_use]
    pub fn frame_bytes(&self) -> u32 {
        frame_bytes(self.entities) as u32
    }

    /// Starts the simulation thread. `tick_hz == 0` runs unthrottled (highest speed).
    #[napi]
    pub fn start(&mut self, tick_hz: u32) {
        self.shared.tick_hz.store(tick_hz, Ordering::Relaxed);
        if self.handle.is_some() {
            return;
        }
        self.shared.running.store(true, Ordering::Release);
        let shared = Arc::clone(&self.shared);
        let entities = self.entities;
        let threads = self.threads;
        let work = self.work;
        self.handle = Some(std::thread::spawn(move || {
            run(&shared, entities, threads, work);
        }));
    }

    #[napi]
    pub fn stop(&mut self) {
        self.shared.running.store(false, Ordering::Release);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }

    /// Copies the latest frame into `out`; returns its tick.
    #[napi]
    pub fn read_frame(&self, mut out: Buffer) -> f64 {
        let latest = self.shared.latest.lock().expect("frame lock");
        let n = latest.bytes.len().min(out.len());
        out[..n].copy_from_slice(&latest.bytes[..n]);
        latest.tick as f64
    }

    #[napi]
    pub fn take_stats(&self) -> Stats {
        let mut s = self.shared.stats.lock().expect("stats lock");
        let taken = std::mem::take(&mut *s);
        summarize(taken)
    }
}

fn summarize(mut s: StepStats) -> Stats {
    s.samples_us.sort_by(f64::total_cmp);
    let len = s.samples_us.len();
    let pick = |q: f64| {
        if len == 0 {
            0.0
        } else {
            s.samples_us[((len - 1) as f64 * q) as usize]
        }
    };
    Stats {
        ticks: s.ticks as f64,
        step_us_mean: if len == 0 {
            0.0
        } else {
            s.samples_us.iter().sum::<f64>() / len as f64
        },
        step_us_p50: pick(0.5),
        step_us_p99: pick(0.99),
        behind_ticks: s.behind_ticks as f64,
    }
}

fn run(shared: &Shared, entities: usize, threads: usize, work: u32) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("thread pool");
    let mut world = World::new(entities, 42);
    world.work = work;
    let mut back = vec![0u8; frame_bytes(entities)];
    let chunk = entities.div_ceil(threads * 4);
    let mut next = Instant::now();
    while shared.running.load(Ordering::Acquire) {
        let started = Instant::now();
        pool.install(|| {
            if threads == 1 {
                world.step_serial();
            } else {
                world.step_parallel(chunk);
            }
        });
        world.write_frame(&mut back);
        {
            let mut latest = shared.latest.lock().expect("frame lock");
            std::mem::swap(&mut latest.bytes, &mut back);
            latest.tick = world.tick;
        }
        let elapsed = started.elapsed();
        let hz = shared.tick_hz.load(Ordering::Relaxed);
        let mut behind = false;
        if hz > 0 {
            next += Duration::from_secs_f64(1.0 / f64::from(hz));
            let now = Instant::now();
            if next > now {
                std::thread::sleep(next - now);
            } else {
                behind = now - next > Duration::from_millis(1);
                next = now;
            }
        }
        let mut s = shared.stats.lock().expect("stats lock");
        s.samples_us.push(elapsed.as_secs_f64() * 1e6);
        s.ticks += 1;
        s.behind_ticks += u64::from(behind);
    }
}
