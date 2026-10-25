//! Real-time driver of the simulation on its own thread: paces the ticks to
//! the game speed (ADR-020), reports lag (S-3), exchanges commands,
//! acknowledgements and events with the caller and publishes a snapshot of
//! the visible state after every work slice (S-1). The bridge connects it
//! to the `MessagePort` (step 2.7). A [`SaveLink`] takes save snapshots
//! and loads save games between two work slices (S-5, step 2.5).
//!
//! Snapshot and events are published together under one short lock, as a
//! triple buffer (spike 1.3): the driver owns its back snapshot, the reader
//! its front snapshot, and both only swap with the middle one. A reader
//! therefore never gets a frame whose events are missing (S-3).

use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::command::Command;
use crate::datapack::Catalog;
use crate::event::{EventRecord, MAX_IN_FLIGHT};
use crate::frame::Snapshot;
use crate::pacer::{LagMonitor, Pacer};
use crate::savegame::{LoadedGame, SaveSnapshot};
use crate::scenario::scenario_world;
use crate::simulation::{CoreState, RecomputeStats, Simulation, SimulationConfig, SimulationError};
use crate::text::Reason;
use crate::time::{Speed, Tick};
use crate::world::World;

/// Longest work slice before the driver looks at new input again.
const SLICE: Duration = Duration::from_millis(16);
/// Wake-up interval while paused; also bounds the event latency (≤ 250 ms).
const IDLE_WAIT: Duration = Duration::from_millis(50);

enum Input {
    Command {
        session: u32,
        id: u32,
        command: Command,
    },
    Invalid {
        session: u32,
        id: u32,
    },
    Ack {
        session: u32,
        seq: u32,
    },
    Restart,
    Content(Arc<Catalog>),
    Snapshot(Sender<SaveSnapshot>),
    Load(Box<LoadedGame>, Sender<Result<(), Reason>>),
    Stop,
}

/// Longest wait of a [`SaveLink`] for the simulation thread; loading
/// computes the flow fields in use and can take seconds on large maps.
const LINK_TIMEOUT: Duration = Duration::from_secs(120);

/// Code of a save or load that the simulation thread did not answer.
pub const CORE_STOPPED: &str = "coreStopped";

/// Handle for saving and loading from another thread (S-5, step 2.5):
/// the bridge encodes and decodes files off the JavaScript thread and
/// only needs the simulation thread for the snapshot and the swap.
#[derive(Clone)]
pub struct SaveLink {
    input: Sender<Input>,
}

impl SaveLink {
    /// Takes a snapshot of the running session between two work slices.
    ///
    /// # Errors
    ///
    /// `coreStopped` if the simulation thread does not answer.
    pub fn snapshot(&self) -> Result<SaveSnapshot, Reason> {
        let (reply, answer) = mpsc::channel();
        self.input
            .send(Input::Snapshot(reply))
            .map_err(|_| stopped())?;
        answer.recv_timeout(LINK_TIMEOUT).map_err(|_| stopped())
    }

    /// Replaces the session by `game` (S-5 "Laden ist atomar"): the state
    /// reads `loading` meanwhile, commands that arrive then are rejected
    /// with `notReady`; a failed load keeps the running session.
    ///
    /// # Errors
    ///
    /// The reason of the failed load, `coreStopped` if the simulation
    /// thread does not answer.
    pub fn load(&self, game: LoadedGame) -> Result<(), Reason> {
        let (reply, answer) = mpsc::channel();
        self.input
            .send(Input::Load(Box::new(game), reply))
            .map_err(|_| stopped())?;
        answer.recv_timeout(LINK_TIMEOUT).map_err(|_| stopped())?
    }
}

fn stopped() -> Reason {
    crate::savegame::error(CORE_STOPPED)
}

