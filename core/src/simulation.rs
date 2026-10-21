//! Tick of the simulation core: sessions, core states, command processing
//! with exactly one answer per command (S-2), event stream with flow control
//! (S-3), time control (ADR-020) and adoption of background results with a
//! verification mode (ADR-010).

use std::collections::VecDeque;
use std::fmt;
use std::sync::Arc;

use rayon::ThreadPool;
use serde::{Deserialize, Serialize};

use crate::adoption::AdoptionMode;
use crate::command::{
    self, Command, DebugAction, MAX_COMMANDS_PER_STEP, Scenario, code, rejection,
};
use crate::datapack::Catalog;
use crate::event::{Event, EventQueue, EventRecord, MAX_IN_FLIGHT, SpeedChangeReason};
use crate::geometry::MapSize;
use crate::hash::StateHasher;
use crate::savegame::{self, LoadedGame, SaveSnapshot, SavedPack};
use crate::text::{Params, Reason, Severity};
use crate::time::{GameTime, Speed, TICKS_PER_DAY, Tick, is_day_start, is_night, next_morning};
pub use crate::wayfinding::RecomputeStats;
use crate::world::World;

/// Code of the notice and rejection when the tick reaches its maximum.
pub const TICK_LIMIT: &str = "tickLimit";

/// State of the core (`docs/architecture.md` section 4, "Zustände des
/// Kerns").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum CoreState {
    /// The core process is starting; no session yet.
    Starting,
    /// A session is running.
    Ready,
    /// A game is being loaded; commands are rejected with `notReady`.
    Loading,
    /// A non-recoverable error occurred; the core process ends.
    Failed,
}

/// Settings of a simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationConfig {
    /// Size of the map.
    pub map: MapSize,
    /// Worker threads of the tick, at least 1.
    pub threads: usize,
    /// Threads for background computations such as flow fields, at least 1.
    /// A separate pool keeps long computations from delaying the tick.
    pub background_threads: usize,
    /// When background results are taken over.
    pub adoption: AdoptionMode,
    /// Number of persons in the world of [`Simulation::new`] and of
    /// [`Simulation::restart`] (ignored by [`Simulation::with_world`]).
    pub persons: u32,
    /// Seed of the start state.
    pub seed: u64,
    /// Tick at which every session begins (time of day of a new game).
    pub start_tick: Tick,
}

/// Default split of `cores` logical cores into tick threads and background
/// threads (step 2.3, observation of 2026-10-13): the tick needs little
/// time, so it gets a quarter of the cores (1 … 4); the background gets the
/// rest except one core for the JavaScript threads of the core process and
/// the renderer. The sum never exceeds `cores` when `cores` ≥ 2; one core
/// gets one thread of each kind.
#[must_use]
pub fn default_threads(cores: usize) -> (usize, usize) {
    let threads = (cores / 4).clamp(1, 4);
    let background = cores.saturating_sub(threads + 1).max(1);
    (threads, background)
}

/// Errors when creating a simulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationError {
    /// `threads` or `background_threads` was 0.
    NoThreads,
    /// The thread pool could not be created.
    ThreadPool(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoThreads => write!(f, "the simulation needs at least one thread"),
            Self::ThreadPool(err) => write!(f, "cannot create the thread pool: {err}"),
        }
    }
}

impl std::error::Error for SimulationError {}

/// Debug counters of discarded input.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counters {
    /// Commands whose id was not larger than the last one.
    pub stale_commands: u32,
    /// Messages carrying another session.
    pub foreign_session: u32,
}

