//! Data packs (S-4, ADR-021, ADR-025): loading, checking and the catalog of
//! loaded content.
//!
//! A data pack is a folder with `manifest.json` and content files in one
//! sub-folder per kind. The loader reads only files it enumerates itself,
//! checks every file against fixed limits before parsing, checks structure
//! (stage 1) and values, ids and references (stage 2), and loads a faulty
//! pack not at all; the game starts with the other packs. The result is a
//! [`Catalog`] for the simulation and one [`PackReport`] per found pack for
//! the platform API S-6 (`packs.list`).

mod catalog;
mod files;
mod load;
pub mod model;
pub mod scan;
#[cfg(feature = "schema")]
pub mod schema;

use serde::Serialize;

pub use catalog::{ActivePack, Catalog, ObjectType};
pub use load::{LoadOutcome, PackFolder, discover, game_version, load};

use crate::text::Reason;

/// Current version of the pack format (`format` in the manifest).
pub const FORMAT: u32 = 1;
/// Identifier of the base pack of the game.
pub const BASE_PACK_ID: &str = "base";
/// Largest file of a pack in bytes.
pub const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
/// Largest pack in bytes (all read files).
pub const MAX_PACK_BYTES: u64 = 256 * 1024 * 1024;
/// Most files of a pack that the loader reads.
pub const MAX_FILES: usize = 10_000;
/// Most entries of a pack.
pub const MAX_ENTRIES: usize = 100_000;
/// Most characters of a string.
pub const MAX_STRING_CHARS: usize = 4_096;
/// Most characters of a pack or entry id.
pub const MAX_ID_CHARS: usize = 64;
/// Most elements of a list.
pub const MAX_LIST_LEN: usize = 65_536;
/// Deepest nesting of objects and lists.
pub const MAX_DEPTH: usize = 32;
/// Most packs (base pack included).
pub const MAX_PACKS: usize = 256;
/// Most errors listed per pack; further ones are only counted.
pub const MAX_REPORTED_ERRORS: usize = 100;

/// Prefix of the localisation keys of pack errors.
pub const ERROR_TEXT_PREFIX: &str = "datapack.error";

/// Error codes of the loader (S-4 "Fehlerverhalten").
pub mod code {
    /// A limit of S-4 was exceeded; params `limit` and `max`.
    pub const LIMIT_EXCEEDED: &str = "limitExceeded";
    /// `dependencies` form a cycle.
    pub const DEPENDENCY_CYCLE: &str = "dependencyCycle";
    /// Symbolic link, reparse point, reserved or invalid name, path outside
    /// the pack folder.
    pub const INVALID_PATH: &str = "invalidPath";
    /// A folder the format does not know, or a folder inside a kind folder.
    pub const UNEXPECTED_ENTRY: &str = "unexpectedEntry";
    /// A kind folder whose format a later step defines; param `kind`.
    pub const KIND_NOT_SUPPORTED: &str = "kindNotSupported";
    /// `manifest.json` is missing.
    pub const MANIFEST_MISSING: &str = "manifestMissing";
    /// A file or folder could not be read.
    pub const IO_ERROR: &str = "ioError";
    /// The file is not valid JSON.
    pub const SYNTAX_ERROR: &str = "syntaxError";
    /// The JSON does not match the format (unknown or missing field, wrong type).
    pub const INVALID_STRUCTURE: &str = "invalidStructure";
    /// A pack or entry id breaks the naming rule.
    pub const INVALID_ID: &str = "invalidId";
    /// A version or version range is not valid Semantic Versioning.
    pub const INVALID_VERSION: &str = "invalidVersion";
    /// The pack needs a newer game ("neuere Spielversion nötig").
    pub const FORMAT_TOO_NEW: &str = "formatTooNew";
    /// The pack format is unknown (0).
    pub const FORMAT_UNSUPPORTED: &str = "formatUnsupported";
    /// The game version is outside the range `game` of the pack.
    pub const GAME_VERSION: &str = "gameVersion";
    /// A dependency is not installed; param `id`.
    pub const DEPENDENCY_MISSING: &str = "dependencyMissing";
    /// A dependency is installed but not active; param `id`.
    pub const DEPENDENCY_INACTIVE: &str = "dependencyInactive";
    /// A dependency has a version outside the required range; params `id`, `version`.
    pub const DEPENDENCY_VERSION: &str = "dependencyVersion";
    /// A dependency failed to load; param `id`.
    pub const DEPENDENCY_FAILED: &str = "dependencyFailed";
    /// Another folder already holds a pack with this id; param `id`.
    pub const DUPLICATE_PACK: &str = "duplicatePack";
    /// The full id of an entry exists twice; param `id`.
    pub const DUPLICATE_ID: &str = "duplicateId";
    /// `replaces` names an entry that does not exist, belongs to a pack this
    /// pack does not depend on, or has another kind; param `target`.
    pub const REPLACE_TARGET: &str = "replaceTarget";
    /// Another pack already replaces this entry; params `target`, `by`.
    pub const REPLACE_CONFLICT: &str = "replaceConflict";
    /// A value is outside its range.
    pub const INVALID_VALUE: &str = "invalidValue";
}

/// Where a pack comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum PackSource {
    /// Shipped with the game; always active and loaded first.
    Base,
    /// Installed by the player in the pack folder of the user data.
    User,
}

/// State of a found pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum PackState {
    /// Loaded; its entries are in the catalog.
    Active,
    /// Found and its manifest is valid, but not chosen by the player.
    Inactive,
    /// Not loaded because of the listed errors.
    Failed,
}