/// Figures of the tick for the performance display and the measurement
/// tool (step 2.7); wall-clock values that never affect the simulated
/// state. Step times cover the ticks of the last full real second.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct TickStats {
    /// Ticks computed in the last full real second.
    pub ticks_per_second: u32,
    /// Mean time of one tick in microseconds.
    pub step_mean_us: u32,
    /// 99th percentile of the tick time in microseconds.
    pub step_p99_us: u32,
    /// Longest tick in microseconds.
    pub step_max_us: u32,
    /// Ticks due but not yet computed after the last work slice.
    pub behind_ticks: u32,
    /// A recomputation of the flow fields runs or waits for adoption.
    pub recomputing: bool,
    /// The last adopted recomputation of this session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub last_recompute: Option<RecomputeStats>,
    /// Worker threads of the tick.
    pub threads: u32,
    /// Background threads.
    pub background_threads: u32,
}

/// What the driver publishes after every slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// State of the core.
    pub state: CoreState,
    /// Current session.
    pub session: u32,
    /// Current tick.
    pub tick: Tick,
    /// Current speed.
    pub speed: Speed,
    /// Figures of the tick.
    pub stats: TickStats,
}

/// Tick times of one real second; yields mean, 99th percentile and maximum.
#[derive(Debug, Default)]
struct StepWindow {
    micros: Vec<u32>,
}

impl StepWindow {
    fn record(&mut self, took: Duration) {
        self.micros
            .push(u32::try_from(took.as_micros()).unwrap_or(u32::MAX));
    }

    /// Mean, 99th percentile and maximum in microseconds, all 0 without
    /// ticks; empties the window.
    fn finish(&mut self) -> (u32, u32, u32) {
        let mut micros = std::mem::take(&mut self.micros);
        if micros.is_empty() {
            return (0, 0, 0);
        }
        micros.sort_unstable();
        let n = micros.len();
        let sum: u64 = micros.iter().map(|&m| u64::from(m)).sum();
        let mean = u32::try_from(sum / n as u64).unwrap_or(u32::MAX);
        // Nearest rank: the smallest value with at least 99 % of the ticks at or below it.
        let rank = (n * 99).div_ceil(100).max(1);
        (mean, micros[rank - 1], micros[n - 1])
    }
}

/// Tick figures of the last full second, kept between the windows.
#[derive(Debug, Clone, Copy, Default)]
struct Measured {
    ticks_per_second: u32,
    step: (u32, u32, u32),
    behind_ticks: u32,
}

/// Events of one session, in stream order.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionEvents {
    /// Session of the events.
    pub session: u32,
    /// The events.
    pub events: Vec<EventRecord>,
}

/// Result of [`Runner::poll`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Poll {
    /// `true` if the front snapshot was replaced by a newer one.
    pub fresh: bool,
    /// Events published since the last poll, grouped by session; a session
    /// change (debug scenario) leaves the old group in front.
    pub events: Vec<SessionEvents>,
}

struct Shared {
    status: Status,
    events: Vec<SessionEvents>,
    middle: Snapshot,
    fresh: bool,
}

impl Shared {
    fn push_events(&mut self, session: u32, records: Vec<EventRecord>) {
        if records.is_empty() {
            return;
        }
        match self.events.last_mut() {
            Some(group) if group.session == session => group.events.extend(records),
            _ => self.events.push(SessionEvents {
                session,
                events: records,
            }),
        }
    }
}

fn lock(shared: &Mutex<Shared>) -> MutexGuard<'_, Shared> {
    shared.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Marks the core as failed if the driver thread ends by a panic.
struct FailGuard(Arc<Mutex<Shared>>);

impl Drop for FailGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            lock(&self.0).status.state = CoreState::Failed;
        }
    }
}

/// Handle to the simulation thread.
pub struct Runner {
    input: Sender<Input>,
    shared: Arc<Mutex<Shared>>,
    thread: Option<JoinHandle<()>>,
}

impl Runner {
    /// Creates the simulation and starts its thread.
    ///
    /// # Errors
    ///
    /// Fails if the simulation cannot be created.
    pub fn start(config: SimulationConfig) -> Result<Self, SimulationError> {
        Self::spawn(Simulation::new(config)?)
    }

    /// Creates the simulation in `world` (flow fields are computed before
    /// the thread starts) and starts its thread.
    ///
    /// # Errors
    ///
    /// Fails if the simulation cannot be created.
    pub fn start_with_world(
        config: SimulationConfig,
        world: World,
    ) -> Result<Self, SimulationError> {
        Self::spawn(Simulation::with_world(config, world)?)
    }

