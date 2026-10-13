//! Spike 1.4: thin napi-rs binding of `bridge-core` (spike 1.3, unchanged) and
//! `panel-core` for the Electron utility process. No logic here.

use bridge_core::{Config, Format, Host, Request};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi(object)]
pub struct SimOptions {
    pub persons: u32,
    pub threads: u32,
    pub hz: u32,
    pub size: u32,
}

#[napi(object)]
pub struct JsStats {
    pub ticks: f64,
    pub step_us_mean: f64,
    pub step_us_p99: f64,
    pub pack_us_mean: f64,
    pub behind_ticks: f64,
    pub cmd_apply_us_mean: f64,
    pub cmd_apply_us_max: f64,
    pub builds: f64,
    pub recompute_ms_mean: f64,
    pub recompute_ms_max: f64,
    pub recompute_ticks_max: f64,
    pub recomputes: f64,
}

#[napi]
pub struct Simulation {
    host: Host,
}

#[napi]
impl Simulation {
    #[napi(constructor)]
    #[must_use]
    pub fn new(opts: SimOptions) -> Self {
        let size = opts.size.max(64) as usize;
        Self {
            host: Host::start(Config {
                persons: opts.persons as usize,
                threads: opts.threads.max(1) as usize,
                hz: opts.hz,
                w: size,
                h: size,
                ..Config::default()
            }),
        }
    }

    /// Initial grid (see `Host::grid_bytes`), copied into a new buffer.
    #[napi]
    #[must_use]
    pub fn grid(&self) -> Buffer {
        self.host.grid_bytes().to_vec().into()
    }

    #[napi]
    #[must_use]
    pub fn max_frame_bytes(&self, format: u32) -> u32 {
        self.host.max_frame_bytes(Format::from_u32(format)) as u32
    }

    /// Encodes the newest frame into `out`; returns the number of bytes.
    /// `floor < 0` = all floors; `x1 <= x0` = no camera rectangle.
    #[napi]
    #[allow(clippy::too_many_arguments)] // flat arguments avoid an object per frame
    pub fn read_frame(
        &mut self,
        mut out: Buffer,
        format: u32,
        floor: i32,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        known_grid_version: u32,
    ) -> u32 {
        let req = Request {
            format: Format::from_u32(format),
            floor: u8::try_from(floor).ok(),
            rect: (x1 > x0).then_some([x0 as f32, y0 as f32, x1 as f32, y1 as f32]),
            known_grid_version,
        };
        self.host.read_frame(&req, &mut out) as u32
    }

    /// Bytes needed for a flight table with `rows` rows (see `panel-core`).
    #[napi]
    #[must_use]
    pub fn max_table_bytes(&self, rows: u32) -> u32 {
        panel_core::max_bytes(rows as usize) as u32
    }

    /// Encodes the flight table at `tick` into `out`; returns the number of bytes.
    #[napi]
    pub fn read_table(&self, mut out: Buffer, rows: u32, tick: f64) -> u32 {
        panel_core::encode(rows as usize, tick.max(0.0) as u64, &mut out) as u32
    }

    /// Texts of the flight table (airlines, places, status names) as JSON.
    #[napi]
    #[must_use]
    pub fn table_dictionary(&self) -> String {
        panel_core::dictionary_json()
    }

    #[napi]
    pub fn build(&self, id: u32, x: u32, y: u32, floor: u32, size: u32, wall: bool) {
        self.host.build(id, x, y, floor, size, wall);
    }

    #[napi]
    pub fn set_hz(&self, hz: u32) {
        self.host.set_hz(hz);
    }

    #[napi]
    #[must_use]
    pub fn take_stats(&self) -> JsStats {
        let s = self.host.take_stats();
        JsStats {
            ticks: s.ticks,
            step_us_mean: s.step_us_mean,
            step_us_p99: s.step_us_p99,
            pack_us_mean: s.pack_us_mean,
            behind_ticks: s.behind_ticks,
            cmd_apply_us_mean: s.cmd_apply_us_mean,
            cmd_apply_us_max: s.cmd_apply_us_max,
            builds: s.builds,
            recompute_ms_mean: s.recompute_ms_mean,
            recompute_ms_max: s.recompute_ms_max,
            recompute_ticks_max: s.recompute_ticks_max,
            recomputes: s.recomputes,
        }
    }

    #[napi]
    pub fn stop(&mut self) {
        self.host.stop();
    }
}
