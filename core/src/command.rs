//! Commands of the command channel S-2 (`docs/architecture.md` section 4,
//! ADR-021) and their structural checks.
//!
//! The command `debug { action }` (FR-009) is added together with the debug
//! panel in roadmap step 2.7, which defines its actions.

use serde::{Deserialize, Serialize};

use crate::geometry::{CellPos, CellRect, MapSize, Rotation, is_valid_floor};
use crate::text::Reason;
use crate::time::Speed;

/// Prefix of the localisation keys of rejected commands.
pub const REJECT_TEXT_PREFIX: &str = "command.rejected";
/// Most commands taken over in one simulation step; the rest waits.
pub const MAX_COMMANDS_PER_STEP: usize = 4096;
/// Largest edge of `buildArea` and `demolish` in cells.
pub const MAX_AREA_EDGE: u16 = 512;
/// Longest content id in characters (S-4 `paket:eintrag`).
pub const MAX_CONTENT_ID_LEN: usize = 129;

/// A command of the player (S-2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Command {
    /// Changes the game speed.
    SetSpeed {
        /// New speed.
        speed: Speed,
    },
    /// Skips the night from 18:00 to 06:00 (META-08).
    SkipNight {},
    /// Places one object.
    Build {
        /// Content id from a data pack (S-4).
        object: String,
        /// Target cell.
        at: CellPos,
        /// Rotation of the object.
        rotation: Rotation,
    },
    /// Draws floors, walls or zones over an area as one command.
    BuildArea {
        /// Content id from a data pack (S-4).
        object: String,
        /// Covered area.
        area: CellRect,
    },
    /// Removes everything in an area.
    Demolish {
        /// Covered area.
        area: CellRect,
    },
}

/// Stable rejection codes of S-2.
pub mod code {
    /// Field out of range or inconsistent.
    pub const INVALID_ARGUMENT: &str = "invalidArgument";
    /// Cell or rectangle outside the map.
    pub const OUT_OF_MAP: &str = "outOfMap";
    /// Content id is not loaded.
    pub const UNKNOWN_CONTENT: &str = "unknownContent";
    /// Area larger than 512 × 512 cells.
    pub const AREA_TOO_LARGE: &str = "areaTooLarge";
    /// Core is loading a game.
    pub const NOT_READY: &str = "notReady";
    /// Night skip outside 18:00 … 06:00.
    pub const NOT_NIGHT: &str = "notNight";
    /// Night skip while passengers or aircraft are active.
    pub const NIGHT_SKIP_BLOCKED: &str = "nightSkipBlocked";
}

/// Builds the reason of a rejected command.
#[must_use]
pub fn rejection(code: &str) -> Reason {
    Reason::new(REJECT_TEXT_PREFIX, code)
}

/// Structural check of a command against the map (S-2 "Prüfung und
/// Ablehnungscodes"). Checks that need the world state follow in the
/// simulation.
///
/// # Errors
///
/// Returns the rejection reason of the first failed check.
pub fn validate(command: &Command, map: MapSize) -> Result<(), Reason> {
    match command {
        Command::SetSpeed { .. } | Command::SkipNight {} => Ok(()),
        Command::Build {
            object,
            at,
            rotation,
        } => {
            check_object(object)?;
            if !rotation.is_valid() || !is_valid_floor(at.floor) {
                return Err(rejection(code::INVALID_ARGUMENT));
            }
            if !map.contains(*at) {
                return Err(rejection(code::OUT_OF_MAP));
            }
            Ok(())
        }
        Command::BuildArea { object, area } => {
            check_object(object)?;
            check_area(*area, map)
        }
        Command::Demolish { area } => check_area(*area, map),
    }
}

fn check_object(object: &str) -> Result<(), Reason> {
    let len = object.chars().count();
    if len == 0 || len > MAX_CONTENT_ID_LEN {
        return Err(rejection(code::INVALID_ARGUMENT));
    }
    Ok(())
}