    /// Like [`Runner::start_with_world`], with the content of the loaded
    /// data packs (S-4) from the first session on.
    ///
    /// # Errors
    ///
    /// Fails if the simulation cannot be created.
    pub fn start_with_content(
        config: SimulationConfig,
        world: World,
        content: Arc<Catalog>,
    ) -> Result<Self, SimulationError> {
        Self::spawn(Simulation::with_world(config, world)?.with_content(content))
    }

    fn spawn(sim: Simulation) -> Result<Self, SimulationError> {
        let mut middle = Snapshot::default();
        middle.capture(sim.session(), sim.tick(), sim.world(), sim.content_arc());
        let shared = Arc::new(Mutex::new(Shared {
            status: status_of(&sim, Measured::default()),
            events: Vec::new(),
            middle,
            fresh: true,
        }));
        let (input, inbox) = mpsc::channel();
        let thread_shared = Arc::clone(&shared);
        let thread = thread::Builder::new()
            .name("sim-driver".into())
            .spawn(move || drive(sim, &inbox, &thread_shared))
            .map_err(|err| SimulationError::ThreadPool(err.to_string()))?;
        Ok(Self {
            input,
            shared,
            thread: Some(thread),
        })
    }

    /// Sends a command (S-2).
    pub fn command(&self, session: u32, id: u32, command: Command) {
        let _ = self.input.send(Input::Command {
            session,
            id,
            command,
        });
    }

    /// Passes a command whose fields could not be read; it is answered with
    /// `invalidArgument` (S-2).
    pub fn invalid_command(&self, session: u32, id: u32) {
        let _ = self.input.send(Input::Invalid { session, id });
    }

    /// Acknowledges events up to `seq` (S-3).
    pub fn ack(&self, session: u32, seq: u32) {
        let _ = self.input.send(Input::Ack { session, seq });
    }

    /// Starts a new session.
    pub fn restart(&self) {
        let _ = self.input.send(Input::Restart);
    }

    /// Uses `content` from the next session on (`packs.setActive`, S-6).
    pub fn set_next_content(&self, content: Arc<Catalog>) {
        let _ = self.input.send(Input::Content(content));
    }

    /// Handle for saving and loading from another thread.
    #[must_use]
    pub fn save_link(&self) -> SaveLink {
        SaveLink {
            input: self.input.clone(),
        }
    }

    /// Takes the events published so far, of all sessions.
    #[must_use]
    pub fn take_events(&self) -> Vec<EventRecord> {
        std::mem::take(&mut lock(&self.shared).events)
            .into_iter()
            .flat_map(|group| group.events)
            .collect()
    }

    /// Swaps `front` with the newest published snapshot, if there is a newer
    /// one, and takes the events published so far, both under one lock.
    pub fn poll(&self, front: &mut Snapshot) -> Poll {
        let mut shared = lock(&self.shared);
        let fresh = shared.fresh;
        if fresh {
            std::mem::swap(&mut shared.middle, front);
            shared.fresh = false;
        }
        Poll {
            fresh,
            events: std::mem::take(&mut shared.events),
        }
    }

    /// Latest published status.
    #[must_use]
    pub fn status(&self) -> Status {
        lock(&self.shared).status
    }

    /// Stops the thread and waits for it.
    ///
    /// # Errors
    ///
    /// Returns an error if the thread ended by a panic.
    pub fn stop(mut self) -> thread::Result<()> {
        let _ = self.input.send(Input::Stop);
        self.thread.take().map_or(Ok(()), JoinHandle::join)
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = self.input.send(Input::Stop);
            let _ = thread.join();
        }
    }
}

fn status_of(sim: &Simulation, measured: Measured) -> Status {
    let config = sim.config();
    let (step_mean_us, step_p99_us, step_max_us) = measured.step;
    Status {
        state: sim.state(),
        session: sim.session(),
        tick: sim.tick(),
        speed: sim.speed(),
        stats: TickStats {
            ticks_per_second: measured.ticks_per_second,
            step_mean_us,
            step_p99_us,
            step_max_us,
            behind_ticks: measured.behind_ticks,
            recomputing: sim.is_recomputing(),
            last_recompute: sim.last_recompute(),
            threads: u32::try_from(config.threads).unwrap_or(u32::MAX),
            background_threads: u32::try_from(config.background_threads).unwrap_or(u32::MAX),
        },
    }
}

