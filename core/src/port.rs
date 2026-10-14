//! Messages on the `MessagePort` between renderer and core process and the
//! port state machine with handshake and session checks
//! (`docs/architecture.md` section 4, "Transportweg", ADR-021).
//!
//! The bridge converts JavaScript values into these types (rule 006,
//! ADR-024) and carries out the [`Action`]s; binary parts (frame buffer S-1)
//! are added by the bridge.

use serde::{Deserialize, Serialize};

use crate::CoreInfo;
use crate::command::{COMMAND_KINDS, Command};
use crate::event::EventRecord;
use crate::frame::FrameFilter;
use crate::time::Tick;

/// Protocol version of the port (S-1, S-2, S-3, S-7); raised by every
/// non-additive change.
pub const PROTOCOL: u32 = 1;
/// Largest message to the core in bytes of its JSON form.
pub const MAX_MESSAGE_BYTES: usize = 1 << 20;

/// Message from the renderer to the core.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ClientMessage {
    /// First message on a new port.
    Hello {
        /// Protocol version of the renderer.
        protocol: u32,
    },
    /// Command S-2.
    Command {
        /// Session from the handshake.
        session: u32,
        /// Strictly increasing per session, starting at 1.
        id: u32,
        /// The command.
        command: Command,
    },
    /// Asks for the newest frame S-1 and acknowledges events (S-3).
    FrameRequest {
        /// Session from the handshake.
        session: u32,
        /// What to include.
        filter: FrameFilter,
        /// Highest `seq` received so far, 0 for none.
        ack: u32,
    },
    /// Acknowledges events without asking for a frame (S-3).
    EventsAck {
        /// Session from the handshake.
        session: u32,
        /// Highest `seq` received.
        seq: u32,
    },
}

/// Message from the core to the renderer without binary parts.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ServerMessage {
    /// Answer to a compatible `hello`.
    Welcome {
        /// Protocol version of the core.
        protocol: u32,
        /// Identification of the core.
        core: CoreInfo,
        /// Current session.
        session: u32,
    },
    /// Answer to an incompatible `hello`; the port accepts nothing more.
    Incompatible {
        /// Protocol version of the core.
        protocol: u32,
    },
    /// A new session began on this port (new game, loaded game).
    SessionStarted {
        /// New session.
        session: u32,
        /// Tick at which it begins.
        tick: Tick,
    },
    /// Events without a frame, at the latest every 250 ms while no frame is
    /// requested (S-3).
    Events {
        /// Session of the events.
        session: u32,
        /// Events in stream order.
        events: Vec<EventRecord>,
    },
    /// Frame S-1 with the events up to its tick; the bridge adds the field
    /// `buf` (`ArrayBuffer`, layout in `frame`).
    Frame {
        /// Session of the frame.
        session: u32,
        /// Tick of the frame.
        tick: Tick,
        /// Events up to and including `tick`.
        events: Vec<EventRecord>,
    },
}

/// Only the parts of a command message needed to answer it when its
/// command cannot be read.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CommandEnvelope {
    /// Value of `type`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Session.
    pub session: u32,
    /// Command id.
    pub id: u32,
    /// The command, reduced to its kind.
    pub command: CommandKind,
}

/// The `kind` of a command.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CommandKind {
    /// Value of `kind`.
    pub kind: String,
}

/// State of a port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortState {
    /// Waiting for `hello`.
    WaitingForHello,
    /// Handshake done.
    Open,
    /// `hello` was incompatible; nothing more is accepted.
    Closed,
}

/// Why a message was discarded; every reason is counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dropped {
    /// Not readable, unknown `type` or unknown command `kind`.
    Unreadable,
    /// Larger than [`MAX_MESSAGE_BYTES`].
    TooLarge,
    /// Before `welcome`, a second `hello` or after `incompatible`.
    OutOfOrder,
    /// Carries another session.
    ForeignSession,
}

