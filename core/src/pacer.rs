//! Real-time pacing of the simulation and lag reporting (S-3
//! `simulationLag`, ADR-020). Both work on elapsed durations handed in by
//! the caller, so they are deterministic in tests.

use std::time::Duration;

use crate::event::Event;
use crate::time::{Speed, TICKS_PER_GAME_SECOND};

const NANOS_PER_SECOND: u128 = 1_000_000_000;

/// Counts the ticks that are due at the current speed.
///
/// The backlog is capped at one real second of ticks; time beyond that is
/// given up, so a machine that is too slow runs the game slower instead of
/// piling up work it can never finish.
#[derive(Debug, Clone)]
pub struct Pacer {
    ticks_per_second: u32,
    remainder: u128,
    due: u32,
    skipped: u64,
}

impl Pacer {
    /// Pacer for `speed`, with nothing due.
    #[must_use]
    pub fn new(speed: Speed) -> Self {
        Self {
            ticks_per_second: speed.ticks_per_second(),
            remainder: 0,
            due: 0,
            skipped: 0,
        }
    }

    /// Changes the speed; the backlog and the fraction of a tick are reset.
    pub fn set_speed(&mut self, speed: Speed) {
        *self = Self {
            skipped: self.skipped,
            ..Self::new(speed)
        };
    }

    /// Adds the ticks that became due during `elapsed`.
    pub fn elapse(&mut self, elapsed: Duration) {
        let tps = u128::from(self.ticks_per_second);
        self.remainder += elapsed.as_nanos() * tps;
        let new = self.remainder / NANOS_PER_SECOND;
        self.remainder %= NANOS_PER_SECOND;
        let due = u128::from(self.due) + new;
        let cap = u128::from(self.ticks_per_second);
        if due > cap {
            self.skipped = self
                .skipped
                .saturating_add(u64::try_from(due - cap).unwrap_or(u64::MAX));
        }
        self.due = u32::try_from(due.min(cap)).unwrap_or(u32::MAX);
    }

    /// Ticks due now.
    #[must_use]
    pub fn due(&self) -> u32 {
        self.due
    }

    /// Marks one due tick as computed.
    pub fn complete_one(&mut self) {
        self.due = self.due.saturating_sub(1);
    }

    /// Ticks given up because the backlog exceeded one real second.
    #[must_use]
    pub fn skipped(&self) -> u64 {
        self.skipped
    }

    /// Time until the next tick becomes due; zero if one is due already.
    #[must_use]
    pub fn until_next(&self) -> Option<Duration> {
        if self.ticks_per_second == 0 {
            return None;
        }
        if self.due > 0 {
            return Some(Duration::ZERO);
        }
        let missing = NANOS_PER_SECOND - self.remainder;
        let tps = u128::from(self.ticks_per_second);
        let nanos = missing.div_ceil(tps);
        Some(Duration::from_nanos(
            u64::try_from(nanos).unwrap_or(u64::MAX),
        ))
    }
}

/// Lag above which `simulationLag` is reported: one game second.
pub const LAG_REPORT_TICKS: u32 = TICKS_PER_GAME_SECOND;
/// Shortest interval between two lag reports.
pub const LAG_REPORT_INTERVAL: Duration = Duration::from_secs(1);

/// Turns the backlog into `simulationLag` / `simulationLagCleared` with
/// hysteresis: report above one game second, clear below a quarter game
/// second, at most one report per second.
#[derive(Debug, Clone, Default)]
pub struct LagMonitor {
    lagging: bool,
    last_report: Option<Duration>,
}

impl LagMonitor {
    /// Monitor that is not lagging.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Observes the backlog after a work slice at time `now` (since an
    /// arbitrary fixed origin) and returns the event to send, if any.
    pub fn observe(
        &mut self,
        now: Duration,
        behind_ticks: u32,
        ticks_per_second: u32,
    ) -> Option<Event> {
        let above = behind_ticks > LAG_REPORT_TICKS;
        // Below a quarter of a game second: 4 · behind < ticks per game second.
        let below = behind_ticks.saturating_mul(4) < TICKS_PER_GAME_SECOND;
        if self.lagging && below {
            self.lagging = false;
            return Some(Event::SimulationLagCleared {});
        }
        let may_report = self
            .last_report
            .is_none_or(|last| now.saturating_sub(last) >= LAG_REPORT_INTERVAL);
        if above && may_report {
            self.lagging = true;
            self.last_report = Some(now);
            return Some(Event::SimulationLag {
                behind_ticks,
                ticks_per_second,
            });
        }
        None
    }

