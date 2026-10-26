//! One `MessagePort` of the core process: connects the port state machine
//! (`port`) with the real-time driver (`runner`), encodes frames (S-1) and
//! delivers events with them or, while no frame is requested, at least
//! every 250 ms (S-3). The bridge converts the messages to JavaScript
//! (rule 006, ADR-024); everything else lives here so that it is testable
//! without Electron.

use std::time::{Duration, Instant};

use crate::core_info;
use crate::display::WorldSender;
use crate::frame::{FrameFilter, Snapshot};
use crate::port::{Action, ClientMessage, CommandEnvelope, Port, PortCounters, ServerMessage};
use crate::query::{Answer, ERROR_TEXT_PREFIX, Query, answer};
use crate::runner::{Poll, Runner};

/// Longest time events wait while no frame is requested (S-3).
pub const EVENT_INTERVAL: Duration = Duration::from_millis(250);
/// Interval of the `coreStats` message.
pub const STATS_INTERVAL: Duration = Duration::from_secs(1);

/// A message for the renderer.
#[derive(Debug, Clone, PartialEq)]
pub enum Outgoing {
    /// A message without binary part.
    Message(ServerMessage),
    /// A message with a binary part `buf`: a frame (layout in `frame`) or
    /// a table (layout in `query`).
    Binary {
        /// Fields of the message.
        message: ServerMessage,
        /// The binary part.
        buf: Vec<u8>,
    },
}

/// State of one port of the core process.
pub struct PortHost {
    port: Port,
    front: Snapshot,
    /// Blocks of the resting world handed out on this port (S-1).
    world: WorldSender,
    /// Session last announced on this port.
    announced: u32,
    last_delivery: Instant,
    last_stats: Instant,
}

impl PortHost {
    /// A port waiting for `hello`; `now` starts the event interval.
    #[must_use]
    pub fn new(now: Instant) -> Self {
        Self {
            port: Port::new(),
            front: Snapshot::default(),
            world: WorldSender::default(),
            announced: 0,
            last_delivery: now,
            last_stats: now,
        }
    }

    /// Counters of discarded messages.
    #[must_use]
    pub fn counters(&self) -> PortCounters {
        self.port.counters()
    }

    /// Handles a parsed message of the renderer.
    pub fn receive(
        &mut self,
        runner: &Runner,
        message: ClientMessage,
        now: Instant,
    ) -> Vec<Outgoing> {
        let session = self.session(runner);
        let action = self.port.receive(message, session, core_info());
        self.act(runner, action, now)
    }

    /// Handles a message that did not parse; `envelope` is its reading as a
    /// command, if that succeeded.
    pub fn receive_unparsed(&mut self, runner: &Runner, envelope: Option<CommandEnvelope>) {
        let session = self.session(runner);
        if let Action::InvalidCommand { session, id } =
            self.port.receive_unparsed(envelope, session)
        {
            runner.invalid_command(session, id);
        }
    }

    /// Session the renderer knows: the one announced on this port, before
    /// the handshake the current one of the core. Messages are checked
    /// against it; the driver discards commands of an ended session.
    fn session(&self, runner: &Runner) -> u32 {
        if self.announced == 0 {
            runner.status().session
        } else {
            self.announced
        }
    }

    /// Counts a message that was too large to read.
    pub fn too_large(&mut self) {
        self.port.drop_message(crate::port::Dropped::TooLarge);
    }

    /// Regular call (about every 50 ms): announces a new session, delivers
    /// events while no frame was requested for [`EVENT_INTERVAL`] and sends
    /// the figures of the tick every [`STATS_INTERVAL`].
    pub fn pump(&mut self, runner: &Runner, now: Instant) -> Vec<Outgoing> {
        let mut out = Vec::new();
        if self.announced == 0 {
            return out;
        }
        if now.duration_since(self.last_delivery) >= EVENT_INTERVAL {
            let poll = runner.poll(&mut self.front);
            self.last_delivery = now;
            self.deliver(poll, None, &mut out);
            self.announce(&mut out);
        }
        if now.duration_since(self.last_stats) >= STATS_INTERVAL {
            self.last_stats = now;
            let status = runner.status();
            // Only figures of the session this port knows; a newer one is
            // announced first by the next frame or pump.
            if status.session == self.announced {
                out.push(Outgoing::Message(ServerMessage::CoreStats {
                    session: status.session,
                    tick: status.tick,
                    stats: status.stats,
                }));
            }
        }
        out
    }

