//! Spike 1.3: Rust side of the bridge between simulation core and presentation.
//!
//! - The simulation (core from spike 1.2) runs on its own thread with a
//!   dedicated rayon pool and publishes one frame per tick.
//! - S-1 state buffer as a triple buffer: the simulation thread owns the back
//!   buffer, the reader owns the front buffer, and both only swap pointers with
//!   the middle buffer under a short lock. Neither side waits for the other's
//!   copy or encoding work; the reader always gets the newest finished tick.
//! - Frames are encoded on request: everything, or only persons on one floor
//!   inside the camera rectangle; positions compact (u16, 1/32 cell) or as f32.
//! - S-2 command channel in minimal form: build/remove walls. A command is
//!   applied at the start of the next tick; the frame that contains it carries
//!   its id and the changed cells, so the presentation can show the wall in the
//!   very next picture and measure the delay.
//!
//! Throwaway code; see README.md.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use wegweiser::grid::{Grid, STAIRS, WALL};
use wegweiser::sim::{Layout, Sim};

/// Positions are sent as fixed point with this many steps per cell.
pub const SUBCELL: f32 = 32.0;
/// Header size of an encoded frame in bytes.
pub const HEADER_BYTES: usize = 48;
/// Upper limit of grid changes per frame; the rest follows in later frames.
pub const MAX_CHANGES_PER_FRAME: usize = 8192;
/// Largest brush edge length of a build command in cells.
pub const MAX_BRUSH: usize = 16;

/// Encoding of person positions in a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// x, y as u16 (1/32 cell), floor u8, field u8: 6 bytes per person.
    Compact = 0,
    /// x, y as f32, floor u8, field u8, 2 bytes padding: 12 bytes per person.
    F32 = 1,
}

impl Format {
    #[must_use]
    pub fn from_u32(v: u32) -> Self {
        if v == 1 { Self::F32 } else { Self::Compact }
    }

    #[must_use]
    pub fn person_bytes(self) -> usize {
        match self {
            Self::Compact => 6,
            Self::F32 => 12,
        }
    }
}

/// What the presentation asks for.
#[derive(Clone, Copy, Debug)]
pub struct Request {
    pub format: Format,
    /// Only persons on this floor; `None` = all floors.
    pub floor: Option<u8>,
    /// Only persons inside this rectangle in cells (x0, y0, x1, y1); `None` = all.
    pub rect: Option<[f32; 4]>,
    /// Grid version the presentation already shows.
    pub known_grid_version: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub persons: usize,
    pub threads: usize,
    pub bg_threads: usize,
    pub w: usize,
    pub h: usize,
    pub gates: u8,
    /// Ticks per second; 0 = unthrottled (highest speed).
    pub hz: u32,
    pub seed: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            persons: 10_000,
            threads: 4,
            bg_threads: 1,
            w: 512,
            h: 512,
            gates: 17,
            hz: 60,
            seed: 42,
        }
    }
}

/// One person as stored in a published frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Packed {
    x: u16,
    y: u16,
    floor: u8,
    field: u8,
}

#[derive(Default)]
struct Frame {
    tick: u64,
    persons: Vec<Packed>,
    grid_version: u32,
    last_cmd: u32,
    fields_applied: u32,
    moved: u32,
}

struct Middle {
    frame: Frame,
    fresh: bool,
}

/// Grid change: version, cell index, new cell flags.
type Change = (u32, u32, u8);

#[derive(Default)]
struct RawStats {
    step_us: Vec<f64>,
    pack_us: Vec<f64>,
    ticks: u64,
    behind_ticks: u64,
    cmd_apply_us: Vec<f64>,
    recompute_ms: Vec<f64>,
    recompute_ticks: Vec<f64>,
}

/// Statistics since the last call to `Host::take_stats`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub ticks: f64,
    pub step_us_mean: f64,
    pub step_us_p99: f64,
    pub pack_us_mean: f64,
    pub behind_ticks: f64,
    /// Time from `Host::build` until the command was applied in a tick.
    pub cmd_apply_us_mean: f64,
    pub cmd_apply_us_max: f64,
    pub builds: f64,
    /// Time until persons follow new direction fields after a build.
    pub recompute_ms_mean: f64,
    pub recompute_ms_max: f64,
    pub recompute_ticks_max: f64,
    pub recomputes: f64,
}