    /// `true` while a lag report has not been cleared.
    #[must_use]
    pub fn is_lagging(&self) -> bool {
        self.lagging
    }

    /// Forgets the lag state, e.g. on pause or a new session.
    pub fn reset(&mut self) -> Option<Event> {
        std::mem::take(&mut self.lagging).then_some(Event::SimulationLagCleared {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MS: Duration = Duration::from_millis(1);

    #[test]
    fn paused_pacer_never_has_ticks_due() {
        let mut pacer = Pacer::new(Speed::Paused);
        pacer.elapse(Duration::from_secs(10));
        assert_eq!(pacer.due(), 0);
        assert_eq!(pacer.until_next(), None);
    }

    #[test]
    fn ticks_become_due_at_the_speed_rate_without_drift() {
        let mut pacer = Pacer::new(Speed::X3);
        for _ in 0..1000 {
            pacer.elapse(MS); // 180 ticks/s → 0.18 per ms
        }
        assert_eq!(pacer.due(), 180);
        let mut pacer = Pacer::new(Speed::X1);
        pacer.elapse(Duration::from_millis(50));
        assert_eq!(pacer.due(), 3);
        pacer.complete_one();
        assert_eq!(pacer.due(), 2);
        assert_eq!(pacer.until_next(), Some(Duration::ZERO));
    }

    #[test]
    fn until_next_rounds_up_to_the_next_tick() {
        let mut pacer = Pacer::new(Speed::X1);
        assert_eq!(pacer.until_next(), Some(Duration::from_nanos(16_666_667)));
        pacer.elapse(Duration::from_millis(10));
        assert_eq!(pacer.until_next(), Some(Duration::from_nanos(6_666_667)));
    }

    #[test]
    fn backlog_is_capped_at_one_second() {
        let mut pacer = Pacer::new(Speed::X2);
        pacer.elapse(Duration::from_secs(3));
        assert_eq!(pacer.due(), 120);
        assert_eq!(pacer.skipped(), 240);
        pacer.set_speed(Speed::X1);
        assert_eq!(pacer.due(), 0);
        assert_eq!(pacer.skipped(), 240);
    }

    #[test]
    fn lag_is_reported_above_one_game_second_and_cleared_below_a_quarter() {
        let mut lag = LagMonitor::new();
        let s = Duration::from_secs;
        assert_eq!(lag.observe(s(0), 3, 180), None);
        assert_eq!(
            lag.observe(s(1), 4, 170),
            Some(Event::SimulationLag {
                behind_ticks: 4,
                ticks_per_second: 170
            })
        );
        assert!(lag.is_lagging());
        // Between the thresholds nothing changes.
        assert_eq!(lag.observe(s(3), 1, 180), None);
        assert_eq!(
            lag.observe(s(4), 0, 180),
            Some(Event::SimulationLagCleared {})
        );
        assert!(!lag.is_lagging());
    }

    #[test]
    fn lag_reports_are_limited_to_one_per_second() {
        let mut lag = LagMonitor::new();
        let t = |ms| Duration::from_millis(ms);
        assert!(lag.observe(t(0), 10, 100).is_some());
        assert_eq!(lag.observe(t(500), 12, 100), None);
        assert!(lag.observe(t(1000), 12, 100).is_some());
        assert_eq!(
            lag.observe(t(1100), 0, 100),
            Some(Event::SimulationLagCleared {})
        );
        assert_eq!(lag.observe(t(1200), 10, 100), None);
        assert!(lag.observe(t(2000), 10, 100).is_some());
        assert_eq!(lag.reset(), Some(Event::SimulationLagCleared {}));
        assert_eq!(lag.reset(), None);
    }
}