/// What the bridge does with a message.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Send this message to the renderer.
    Reply(ServerMessage),
    /// Pass the command to the simulation.
    Command {
        /// Session.
        session: u32,
        /// Command id.
        id: u32,
        /// The command.
        command: Command,
    },
    /// Answer the command with `invalidArgument` in its turn.
    InvalidCommand {
        /// Session.
        session: u32,
        /// Command id.
        id: u32,
    },
    /// Send the newest frame after acknowledging events up to `ack`.
    Frame {
        /// What to include.
        filter: FrameFilter,
        /// Highest received `seq`.
        ack: u32,
    },
    /// Acknowledge events up to `seq`.
    Ack {
        /// Highest received `seq`.
        seq: u32,
    },
    /// Nothing to do; the message was counted.
    Drop(Dropped),
}

/// Counters of discarded messages, shown in the debug panel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PortCounters {
    /// Unreadable messages.
    pub unreadable: u32,
    /// Too large messages.
    pub too_large: u32,
    /// Messages out of order.
    pub out_of_order: u32,
    /// Messages of another session.
    pub foreign_session: u32,
}

/// Handshake and session checks of one port.
#[derive(Debug, Clone)]
pub struct Port {
    state: PortState,
    counters: PortCounters,
}

impl Default for Port {
    fn default() -> Self {
        Self::new()
    }
}