    fn act(&mut self, runner: &Runner, action: Action, now: Instant) -> Vec<Outgoing> {
        let mut out = Vec::new();
        match action {
            Action::Reply(message) => {
                if let ServerMessage::Welcome { session, .. } = message {
                    self.announced = session;
                    self.last_delivery = now;
                }
                out.push(Outgoing::Message(message));
            }
            Action::Command {
                session,
                id,
                command,
            } => runner.command(session, id, command),
            Action::InvalidCommand { session, id } => runner.invalid_command(session, id),
            Action::Ack { seq } => runner.ack(self.announced, seq),
            Action::Frame {
                filter,
                ack,
                world_ack,
            } => {
                if ack > 0 {
                    runner.ack(self.announced, ack);
                }
                self.world.acknowledge(world_ack);
                let poll = runner.poll(&mut self.front);
                self.last_delivery = now;
                self.deliver(poll, Some(filter), &mut out);
                self.announce(&mut out);
            }
            Action::Query { id, query } => out.push(self.query(id, &query)),
            Action::Drop(_) => {}
        }
        out
    }

    /// Answers a query from the snapshot of the newest frame handed out
    /// (S-7 "Konsistenz"), outside the simulation step.
    fn query(&self, id: u32, query: &Query) -> Outgoing {
        let session = self.announced;
        let result = |dictionary, detail, catalog| ServerMessage::QueryResult {
            session,
            id,
            dictionary,
            detail,
            catalog,
        };
        match answer(query, &self.front) {
            Ok(Answer::Table(buf)) => Outgoing::Binary {
                message: result(None, None, None),
                buf,
            },
            Ok(Answer::Dictionary(dictionary)) => {
                Outgoing::Message(result(Some(dictionary), None, None))
            }
            Ok(Answer::Detail(detail)) => Outgoing::Message(result(None, Some(detail), None)),
            Ok(Answer::Catalog(catalog)) => Outgoing::Message(result(None, None, Some(catalog))),
            Err(code) => Outgoing::Message(ServerMessage::QueryError {
                session,
                id,
                code: code.to_owned(),
                text_key: format!("{ERROR_TEXT_PREFIX}.{code}"),
            }),
        }
    }

    /// Sends the events of `poll`; with a filter, the events of the
    /// snapshot's session go with the frame.
    fn deliver(&mut self, poll: Poll, filter: Option<FrameFilter>, out: &mut Vec<Outgoing>) {
        let frame_session = self.front.session();
        let mut with_frame = Vec::new();
        for group in poll.events {
            if filter.is_some() && group.session == frame_session {
                with_frame.extend(group.events);
            } else {
                out.push(Outgoing::Message(ServerMessage::Events {
                    session: group.session,
                    events: group.events,
                }));
            }
        }
        if let Some(filter) = filter {
            let mut buf = Vec::with_capacity(self.front.max_frame_bytes());
            self.front.encode(&filter, &mut self.world, &mut buf);
            out.push(Outgoing::Binary {
                message: ServerMessage::Frame {
                    session: frame_session,
                    tick: self.front.tick(),
                    events: with_frame,
                },
                buf,
            });
        }
    }