fn check_area(area: CellRect, map: MapSize) -> Result<(), Reason> {
    if !area.is_non_empty() || !is_valid_floor(area.floor) {
        return Err(rejection(code::INVALID_ARGUMENT));
    }
    if area.width() > MAX_AREA_EDGE || area.height() > MAX_AREA_EDGE {
        return Err(rejection(code::AREA_TOO_LARGE).with_number("max", f64::from(MAX_AREA_EDGE)));
    }
    if !map.contains_rect(area) {
        return Err(rejection(code::OUT_OF_MAP));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map() -> MapSize {
        MapSize::new(1024, 1024).unwrap()
    }

    fn area(x0: u16, y0: u16, x1: u16, y1: u16) -> CellRect {
        CellRect {
            floor: 0,
            x0,
            y0,
            x1,
            y1,
        }
    }

    fn code_of(command: &Command) -> Option<String> {
        validate(command, map()).err().map(|r| r.code)
    }

    #[test]
    fn speed_and_night_skip_need_no_structural_check() {
        assert_eq!(code_of(&Command::SetSpeed { speed: Speed::X3 }), None);
        assert_eq!(code_of(&Command::SkipNight {}), None);
    }

    #[test]
    fn build_checks_object_rotation_floor_and_map() {
        let ok = Command::Build {
            object: "base:bench".into(),
            at: CellPos {
                x: 1,
                y: 1,
                floor: 0,
            },
            rotation: Rotation(0),
        };
        assert_eq!(code_of(&ok), None);
        let Command::Build { at, .. } = ok.clone() else {
            unreachable!()
        };
        let with = |object: &str, at: CellPos, rotation: u8| Command::Build {
            object: object.into(),
            at,
            rotation: Rotation(rotation),
        };
        assert_eq!(
            code_of(&with("", at, 0)).as_deref(),
            Some(code::INVALID_ARGUMENT)
        );
        assert_eq!(
            code_of(&with(&"a".repeat(130), at, 0)).as_deref(),
            Some(code::INVALID_ARGUMENT)
        );
        assert_eq!(code_of(&with(&"a".repeat(129), at, 0)), None);
        assert_eq!(
            code_of(&with("x", at, 4)).as_deref(),
            Some(code::INVALID_ARGUMENT)
        );
        let low = CellPos { floor: -3, ..at };
        assert_eq!(
            code_of(&with("x", low, 0)).as_deref(),
            Some(code::INVALID_ARGUMENT)
        );
        let outside = CellPos { x: 1024, ..at };
        assert_eq!(
            code_of(&with("x", outside, 0)).as_deref(),
            Some(code::OUT_OF_MAP)
        );
    }

    #[test]
    fn areas_must_be_non_empty_bounded_and_on_the_map() {
        let demolish = |area| Command::Demolish { area };
        assert_eq!(code_of(&demolish(area(0, 0, 512, 512))), None);
        assert_eq!(
            code_of(&demolish(area(5, 0, 5, 3))).as_deref(),
            Some(code::INVALID_ARGUMENT)
        );
        assert_eq!(
            code_of(&demolish(CellRect {
                floor: 3,
                ..area(0, 0, 1, 1)
            }))
            .as_deref(),
            Some(code::INVALID_ARGUMENT)
        );
        let too_large = validate(&demolish(area(0, 0, 513, 1)), map()).unwrap_err();
        assert_eq!(too_large.code, code::AREA_TOO_LARGE);
        assert!(too_large.params.contains_key("max"));
        assert_eq!(
            code_of(&demolish(area(1000, 0, 1025, 1))).as_deref(),
            Some(code::OUT_OF_MAP)
        );
        let build_area = Command::BuildArea {
            object: String::new(),
            area: area(0, 0, 1, 1),
        };
        assert_eq!(
            code_of(&build_area).as_deref(),
            Some(code::INVALID_ARGUMENT)
        );
        let build_area = Command::BuildArea {
            object: "base:floor".into(),
            area: area(0, 0, 1, 1),
        };
        assert_eq!(code_of(&build_area), None);
    }
}