/// What the driver does after an input.
enum Applied {
    Continue,
    Load(Box<LoadedGame>, Sender<Result<(), Reason>>),
    Stop,
}

/// Applies one input; loading is left to the driver.
fn apply(sim: &mut Simulation, input: Input, shared: &Mutex<Shared>) -> Applied {
    match input {
        Input::Command {
            session,
            id,
            command,
        } => sim.submit(session, id, command),
        Input::Invalid { session, id } => sim.submit_invalid(session, id),
        Input::Ack { session, seq } => sim.ack(session, seq),
        Input::Restart => {
            sim.restart();
            lock(shared).events.clear();
        }
        Input::Content(content) => sim.set_next_content(content),
        Input::Snapshot(reply) => {
            let _ = reply.send(sim.save_snapshot());
        }
        Input::Load(game, reply) => return Applied::Load(game, reply),
        Input::Stop => return Applied::Stop,
    }
    Applied::Continue
}

/// Result of [`apply_all`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flow {
    /// Go on; `loaded` if a save game was loaded (successfully or not),
    /// which took real time the pacer must not catch up on.
    Run {
        loaded: bool,
    },
    Stop,
}

/// Applies the inputs waiting in `inbox` after `first`.
fn apply_all(
    sim: &mut Simulation,
    first: Input,
    inbox: &Receiver<Input>,
    back: &mut Snapshot,
    shared: &Mutex<Shared>,
) -> Flow {
    let mut loaded = false;
    let mut next = Some(first);
    while let Some(input) = next.take().or_else(|| inbox.try_recv().ok()) {
        match apply(sim, input, shared) {
            Applied::Continue => {}
            Applied::Stop => return Flow::Stop,
            Applied::Load(game, reply) => {
                loaded = true;
                if !load_game(sim, *game, &reply, inbox, back, shared) {
                    return Flow::Stop;
                }
            }
        }
    }
    Flow::Run { loaded }
}

/// Loads a save game (S-5): the state reads `loading` while the flow
/// fields in use are computed; inputs that arrived meanwhile are applied
/// afterwards, their commands answered with `notReady` (a successful load
/// discards them as commands of the old session). Returns `false` on stop.
fn load_game(
    sim: &mut Simulation,
    game: LoadedGame,
    reply: &Sender<Result<(), Reason>>,
    inbox: &Receiver<Input>,
    back: &mut Snapshot,
    shared: &Mutex<Shared>,
) -> bool {
    sim.begin_loading();
    lock(shared).status.state = CoreState::Loading;
    let result = sim.load_game(game);
    let mut running = true;
    while let Ok(input) = inbox.try_recv() {
        match apply(sim, input, shared) {
            Applied::Continue => {}
            Applied::Stop => running = false,
            Applied::Load(_, other) => {
                let _ = other.send(Err(crate::savegame::error("busy")));
            }
        }
    }
    sim.reject_waiting();
    sim.end_loading();
    // Published first: whoever gets the answer sees the new status.
    publish(sim, back, shared, Measured::default());
    let _ = reply.send(result);
    running
}