struct Shared {
    middle: Mutex<Middle>,
    changes: Mutex<Vec<Change>>,
    stats: Mutex<RawStats>,
    running: AtomicBool,
    hz: AtomicU32,
}

struct Build {
    id: u32,
    cells: Vec<u32>,
    wall: bool,
    sent: Instant,
}

/// Simulation host: owns the simulation thread and the reader side of S-1.
pub struct Host {
    shared: Arc<Shared>,
    tx: Option<mpsc::Sender<Build>>,
    handle: Option<JoinHandle<()>>,
    front: Frame,
    persons: usize,
    w: usize,
    h: usize,
    grid_bytes: Vec<u8>,
}

impl Host {
    /// Builds the world (blocks for the initial direction fields) and starts
    /// the simulation thread.
    #[must_use]
    pub fn start(cfg: Config) -> Self {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cfg.threads.max(1))
            .build()
            .expect("thread pool");
        let grid = Grid::reference_like(cfg.w, cfg.h, cfg.gates);
        let sim = pool.install(|| {
            Sim::new(
                grid,
                cfg.persons,
                Layout::Split,
                cfg.bg_threads.max(1),
                cfg.seed,
            )
        });
        let grid_bytes = encode_grid(&sim.grid);
        let shared = Arc::new(Shared {
            middle: Mutex::new(Middle {
                frame: Frame::default(),
                fresh: false,
            }),
            changes: Mutex::new(Vec::new()),
            stats: Mutex::new(RawStats::default()),
            running: AtomicBool::new(true),
            hz: AtomicU32::new(cfg.hz),
        });
        let (tx, rx) = mpsc::channel();
        let thread_shared = Arc::clone(&shared);
        let threads = cfg.threads.max(1);
        let handle = std::thread::spawn(move || run(&thread_shared, sim, &pool, threads, &rx));
        Self {
            shared,
            tx: Some(tx),
            handle: Some(handle),
            front: Frame::default(),
            persons: cfg.persons,
            w: cfg.w,
            h: cfg.h,
            grid_bytes,
        }
    }

    #[must_use]
    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    /// Initial grid: u32 w, u32 h, u32 floors, then flags (bit 0 wall,
    /// bit 1 stairs) and zone per cell, floor-major.
    #[must_use]
    pub fn grid_bytes(&self) -> &[u8] {
        &self.grid_bytes
    }

    #[must_use]
    pub fn max_frame_bytes(&self, format: Format) -> usize {
        // A single build command may exceed the per-frame limit on its own.
        HEADER_BYTES
            + self.persons * format.person_bytes()
            + 4
            + (MAX_CHANGES_PER_FRAME + MAX_BRUSH * MAX_BRUSH) * 8
    }

    pub fn set_hz(&self, hz: u32) {
        self.shared.hz.store(hz, Ordering::Relaxed);
    }

    /// Queues a build command: a square of `size` × `size` cells with the top
    /// left corner at (x, y). Applied at the start of the next tick.
    pub fn build(&self, id: u32, x: u32, y: u32, floor: u32, size: u32, wall: bool) {
        let (w, h) = (self.w as u32, self.h as u32);
        let size = size.min(MAX_BRUSH as u32);
        let mut cells = Vec::new();
        for yy in y..(y + size).min(h - 1) {
            for xx in x..(x + size).min(w - 1) {
                if xx > 0 && yy > 0 && floor < 2 {
                    cells.push((floor * h + yy) * w + xx);
                }
            }
        }
        if let Some(tx) = &self.tx {
            let _ = tx.send(Build {
                id,
                cells,
                wall,
                sent: Instant::now(),
            });
        }
    }

    /// Encodes the newest frame into `out`; returns the number of bytes.
    /// `out` must hold at least `max_frame_bytes(req.format)` bytes.
    pub fn read_frame(&mut self, req: &Request, out: &mut [u8]) -> usize {
        {
            let mut m = self.shared.middle.lock().expect("middle lock");
            if m.fresh {
                std::mem::swap(&mut m.frame, &mut self.front);
                m.fresh = false;
            }
        }
        let t0 = Instant::now();
        let changes = self.shared.changes.lock().expect("changes lock");
        let n = encode_frame(&self.front, &changes, req, out);
        drop(changes);
        let encode_us = t0.elapsed().as_secs_f64() * 1e6;
        out[32..36].copy_from_slice(&(encode_us as f32).to_le_bytes());
        n
    }

    #[must_use]
    pub fn take_stats(&self) -> Stats {
        let raw = std::mem::take(&mut *self.shared.stats.lock().expect("stats lock"));
        summarize(raw)
    }

    pub fn stop(&mut self) {
        self.shared.running.store(false, Ordering::Release);
        self.tx = None;
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        self.stop();
    }
}