/// One error of a pack with its place (S-4 "Fehlerverhalten").
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PackError {
    /// File relative to the pack folder with `/`, e.g. `objects/seats.json`.
    pub file: Option<String>,
    /// One-based line.
    pub line: Option<u32>,
    /// One-based column.
    pub column: Option<u32>,
    /// JSON path, e.g. `objects[3].placement`.
    pub path: Option<String>,
    /// Code, localisation key and parameters.
    pub reason: Reason,
}

impl PackError {
    /// Error of the whole pack, without file.
    #[must_use]
    pub fn pack(reason: Reason) -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            path: None,
            reason,
        }
    }

    /// Error of a file, without place in it.
    #[must_use]
    pub fn file(file: &str, reason: Reason) -> Self {
        Self {
            file: Some(file.to_owned()),
            ..Self::pack(reason)
        }
    }
}

/// What the loader found in one pack folder.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PackReport {
    /// Name of the pack folder (no path).
    pub folder: String,
    /// Origin of the pack.
    pub source: PackSource,
    /// Pack id from a readable manifest.
    pub id: Option<String>,
    /// Display name from a readable manifest.
    pub name: Option<String>,
    /// Version from a readable manifest.
    pub version: Option<String>,
    /// Loaded, not chosen, or failed.
    pub state: PackState,
    /// Number of loaded entries.
    pub entries: u32,
    /// Errors in the order found, at most [`MAX_REPORTED_ERRORS`].
    pub errors: Vec<PackError>,
    /// Errors found beyond [`MAX_REPORTED_ERRORS`].
    pub errors_omitted: u32,
}

impl PackReport {
    fn new(folder: String, source: PackSource) -> Self {
        Self {
            folder,
            source,
            id: None,
            name: None,
            version: None,
            state: PackState::Inactive,
            entries: 0,
            errors: Vec::new(),
            errors_omitted: 0,
        }
    }

    /// Records an error and marks the pack as failed.
    fn fail(&mut self, error: PackError) {
        self.state = PackState::Failed;
        if self.errors.len() < MAX_REPORTED_ERRORS {
            self.errors.push(error);
        } else {
            self.errors_omitted = self.errors_omitted.saturating_add(1);
        }
    }
}

/// Reason of a pack error with the key `datapack.error.<code>`.
#[must_use]
pub fn error(code: &str) -> Reason {
    Reason::new(ERROR_TEXT_PREFIX, code)
}

/// `limitExceeded` with the name of the limit and its maximum.
#[must_use]
pub fn limit_error(limit: &str, max: u32) -> Reason {
    error(code::LIMIT_EXCEEDED)
        .with_text("limit", limit)
        .with_number("max", f64::from(max))
}

/// Checks the naming rule of a pack id (`[a-z0-9_-]{3,64}`).
#[must_use]
pub fn is_pack_id(id: &str) -> bool {
    (3..=MAX_ID_CHARS).contains(&id.len()) && id.bytes().all(is_id_byte)
}

/// Checks the naming rule of an entry id (`[a-z0-9_-]{1,64}`).
#[must_use]
pub fn is_entry_id(id: &str) -> bool {
    (1..=MAX_ID_CHARS).contains(&id.len()) && id.bytes().all(is_id_byte)
}

/// Checks the active ids of `packs.setActive` (S-6): at most
/// [`MAX_PACKS`], each a valid pack id, none twice.
///
/// # Errors
///
/// Returns `invalidId` or `limitExceeded` with the parameter `id` or `max`.
pub fn check_active(ids: &[String]) -> Result<(), Reason> {
    if ids.len() > MAX_PACKS {
        return Err(limit_error(
            "packs",
            u32::try_from(MAX_PACKS).unwrap_or(u32::MAX),
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for id in ids {
        if !is_pack_id(id) || !seen.insert(id.as_str()) {
            return Err(error(code::INVALID_ID).with_text("id", id));
        }
    }
    Ok(())
}

fn is_id_byte(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_follow_the_naming_rules() {
        assert!(is_pack_id("base"));
        assert!(is_pack_id("my_mod-2"));
        assert!(!is_pack_id("ab"));
        assert!(!is_pack_id("Base"));
        assert!(!is_pack_id("a:b"));
        assert!(!is_pack_id(&"a".repeat(65)));
        assert!(is_entry_id("x"));
        assert!(!is_entry_id(""));
        assert!(!is_entry_id("ä"));
    }

    #[test]
    fn active_ids_are_checked() {
        let ids = |list: &[&str]| list.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        assert!(check_active(&ids(&["base", "my-mod"])).is_ok());
        assert!(check_active(&[]).is_ok());
        assert_eq!(
            check_active(&ids(&["ok-id", "ok-id"])).unwrap_err().code,
            code::INVALID_ID
        );
        assert_eq!(
            check_active(&ids(&["no"])).unwrap_err().code,
            code::INVALID_ID
        );
        let many: Vec<String> = (0..=MAX_PACKS).map(|i| format!("p{i:04}")).collect();
        assert_eq!(check_active(&many).unwrap_err().code, code::LIMIT_EXCEEDED);
    }

    #[test]
    fn reports_cap_the_listed_errors() {
        let mut report = PackReport::new("x".into(), PackSource::User);
        for _ in 0..MAX_REPORTED_ERRORS + 3 {
            report.fail(PackError::pack(error(code::IO_ERROR)));
        }
        assert_eq!(report.state, PackState::Failed);
        assert_eq!(report.errors.len(), MAX_REPORTED_ERRORS);
        assert_eq!(report.errors_omitted, 3);
    }

    #[test]
    fn limit_errors_name_the_limit() {
        let reason = limit_error("files", 10_000);
        assert_eq!(reason.text_key, "datapack.error.limitExceeded");
        assert_eq!(
            reason.params.get("max"),
            Some(&crate::text::ParamValue::Number(10_000.0))
        );
    }
}
