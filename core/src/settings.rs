//! Type of the player's settings for the platform API S-6 (`settings.get`,
//! `settings.set`; step 2.6, ADR-031). The core does not read settings; the
//! type lives here so that the TypeScript type is generated (ts-rs) like
//! every other type of S-6. The main process stores and checks the values;
//! its bounds in `app/src/main/settings.ts` mirror the constants below.

use serde::{Deserialize, Serialize};

/// Shortest autosave interval in minutes of running game time.
pub const MIN_AUTOSAVE_MINUTES: u8 = 1;
/// Longest autosave interval; an estimate (a rarer autosave hardly
/// protects), reviewed with the game menu in step 5.1.
pub const MAX_AUTOSAVE_MINUTES: u8 = 60;
/// Autosave interval of a new installation (as in step 2.5).
pub const DEFAULT_AUTOSAVE_MINUTES: u8 = 10;
/// Fewest autosave files in rotation (S-5).
pub const MIN_AUTOSAVE_COUNT: u8 = 1;
/// Most autosave files in rotation (S-5).
pub const MAX_AUTOSAVE_COUNT: u8 = 20;
/// Autosave files of a new installation (as in step 2.5).
pub const DEFAULT_AUTOSAVE_COUNT: u8 = 5;

/// The player's settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Language code of the user interface (S-6 `lang.list`), e.g. `de`.
    pub language: String,
    /// Minutes of running game time between two autosaves (1 to 60).
    pub autosave_minutes: u8,
    /// Autosave files in rotation (1 to 20).
    pub autosave_count: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_lie_within_the_bounds() {
        assert!((MIN_AUTOSAVE_MINUTES..=MAX_AUTOSAVE_MINUTES).contains(&DEFAULT_AUTOSAVE_MINUTES));
        assert!((MIN_AUTOSAVE_COUNT..=MAX_AUTOSAVE_COUNT).contains(&DEFAULT_AUTOSAVE_COUNT));
        let json = serde_json::to_string(&Settings {
            language: "de".into(),
            autosave_minutes: DEFAULT_AUTOSAVE_MINUTES,
            autosave_count: DEFAULT_AUTOSAVE_COUNT,
        })
        .expect("serialise");
        assert_eq!(
            json,
            r#"{"language":"de","autosaveMinutes":10,"autosaveCount":5}"#
        );
    }
}
