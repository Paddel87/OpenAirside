//! Counts of the background work of the module "Wegfindung & Passagierführung"
//! (ADR-041): recomputations in flight, flow field sets and grid snapshot
//! copies alive at the same time, with their peaks. Measurement only; the
//! counts never affect the simulated state or the state hash.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

/// A count of things alive at the same time and its highest value.
#[derive(Debug, Default)]
pub struct Gauge {
    live: AtomicU32,
    peak: AtomicU32,
}

impl Gauge {
    /// Counts one more thing until the returned token is dropped.
    #[must_use]
    pub fn acquire(self: &Arc<Self>) -> Token {
        let live = self.live.fetch_add(1, Ordering::AcqRel) + 1;
        self.peak.fetch_max(live, Ordering::AcqRel);
        Token(Arc::clone(self))
    }

    /// Things alive now.
    #[must_use]
    pub fn live(&self) -> u32 {
        self.live.load(Ordering::Acquire)
    }

    /// Most things alive at the same time since the start of the session.
    #[must_use]
    pub fn peak(&self) -> u32 {
        self.peak.load(Ordering::Acquire)
    }
}

/// Counts one thing in a [`Gauge`] for as long as it lives.
#[derive(Debug)]
pub struct Token(Arc<Gauge>);

impl Drop for Token {
    fn drop(&mut self) {
        self.0.live.fetch_sub(1, Ordering::AcqRel);
    }
}

/// The three counts of a [`crate::wayfinding::Wayfinding`].
#[derive(Debug, Clone, Default)]
pub struct Gauges {
    /// Recomputations requested and not yet finished (running or waiting
    /// for a thread).
    pub jobs: Arc<Gauge>,
    /// Flow field sets alive: in use, in computation, waiting to be freed.
    pub field_sets: Arc<Gauge>,
    /// Copies of the grid and targets made for background computations
    /// (the job's and the one kept for save games).
    pub snapshots: Arc<Gauge>,
}

/// Values of [`Gauges`] at one moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GaugeReading {
    /// Recomputations in flight now.
    pub jobs_live: u32,
    /// Most recomputations in flight at the same time.
    pub jobs_peak: u32,
    /// Flow field sets alive now.
    pub field_sets_live: u32,
    /// Most flow field sets alive at the same time.
    pub field_sets_peak: u32,
    /// Snapshot copies alive now.
    pub snapshots_live: u32,
    /// Most snapshot copies alive at the same time.
    pub snapshots_peak: u32,
}

impl Gauges {
    /// Current values.
    #[must_use]
    pub fn read(&self) -> GaugeReading {
        GaugeReading {
            jobs_live: self.jobs.live(),
            jobs_peak: self.jobs.peak(),
            field_sets_live: self.field_sets.live(),
            field_sets_peak: self.field_sets.peak(),
            snapshots_live: self.snapshots.live(),
            snapshots_peak: self.snapshots.peak(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_count_while_alive_and_keep_the_peak() {
        let gauge = Arc::new(Gauge::default());
        let a = gauge.acquire();
        let b = gauge.acquire();
        assert_eq!((gauge.live(), gauge.peak()), (2, 2));
        drop(a);
        assert_eq!((gauge.live(), gauge.peak()), (1, 2));
        drop(b);
        assert_eq!((gauge.live(), gauge.peak()), (0, 2));
    }
}
