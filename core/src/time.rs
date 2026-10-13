//! Time model of the simulation (ADR-020): fixed tick length of 1/3 game
//! second; 1× = 60, 2× = 120, 3× = 180 ticks per second; a game day has
//! 24 game hours, a game year 12 game days (META-09).
//!
//! Calendar origin: tick 0 is 00:00 on day 1 of year 1. The time of day at
//! which a new game starts is a matter of the new-game settings, not of the
//! clock.

use serde::{Deserialize, Serialize};

/// Number of the simulation step since the start of the game (S-3 "Zeit").
pub type Tick = u32;

/// Ticks per game second (1 tick = 1/3 game second).
pub const TICKS_PER_GAME_SECOND: u32 = 3;
/// Ticks per game minute.
pub const TICKS_PER_GAME_MINUTE: u32 = 60 * TICKS_PER_GAME_SECOND;
/// Ticks per game hour.
pub const TICKS_PER_GAME_HOUR: u32 = 60 * TICKS_PER_GAME_MINUTE;
/// Ticks per game day (24 game hours).
pub const TICKS_PER_DAY: u32 = 24 * TICKS_PER_GAME_HOUR;
/// Game days per game year (META-09).
pub const DAYS_PER_YEAR: u32 = 12;
/// Start of the night for the night skip (META-08), in game hours.
pub const NIGHT_START_HOUR: u32 = 18;
/// End of the night for the night skip (META-08), in game hours.
pub const NIGHT_END_HOUR: u32 = 6;

/// Game speed (S-2 `setSpeed`, META-08).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum Speed {
    /// Game time stands still; commands are still processed.
    Paused,
    /// Normal speed, 60 ticks per second.
    X1,
    /// Double speed, 120 ticks per second.
    X2,
    /// Triple speed, 180 ticks per second.
    X3,
}

impl Speed {
    /// Target number of ticks per real second.
    #[must_use]
    pub fn ticks_per_second(self) -> u32 {
        match self {
            Self::Paused => 0,
            Self::X1 => 60,
            Self::X2 => 120,
            Self::X3 => 180,
        }
    }

    /// Returns `true` unless the game is paused.
    #[must_use]
    pub fn is_running(self) -> bool {
        self != Self::Paused
    }
}

/// Calendar position of a tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameTime {
    /// Game year, starting at 1.
    pub year: u32,
    /// Day within the year, 1 … [`DAYS_PER_YEAR`].
    pub day: u32,
    /// Hour of the day, 0 … 23.
    pub hour: u32,
    /// Minute of the hour, 0 … 59.
    pub minute: u32,
}

impl GameTime {
    /// Calendar position of `tick`.
    #[must_use]
    pub fn at(tick: Tick) -> Self {
        let days = tick / TICKS_PER_DAY;
        let in_day = tick % TICKS_PER_DAY;
        Self {
            year: days / DAYS_PER_YEAR + 1,
            day: days % DAYS_PER_YEAR + 1,
            hour: in_day / TICKS_PER_GAME_HOUR,
            minute: in_day % TICKS_PER_GAME_HOUR / TICKS_PER_GAME_MINUTE,
        }
    }
}

/// Returns `true` if `tick` is the first tick of a game day (00:00).
#[must_use]
pub fn is_day_start(tick: Tick) -> bool {
    tick.is_multiple_of(TICKS_PER_DAY)
}

/// Returns `true` between 18:00 (inclusive) and 06:00 (exclusive).
#[must_use]
pub fn is_night(tick: Tick) -> bool {
    let hour = tick % TICKS_PER_DAY / TICKS_PER_GAME_HOUR;
    !(NIGHT_END_HOUR..NIGHT_START_HOUR).contains(&hour)
}

/// Tick of the next 06:00 after `tick`; `None` if it lies beyond the
/// largest tick.
#[must_use]
pub fn next_morning(tick: Tick) -> Option<Tick> {
    let day_start = tick - tick % TICKS_PER_DAY;
    let morning = day_start.checked_add(NIGHT_END_HOUR * TICKS_PER_GAME_HOUR)?;
    if morning > tick {
        Some(morning)
    } else {
        morning.checked_add(TICKS_PER_DAY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speeds_follow_adr_020() {
        assert_eq!(Speed::Paused.ticks_per_second(), 0);
        assert_eq!(Speed::X1.ticks_per_second(), 60);
        assert_eq!(Speed::X2.ticks_per_second(), 120);
        assert_eq!(Speed::X3.ticks_per_second(), 180);
        assert!(!Speed::Paused.is_running());
        assert!(Speed::X1.is_running());
    }

    #[test]
    fn a_day_lasts_72_minutes_at_1x_and_24_at_3x() {
        assert_eq!(TICKS_PER_DAY, 259_200);
        assert_eq!(TICKS_PER_DAY / Speed::X1.ticks_per_second(), 72 * 60);
        assert_eq!(TICKS_PER_DAY / Speed::X3.ticks_per_second(), 24 * 60);
    }

    #[test]
    fn calendar_rolls_over_days_and_years() {
        assert_eq!(
            GameTime::at(0),
            GameTime {
                year: 1,
                day: 1,
                hour: 0,
                minute: 0
            }
        );
        let t = GameTime::at(TICKS_PER_DAY * 12 + 13 * TICKS_PER_GAME_HOUR + 5 * 180);
        assert_eq!((t.year, t.day, t.hour, t.minute), (2, 1, 13, 5));
        assert_eq!(GameTime::at(TICKS_PER_DAY * 11).day, 12);
        assert!(is_day_start(TICKS_PER_DAY));
        assert!(!is_day_start(TICKS_PER_DAY + 1));
    }

    #[test]
    fn night_lasts_from_18_to_6() {
        let h = TICKS_PER_GAME_HOUR;
        assert!(is_night(0));
        assert!(is_night(6 * h - 1));
        assert!(!is_night(6 * h));
        assert!(!is_night(18 * h - 1));
        assert!(is_night(18 * h));
    }

    #[test]
    fn next_morning_is_strictly_later() {
        let h = TICKS_PER_GAME_HOUR;
        assert_eq!(next_morning(2 * h), Some(6 * h));
        assert_eq!(next_morning(6 * h), Some(TICKS_PER_DAY + 6 * h));
        assert_eq!(next_morning(20 * h), Some(TICKS_PER_DAY + 6 * h));
        assert_eq!(next_morning(Tick::MAX), None);
    }
}