fn encode_grid(g: &Grid) -> Vec<u8> {
    let cells = g.cells();
    let mut out = Vec::with_capacity(12 + 2 * cells);
    out.extend_from_slice(&(g.w as u32).to_le_bytes());
    out.extend_from_slice(&(g.h as u32).to_le_bytes());
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend(g.flags.iter().map(|f| f & (WALL | STAIRS)));
    out.extend_from_slice(&g.zone);
    out
}

fn encode_frame(frame: &Frame, changes: &[Change], req: &Request, out: &mut [u8]) -> usize {
    let mut pos = HEADER_BYTES;
    let mut count: u32 = 0;
    let (lo, hi) = match req.rect {
        Some([x0, y0, x1, y1]) => (
            [
                (x0 * SUBCELL).max(0.0) as u16,
                (y0 * SUBCELL).max(0.0) as u16,
            ],
            [
                (x1 * SUBCELL).min(65535.0) as u16,
                (y1 * SUBCELL).min(65535.0) as u16,
            ],
        ),
        None => ([0, 0], [u16::MAX, u16::MAX]),
    };
    for p in &frame.persons {
        if req.floor.is_some_and(|f| f != p.floor)
            || p.x < lo[0]
            || p.y < lo[1]
            || p.x > hi[0]
            || p.y > hi[1]
        {
            continue;
        }
        match req.format {
            Format::Compact => {
                out[pos..pos + 2].copy_from_slice(&p.x.to_le_bytes());
                out[pos + 2..pos + 4].copy_from_slice(&p.y.to_le_bytes());
                out[pos + 4] = p.floor;
                out[pos + 5] = p.field;
                pos += 6;
            }
            Format::F32 => {
                let x = f32::from(p.x) / SUBCELL;
                let y = f32::from(p.y) / SUBCELL;
                out[pos..pos + 4].copy_from_slice(&x.to_le_bytes());
                out[pos + 4..pos + 8].copy_from_slice(&y.to_le_bytes());
                out[pos + 8] = p.floor;
                out[pos + 9] = p.field;
                out[pos + 10] = 0;
                out[pos + 11] = 0;
                pos += 12;
            }
        }
        count += 1;
    }
    pos = pos.next_multiple_of(4);
    // Grid changes the presentation does not know yet, up to this frame's
    // version and only as whole versions (one build command each).
    let mut i = changes.partition_point(|c| c.0 <= req.known_grid_version);
    let mut included_version = req.known_grid_version;
    let mut n_changes: usize = 0;
    while i < changes.len() && changes[i].0 <= frame.grid_version {
        let version = changes[i].0;
        let end = i + changes[i..].partition_point(|c| c.0 == version);
        if n_changes > 0 && n_changes + (end - i) > MAX_CHANGES_PER_FRAME {
            break;
        }
        for &(_, cell, flags) in &changes[i..end] {
            out[pos..pos + 4].copy_from_slice(&cell.to_le_bytes());
            out[pos + 4..pos + 8].copy_from_slice(&u32::from(flags).to_le_bytes());
            pos += 8;
        }
        n_changes += end - i;
        included_version = version;
        i = end;
    }
    if i == changes.len() || changes[i].0 > frame.grid_version {
        // Everything up to this frame is included (also commands without changes).
        included_version = included_version.max(frame.grid_version);
    }
    let total = frame.persons.len();
    let moved_share = if total == 0 {
        0.0
    } else {
        frame.moved as f32 / total as f32
    };
    out[0..8].copy_from_slice(&(frame.tick as f64).to_le_bytes());
    out[8..12].copy_from_slice(&count.to_le_bytes());
    out[12..16].copy_from_slice(&(total as u32).to_le_bytes());
    out[16..20].copy_from_slice(&included_version.to_le_bytes());
    out[20..24].copy_from_slice(&frame.last_cmd.to_le_bytes());
    out[24..28].copy_from_slice(&(n_changes as u32).to_le_bytes());
    out[28..32].copy_from_slice(&frame.fields_applied.to_le_bytes());
    out[32..36].copy_from_slice(&0f32.to_le_bytes());
    out[36..40].copy_from_slice(&moved_share.to_le_bytes());
    out[40..44].copy_from_slice(&(req.format as u32).to_le_bytes());
    out[44..48].copy_from_slice(&0u32.to_le_bytes());
    pos
}