/// The simulation of one core process with its current session.
pub struct Simulation {
    config: SimulationConfig,
    pool: ThreadPool,
    background: ThreadPool,
    session: u32,
    state: CoreState,
    tick: Tick,
    speed: Speed,
    backpressure: Option<Speed>,
    /// Waiting commands; `Err` marks a command that could not be read
    /// and is answered with `invalidArgument`.
    inbox: VecDeque<(u32, Result<Command, Reason>)>,
    /// Scenario requested by the debug action `loadScenario`, started by
    /// the caller after the answer went out ([`Simulation::take_scenario`]).
    scenario: Option<(Scenario, u32)>,
    last_command_id: u32,
    counters: Counters,
    events: EventQueue,
    world: World,
    /// Content of the loaded data packs (S-4) for this session.
    content: Arc<Catalog>,
    /// Content chosen for the next session ([`Simulation::set_next_content`]).
    next_content: Option<Arc<Catalog>>,
}

impl Simulation {
    /// Creates the simulation with session 1, paused at `start_tick`, in an
    /// open hall with `persons` persons ([`World::open_hall`]).
    ///
    /// # Errors
    ///
    /// Fails if a thread count is 0 or a thread pool cannot be created.
    pub fn new(config: SimulationConfig) -> Result<Self, SimulationError> {
        let world = World::open_hall(config.map, config.persons, config.seed);
        Self::with_world(config, world)
    }

