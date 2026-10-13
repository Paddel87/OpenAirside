//! Event stream S-3 (`docs/architecture.md` section 4, ADR-021): event
//! types, gap-free sequence numbers per session, flow control and overflow
//! handling.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::geometry::{CellPos, Handle};
use crate::text::{Params, Severity};
use crate::time::{Speed, Tick};

/// Most events handed out and not yet acknowledged by the renderer.
pub const MAX_IN_FLIGHT: u32 = 4096;
/// Most events waiting in the core; above this, notices are dropped.
pub const QUEUE_CAPACITY: usize = 65_536;

/// Why the speed changed without a player command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum SpeedChangeReason {
    /// The renderer stopped acknowledging events (S-3 "Überlauf").
    Backpressure,
}

/// What a notice refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum About {
    /// An object in the world.
    Handle(Handle),
    /// A cell of the grid.
    Cell(CellPos),
}

/// An event of the core (S-3 "Ereignisarten in Phase 2").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Event {
    /// A command was accepted; its effect is part of the frame of `tick`.
    CommandAccepted {
        /// Command id from S-2.
        id: u32,
        /// First tick whose state contains the effect.
        tick: Tick,
    },
    /// A command was rejected.
    CommandRejected {
        /// Command id from S-2.
        id: u32,
        /// Tick at which the command was examined.
        tick: Tick,
        /// Why it was rejected.
        reason: crate::text::Reason,
    },
    /// The game speed changed.
    SpeedChanged {
        /// New speed.
        speed: Speed,
        /// Set when the core changed the speed by itself.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        reason: Option<SpeedChangeReason>,
    },
    /// A new game day began at 00:00.
    DayStarted {
        /// Day within the year, 1 … 12.
        day: u32,
        /// Game year, starting at 1.
        year: u32,
    },
    /// A message for the player.
    Notice {
        /// Importance; decides the order of dropping on overflow.
        severity: Severity,
        /// Stable identifier.
        code: String,
        /// Localisation key.
        text_key: String,
        /// Parameters for the text.
        params: Params,
        /// Object or cell the notice refers to.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        about: Option<About>,
    },
    /// The simulation fell behind real time by more than one game second.
    SimulationLag {
        /// Ticks the simulation is behind.
        behind_ticks: u32,
        /// Ticks actually computed in the last real second.
        ticks_per_second: u32,
    },
    /// The simulation caught up again.
    SimulationLagCleared {},
    /// Derived data (flow fields) of the given revision are now in use.
    WaypointsAdopted {
        /// Revision of the adopted data.
        revision: u32,
    },
    /// Notices were dropped because the queue was full.
    EventsDropped {
        /// Number of dropped notices since the last report.
        count: u32,
    },
}

impl Event {
    /// Severity used for dropping on overflow; `None` for events that are
    /// never dropped.
    fn droppable_severity(&self) -> Option<Severity> {
        match self {
            Self::Notice { severity, .. } if *severity != Severity::Alert => Some(*severity),
            _ => None,
        }
    }
}

/// An event with its place in the stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct EventRecord {
    /// Gap-free number per session, starting at 1.
    pub seq: u32,
    /// Tick the event refers to.
    pub tick: Tick,
    /// The event.
    pub event: Event,
}

/// Queue of the event stream of one session.
///
/// Sequence numbers are assigned when events are handed out, so dropped
/// notices never leave gaps; the drop is reported by `eventsDropped`.
#[derive(Debug, Default)]
pub struct EventQueue {
    pending: VecDeque<(Tick, Event)>,
    last_sent: u32,
    last_acked: u32,
    dropped: u32,
    invalid_acks: u32,
}

impl EventQueue {
    /// Creates an empty queue; the first event gets `seq` 1.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an event. On a full queue the oldest `info` notice is dropped,
    /// then the oldest `warning`; command answers and alerts stay.
    pub fn push(&mut self, tick: Tick, event: Event) {
        self.pending.push_back((tick, event));
        if self.pending.len() > QUEUE_CAPACITY {
            self.drop_one_notice();
        }
    }

    fn drop_one_notice(&mut self) {
        for severity in [Severity::Info, Severity::Warning] {
            let victim = self
                .pending
                .iter()
                .position(|(_, e)| e.droppable_severity() == Some(severity));
            if let Some(index) = victim {
                self.pending.remove(index);
                self.dropped = self.dropped.saturating_add(1);
                return;
            }
        }
    }

    /// Reports dropped notices as one `eventsDropped` at `tick`, if any.
    pub fn flush_dropped(&mut self, tick: Tick) {
        if self.dropped > 0 {
            let count = std::mem::take(&mut self.dropped);
            self.pending
                .push_back((tick, Event::EventsDropped { count }));
        }
    }