impl Port {
    /// A new port waiting for `hello`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: PortState::WaitingForHello,
            counters: PortCounters::default(),
        }
    }

    /// Current state.
    #[must_use]
    pub fn state(&self) -> PortState {
        self.state
    }

    /// Counters of discarded messages.
    #[must_use]
    pub fn counters(&self) -> PortCounters {
        self.counters
    }

    /// Counts a discarded message and returns the matching action.
    pub fn drop_message(&mut self, reason: Dropped) -> Action {
        let counter = match reason {
            Dropped::Unreadable => &mut self.counters.unreadable,
            Dropped::TooLarge => &mut self.counters.too_large,
            Dropped::OutOfOrder => &mut self.counters.out_of_order,
            Dropped::ForeignSession => &mut self.counters.foreign_session,
        };
        *counter = counter.saturating_add(1);
        Action::Drop(reason)
    }

    /// Handles a message that did not parse as [`ClientMessage`]. If it is a
    /// command with a known `kind`, its id is answered with
    /// `invalidArgument`; anything else is discarded.
    pub fn receive_unparsed(&mut self, envelope: Option<CommandEnvelope>, session: u32) -> Action {
        let Some(envelope) = envelope else {
            return self.drop_message(Dropped::Unreadable);
        };
        if envelope.kind != "command" || !COMMAND_KINDS.contains(&envelope.command.kind.as_str()) {
            return self.drop_message(Dropped::Unreadable);
        }
        if self.state != PortState::Open {
            return self.drop_message(Dropped::OutOfOrder);
        }
        if envelope.session != session {
            return self.drop_message(Dropped::ForeignSession);
        }
        Action::InvalidCommand {
            session: envelope.session,
            id: envelope.id,
        }
    }

    /// Handles a message; `session` is the current session of the core.
    pub fn receive(&mut self, message: ClientMessage, session: u32, core: CoreInfo) -> Action {
        match (self.state, message) {
            (PortState::WaitingForHello, ClientMessage::Hello { protocol }) => {
                if protocol == PROTOCOL {
                    self.state = PortState::Open;
                    Action::Reply(ServerMessage::Welcome {
                        protocol: PROTOCOL,
                        core,
                        session,
                    })
                } else {
                    self.state = PortState::Closed;
                    Action::Reply(ServerMessage::Incompatible { protocol: PROTOCOL })
                }
            }
            (PortState::Open, message) => self.receive_open(message, session),
            _ => self.drop_message(Dropped::OutOfOrder),
        }
    }

    fn receive_open(&mut self, message: ClientMessage, session: u32) -> Action {
        let carried = match &message {
            ClientMessage::Hello { .. } => return self.drop_message(Dropped::OutOfOrder),
            ClientMessage::Command { session, .. }
            | ClientMessage::FrameRequest { session, .. }
            | ClientMessage::EventsAck { session, .. } => *session,
        };
        if carried != session {
            return self.drop_message(Dropped::ForeignSession);
        }
        match message {
            ClientMessage::Hello { .. } => self.drop_message(Dropped::OutOfOrder),
            ClientMessage::Command {
                session,
                id,
                command,
                ..
            } => Action::Command {
                session,
                id,
                command,
            },
            ClientMessage::FrameRequest { filter, ack, .. } => Action::Frame { filter, ack },
            ClientMessage::EventsAck { seq, .. } => Action::Ack { seq },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_info;
    use crate::time::Speed;

    fn open() -> Port {
        let mut port = Port::new();
        let action = port.receive(ClientMessage::Hello { protocol: PROTOCOL }, 4, core_info());
        assert_eq!(
            action,
            Action::Reply(ServerMessage::Welcome {
                protocol: PROTOCOL,
                core: core_info(),
                session: 4,
            })
        );
        assert_eq!(port.state(), PortState::Open);
        port
    }

    fn command(session: u32) -> ClientMessage {
        ClientMessage::Command {
            session,
            id: 1,
            command: Command::SetSpeed { speed: Speed::X1 },
        }
    }

    #[test]
    fn messages_before_hello_are_dropped() {
        let mut port = Port::default();
        assert_eq!(
            port.receive(command(1), 1, core_info()),
            Action::Drop(Dropped::OutOfOrder)
        );
        assert_eq!(port.counters().out_of_order, 1);
        assert_eq!(port.state(), PortState::WaitingForHello);
    }

    #[test]
    fn incompatible_hello_closes_the_port() {
        let mut port = Port::new();
        assert_eq!(
            port.receive(ClientMessage::Hello { protocol: 99 }, 1, core_info()),
            Action::Reply(ServerMessage::Incompatible { protocol: PROTOCOL })
        );
        assert_eq!(port.state(), PortState::Closed);
        assert_eq!(
            port.receive(ClientMessage::Hello { protocol: PROTOCOL }, 1, core_info()),
            Action::Drop(Dropped::OutOfOrder)
        );
    }

    #[test]
    fn open_port_forwards_messages_of_the_current_session() {
        let mut port = open();
        assert_eq!(
            port.receive(ClientMessage::Hello { protocol: PROTOCOL }, 4, core_info()),
            Action::Drop(Dropped::OutOfOrder),
            "a second hello is dropped"
        );
        assert!(matches!(
            port.receive(command(4), 4, core_info()),
            Action::Command {
                session: 4,
                id: 1,
                ..
            }
        ));
        assert_eq!(
            port.receive(command(3), 4, core_info()),
            Action::Drop(Dropped::ForeignSession)
        );
        assert_eq!(
            port.receive(
                ClientMessage::FrameRequest {
                    session: 4,
                    filter: FrameFilter::default(),
                    ack: 7,
                },
                4,
                core_info()
            ),
            Action::Frame {
                filter: FrameFilter::default(),
                ack: 7
            }
        );
        assert_eq!(
            port.receive(
                ClientMessage::EventsAck { session: 4, seq: 9 },
                4,
                core_info()
            ),
            Action::Ack { seq: 9 }
        );
        let c = port.counters();
        assert_eq!((c.out_of_order, c.foreign_session), (1, 1));
    }

    #[test]
    fn unreadable_commands_with_known_kind_are_answered() {
        let mut port = open();
        let envelope = |kind: &str, session| CommandEnvelope {
            kind: "command".into(),
            session,
            id: 5,
            command: CommandKind { kind: kind.into() },
        };
        assert_eq!(
            port.receive_unparsed(Some(envelope("build", 4)), 4),
            Action::InvalidCommand { session: 4, id: 5 }
        );
        assert_eq!(
            port.receive_unparsed(Some(envelope("fly", 4)), 4),
            Action::Drop(Dropped::Unreadable)
        );
        assert_eq!(
            port.receive_unparsed(Some(envelope("build", 2)), 4),
            Action::Drop(Dropped::ForeignSession)
        );
        assert_eq!(
            port.receive_unparsed(None, 4),
            Action::Drop(Dropped::Unreadable)
        );
        assert_eq!(
            port.drop_message(Dropped::TooLarge),
            Action::Drop(Dropped::TooLarge)
        );
        let c = port.counters();
        assert_eq!((c.unreadable, c.too_large, c.foreign_session), (2, 1, 1));
        let mut closed = Port::new();
        assert_eq!(
            closed.receive_unparsed(Some(envelope("build", 4)), 4),
            Action::Drop(Dropped::OutOfOrder)
        );
    }
}
