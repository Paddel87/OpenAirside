//! Internal events between the domain modules of the core (ADR-039).
//!
//! A module that changes something other modules react to lays down an
//! event; it does not call into them. Each event carries the stage that
//! produced it. A stage receives, in the order they were laid down, first
//! the events of the previous step from stages at or after it, then the
//! events of the current step from stages before it. So every event reaches
//! every stage exactly once: forwards in the same step, backwards in the
//! next one, deterministically. Events that have not reached all stages
//! between two steps are part of the world state (save game, state hash).

use serde::{Deserialize, Serialize};

use crate::hash::StateHasher;

/// Stages of a step in their fixed order (ADR-039). Commands of the
/// presentation (S-2) come first; each domain module follows in the order
/// of its dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Stage {
    /// Commands of the presentation (S-2).
    Commands,
    /// Module "Welt & Bau".
    Build,
    /// Module "Wegfindung & Passagierführung".
    Wayfinding,
    /// Module "Personen".
    Persons,
}

impl Stage {
    /// All stages in their order.
    pub const ALL: [Self; 4] = [Self::Commands, Self::Build, Self::Wayfinding, Self::Persons];
}

/// What one module tells the others.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InternalEvent {
    /// Cells of the grid changed: the flow fields must be recomputed.
    GridChanged,
}

impl InternalEvent {
    fn code(self) -> u8 {
        match self {
            Self::GridChanged => 0,
        }
    }
}

/// An event with the stage that laid it down.
pub type Entry = (Stage, InternalEvent);

/// The internal events of the current and the previous step.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InternalEvents {
    previous: Vec<Entry>,
    current: Vec<Entry>,
}

impl InternalEvents {
    /// Lays down `event`, produced by stage `from`.
    pub fn push(&mut self, from: Stage, event: InternalEvent) {
        self.current.push((from, event));
    }

    /// The events for `stage`, in the order they were laid down.
    pub fn for_stage(&self, stage: Stage) -> impl Iterator<Item = InternalEvent> + '_ {
        let earlier = self.previous.iter().filter(move |(from, _)| *from >= stage);
        let now = self.current.iter().filter(move |(from, _)| *from < stage);
        earlier.chain(now).map(|&(_, event)| event)
    }

    /// Ends a step: the events of this step are kept for the stages at or
    /// before their producer; events of the previous step are done. Events
    /// of the commands have reached every stage already.
    pub fn end_step(&mut self) {
        self.previous = std::mem::take(&mut self.current);
        self.previous.retain(|(from, _)| *from != Stage::Commands);
    }

    /// Events carried into the next step, as stored in a save game (taken
    /// between two steps).
    #[must_use]
    pub fn saved(&self) -> Vec<Entry> {
        self.previous.clone()
    }

    /// Events of a loaded save game.
    ///
    /// # Errors
    ///
    /// `events` if an event comes from the commands, which never carry
    /// over.
    pub fn restore(saved: Vec<Entry>) -> Result<Self, &'static str> {
        if saved.iter().any(|(from, _)| *from == Stage::Commands) {
            return Err("events");
        }
        Ok(Self {
            previous: saved,
            current: Vec::new(),
        })
    }

    /// Feeds the carried events into the state hash. Nothing is added while
    /// none are carried, so the hash of a world without them stays as it
    /// was before ADR-039; the events come last in the hash.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        for (from, event) in &self.previous {
            hasher.bytes(&[*from as u8, event.code()]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn got(queue: &InternalEvents, stage: Stage) -> usize {
        queue.for_stage(stage).count()
    }

    #[test]
    fn events_reach_later_stages_now_and_earlier_stages_in_the_next_step() {
        let mut q = InternalEvents::default();
        q.push(Stage::Wayfinding, InternalEvent::GridChanged);
        // Same step: only stages after the producer.
        assert_eq!(
            Stage::ALL.map(|s| got(&q, s)),
            [0, 0, 0, 1],
            "commands, build, wayfinding, persons"
        );
        q.end_step();
        // Next step: the producer and the stages before it.
        assert_eq!(Stage::ALL.map(|s| got(&q, s)), [1, 1, 1, 0]);
        q.end_step();
        assert_eq!(Stage::ALL.map(|s| got(&q, s)), [0; 4], "delivered once");
    }

    #[test]
    fn events_of_the_commands_do_not_carry_over() {
        let mut q = InternalEvents::default();
        q.push(Stage::Commands, InternalEvent::GridChanged);
        assert_eq!(Stage::ALL.map(|s| got(&q, s)), [0, 1, 1, 1]);
        q.end_step();
        assert!(q.saved().is_empty());
        assert_eq!(q, InternalEvents::default());
    }

    #[test]
    fn carried_events_are_saved_restored_and_hashed() {
        let hash = |q: &InternalEvents| {
            let mut h = StateHasher::new();
            q.hash_into(&mut h);
            h.finish()
        };
        let empty = hash(&InternalEvents::default());
        assert_eq!(
            empty,
            StateHasher::new().finish(),
            "adds nothing when empty"
        );
        let mut q = InternalEvents::default();
        q.push(Stage::Persons, InternalEvent::GridChanged);
        q.end_step();
        let saved = q.saved();
        assert_eq!(saved, vec![(Stage::Persons, InternalEvent::GridChanged)]);
        let restored = InternalEvents::restore(saved).unwrap();
        assert_eq!(restored, q);
        assert_ne!(hash(&restored), empty);
        assert_eq!(
            InternalEvents::restore(vec![(Stage::Commands, InternalEvent::GridChanged)]),
            Err("events")
        );
    }
}
