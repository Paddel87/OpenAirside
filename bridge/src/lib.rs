//! napi-rs binding of the OpenAirside simulation core (ADR-009, rule 006).
//!
//! Only translates between JavaScript and the public Rust API of
//! `openairside-core` (with napi `serde-json`, ADR-024); the port logic,
//! frames and events live in `openairside_core::host`.

use std::thread;
use std::time::{Duration, Instant};

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use openairside_core::adoption::AdoptionMode;
use openairside_core::command::Scenario;
use openairside_core::geometry::MapSize;
use openairside_core::host::{Outgoing, PortHost};
use openairside_core::port::{ClientMessage, CommandEnvelope, MAX_MESSAGE_BYTES};
use openairside_core::runner::Runner;
use openairside_core::scenario::scenario_world;
use openairside_core::simulation::{SimulationConfig, default_threads};

/// Longest wait for the new session when a second port opens.
const RESTART_TIMEOUT: Duration = Duration::from_secs(30);

/// Identification of the loaded core, as seen from JavaScript.
#[napi(object)]
pub struct CoreInfo {
    /// Crate name of the core.
    pub name: String,
    /// Semantic version of the core.
    pub version: String,
}

/// Returns name and version of the simulation core.
#[napi]
#[must_use]
pub fn core_info() -> CoreInfo {
    let info = openairside_core::core_info();
    CoreInfo {
        name: info.name.to_owned(),
        version: info.version.to_owned(),
    }
}

/// Start options of the core process.
#[napi(object)]
pub struct CoreOptions {
    /// Map edge in cells: 1536 or 2048 (ADR-020); default 2048.
    pub map_size: Option<u32>,
    /// Tick threads; default from the number of cores.
    pub threads: Option<u32>,
    /// Background threads; default from the number of cores.
    pub background_threads: Option<u32>,
    /// Start scenario `openHall` or `referenceTerminal`; default `openHall`.
    pub scenario: Option<String>,
    /// Persons at the start; default 0.
    pub persons: Option<u32>,
}

/// A message for the renderer: `message` as structured object, `buf` the
/// binary part of a `frame` or table `queryResult` message.
#[napi(object)]
pub struct OutgoingMessage {
    /// Message without binary part.
    pub message: serde_json::Value,
    /// Frame (S-1) or table (S-7) buffer.
    pub buf: Option<Buffer>,
}

/// Status of the core for the main process and the debug panel.
#[napi(object)]
pub struct CoreStatus {
    /// `starting`, `ready`, `loading` or `failed`.
    pub state: String,
    /// Current session.
    pub session: u32,
    /// Current tick.
    pub tick: u32,
    /// Ticks computed in the last real second.
    pub ticks_per_second: u32,
    /// Tick threads.
    pub threads: u32,
    /// Background threads.
    pub background_threads: u32,
}

fn napi_error(message: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(message.to_string())
}

fn to_js(out: Vec<Outgoing>) -> napi::Result<Vec<OutgoingMessage>> {
    out.into_iter()
        .map(|o| match o {
            Outgoing::Message(message) => Ok(OutgoingMessage {
                message: serde_json::to_value(message).map_err(napi_error)?,
                buf: None,
            }),
            Outgoing::Binary { message, buf } => Ok(OutgoingMessage {
                message: serde_json::to_value(message).map_err(napi_error)?,
                buf: Some(buf.into()),
            }),
        })
        .collect()
}

/// The simulation core of one core process with its current port.
#[napi]
pub struct Core {
    runner: Option<Runner>,
    host: Option<PortHost>,
    threads: (usize, usize),
}