    /// Creates the simulation with session 1 in the given world. The flow
    /// fields of the world are computed before the first tick.
    ///
    /// # Errors
    ///
    /// Fails if a thread count is 0 or a thread pool cannot be created.
    pub fn with_world(config: SimulationConfig, mut world: World) -> Result<Self, SimulationError> {
        if config.threads == 0 || config.background_threads == 0 {
            return Err(SimulationError::NoThreads);
        }
        let build = |threads: usize, name: &'static str| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .thread_name(move |i| format!("{name}-{i}"))
                .build()
                .map_err(|err| SimulationError::ThreadPool(err.to_string()))
        };
        let pool = build(config.threads, "sim")?;
        let background = build(config.background_threads, "bg")?;
        world.reset_adoption(config.adoption);
        world.rebuild_fields_now(&background);
        Ok(Self {
            config,
            pool,
            background,
            session: 1,
            state: CoreState::Ready,
            tick: config.start_tick,
            speed: Speed::Paused,
            backpressure: None,
            inbox: VecDeque::new(),
            scenario: None,
            last_command_id: 0,
            counters: Counters::default(),
            events: EventQueue::new(),
            world,
            content: Arc::new(Catalog::empty()),
            next_content: None,
        })
    }

    /// Uses `content` from now on, in the current session (start of the
    /// core process, before the first command).
    #[must_use]
    pub fn with_content(mut self, content: Arc<Catalog>) -> Self {
        self.content = content;
        self
    }

    /// Uses `content` from the next session on (new game, loaded game,
    /// reload): changing the packs of a running game would orphan objects
    /// that refer to removed entries.
    pub fn set_next_content(&mut self, content: Arc<Catalog>) {
        self.next_content = Some(content);
    }

    /// Content of the current session.
    #[must_use]
    pub fn content(&self) -> &Catalog {
        &self.content
    }

    /// Starts a new session with a fresh world: ids, `seq` and the tick
    /// (at `start_tick`) begin anew; unprocessed commands and events of the old session are
    /// discarded.
    pub fn restart(&mut self) {
        let world = World::open_hall(self.config.map, self.config.persons, self.config.seed);
        self.restart_with_world(world);
    }

    /// Like [`Simulation::restart`], with the given world (new game or a
    /// loaded save game).
    pub fn restart_with_world(&mut self, mut world: World) {
        world.reset_adoption(self.config.adoption);
        world.rebuild_fields_now(&self.background);
        self.restart_with_world_prepared(world);
    }

    /// Like [`Simulation::restart_with_world`] for a world whose flow
    /// fields are computed and whose adoption is set already.
    fn restart_with_world_prepared(&mut self, world: World) {
        self.session = self.session.wrapping_add(1).max(1);
        self.state = CoreState::Ready;
        self.tick = self.config.start_tick;
        self.speed = Speed::Paused;
        self.backpressure = None;
        self.inbox.clear();
        self.scenario = None;
        self.last_command_id = 0;
        self.events = EventQueue::new();
        self.world = world;
        if let Some(content) = self.next_content.take() {
            self.content = content;
        }
    }

    /// State of the session for a save game, taken between two ticks;
    /// cheap, the encoding ([`SaveSnapshot::encode`]) can run elsewhere.
    #[must_use]
    pub fn save_snapshot(&self) -> SaveSnapshot {
        SaveSnapshot {
            tick: self.tick,
            speed: self.speed,
            world: self.world.snapshot(),
            packs: self
                .content
                .packs()
                .iter()
                .map(|p| SavedPack {
                    id: p.id.clone(),
                    version: p.version.clone(),
                    format: p.format,
                })
                .collect(),
        }
    }

    /// Starts a new session with a loaded save game (S-5): checks the data
    /// packs against the content of the next session, computes the flow
    /// fields in use and only then replaces the running session, which
    /// stays unchanged if anything fails. A recomputation that was running
    /// when the game was saved starts again in the background with its
    /// revision and adoption tick.
    ///
    /// # Errors
    ///
    /// `packMissing` or `packVersionMismatch` with the pack ids in
    /// `packs`, `invalidState` for a world that breaks an invariant.
    pub fn load_game(&mut self, game: LoadedGame) -> Result<(), Reason> {
        let content = Arc::clone(self.next_content.as_ref().unwrap_or(&self.content));
        check_packs(&game.meta.packs, &content)?;
        let (tick, speed) = (game.restored.tick, game.restored.speed);
        let world = game
            .restored
            .into_world(&self.background, self.config.adoption)?;
        self.restart_with_world_prepared(world);
        self.tick = tick;
        self.speed = speed;
        Ok(())
    }

    /// Current session number.
    #[must_use]
    pub fn session(&self) -> u32 {
        self.session
    }

    /// Current state of the core.
    #[must_use]
    pub fn state(&self) -> CoreState {
        self.state
    }

    /// Ticks computed in this session.
    #[must_use]
    pub fn tick(&self) -> Tick {
        self.tick
    }

    /// Current game speed.
    #[must_use]
    pub fn speed(&self) -> Speed {
        self.speed
    }

    /// World state of the session.
    #[must_use]
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Settings of the simulation.
    #[must_use]
    pub fn config(&self) -> SimulationConfig {
        self.config
    }

    /// Counters of discarded input.
    #[must_use]
    pub fn counters(&self) -> Counters {
        self.counters
    }

    /// Number of ignored event acknowledgements.
    #[must_use]
    pub fn invalid_acks(&self) -> u32 {
        self.events.invalid_acks()
    }

    /// `true` while the event stream holds the game (S-3 "Überlauf").
    #[must_use]
    pub fn is_backpressured(&self) -> bool {
        self.backpressure.is_some()
    }

    /// `true` if commands wait for the next step.
    #[must_use]
    pub fn has_pending_commands(&self) -> bool {
        !self.inbox.is_empty()
    }

    /// Marks the start of loading a game (S-5, step 2.5).
    pub fn begin_loading(&mut self) {
        self.state = CoreState::Loading;
    }

    /// Answers every waiting command with `notReady`; used for commands
    /// that arrived while a game was being loaded and the load failed
    /// (S-5 "Laden ist atomar"), without computing a tick.
    pub fn reject_waiting(&mut self) {
        for (id, _) in std::mem::take(&mut self.inbox) {
            self.events.push(
                self.tick,
                Event::CommandRejected {
                    id,
                    tick: self.tick,
                    reason: rejection(code::NOT_READY),
                },
            );
        }
    }

    /// Ends loading; the session continues as before when loading failed.
    pub fn end_loading(&mut self) {
        self.state = CoreState::Ready;
    }

    /// Accepts a command of the renderer (S-2). Commands of another session
    /// and ids not larger than the last one are discarded and counted.
    pub fn submit(&mut self, session: u32, id: u32, command: Command) {
        self.enqueue(session, id, Ok(command));
    }

    /// Accepts a command whose `kind` is known but whose fields could not be
    /// read; it is answered with `invalidArgument` in its turn (S-2).
    pub fn submit_invalid(&mut self, session: u32, id: u32) {
        self.enqueue(session, id, Err(rejection(code::INVALID_ARGUMENT)));
    }

    fn enqueue(&mut self, session: u32, id: u32, command: Result<Command, Reason>) {
        if session != self.session {
            self.counters.foreign_session = self.counters.foreign_session.saturating_add(1);
        } else if id <= self.last_command_id {
            self.counters.stale_commands = self.counters.stale_commands.saturating_add(1);
        } else {
            self.last_command_id = id;
            self.inbox.push_back((id, command));
        }
    }

    /// Takes the scenario requested by the debug action `loadScenario`. The
    /// caller hands out the events of the old session first and then starts
    /// the new one with [`Simulation::restart_with_world`].
    pub fn take_scenario(&mut self) -> Option<(Scenario, u32)> {
        self.scenario.take()
    }

    /// Acknowledges events up to `seq` (S-3 flow control).
    pub fn ack(&mut self, session: u32, seq: u32) {
        if session == self.session {
            self.events.ack(seq);
        } else {
            self.counters.foreign_session = self.counters.foreign_session.saturating_add(1);
        }
    }

    /// Hands out up to `max` events within the flow-control window.
    pub fn take_events(&mut self, max: usize) -> Vec<EventRecord> {
        self.events.take(max)
    }

    /// Adds an event that does not stem from the tick itself, such as a lag
    /// report of the real-time driver.
    pub fn report(&mut self, event: Event) {
        self.events.push(self.tick, event);
    }

    /// Starts recomputing the flow fields from the current grid and targets
    /// in the background and returns the revision. Building (step 3.1)
    /// triggers it after every change.
    pub fn request_recompute(&mut self) -> u32 {
        self.world.request_recompute(&self.background, self.tick)
    }

    /// `true` while a recomputation of the flow fields runs or waits for
    /// adoption.
    #[must_use]
    pub fn is_recomputing(&self) -> bool {
        self.world.wayfinding().is_recomputing()
    }

    /// Duration of the last adopted recomputation in this session.
    #[must_use]
    pub fn last_recompute(&self) -> Option<RecomputeStats> {
        self.world.wayfinding().last_recompute()
    }

    /// World for building and for tests; changes take effect for the flow
    /// fields with the next [`Simulation::request_recompute`].
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Deterministic hash over session-independent state: tick, speed and
    /// world.
    #[must_use]
    pub fn state_hash(&self) -> u64 {
        let mut hasher = StateHasher::new();
        hasher.u32(self.tick);
        hasher.u32(self.speed.ticks_per_second());
        self.world.hash_into(&mut hasher);
        hasher.finish()
    }

    /// One pass of the tick: takes over up to 4,096 waiting commands and
    /// answers each, then runs the stages of the domain modules in their
    /// order (ADR-039): each takes its internal events and, unless the game
    /// is paused, computes its simulation step. Background results that are
    /// due are adopted in the stage "Wegfindung".
    pub fn step(&mut self) {
        if let Some(previous) = self.backpressure {
            if self.events.pending_len() > MAX_IN_FLIGHT as usize {
                return;
            }
            self.backpressure = None;
            self.speed = previous;
            self.events.push(
                self.tick,
                Event::SpeedChanged {
                    speed: previous,
                    reason: Some(SpeedChangeReason::Backpressure),
                },
            );
        }

        let mut answers = Vec::new();
        let mut others = Vec::new();
        let batch = self.inbox.len().min(MAX_COMMANDS_PER_STEP);
        for (id, command) in self.inbox.drain(..batch).collect::<Vec<_>>() {
            let result = command.and_then(|command| self.execute(&command, &mut others));
            answers.push((id, result));
        }

        let at_limit = self.speed.is_running() && self.tick == Tick::MAX;
        let running = self.speed.is_running() && !at_limit;
        if let Some(revision) = self
            .world
            .step(&self.pool, &self.background, self.tick, running)
        {
            others.push(Event::WaypointsAdopted { revision });
        }

        if at_limit {
            self.speed = Speed::Paused;
            others.push(Event::SpeedChanged {
                speed: Speed::Paused,
                reason: None,
            });
            others.push(tick_limit_notice());
        } else if running {
            self.tick += 1;
            if is_day_start(self.tick) {
                others.push(day_started(self.tick));
            }
        }

        for (id, result) in answers {
            let event = match result {
                Ok(()) => Event::CommandAccepted {
                    id,
                    tick: self.tick,
                },
                Err(reason) => Event::CommandRejected {
                    id,
                    tick: self.tick,
                    reason,
                },
            };
            self.events.push(self.tick, event);
        }
        for event in others {
            self.events.push(self.tick, event);
        }
        self.events.flush_dropped(self.tick);

        if self.events.is_full() && self.backpressure.is_none() {
            self.backpressure = Some(self.speed);
            self.speed = Speed::Paused;
            self.events.push(
                self.tick,
                Event::SpeedChanged {
                    speed: Speed::Paused,
                    reason: Some(SpeedChangeReason::Backpressure),
                },
            );
        }
    }

    fn execute(&mut self, command: &Command, others: &mut Vec<Event>) -> Result<(), Reason> {
        if self.state == CoreState::Loading {
            return Err(rejection(code::NOT_READY));
        }
        command::validate(command, self.world.map())?;
        match command {
            Command::SetSpeed { speed } => {
                if *speed != self.speed {
                    self.speed = *speed;
                    others.push(Event::SpeedChanged {
                        speed: *speed,
                        reason: None,
                    });
                }
                Ok(())
            }
            Command::SkipNight {} => self.skip_night(others),
            // Content ids are full ids of the loaded data packs (S-4).
            // TODO(fahrplan-ref: 3.1): build the object; check that its
            // placement fits the command (`single` for `build`, `area` for
            // `buildArea`).
            Command::Build { object, .. } | Command::BuildArea { object, .. } => {
                if self.content.object(object).is_none() {
                    return Err(rejection(code::UNKNOWN_CONTENT));
                }
                Ok(())
            }
            // The world holds no buildings yet, so there is nothing to remove.
            // TODO(fahrplan-ref: 3.1): remove the objects in the area.
            Command::Demolish { .. } => Ok(()),
            Command::Debug { action } => self.debug(action),
        }
    }

    fn debug(&mut self, action: &DebugAction) -> Result<(), Reason> {
        match action {
            DebugAction::SpawnPersons { count } => {
                let seed =
                    self.config.seed ^ u64::from(self.tick) ^ u64::from(self.last_command_id);
                self.world.spawn_random(*count, seed);
                Ok(())
            }
            DebugAction::ClearPersons {} => {
                self.world.clear_persons();
                Ok(())
            }
            // The flow fields follow through the internal event of the
            // module "Welt & Bau" in the stage "Wegfindung" (ADR-039).
            DebugAction::SetWalls { area, wall } => self
                .world
                .set_walls(*area, *wall)
                .map_err(|_| rejection(code::OUT_OF_MAP)),
            DebugAction::LoadScenario { scenario, persons } => {
                if *scenario == Scenario::ReferenceTerminal
                    && !crate::scenario::fits_reference_terminal(self.config.map)
                {
                    return Err(rejection(code::OUT_OF_MAP));
                }
                self.scenario = Some((*scenario, *persons));
                Ok(())
            }
        }
    }

    fn skip_night(&mut self, others: &mut Vec<Event>) -> Result<(), Reason> {
        if !is_night(self.tick) {
            return Err(rejection(code::NOT_NIGHT));
        }
        // TODO(fahrplan-ref: 3.4): also block while aircraft are moving.
        if self.world.persons().has_active() {
            return Err(rejection(code::NIGHT_SKIP_BLOCKED));
        }
        let morning = next_morning(self.tick).ok_or_else(|| rejection(TICK_LIMIT))?;
        let crosses_midnight = morning / TICKS_PER_DAY != self.tick / TICKS_PER_DAY;
        self.tick = morning;
        if crosses_midnight {
            others.push(day_started(morning));
        }
        Ok(())
    }
}