fn drive(mut sim: Simulation, inbox: &Receiver<Input>, shared: &Arc<Mutex<Shared>>) {
    let _guard = FailGuard(Arc::clone(shared));
    let origin = Instant::now();
    let mut last = origin;
    let mut paced = sim.speed();
    let mut pacer = Pacer::new(paced);
    let mut lag = LagMonitor::new();
    let mut window_start = origin;
    let mut window_ticks = 0u32;
    let mut steps = StepWindow::default();
    let mut measured = Measured::default();
    let mut back = Snapshot::default();
    loop {
        let wait = pacer.until_next().unwrap_or(IDLE_WAIT).min(IDLE_WAIT);
        match inbox.recv_timeout(wait) {
            Ok(input) => match apply_all(&mut sim, input, inbox, &mut back, shared) {
                Flow::Stop => return,
                Flow::Run { loaded: true } => {
                    // The load took real time; the loaded game starts afresh.
                    paced = sim.speed();
                    pacer = Pacer::new(paced);
                    lag = LagMonitor::new();
                    last = Instant::now();
                }
                Flow::Run { loaded: false } => {}
            },
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return,
        }

        let now = Instant::now();
        pacer.elapse(now - last);
        last = now;
        if sim.speed().is_running() {
            while pacer.due() > 0 && sim.speed().is_running() {
                let before = sim.tick();
                let started = Instant::now();
                sim.step();
                if sim.tick() != before {
                    steps.record(started.elapsed());
                    pacer.complete_one();
                    window_ticks += 1;
                }
                if now.elapsed() >= SLICE || sim.is_backpressured() {
                    break;
                }
            }
        } else {
            // Paused: commands are still processed (S-2).
            sim.step();
        }
        if sim.speed() != paced {
            paced = sim.speed();
            pacer.set_speed(paced);
        }

        let now = Instant::now();
        if now - window_start >= Duration::from_secs(1) {
            measured.ticks_per_second = window_ticks;
            measured.step = steps.finish();
            window_ticks = 0;
            window_start = now;
        }
        measured.behind_ticks = if sim.speed().is_running() {
            pacer.due()
        } else {
            0
        };
        let lag_event = if sim.speed().is_running() {
            lag.observe(now - origin, pacer.due(), measured.ticks_per_second)
        } else {
            lag.reset()
        };
        if let Some(event) = lag_event {
            sim.report(event);
        }

        publish(&mut sim, &mut back, shared, measured);
        if let Some((scenario, persons)) = sim.take_scenario() {
            load_scenario(&mut sim, scenario, persons, &mut back, shared);
        }
    }
}

/// Publishes the events and a snapshot of the current state.
fn publish(sim: &mut Simulation, back: &mut Snapshot, shared: &Mutex<Shared>, measured: Measured) {
    let records = sim.take_events(MAX_IN_FLIGHT as usize);
    back.capture(sim.session(), sim.tick(), sim.world(), sim.content_arc());
    let mut out = lock(shared);
    out.push_events(sim.session(), records);
    std::mem::swap(&mut out.middle, back);
    out.fresh = true;
    out.status = status_of(sim, measured);
}