#[napi]
impl Core {
    /// Creates the simulation and starts its thread.
    ///
    /// # Errors
    ///
    /// Fails on invalid options or if the simulation cannot start.
    #[napi(constructor)]
    pub fn new(options: Option<CoreOptions>) -> napi::Result<Self> {
        let options = options.unwrap_or(CoreOptions {
            map_size: None,
            threads: None,
            background_threads: None,
            scenario: None,
            persons: None,
        });
        let edge = options.map_size.unwrap_or(2048);
        if edge != 1536 && edge != 2048 {
            return Err(napi_error("mapSize must be 1536 or 2048"));
        }
        let edge = u16::try_from(edge).map_err(napi_error)?;
        let map = MapSize::new(edge, edge).ok_or_else(|| napi_error("invalid map size"))?;
        let cores = thread::available_parallelism().map_or(1, usize::from);
        let (default_tick, default_background) = default_threads(cores);
        let as_count = |value: Option<u32>, default: usize| {
            value.map_or(Ok(default), |v| usize::try_from(v).map_err(napi_error))
        };
        let threads = as_count(options.threads, default_tick)?;
        let background_threads = as_count(options.background_threads, default_background)?;
        let scenario: Scenario = serde_json::from_value(serde_json::Value::String(
            options.scenario.unwrap_or_else(|| "openHall".to_owned()),
        ))
        .map_err(napi_error)?;
        let persons = options.persons.unwrap_or(0);
        let config = SimulationConfig {
            map,
            threads,
            background_threads,
            adoption: AdoptionMode::WhenReady,
            persons,
            seed: 0x0a1f_51de,
            start_tick: 0,
        };
        let world = scenario_world(scenario, map, persons, config.seed)
            .ok_or_else(|| napi_error("scenario does not fit on the map"))?;
        let runner = Runner::start_with_world(config, world).map_err(napi_error)?;
        Ok(Self {
            runner: Some(runner),
            host: None,
            threads: (threads, background_threads),
        })
    }

    fn runner(&self) -> napi::Result<&Runner> {
        self.runner
            .as_ref()
            .ok_or_else(|| napi_error("core stopped"))
    }

    /// Starts a new port (window loaded or reloaded). From the second port
    /// on, a new session begins (S-6 "Port-Übergabe"); returns when it is
    /// visible.
    ///
    /// # Errors
    ///
    /// Fails if the core is stopped or the new session does not start.
    #[napi]
    pub fn open_port(&mut self) -> napi::Result<()> {
        let runner = self.runner()?;
        if self.host.is_some() {
            let old = runner.status().session;
            runner.restart();
            let deadline = Instant::now() + RESTART_TIMEOUT;
            while runner.status().session == old {
                if Instant::now() >= deadline {
                    return Err(napi_error("new session did not start"));
                }
                thread::sleep(Duration::from_millis(2));
            }
        }
        self.host = Some(PortHost::new(Instant::now()));
        Ok(())
    }

    /// Handles a message from the renderer and returns the answers.
    ///
    /// # Errors
    ///
    /// Fails if no port is open or an answer cannot be converted.
    #[napi]
    pub fn receive(&mut self, message: serde_json::Value) -> napi::Result<Vec<OutgoingMessage>> {
        let runner = self
            .runner
            .as_ref()
            .ok_or_else(|| napi_error("core stopped"))?;
        let host = self.host.as_mut().ok_or_else(|| napi_error("no port"))?;
        let size = serde_json::to_vec(&message).map_or(usize::MAX, |bytes| bytes.len());
        if size > MAX_MESSAGE_BYTES {
            host.too_large();
            return Ok(Vec::new());
        }
        match serde_json::from_value::<ClientMessage>(message.clone()) {
            Ok(parsed) => to_js(host.receive(runner, parsed, Instant::now())),
            Err(_) => {
                let envelope = serde_json::from_value::<CommandEnvelope>(message).ok();
                host.receive_unparsed(runner, envelope);
                Ok(Vec::new())
            }
        }
    }

    /// Regular call (about every 50 ms): session changes and events while no
    /// frame is requested.
    ///
    /// # Errors
    ///
    /// Fails if an answer cannot be converted.
    #[napi]
    pub fn pump(&mut self) -> napi::Result<Vec<OutgoingMessage>> {
        let (Some(runner), Some(host)) = (self.runner.as_ref(), self.host.as_mut()) else {
            return Ok(Vec::new());
        };
        to_js(host.pump(runner, Instant::now()))
    }

    /// Current status.
    ///
    /// # Errors
    ///
    /// Fails if the core is stopped.
    #[napi]
    pub fn status(&self) -> napi::Result<CoreStatus> {
        let status = self.runner()?.status();
        let state = serde_json::to_value(status.state)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned))
            .unwrap_or_default();
        Ok(CoreStatus {
            state,
            session: status.session,
            tick: status.tick,
            ticks_per_second: status.ticks_per_second,
            threads: u32::try_from(self.threads.0).unwrap_or(u32::MAX),
            background_threads: u32::try_from(self.threads.1).unwrap_or(u32::MAX),
        })
    }

    /// Stops the simulation thread.
    #[napi]
    pub fn stop(&mut self) {
        self.host = None;
        if let Some(runner) = self.runner.take() {
            let _ = runner.stop();
        }
    }
}