    /// Announces a session change seen in the newest snapshot.
    fn announce(&mut self, out: &mut Vec<Outgoing>) {
        let session = self.front.session();
        if self.announced != 0 && session != 0 && session != self.announced {
            self.announced = session;
            out.push(Outgoing::Message(ServerMessage::SessionStarted {
                session,
                tick: self.front.tick(),
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adoption::AdoptionMode;
    use crate::command::{Command, DebugAction, Scenario};
    use crate::event::Event;
    use crate::geometry::MapSize;
    use crate::port::{CommandKind, PROTOCOL};
    use crate::simulation::SimulationConfig;
    use crate::time::Speed;

    fn runner() -> Runner {
        Runner::start(SimulationConfig {
            map: MapSize::new(64, 64).unwrap(),
            threads: 1,
            background_threads: 1,
            adoption: AdoptionMode::WhenReady,
            persons: 10,
            seed: 3,
            start_tick: 0,
        })
        .unwrap()
    }

    fn hello(host: &mut PortHost, runner: &Runner, now: Instant) {
        let out = host.receive(runner, ClientMessage::Hello { protocol: PROTOCOL }, now);
        assert!(matches!(
            out.as_slice(),
            [Outgoing::Message(ServerMessage::Welcome { session: 1, .. })]
        ));
    }

    fn frame(host: &mut PortHost, runner: &Runner, ack: u32, now: Instant) -> Vec<Outgoing> {
        host.receive(
            runner,
            ClientMessage::FrameRequest {
                session: host.announced,
                filter: FrameFilter::default(),
                ack,
                world_ack: 0,
            },
            now,
        )
    }

    fn events_of(out: &[Outgoing]) -> Vec<Event> {
        out.iter()
            .flat_map(|o| match o {
                Outgoing::Message(ServerMessage::Events { events, .. })
                | Outgoing::Binary {
                    message: ServerMessage::Frame { events, .. },
                    ..
                } => events.iter().map(|r| r.event.clone()).collect(),
                _ => Vec::new(),
            })
            .collect()
    }

    /// Requests frames until `done` holds for the collected events.
    fn until(
        host: &mut PortHost,
        runner: &Runner,
        mut done: impl FnMut(&[Event], &[Outgoing]) -> bool,
    ) -> Vec<Outgoing> {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut all = Vec::new();
        let mut seen = Vec::new();
        let mut ack = 0;
        loop {
            let out = frame(host, runner, ack, Instant::now());
            for o in &out {
                if let Outgoing::Binary {
                    message: ServerMessage::Frame { events, .. },
                    ..
                }
                | Outgoing::Message(ServerMessage::Events { events, .. }) = o
                    && let Some(last) = events.last()
                {
                    ack = last.seq;
                }
            }
            seen.extend(events_of(&out));
            all.extend(out);
            if done(&seen, &all) {
                return all;
            }
            assert!(Instant::now() < deadline, "timed out: {seen:?}");
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    fn world_of(out: &[Outgoing]) -> (u32, u32) {
        let Some(Outgoing::Binary { buf, .. }) = out.last() else {
            panic!("no frame: {out:?}");
        };
        let section = crate::frame::HEADER_BYTES + 10 * crate::frame::PERSON_BYTES;
        (
            crate::frame::header_u32(buf, 32).unwrap(),
            crate::frame::header_u32(buf, section + 4).unwrap(),
        )
    }

    #[test]
    fn the_world_follows_the_acknowledgement_of_the_renderer() {
        let runner = runner();
        let now = Instant::now();
        let mut host = PortHost::new(now);
        hello(&mut host, &runner, now);
        let request = |host: &mut PortHost, world_ack: u32| {
            host.receive(
                &runner,
                ClientMessage::FrameRequest {
                    session: host.announced,
                    filter: FrameFilter::default(),
                    ack: 0,
                    world_ack,
                },
                now,
            )
        };
        // The first frame carries the four blocks of the hall.
        assert_eq!(world_of(&request(&mut host, 0)), (1, 4));
        // Not acknowledged (the renderer did not process the frame): again.
        assert_eq!(world_of(&request(&mut host, 0)), (2, 4));
        // Acknowledged: nothing new to send, revision 0.
        assert_eq!(world_of(&request(&mut host, 2)), (0, 0));
        assert_eq!(world_of(&request(&mut host, 2)), (0, 0));
        // A new session starts the transfer over, without any acknowledgement.
        host.receive(
            &runner,
            ClientMessage::Command {
                session: 1,
                id: 1,
                command: Command::Debug {
                    action: DebugAction::LoadScenario {
                        scenario: Scenario::OpenHall,
                        persons: 10,
                    },
                },
            },
            now,
        );
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let out = request(&mut host, 2);
            if host.announced == 2 || Instant::now() > deadline {
                assert_eq!(host.announced, 2, "the session switched");
                break;
            }
            drop(out);
        }
        let mut seen = world_of(&request(&mut host, 0));
        while seen.1 == 0 && Instant::now() < deadline {
            seen = world_of(&request(&mut host, 0));
        }
        assert_eq!(seen.1, 4, "the first frame of the new session");
        runner.stop().unwrap();
    }

    #[test]
    fn frames_carry_the_answers_to_commands() {
        let runner = runner();
        let now = Instant::now();
        let mut host = PortHost::new(now);
        assert!(
            host.pump(&runner, now + EVENT_INTERVAL).is_empty(),
            "silent before hello"
        );
        hello(&mut host, &runner, now);
        let out = frame(&mut host, &runner, 0, now);
        let Some(Outgoing::Binary { buf, .. }) = out.last() else {
            panic!("no frame: {out:?}");
        };
        let section = crate::frame::HEADER_BYTES + 10 * crate::frame::PERSON_BYTES;
        assert_eq!(
            crate::frame::header_u32(buf, section).map(|bytes| section + bytes as usize),
            Some(buf.len()),
            "persons, then the world section"
        );
        host.receive(
            &runner,
            ClientMessage::Command {
                session: 1,
                id: 1,
                command: Command::SetSpeed { speed: Speed::X3 },
            },
            now,
        );
        until(&mut host, &runner, |seen, _| {
            seen.iter()
                .any(|e| matches!(e, Event::CommandAccepted { id: 1, .. }))
        });
        host.receive(
            &runner,
            ClientMessage::EventsAck { session: 1, seq: 1 },
            now,
        );
        runner.stop().unwrap();
    }

    #[test]
    fn events_arrive_without_frames_after_the_interval() {
        let runner = runner();
        let start = Instant::now();
        let mut host = PortHost::new(start);
        hello(&mut host, &runner, start);
        host.receive_unparsed(
            &runner,
            Some(CommandEnvelope {
                kind: "command".into(),
                session: 1,
                id: 1,
                command: CommandKind {
                    kind: "build".into(),
                },
            }),
        );
        host.too_large();
        assert_eq!(host.counters().too_large, 1);
        assert!(host.pump(&runner, start).is_empty(), "interval not over");
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut seen = Vec::new();
        let mut at = start;
        while !seen
            .iter()
            .any(|e| matches!(e, Event::CommandRejected { id: 1, .. }))
        {
            at += EVENT_INTERVAL;
            let out = host.pump(&runner, at);
            assert!(out.iter().all(|o| matches!(o, Outgoing::Message(_))));
            seen.extend(events_of(&out));
            assert!(Instant::now() < deadline, "no answer");
            std::thread::sleep(Duration::from_millis(10));
        }
        runner.stop().unwrap();
    }

    #[test]
    fn tick_figures_arrive_every_second() {
        let runner = runner();
        let start = Instant::now();
        let mut host = PortHost::new(start);
        let stats_of = |out: &[Outgoing]| {
            out.iter()
                .filter_map(|o| match o {
                    Outgoing::Message(ServerMessage::CoreStats { session, stats, .. }) => {
                        Some((*session, *stats))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        assert!(
            host.pump(&runner, start + STATS_INTERVAL).is_empty(),
            "silent before hello"
        );
        hello(&mut host, &runner, start);
        let early = host.pump(&runner, start + STATS_INTERVAL / 2);
        assert!(stats_of(&early).is_empty(), "interval not over");
        let out = host.pump(&runner, start + STATS_INTERVAL);
        let figures = stats_of(&out);
        assert_eq!(figures.len(), 1);
        assert_eq!(figures[0].0, 1);
        assert_eq!(
            (figures[0].1.threads, figures[0].1.background_threads),
            (1, 1)
        );
        let again = host.pump(&runner, start + STATS_INTERVAL + EVENT_INTERVAL);
        assert!(stats_of(&again).is_empty(), "at most one per interval");
        runner.stop().unwrap();
    }

    #[test]
    fn session_change_is_announced_after_the_old_events() {
        let runner = runner();
        let now = Instant::now();
        let mut host = PortHost::new(now);
        hello(&mut host, &runner, now);
        host.receive(
            &runner,
            ClientMessage::Command {
                session: 1,
                id: 1,
                command: Command::Debug {
                    action: DebugAction::LoadScenario {
                        scenario: Scenario::OpenHall,
                        persons: 3,
                    },
                },
            },
            now,
        );
        let all = until(&mut host, &runner, |_, all| {
            all.iter().any(|o| {
                matches!(
                    o,
                    Outgoing::Message(ServerMessage::SessionStarted { session: 2, .. })
                )
            })
        });
        let accepted = all
            .iter()
            .position(|o| {
                events_of(std::slice::from_ref(o))
                    .iter()
                    .any(|e| matches!(e, Event::CommandAccepted { id: 1, .. }))
            })
            .expect("answer of the old session");
        let started = all
            .iter()
            .position(|o| matches!(o, Outgoing::Message(ServerMessage::SessionStarted { .. })))
            .unwrap();
        assert!(accepted < started);
        // Messages of the old session are now foreign.
        let out = host.receive(
            &runner,
            ClientMessage::EventsAck { session: 1, seq: 1 },
            now,
        );
        assert!(out.is_empty());
        assert_eq!(host.counters().foreign_session, 1);
        runner.stop().unwrap();
    }

    #[test]
    fn queries_are_answered_from_the_newest_frame() {
        use crate::query::{Query, Window};
        let runner = runner();
        let now = Instant::now();
        let mut host = PortHost::new(now);
        hello(&mut host, &runner, now);
        frame(&mut host, &runner, 0, now);
        let ask = |host: &mut PortHost, id, query| {
            host.receive(
                &runner,
                ClientMessage::Query {
                    session: 1,
                    id,
                    query,
                },
                now,
            )
        };
        let table = Query::Table {
            panel: 1,
            table: "debugPersons".into(),
            columns: vec!["handle".into()],
            filter: Vec::new(),
            sort: Vec::new(),
            window: Window {
                offset: 0,
                limit: 4,
            },
        };
        let out = ask(&mut host, 7, table);
        let [Outgoing::Binary { message, buf }] = out.as_slice() else {
            panic!("no table: {out:?}");
        };
        assert!(matches!(
            message,
            ServerMessage::QueryResult {
                id: 7,
                session: 1,
                ..
            }
        ));
        assert_eq!(
            crate::frame::header_u32(buf, 8),
            Some(10),
            "all 10 persons match"
        );
        assert_eq!(crate::frame::header_u32(buf, 16), Some(4));
        let out = ask(
            &mut host,
            8,
            Query::Dictionary {
                table: "debugPersons".into(),
            },
        );
        assert!(matches!(
            out.as_slice(),
            [Outgoing::Message(ServerMessage::QueryResult {
                id: 8,
                dictionary: Some(_),
                ..
            })]
        ));
        let out = ask(
            &mut host,
            9,
            Query::Detail {
                about: crate::geometry::Handle {
                    index: 0,
                    generation: 1,
                },
            },
        );
        assert!(matches!(
            out.as_slice(),
            [Outgoing::Message(ServerMessage::QueryResult {
                id: 9,
                detail: Some(_),
                ..
            })]
        ));
        let out = ask(&mut host, 11, Query::Catalog {});
        assert!(matches!(
            out.as_slice(),
            [Outgoing::Message(ServerMessage::QueryResult {
                id: 11,
                catalog: Some(entries),
                ..
            })] if entries.is_empty()
        ));
        let out = ask(
            &mut host,
            10,
            Query::Dictionary {
                table: "flights".into(),
            },
        );
        assert_eq!(
            out,
            vec![Outgoing::Message(ServerMessage::QueryError {
                session: 1,
                id: 10,
                code: "unknownTable".into(),
                text_key: "query.error.unknownTable".into(),
            })]
        );
        runner.stop().unwrap();
    }
}