/// Starts a new session with a debug scenario after the answer to
/// `loadScenario` went out; the state reads `loading` meanwhile.
fn load_scenario(
    sim: &mut Simulation,
    scenario: crate::command::Scenario,
    persons: u32,
    back: &mut Snapshot,
    shared: &Mutex<Shared>,
) {
    lock(shared).status.state = CoreState::Loading;
    let config = sim.config();
    // Checked by the command (`fits_reference_terminal`); an impossible
    // scenario keeps the old session.
    if let Some(world) = scenario_world(scenario, config.map, persons, config.seed) {
        sim.restart_with_world(world);
    }
    publish(sim, back, shared, Measured::default());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adoption::AdoptionMode;
    use crate::event::Event;
    use crate::geometry::MapSize;

    fn runner() -> Runner {
        Runner::start(SimulationConfig {
            map: MapSize::new(64, 64).unwrap(),
            threads: 1,
            background_threads: 1,
            adoption: AdoptionMode::WhenReady,
            persons: 100,
            seed: 3,
            start_tick: 0,
        })
        .unwrap()
    }

    /// Collects events until `done` holds or five seconds pass.
    fn wait_for(
        runner: &Runner,
        seen: &mut Vec<EventRecord>,
        done: impl Fn(&[EventRecord], Status) -> bool,
    ) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let batch = runner.take_events();
            if let Some(last) = batch.last() {
                runner.ack(runner.status().session, last.seq);
            }
            seen.extend(batch);
            if done(seen, runner.status()) {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "timed out; events so far: {seen:?}"
            );
            thread::sleep(Duration::from_millis(5));
        }
    }

    fn answered(id: u32) -> impl Fn(&[EventRecord], Status) -> bool {
        move |seen, _| {
            seen.iter()
                .any(|r| matches!(r.event, Event::CommandAccepted { id: got, .. } if got == id))
        }
    }

    #[test]
    fn runs_at_game_speed_and_answers_commands() {
        let runner = runner();
        let mut seen = Vec::new();
        assert_eq!(runner.status().state, CoreState::Ready);
        runner.command(1, 1, Command::SetSpeed { speed: Speed::X3 });
        wait_for(&runner, &mut seen, |seen, status| {
            answered(1)(seen, status) && status.tick >= 30
        });
        assert!(seen.iter().any(|r| r.event
            == Event::SpeedChanged {
                speed: Speed::X3,
                reason: None
            }));
        let seqs: Vec<u32> = seen.iter().map(|r| r.seq).collect();
        assert_eq!(
            seqs,
            (1..=u32::try_from(seqs.len()).unwrap()).collect::<Vec<_>>()
        );

        runner.command(
            1,
            2,
            Command::SetSpeed {
                speed: Speed::Paused,
            },
        );
        wait_for(&runner, &mut seen, answered(2));
        let frozen = runner.status().tick;
        thread::sleep(Duration::from_millis(120));
        assert_eq!(runner.status().tick, frozen, "paused game does not tick");
        assert_eq!(runner.status().speed, Speed::Paused);

        // Commands are answered in the pause as well.
        runner.command(1, 3, Command::SkipNight {});
        wait_for(&runner, &mut seen, |seen, _| {
            seen.iter()
                .any(|r| matches!(r.event, Event::CommandRejected { id: 3, .. }))
        });
        runner.stop().unwrap();
    }

    #[test]
    fn new_content_applies_from_the_next_session_on() {
        let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/base");
        let folders = crate::datapack::discover(Some(&base), None);
        let loaded = crate::datapack::load(&folders, &[], &crate::datapack::game_version());
        let config = SimulationConfig {
            map: MapSize::new(64, 64).unwrap(),
            threads: 1,
            background_threads: 1,
            adoption: AdoptionMode::WhenReady,
            persons: 0,
            seed: 3,
            start_tick: 0,
        };
        let world = World::open_hall(config.map, 0, config.seed);
        let runner = Runner::start_with_content(config, world, Arc::new(Catalog::empty())).unwrap();
        let bench = || Command::Build {
            object: "base:bench".into(),
            at: crate::geometry::CellPos {
                x: 1,
                y: 1,
                floor: 0,
            },
            rotation: crate::geometry::Rotation(0),
        };
        let mut seen = Vec::new();
        runner.command(1, 1, bench());
        wait_for(&runner, &mut seen, |seen, _| {
            seen.iter()
                .any(|r| matches!(r.event, Event::CommandRejected { id: 1, .. }))
        });
        runner.set_next_content(Arc::new(loaded.catalog));
        runner.restart();
        let mut fresh = Vec::new();
        wait_for(&runner, &mut fresh, |_, status| status.session == 2);
        runner.command(2, 1, bench());
        wait_for(&runner, &mut fresh, answered(1));
        runner.stop().unwrap();
    }

    #[test]
    fn restart_begins_a_new_session() {
        let runner = runner();
        runner.command(1, 1, Command::SetSpeed { speed: Speed::X1 });
        let mut seen = Vec::new();
        wait_for(&runner, &mut seen, answered(1));
        runner.restart();
        let mut fresh = Vec::new();
        wait_for(&runner, &mut fresh, |_, status| status.session == 2);
        let status = runner.status();
        assert_eq!((status.tick, status.speed), (0, Speed::Paused));
        // Old session ids are discarded, ids restart at 1.
        runner.command(1, 5, Command::SetSpeed { speed: Speed::X2 });
        runner.command(2, 1, Command::SetSpeed { speed: Speed::X2 });
        wait_for(&runner, &mut fresh, answered(1));
        assert_eq!(fresh.first().map(|r| r.seq), Some(1));
        drop(runner); // stops the thread
    }

    #[test]
    fn poll_hands_out_snapshots_with_their_events() {
        let runner = runner();
        let mut front = Snapshot::default();
        let first = runner.poll(&mut front);
        assert!(first.fresh, "the start state is published");
        assert_eq!((front.session(), front.persons()), (1, 100));
        runner.command(1, 1, Command::SetSpeed { speed: Speed::X3 });
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut seen = Vec::new();
        while front.tick() < 10 {
            let poll = runner.poll(&mut front);
            for group in poll.events {
                assert_eq!(group.session, 1);
                seen.extend(group.events);
            }
            if let Some(last) = seen.last() {
                runner.ack(1, last.seq);
            }
            assert!(Instant::now() < deadline, "no frames");
            thread::sleep(Duration::from_millis(5));
        }
        // Every event handed out so far belongs to a tick the reader has seen.
        assert!(seen.iter().all(|r| r.tick <= front.tick()));
        assert!(
            seen.iter()
                .any(|r| matches!(r.event, Event::CommandAccepted { id: 1, .. }))
        );
        runner.stop().unwrap();
    }

    #[test]
    fn unreadable_command_is_answered_with_invalid_argument() {
        let runner = runner();
        runner.invalid_command(1, 1);
        let mut seen = Vec::new();
        wait_for(&runner, &mut seen, |seen, _| {
            seen.iter().any(|r| {
                matches!(&r.event, Event::CommandRejected { id: 1, reason, .. }
                    if reason.code == "invalidArgument")
            })
        });
        runner.stop().unwrap();
    }

    #[test]
    fn debug_scenario_starts_a_new_session_after_the_answer() {
        use crate::command::{DebugAction, Scenario};
        let runner = runner();
        runner.command(
            1,
            1,
            Command::Debug {
                action: DebugAction::LoadScenario {
                    scenario: Scenario::OpenHall,
                    persons: 7,
                },
            },
        );
        let mut front = Snapshot::default();
        let mut groups = Vec::new();
        let deadline = Instant::now() + Duration::from_secs(5);
        while front.session() != 2 {
            groups.extend(runner.poll(&mut front).events);
            assert!(Instant::now() < deadline, "no new session");
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(front.persons(), 7);
        let old = groups.iter().find(|g| g.session == 1).expect("old group");
        assert!(
            old.events
                .iter()
                .any(|r| matches!(r.event, Event::CommandAccepted { id: 1, .. }))
        );
        assert_eq!(runner.status().state, CoreState::Ready);
        runner.stop().unwrap();
    }

    fn save_request() -> crate::savegame::SaveRequest {
        crate::savegame::SaveRequest {
            name: "Test".into(),
            kind: crate::savegame::SaveKind::Manual,
            saved_at: "2026-10-19T12:00:00Z".into(),
            preview: Vec::new(),
        }
    }

    fn limits() -> crate::savegame::Limits {
        crate::savegame::Limits { map_edges: &[64] }
    }

    #[test]
    fn save_link_saves_and_loads_a_new_session() {
        let runner = runner();
        let link = runner.save_link();
        let mut seen = Vec::new();
        runner.command(1, 1, Command::SetSpeed { speed: Speed::X3 });
        wait_for(&runner, &mut seen, |_, status| status.tick >= 20);
        let snapshot = link.snapshot().unwrap();
        assert!(snapshot.tick() >= 20);
        let bytes = snapshot.encode(&save_request(), "", "0.0.0").unwrap();
        let game = crate::savegame::decode(&bytes, &limits()).unwrap();
        link.load(game).unwrap();
        let status = runner.status();
        assert_eq!((status.session, status.state), (2, CoreState::Ready));
        // The saved speed goes on; the tick continues from the saved one.
        assert_eq!(status.speed, Speed::X3);
        assert!(status.tick >= snapshot.tick());
        let mut fresh = Vec::new();
        runner.command(
            2,
            1,
            Command::SetSpeed {
                speed: Speed::Paused,
            },
        );
        wait_for(&runner, &mut fresh, answered(1));
        runner.stop().unwrap();
    }

    #[test]
    fn a_failed_load_keeps_the_session() {
        let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/base");
        let folders = crate::datapack::discover(Some(&base), None);
        let loaded = crate::datapack::load(&folders, &[], &crate::datapack::game_version());
        let config = SimulationConfig {
            map: MapSize::new(64, 64).unwrap(),
            threads: 1,
            background_threads: 1,
            adoption: AdoptionMode::WhenReady,
            persons: 10,
            seed: 3,
            start_tick: 0,
        };
        let with_base = Runner::start_with_content(
            config,
            World::open_hall(config.map, 10, 3),
            Arc::new(loaded.catalog),
        )
        .unwrap();
        let bytes = with_base
            .save_link()
            .snapshot()
            .unwrap()
            .encode(&save_request(), "", "0.0.0")
            .unwrap();
        with_base.stop().unwrap();

        // Without the base pack the game cannot be loaded (`packMissing`).
        let runner = runner();
        let game = crate::savegame::decode(&bytes, &limits()).unwrap();
        let reason = runner.save_link().load(game).unwrap_err();
        assert_eq!(reason.code, "packMissing");
        let status = runner.status();
        assert_eq!((status.session, status.state), (1, CoreState::Ready));
        let mut seen = Vec::new();
        runner.command(1, 1, Command::SetSpeed { speed: Speed::X1 });
        wait_for(&runner, &mut seen, answered(1));
        runner.stop().unwrap();
    }

    #[test]
    fn a_stopped_runner_answers_the_link_with_core_stopped() {
        let runner = runner();
        let link = runner.save_link();
        runner.stop().unwrap();
        let reason = link.snapshot().err().unwrap();
        assert_eq!(reason.code, CORE_STOPPED);
        assert_eq!(reason.text_key, "save.error.coreStopped");
    }

    #[test]
    fn step_window_reports_mean_percentile_and_maximum() {
        let mut window = StepWindow::default();
        assert_eq!(window.finish(), (0, 0, 0));
        for micros in 1..=200 {
            window.record(Duration::from_micros(micros));
        }
        // 200 ticks: the 99th percentile is the 198th value.
        assert_eq!(window.finish(), (100, 198, 200));
        assert_eq!(window.finish(), (0, 0, 0), "finish empties the window");
        window.record(Duration::from_micros(7));
        assert_eq!(window.finish(), (7, 7, 7));
    }

    #[test]
    fn running_driver_reports_tick_figures() {
        let runner = runner();
        let mut seen = Vec::new();
        runner.command(1, 1, Command::SetSpeed { speed: Speed::X3 });
        wait_for(&runner, &mut seen, |_, status| {
            status.stats.ticks_per_second > 0
        });
        let stats = runner.status().stats;
        assert!(stats.step_mean_us <= stats.step_p99_us);
        assert!(stats.step_p99_us <= stats.step_max_us);
        assert!(stats.step_max_us > 0);
        assert_eq!((stats.threads, stats.background_threads), (1, 1));
        assert!(!stats.recomputing);
        assert_eq!(stats.last_recompute, None);
        runner.stop().unwrap();
    }

    #[test]
    fn start_with_world_uses_the_given_world() {
        let config = SimulationConfig {
            map: MapSize::new(64, 64).unwrap(),
            threads: 1,
            background_threads: 1,
            adoption: AdoptionMode::WhenReady,
            persons: 0,
            seed: 3,
            start_tick: 0,
        };
        let world = World::open_hall(config.map, 5, 1);
        let runner = Runner::start_with_world(config, world).unwrap();
        let mut front = Snapshot::default();
        assert!(runner.poll(&mut front).fresh);
        assert_eq!(front.persons(), 5);
        let zero = SimulationConfig {
            threads: 0,
            ..config
        };
        assert!(Runner::start_with_world(zero, World::new(config.map)).is_err());
    }

    #[test]
    fn panicking_driver_marks_the_core_as_failed() {
        let shared = Arc::new(Mutex::new(Shared {
            status: Status {
                state: CoreState::Ready,
                session: 1,
                tick: 0,
                speed: Speed::Paused,
                stats: TickStats::default(),
            },
            events: Vec::new(),
            middle: Snapshot::default(),
            fresh: false,
        }));
        let guard_shared = Arc::clone(&shared);
        let result = thread::spawn(move || {
            let _guard = FailGuard(guard_shared);
            panic!("simulated failure");
        })
        .join();
        assert!(result.is_err());
        assert_eq!(lock(&shared).status.state, CoreState::Failed);
    }
}