fn run(
    shared: &Shared,
    mut sim: Sim,
    pool: &rayon::ThreadPool,
    threads: usize,
    rx: &mpsc::Receiver<Build>,
) {
    let mut back = Frame::default();
    let mut prev: Vec<Packed> = Vec::new();
    let mut grid_version: u32 = 0;
    let mut last_cmd: u32 = 0;
    let mut fields_applied: u32 = 0;
    let mut persons = 0usize;
    sim.visit_hot(|_| persons += 1);
    let chunk = persons.div_ceil(threads * 4).max(64);
    let mut next = Instant::now();
    while shared.running.load(Ordering::Acquire) {
        let started = Instant::now();
        // S-2: apply all queued build commands before the tick.
        while let Ok(cmd) = rx.try_recv() {
            let changed = sim.set_walls(&cmd.cells, cmd.wall);
            grid_version += 1;
            if !changed.is_empty() {
                let mut log = shared.changes.lock().expect("changes lock");
                for c in changed {
                    log.push((
                        grid_version,
                        c,
                        sim.grid.flags[c as usize] & (WALL | STAIRS),
                    ));
                }
            }
            last_cmd = cmd.id;
            shared
                .stats
                .lock()
                .expect("stats lock")
                .cmd_apply_us
                .push(cmd.sent.elapsed().as_secs_f64() * 1e6);
        }
        pool.install(|| sim.step(chunk));
        if let Some(a) = sim.poll_async() {
            fields_applied += 1;
            let mut s = shared.stats.lock().expect("stats lock");
            s.recompute_ms.push(a.ms);
            s.recompute_ticks.push(a.ticks as f64);
        }
        let step_us = started.elapsed().as_secs_f64() * 1e6;

        // Pack the frame into the back buffer.
        let t_pack = Instant::now();
        back.persons.clear();
        sim.visit_hot(|h| {
            back.persons.push(Packed {
                x: (h.x * SUBCELL) as u16,
                y: (h.y * SUBCELL) as u16,
                floor: h.floor,
                field: h.field,
            });
        });
        let moved = if prev.len() == back.persons.len() {
            prev.iter()
                .zip(&back.persons)
                .filter(|(a, b)| a != b)
                .count()
        } else {
            back.persons.len()
        };
        prev.clear();
        prev.extend_from_slice(&back.persons);
        back.tick = sim.tick;
        back.grid_version = grid_version;
        back.last_cmd = last_cmd;
        back.fields_applied = fields_applied;
        back.moved = moved as u32;
        {
            let mut m = shared.middle.lock().expect("middle lock");
            std::mem::swap(&mut m.frame, &mut back);
            m.fresh = true;
        }
        let pack_us = t_pack.elapsed().as_secs_f64() * 1e6;

        let hz = shared.hz.load(Ordering::Relaxed);
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
        s.step_us.push(step_us);
        s.pack_us.push(pack_us);
        s.ticks += 1;
        s.behind_ticks += u64::from(behind);
    }
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

fn summarize(mut s: RawStats) -> Stats {
    s.step_us.sort_by(f64::total_cmp);
    let p99 = s
        .step_us
        .get((s.step_us.len().saturating_sub(1) as f64 * 0.99) as usize)
        .copied()
        .unwrap_or(0.0);
    let max = |v: &[f64]| v.iter().copied().fold(0.0, f64::max);
    Stats {
        ticks: s.ticks as f64,
        step_us_mean: mean(&s.step_us),
        step_us_p99: p99,
        pack_us_mean: mean(&s.pack_us),
        behind_ticks: s.behind_ticks as f64,
        cmd_apply_us_mean: mean(&s.cmd_apply_us),
        cmd_apply_us_max: max(&s.cmd_apply_us),
        builds: s.cmd_apply_us.len() as f64,
        recompute_ms_mean: mean(&s.recompute_ms),
        recompute_ms_max: max(&s.recompute_ms),
        recompute_ticks_max: max(&s.recompute_ticks),
        recomputes: s.recompute_ms.len() as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header_u32(b: &[u8], off: usize) -> u32 {
        u32::from_le_bytes(b[off..off + 4].try_into().expect("4 bytes"))
    }

    fn frame_with(persons: &[(u16, u16, u8)]) -> Frame {
        Frame {
            tick: 7,
            persons: persons
                .iter()
                .map(|&(x, y, floor)| Packed {
                    x,
                    y,
                    floor,
                    field: 3,
                })
                .collect(),
            grid_version: 2,
            last_cmd: 5,
            fields_applied: 1,
            moved: 1,
        }
    }

    #[test]
    fn filters_by_floor_and_rect() {
        let f = frame_with(&[(32, 32, 0), (320, 320, 0), (32, 32, 1)]);
        let req = Request {
            format: Format::Compact,
            floor: Some(0),
            rect: Some([0.0, 0.0, 5.0, 5.0]),
            known_grid_version: 2,
        };
        let mut out = vec![0u8; 1024];
        let n = encode_frame(&f, &[], &req, &mut out);
        assert_eq!(header_u32(&out, 8), 1, "count");
        assert_eq!(header_u32(&out, 12), 3, "total");
        assert_eq!(n, HEADER_BYTES + 8, "one person, padded to 4");
        assert_eq!(&out[HEADER_BYTES..HEADER_BYTES + 6], &[32, 0, 32, 0, 0, 3]);
    }

    #[test]
    fn f32_format_decodes_cells() {
        let f = frame_with(&[(48, 16, 1)]);
        let req = Request {
            format: Format::F32,
            floor: None,
            rect: None,
            known_grid_version: 0,
        };
        let mut out = vec![0u8; 1024];
        encode_frame(&f, &[], &req, &mut out);
        let x = f32::from_le_bytes(out[48..52].try_into().expect("4"));
        let y = f32::from_le_bytes(out[52..56].try_into().expect("4"));
        assert!((x - 1.5).abs() < 1e-6 && (y - 0.5).abs() < 1e-6);
        assert_eq!(out[56], 1);
    }

    #[test]
    fn sends_only_unknown_changes_up_to_frame_version() {
        let f = frame_with(&[]);
        let changes = vec![(1, 10, 1), (2, 11, 1), (2, 12, 1), (3, 13, 1)];
        let req = Request {
            format: Format::Compact,
            floor: None,
            rect: None,
            known_grid_version: 1,
        };
        let mut out = vec![0u8; 1024];
        let n = encode_frame(&f, &changes, &req, &mut out);
        assert_eq!(header_u32(&out, 24), 2, "changes of version 2 only");
        assert_eq!(header_u32(&out, 16), 2, "included version");
        assert_eq!(header_u32(&out, n - 8), 12);
    }

    #[test]
    fn build_is_visible_in_next_frame() {
        let mut host = Host::start(Config {
            persons: 200,
            threads: 2,
            w: 128,
            h: 128,
            gates: 4,
            hz: 0,
            ..Config::default()
        });
        let req = Request {
            format: Format::Compact,
            floor: None,
            rect: None,
            known_grid_version: 0,
        };
        let mut out = vec![0u8; host.max_frame_bytes(Format::Compact)];
        host.build(9, 10, 10, 0, 2, true);
        let mut seen = false;
        for _ in 0..2000 {
            let n = host.read_frame(&req, &mut out);
            if header_u32(&out, 20) == 9 {
                assert_eq!(header_u32(&out, 24), 4, "2 x 2 cells");
                let first = header_u32(&out, n - 32);
                assert_eq!(first, 10 * 128 + 10);
                seen = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(seen, "build command never showed up");
        let stats = host.take_stats();
        assert_eq!(stats.builds, 1.0);
        host.stop();
    }
}