    /// Hands out up to `max` events, limited by the flow-control window of
    /// [`MAX_IN_FLIGHT`] unacknowledged events.
    pub fn take(&mut self, max: usize) -> Vec<EventRecord> {
        let window = MAX_IN_FLIGHT - (self.last_sent - self.last_acked);
        let count = max.min(window as usize).min(self.pending.len());
        self.pending
            .drain(..count)
            .map(|(tick, event)| {
                self.last_sent += 1;
                EventRecord {
                    seq: self.last_sent,
                    tick,
                    event,
                }
            })
            .collect()
    }

    /// Acknowledges all events up to `seq`. Acknowledgements of events not
    /// yet handed out are ignored and counted.
    pub fn ack(&mut self, seq: u32) {
        if seq > self.last_sent {
            self.invalid_acks = self.invalid_acks.saturating_add(1);
        } else if seq > self.last_acked {
            self.last_acked = seq;
        }
    }

    /// Number of events waiting to be handed out.
    #[must_use]
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    /// Number of events handed out but not yet acknowledged.
    #[must_use]
    pub fn in_flight(&self) -> u32 {
        self.last_sent - self.last_acked
    }

    /// `true` once the queue holds [`QUEUE_CAPACITY`] events or more, which
    /// only happens with undroppable events.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.pending.len() >= QUEUE_CAPACITY
    }

    /// Number of ignored acknowledgements (debug counter).
    #[must_use]
    pub fn invalid_acks(&self) -> u32 {
        self.invalid_acks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::Params;

    fn notice(severity: Severity) -> Event {
        Event::Notice {
            severity,
            code: "test".into(),
            text_key: "notice.test".into(),
            params: Params::new(),
            about: None,
        }
    }

    fn accepted(id: u32) -> Event {
        Event::CommandAccepted { id, tick: 0 }
    }

    #[test]
    fn sequence_numbers_start_at_one_and_have_no_gaps() {
        let mut queue = EventQueue::new();
        for id in 1..=3 {
            queue.push(7, accepted(id));
        }
        let records = queue.take(10);
        assert_eq!(
            records.iter().map(|r| r.seq).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert!(records.iter().all(|r| r.tick == 7));
        queue.push(8, accepted(4));
        assert_eq!(queue.take(10)[0].seq, 4);
    }

    #[test]
    fn flow_control_limits_unacknowledged_events() {
        let mut queue = EventQueue::new();
        for id in 0..5000 {
            queue.push(0, accepted(id));
        }
        assert_eq!(queue.take(usize::MAX).len(), 4096);
        assert!(queue.take(usize::MAX).is_empty());
        assert_eq!(queue.in_flight(), 4096);
        queue.ack(100);
        assert_eq!(queue.take(usize::MAX).len(), 100);
        queue.ack(50); // older acknowledgement changes nothing
        assert_eq!(queue.in_flight(), 4096);
        queue.ack(9999); // never handed out
        assert_eq!(queue.invalid_acks(), 1);
        queue.ack(4196);
        assert_eq!(queue.in_flight(), 0);
        assert_eq!(queue.take(3).len(), 3);
    }

    #[test]
    fn overflow_drops_info_before_warning_and_keeps_answers_and_alerts() {
        let mut queue = EventQueue::new();
        queue.push(0, notice(Severity::Warning));
        queue.push(0, notice(Severity::Info));
        queue.push(0, notice(Severity::Alert));
        for id in 0..(QUEUE_CAPACITY as u32 - 3) {
            queue.push(0, accepted(id));
        }
        assert_eq!(queue.pending_len(), QUEUE_CAPACITY);
        queue.push(0, accepted(u32::MAX)); // drops the info notice
        queue.push(0, accepted(u32::MAX)); // drops the warning
        queue.push(0, accepted(u32::MAX)); // nothing droppable left
        assert_eq!(queue.pending_len(), QUEUE_CAPACITY + 1);
        assert!(queue.is_full());
        queue.flush_dropped(5);
        let all = queue.take(usize::MAX);
        assert!(all.iter().all(|r| r.event.droppable_severity().is_none()));
        assert!(all.iter().any(|r| matches!(
            r.event,
            Event::Notice {
                severity: Severity::Alert,
                ..
            }
        )));
        let mut rest = Vec::new();
        while queue.pending_len() > 0 {
            queue.ack(queue.last_sent);
            rest.extend(queue.take(usize::MAX));
        }
        assert_eq!(
            rest.last().map(|r| (&r.event, r.tick)),
            Some((&Event::EventsDropped { count: 2 }, 5))
        );
    }

    #[test]
    fn flush_without_drops_adds_nothing() {
        let mut queue = EventQueue::new();
        queue.flush_dropped(1);
        assert_eq!(queue.pending_len(), 0);
    }
}