fn day_started(tick: Tick) -> Event {
    let time = GameTime::at(tick);
    Event::DayStarted {
        day: time.day,
        year: time.year,
    }
}

fn tick_limit_notice() -> Event {
    Event::Notice {
        severity: Severity::Warning,
        code: TICK_LIMIT.to_owned(),
        text_key: format!("notice.{TICK_LIMIT}"),
        params: Params::new(),
        about: None,
    }
}

/// The packs of a save game against the active content (S-5 "Fehlende
/// Datenpakete"): a pack must be active with the same major version.
fn check_packs(saved: &[SavedPack], content: &Catalog) -> Result<(), Reason> {
    let major = |v: &str| semver::Version::parse(v).ok().map(|v| v.major);
    let mut missing = Vec::new();
    let mut mismatched = Vec::new();
    for pack in saved {
        match content.packs().iter().find(|p| p.id == pack.id) {
            None => missing.push(pack.id.as_str()),
            Some(active) if major(&active.version) != major(&pack.version) => {
                mismatched.push(pack.id.as_str());
            }
            Some(_) => {}
        }
    }
    if !missing.is_empty() {
        return Err(
            savegame::error(savegame::code::PACK_MISSING).with_text("packs", &missing.join(", "))
        );
    }
    if !mismatched.is_empty() {
        return Err(savegame::error(savegame::code::PACK_VERSION_MISMATCH)
            .with_text("packs", &mismatched.join(", ")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::QUEUE_CAPACITY;

    fn config(persons: u32) -> SimulationConfig {
        SimulationConfig {
            map: MapSize::new(256, 256).unwrap(),
            threads: 2,
            background_threads: 1,
            adoption: AdoptionMode::FixedTick { delay: 3 },
            persons,
            seed: 1,
            start_tick: 0,
        }
    }

    fn events(sim: &mut Simulation) -> Vec<Event> {
        let records = sim.take_events(usize::MAX);
        if let Some(last) = records.last() {
            sim.ack(sim.session(), last.seq);
        }
        records.into_iter().map(|r| r.event).collect()
    }

    #[test]
    fn commands_waiting_after_a_failed_load_are_rejected_without_a_tick() {
        let mut sim = Simulation::new(config(0)).unwrap();
        sim.submit(1, 1, Command::SetSpeed { speed: Speed::X1 });
        sim.step();
        let _ = events(&mut sim);
        sim.begin_loading();
        assert_eq!(sim.state(), CoreState::Loading);
        sim.submit(1, 2, Command::SetSpeed { speed: Speed::X3 });
        let tick = sim.tick();
        sim.reject_waiting();
        sim.end_loading();
        assert_eq!(
            (sim.tick(), sim.state(), sim.speed()),
            (tick, CoreState::Ready, Speed::X1)
        );
        assert!(!sim.has_pending_commands());
        assert!(events(&mut sim).iter().any(|e| matches!(e,
            Event::CommandRejected { id: 2, reason, .. } if reason.code == code::NOT_READY)));
    }

    #[test]
    fn zero_threads_are_rejected() {
        let err = Simulation::new(SimulationConfig {
            threads: 0,
            ..config(0)
        })
        .err()
        .unwrap();
        assert_eq!(err, SimulationError::NoThreads);
        let err = Simulation::new(SimulationConfig {
            background_threads: 0,
            ..config(0)
        })
        .err()
        .unwrap();
        assert_eq!(err, SimulationError::NoThreads);
        assert!(err.to_string().contains("thread"));
        assert!(
            SimulationError::ThreadPool("x".into())
                .to_string()
                .contains("x")
        );
    }

    #[test]
    fn adopted_recomputation_reports_its_duration() {
        let mut sim = Simulation::new(config(0)).unwrap();
        assert_eq!(sim.last_recompute(), None);
        assert!(!sim.is_recomputing());
        let revision = sim.request_recompute();
        assert!(sim.is_recomputing());
        sim.submit(1, 1, Command::SetSpeed { speed: Speed::X1 });
        // Fixed adoption 3 ticks after the request (verification mode).
        for _ in 0..4 {
            sim.step();
        }
        assert!(!sim.is_recomputing());
        let stats = sim.last_recompute().unwrap();
        assert_eq!(stats.revision, revision);
        // A new session forgets the measurement of the old one.
        sim.restart();
        assert_eq!(sim.last_recompute(), None);
    }

    #[test]
    fn tick_limit_stops_the_time_instead_of_overflowing() {
        let mut sim = Simulation::new(SimulationConfig {
            start_tick: Tick::MAX,
            ..config(0)
        })
        .unwrap();
        sim.submit(1, 1, Command::SetSpeed { speed: Speed::X1 });
        sim.step();
        assert_eq!(sim.tick(), Tick::MAX);
        assert_eq!(sim.speed(), Speed::Paused);
        let got = events(&mut sim);
        assert!(got.contains(&tick_limit_notice()));
    }

    #[test]
    fn night_skip_at_the_tick_limit_is_rejected() {
        let mut sim = Simulation::new(SimulationConfig {
            start_tick: Tick::MAX - 10,
            ..config(0)
        })
        .unwrap();
        assert!(is_night(sim.tick()));
        sim.submit(1, 1, Command::SkipNight {});
        sim.step();
        let got = events(&mut sim);
        assert!(matches!(
            &got[0],
            Event::CommandRejected { reason, .. } if reason.code == TICK_LIMIT
        ));
    }

    #[test]
    fn full_event_queue_holds_the_game_until_drained() {
        let mut sim = Simulation::new(config(0)).unwrap();
        sim.submit(1, 1, Command::SetSpeed { speed: Speed::X2 });
        sim.step();
        // Never acknowledged command answers fill the queue.
        let mut id = 1;
        while !sim.is_backpressured() {
            for _ in 0..4096 {
                id += 1;
                sim.submit(
                    1,
                    id,
                    Command::Demolish {
                        area: crate::geometry::CellRect {
                            floor: 0,
                            x0: 0,
                            y0: 0,
                            x1: 1,
                            y1: 1,
                        },
                    },
                );
            }
            sim.step();
        }
        assert_eq!(sim.speed(), Speed::Paused);
        let tick = sim.tick();
        sim.submit(1, id + 1, Command::SetSpeed { speed: Speed::X3 });
        sim.step();
        assert_eq!(sim.tick(), tick, "no tick while held");
        assert!(
            sim.has_pending_commands(),
            "no command processing while held"
        );
        // The renderer catches up.
        let mut drained = Vec::new();
        while sim.events.pending_len() > MAX_IN_FLIGHT as usize {
            drained.extend(events(&mut sim));
        }
        assert!(drained.len() >= QUEUE_CAPACITY - MAX_IN_FLIGHT as usize);
        sim.step();
        assert!(!sim.is_backpressured());
        let mut after = Vec::new();
        loop {
            let batch = events(&mut sim);
            if batch.is_empty() {
                break;
            }
            after.extend(batch);
        }
        let resumed = Event::SpeedChanged {
            speed: Speed::X2,
            reason: Some(SpeedChangeReason::Backpressure),
        };
        assert!(after.contains(&resumed), "resume is reported");
        assert_eq!(sim.speed(), Speed::X3, "the waiting command was processed");
    }

    #[test]
    fn default_threads_never_exceed_the_cores() {
        assert_eq!(default_threads(1), (1, 1));
        assert_eq!(default_threads(2), (1, 1));
        assert_eq!(default_threads(4), (1, 2));
        assert_eq!(default_threads(8), (2, 5));
        assert_eq!(default_threads(16), (4, 11));
        assert_eq!(default_threads(64), (4, 59));
        for cores in 2..=64 {
            let (t, b) = default_threads(cores);
            assert!(t + b <= cores && t >= 1 && b >= 1, "{cores}");
        }
    }

    fn debug(sim: &mut Simulation, id: u32, action: DebugAction) -> Event {
        sim.submit(sim.session(), id, Command::Debug { action });
        sim.step();
        events(sim).into_iter().next().unwrap()
    }

    #[test]
    fn debug_actions_change_persons_and_walls() {
        let mut sim = Simulation::new(config(0)).unwrap();
        let ok = |e: &Event| matches!(e, Event::CommandAccepted { .. });
        assert!(ok(&debug(
            &mut sim,
            1,
            DebugAction::SpawnPersons { count: 50 }
        )));
        assert_eq!(sim.world().persons().len(), 50);
        let area = crate::geometry::CellRect {
            floor: 0,
            x0: 10,
            y0: 10,
            x1: 20,
            y1: 20,
        };
        assert!(ok(&debug(
            &mut sim,
            2,
            DebugAction::SetWalls { area, wall: true }
        )));
        let inside = crate::geometry::CellPos {
            x: 15,
            y: 15,
            floor: 0,
        };
        assert!(!sim.world().build().grid().walkable(inside));
        assert!(ok(&debug(
            &mut sim,
            3,
            DebugAction::SetWalls { area, wall: false }
        )));
        assert!(sim.world().build().grid().walkable(inside));
        assert!(ok(&debug(&mut sim, 4, DebugAction::ClearPersons {})));
        assert_eq!(sim.world().persons().len(), 0);
        let rejected = debug(&mut sim, 5, DebugAction::SpawnPersons { count: 10_001 });
        assert!(
            matches!(rejected, Event::CommandRejected { reason, .. } if reason.code == "limitExceeded")
        );
    }

    #[test]
    fn reference_scenario_needs_room_on_the_map() {
        let mut sim = Simulation::new(config(0)).unwrap();
        let action = DebugAction::LoadScenario {
            scenario: Scenario::ReferenceTerminal,
            persons: 10,
        };
        let rejected = debug(&mut sim, 1, action);
        assert!(
            matches!(rejected, Event::CommandRejected { reason, .. } if reason.code == "outOfMap")
        );
        assert_eq!(sim.take_scenario(), None);
        let open = DebugAction::LoadScenario {
            scenario: Scenario::OpenHall,
            persons: 10,
        };
        assert!(matches!(
            debug(&mut sim, 2, open),
            Event::CommandAccepted { .. }
        ));
        assert_eq!(sim.take_scenario(), Some((Scenario::OpenHall, 10)));
    }

    #[test]
    fn invalid_command_is_answered_in_order() {
        let mut sim = Simulation::new(config(0)).unwrap();
        sim.submit_invalid(1, 1);
        sim.submit(1, 2, Command::SetSpeed { speed: Speed::X1 });
        sim.submit_invalid(1, 2); // stale id, dropped
        sim.step();
        let got = events(&mut sim);
        assert!(
            matches!(&got[0], Event::CommandRejected { id: 1, reason, .. } if reason.code == "invalidArgument")
        );
        assert!(matches!(got[1], Event::CommandAccepted { id: 2, .. }));
        assert_eq!(sim.counters().stale_commands, 1);
    }

    #[test]
    fn report_adds_an_event_at_the_current_tick() {
        let mut sim = Simulation::new(config(0)).unwrap();
        sim.report(Event::SimulationLagCleared {});
        let records = sim.take_events(10);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].tick, 0);
        assert_eq!(sim.config().persons, 0);
    }
}
