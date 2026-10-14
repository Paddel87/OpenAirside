//! Real-time driver of the simulation on its own thread: paces the ticks to
//! the game speed (ADR-020), reports lag (S-3) and exchanges commands,
//! acknowledgements and events with the caller. The bridge (step 2.7)
//! connects it to the `MessagePort`.

use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::command::Command;
use crate::event::{EventRecord, MAX_IN_FLIGHT};
use crate::pacer::{LagMonitor, Pacer};
use crate::simulation::{CoreState, Simulation, SimulationConfig, SimulationError};
use crate::time::{Speed, Tick};

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
    Ack {
        session: u32,
        seq: u32,
    },
    Restart,
    Stop,
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
    /// Ticks computed in the last full real second.
    pub ticks_per_second: u32,
}

struct Shared {
    status: Status,
    events: Vec<EventRecord>,
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
        let sim = Simulation::new(config)?;
        let shared = Arc::new(Mutex::new(Shared {
            status: status_of(&sim, 0),
            events: Vec::new(),
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

    /// Acknowledges events up to `seq` (S-3).
    pub fn ack(&self, session: u32, seq: u32) {
        let _ = self.input.send(Input::Ack { session, seq });
    }

    /// Starts a new session.
    pub fn restart(&self) {
        let _ = self.input.send(Input::Restart);
    }

    /// Takes the events published so far.
    #[must_use]
    pub fn take_events(&self) -> Vec<EventRecord> {
        std::mem::take(&mut lock(&self.shared).events)
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

fn status_of(sim: &Simulation, ticks_per_second: u32) -> Status {
    Status {
        state: sim.state(),
        session: sim.session(),
        tick: sim.tick(),
        speed: sim.speed(),
        ticks_per_second,
    }
}

/// Applies one input; returns `false` on stop.
fn apply(sim: &mut Simulation, input: Input, shared: &Mutex<Shared>) -> bool {
    match input {
        Input::Command {
            session,
            id,
            command,
        } => sim.submit(session, id, command),
        Input::Ack { session, seq } => sim.ack(session, seq),
        Input::Restart => {
            sim.restart();
            lock(shared).events.clear();
        }
        Input::Stop => return false,
    }
    true
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
    let mut measured_tps = 0u32;
    loop {
        let wait = pacer.until_next().unwrap_or(IDLE_WAIT).min(IDLE_WAIT);
        match inbox.recv_timeout(wait) {
            Ok(input) => {
                if !apply(&mut sim, input, shared) {
                    return;
                }
                while let Ok(input) = inbox.try_recv() {
                    if !apply(&mut sim, input, shared) {
                        return;
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return,
        }

        let now = Instant::now();
        pacer.elapse(now - last);
        last = now;
        if sim.speed().is_running() {
            while pacer.due() > 0 && sim.speed().is_running() {
                let before = sim.tick();
                sim.step();
                if sim.tick() != before {
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
            measured_tps = window_ticks;
            window_ticks = 0;
            window_start = now;
        }
        let lag_event = if sim.speed().is_running() {
            lag.observe(now - origin, pacer.due(), measured_tps)
        } else {
            lag.reset()
        };
        if let Some(event) = lag_event {
            sim.report(event);
        }

        let records = sim.take_events(MAX_IN_FLIGHT as usize);
        let mut out = lock(shared);
        out.events.extend(records);
        out.status = status_of(&sim, measured_tps);
    }
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
    fn panicking_driver_marks_the_core_as_failed() {
        let shared = Arc::new(Mutex::new(Shared {
            status: Status {
                state: CoreState::Ready,
                session: 1,
                tick: 0,
                speed: Speed::Paused,
                ticks_per_second: 0,
            },
            events: Vec::new(),
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
