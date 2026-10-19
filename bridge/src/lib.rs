//! napi-rs binding of the OpenAirside simulation core (ADR-009, rule 006).
//!
//! Only translates between JavaScript and the public Rust API of
//! `openairside-core` (with napi `serde-json`, ADR-024); the port logic,
//! frames and events live in `openairside_core::host`. Save games (S-5,
//! step 2.5) are encoded and checked off the JavaScript thread; a failed
//! save or load rejects with the reason as JSON text in the message, so
//! that the core host can pass on `{ code, textKey, params }` (S-6).

use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use napi::bindgen_prelude::{AsyncTask, Buffer};
use napi::{Env, Task};
use napi_derive::napi;
use openairside_core::adoption::AdoptionMode;
use openairside_core::command::Scenario;
use openairside_core::datapack::{self, PackReport};
use openairside_core::geometry::MapSize;
use openairside_core::host::{Outgoing, PortHost};
use openairside_core::port::{ClientMessage, CommandEnvelope, MAX_MESSAGE_BYTES};
use openairside_core::runner::{Runner, SaveLink};
use openairside_core::savegame::{self, SaveKind, SaveRequest};
use openairside_core::scenario::scenario_world;
use openairside_core::simulation::{SimulationConfig, default_threads};
use openairside_core::text::Reason;

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
    /// Folder of the base pack (S-4); without it no content is loaded.
    pub base_pack: Option<String>,
    /// Folder of the player's packs, one sub-folder per pack.
    pub user_packs: Option<String>,
    /// Ids of the player's active packs in the chosen order.
    pub active_packs: Option<Vec<String>>,
}

/// Where the core looks for data packs.
struct PackDirs {
    base: Option<PathBuf>,
    user: Option<PathBuf>,
}

impl PackDirs {
    /// Loads the base pack and the active user packs (S-4).
    fn load(&self, active: &[String]) -> datapack::LoadOutcome {
        let folders = datapack::discover(self.base.as_deref(), self.user.as_deref());
        datapack::load(&folders, active, &datapack::game_version())
    }
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
    /// The game runs (not paused); counts towards the autosave interval.
    pub running: bool,
}

fn napi_error(message: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(message.to_string())
}

/// Error whose message is `reason` as JSON (read by app/src/main/core-host.ts).
fn reason_error(reason: &Reason) -> napi::Error {
    napi_error(serde_json::to_string(reason).unwrap_or_else(|_| reason.code.clone()))
}

/// Encodes a save game on a libuv thread (S-6 `saves.save`).
pub struct SaveTask {
    link: SaveLink,
    request: SaveRequest,
}

impl Task for SaveTask {
    type Output = Vec<u8>;
    type JsValue = Buffer;

    fn compute(&mut self) -> napi::Result<Vec<u8>> {
        let snapshot = self.link.snapshot().map_err(|r| reason_error(&r))?;
        let version = openairside_core::core_info().version;
        // TODO(fahrplan-ref: 5.1): the airport name enters the world state
        // with the new-game settings (META-02); until then it is empty.
        snapshot
            .encode(&self.request, "", version)
            .map_err(|r| reason_error(&r))
    }

    fn resolve(&mut self, _env: Env, output: Vec<u8>) -> napi::Result<Buffer> {
        Ok(output.into())
    }
}

/// Checks and loads a save game on a libuv thread (S-6 `saves.load`).
pub struct LoadTask {
    link: SaveLink,
    bytes: Vec<u8>,
}

impl Task for LoadTask {
    /// Metadata of the loaded game as JSON text.
    type Output = String;
    type JsValue = String;

    fn compute(&mut self) -> napi::Result<String> {
        let bytes = std::mem::take(&mut self.bytes);
        let game =
            savegame::decode(&bytes, &savegame::Limits::default()).map_err(|r| reason_error(&r))?;
        drop(bytes);
        let meta = serde_json::to_string(game.meta()).map_err(napi_error)?;
        self.link.load(game).map_err(|r| reason_error(&r))?;
        Ok(meta)
    }

    fn resolve(&mut self, _env: Env, output: String) -> napi::Result<String> {
        Ok(output)
    }
}

/// Reads header and metadata of a save game for the list (S-5): `prefix`
/// is the start of the file, `file_len` its length. Returns
/// `{ gameVersion, meta, previewOffset, previewLen }`.
///
/// # Errors
///
/// The reason as JSON for a damaged file.
#[napi]
pub fn save_info(prefix: Buffer, file_len: f64) -> napi::Result<serde_json::Value> {
    if !(0.0..=9_007_199_254_740_991.0).contains(&file_len) || file_len.fract() != 0.0 {
        return Err(reason_error(&savegame::error(
            savegame::code::INVALID_ARGUMENT,
        )));
    }
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "checked above: a non-negative whole number below 2^53"
    )]
    let file_len = file_len as u64;
    let info = savegame::read_info(&prefix, file_len).map_err(|r| reason_error(&r))?;
    Ok(serde_json::json!({
        "gameVersion": info.game_version,
        "meta": info.meta,
        "previewOffset": info.preview_offset,
        "previewLen": info.preview_len,
    }))
}

