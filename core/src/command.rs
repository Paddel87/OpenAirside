//! Commands of the command channel S-2 (`docs/architecture.md` section 4,
//! ADR-021) and their structural checks, including the debug actions of
//! FR-009 defined in roadmap step 2.7.

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
/// Most persons one `spawnPersons` debug action adds.
pub const MAX_SPAWN_PER_COMMAND: u32 = 10_000;
/// Values of `kind` of [`Command`]; a command with another `kind` is
/// discarded and counted, one with a known `kind` but wrong fields is
/// rejected with `invalidArgument` (S-2, "Prüfung").
pub const COMMAND_KINDS: [&str; 6] = [
    "setSpeed",
    "skipNight",
    "build",
    "buildArea",
    "demolish",
    "debug",
];

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
    /// Debug action of the console (FR-009, META-15).
    Debug {
        /// What to do.
        action: DebugAction,
    },
}

/// Actions of the debug panel (FR-009, defined in roadmap step 2.7). They
/// serve tests and measurements before building and data packs exist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum DebugAction {
    /// Adds test persons on random walkable cells of the ground floor.
    SpawnPersons {
        /// Number of persons, 1 … 10,000.
        count: u32,
    },
    /// Removes all persons.
    ClearPersons {},
    /// Sets or removes walls in an area and recomputes the flow fields in
    /// the background, like a building action (ADR-010).
    SetWalls {
        /// Covered area, at most 512 × 512 cells.
        area: CellRect,
        /// `true` sets walls, `false` removes them.
        wall: bool,
    },
    /// Starts a new session with a test scenario.
    LoadScenario {
        /// Which scenario.
        scenario: Scenario,
        /// Persons at the start, at most 200,000.
        persons: u32,
    },
}

/// Test scenarios of the debug action `loadScenario`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum Scenario {
    /// Walkable ground floor without walls and one target.
    OpenHall,
    /// Reference terminal 768² × 5 with 17 gates (36 targets) in the middle
    /// of the map (spike 1.5, ADR-020).
    ReferenceTerminal,
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
    /// A quantity above its limit.
    pub const LIMIT_EXCEEDED: &str = "limitExceeded";
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
        Command::Debug { action } => validate_debug(action, map),
    }
}

fn validate_debug(action: &DebugAction, map: MapSize) -> Result<(), Reason> {
    match action {
        DebugAction::SpawnPersons { count } => {
            if *count == 0 {
                Err(rejection(code::INVALID_ARGUMENT))
            } else if *count > MAX_SPAWN_PER_COMMAND {
                Err(rejection(code::LIMIT_EXCEEDED)
                    .with_number("max", f64::from(MAX_SPAWN_PER_COMMAND)))
            } else {
                Ok(())
            }
        }
        DebugAction::ClearPersons {} => Ok(()),
        DebugAction::SetWalls { area, .. } => check_area(*area, map),
        DebugAction::LoadScenario { persons, .. } => {
            if usize::try_from(*persons).map_or(true, |p| p > crate::world::MAX_PERSONS) {
                Err(rejection(code::LIMIT_EXCEEDED))
            } else {
                Ok(())
            }
        }
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

    #[test]
    fn debug_actions_are_checked() {
        let spawn = |count| Command::Debug {
            action: DebugAction::SpawnPersons { count },
        };
        assert_eq!(code_of(&spawn(0)).as_deref(), Some(code::INVALID_ARGUMENT));
        assert_eq!(code_of(&spawn(10_000)), None);
        assert_eq!(
            code_of(&spawn(10_001)).as_deref(),
            Some(code::LIMIT_EXCEEDED)
        );
        let walls = Command::Debug {
            action: DebugAction::SetWalls {
                area: area(0, 0, 2000, 2),
                wall: true,
            },
        };
        assert_eq!(code_of(&walls).as_deref(), Some(code::AREA_TOO_LARGE));
        let clear = Command::Debug {
            action: DebugAction::ClearPersons {},
        };
        assert_eq!(code_of(&clear), None);
        let load = |persons| Command::Debug {
            action: DebugAction::LoadScenario {
                scenario: Scenario::OpenHall,
                persons,
            },
        };
        assert_eq!(code_of(&load(200_000)), None);
        assert_eq!(
            code_of(&load(200_001)).as_deref(),
            Some(code::LIMIT_EXCEEDED)
        );
        assert!(COMMAND_KINDS.contains(&"debug"));
    }
}