/// Checks a preview image as loading does (S-5: PNG, at most 1920 × 1080).
///
/// # Errors
///
/// The reason as JSON for an image that is not allowed.
#[napi]
pub fn check_preview(png: Buffer) -> napi::Result<()> {
    if png.len() > savegame::MAX_PREVIEW_BYTES as usize {
        return Err(reason_error(
            &savegame::error(savegame::code::LIMIT_EXCEEDED).with_text("limit", "preview"),
        ));
    }
    savegame::check_preview(&png).map_err(|r| reason_error(&r))
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
    pack_dirs: PackDirs,
    packs: Vec<PackReport>,
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
            base_pack: None,
            user_packs: None,
            active_packs: None,
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
        let pack_dirs = PackDirs {
            base: options.base_pack.map(PathBuf::from),
            user: options.user_packs.map(PathBuf::from),
        };
        let active = options.active_packs.unwrap_or_default();
        // An invalid saved choice loads only the base pack.
        let active = if datapack::check_active(&active).is_ok() {
            active
        } else {
            Vec::new()
        };
        let loaded = pack_dirs.load(&active);
        let runner = Runner::start_with_content(config, world, Arc::new(loaded.catalog))
            .map_err(napi_error)?;
        Ok(Self {
            runner: Some(runner),
            host: None,
            threads: (threads, background_threads),
            pack_dirs,
            packs: loaded.reports,
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

    /// Found data packs with state and errors (S-6 `packs.list`): the
    /// choice that applies from the next session on.
    ///
    /// # Errors
    ///
    /// Fails if a report cannot be converted.
    #[napi]
    pub fn packs(&self) -> napi::Result<serde_json::Value> {
        serde_json::to_value(&self.packs).map_err(napi_error)
    }

    /// Chooses the active user packs in load order (S-6 `packs.setActive`),
    /// reloads all packs and uses the result from the next session on.
    /// Returns the new reports.
    ///
    /// # Errors
    ///
    /// Fails on invalid ids (code in the message) or if the core is stopped.
    #[napi]
    pub fn set_active_packs(&mut self, ids: Vec<String>) -> napi::Result<serde_json::Value> {
        datapack::check_active(&ids).map_err(|reason| napi_error(reason.code))?;
        let loaded = self.pack_dirs.load(&ids);
        self.runner()?.set_next_content(Arc::new(loaded.catalog));
        self.packs = loaded.reports;
        self.packs()
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
            ticks_per_second: status.stats.ticks_per_second,
            threads: u32::try_from(self.threads.0).unwrap_or(u32::MAX),
            background_threads: u32::try_from(self.threads.1).unwrap_or(u32::MAX),
            running: status.speed.is_running(),
        })
    }

    /// Saves the running session (S-6 `saves.save`): resolves with the
    /// complete file. `kind` is `manual`, `quick` or `auto`; `saved_at` an
    /// ISO 8601 time; `preview` a PNG or empty.
    ///
    /// # Errors
    ///
    /// Fails with the reason as JSON for an invalid request or if the core
    /// is stopped.
    #[napi(ts_return_type = "Promise<Buffer>")]
    pub fn save_game(
        &self,
        name: String,
        kind: String,
        saved_at: String,
        preview: Buffer,
    ) -> napi::Result<AsyncTask<SaveTask>> {
        let kind: SaveKind =
            serde_json::from_value(serde_json::Value::String(kind)).map_err(|_| {
                reason_error(
                    &savegame::error(savegame::code::INVALID_ARGUMENT).with_text("field", "kind"),
                )
            })?;
        let link = self.runner()?.save_link();
        Ok(AsyncTask::new(SaveTask {
            link,
            request: SaveRequest {
                name,
                kind,
                saved_at,
                preview: preview.to_vec(),
            },
        }))
    }

    /// Checks a save game file and replaces the session by it (S-6
    /// `saves.load`); resolves with its metadata as JSON text once the new
    /// session runs.
    ///
    /// # Errors
    ///
    /// Fails with the reason as JSON if the core is stopped; the checks
    /// reject the promise.
    #[napi(ts_return_type = "Promise<string>")]
    pub fn load_game(&self, bytes: Buffer) -> napi::Result<AsyncTask<LoadTask>> {
        let link = self.runner()?.save_link();
        Ok(AsyncTask::new(LoadTask {
            link,
            bytes: bytes.to_vec(),
        }))
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
