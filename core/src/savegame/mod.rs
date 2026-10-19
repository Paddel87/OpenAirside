//! Save games after contract S-5 (`docs/architecture.md` section 4): a
//! binary container `.oasave` with header, metadata, preview image, packed
//! world state and checksum (ADR-027).
//!
//! The core builds and checks the container; the platform supplies the
//! preview image, writes the file and hands it back for loading and, cut to
//! header and metadata, for the save game list. Flow fields are not saved:
//! the input of the fields in use is, and loading computes them again
//! (ADR-010).

mod container;
pub(crate) mod state;

use serde::{Deserialize, Serialize};

use crate::geometry::MapSize;
use crate::text::Reason;
use crate::time::Tick;

pub use container::{SaveInfo, decode, read_info};
pub use state::{LoadedGame, SaveSnapshot};

/// File name extension of save games.
pub const EXTENSION: &str = "oasave";
/// Identification at the start of every save game.
pub const MAGIC: &[u8; 6] = b"OASAVE";
/// Version of the container layout.
pub const CONTAINER_VERSION: u16 = 1;
/// Version of the world state format; rises with every change that is not
/// additive. Before version 1.0 older save games are rejected with
/// `migrationFailed` (S-5).
pub const FORMAT_VERSION: u32 = 1;

/// Largest save game file (1 GB).
pub const MAX_FILE_BYTES: u64 = 1 << 30;
/// Largest metadata section (64 KB).
pub const MAX_META_BYTES: u32 = 64 * 1024;
/// Largest preview image (2 MB).
pub const MAX_PREVIEW_BYTES: u32 = 2 * 1024 * 1024;
/// Largest preview image width in pixels.
pub const MAX_PREVIEW_WIDTH: u32 = 1920;
/// Largest preview image height in pixels.
pub const MAX_PREVIEW_HEIGHT: u32 = 1080;
/// Largest unpacked world state (2 GB).
pub const MAX_UNPACKED_BYTES: u32 = 1 << 31;
/// Longest save game name and airport name in characters.
pub const MAX_NAME_CHARS: usize = 64;
/// Longest save time text (ISO 8601) in characters.
pub const MAX_SAVED_AT_CHARS: usize = 40;
/// Bytes that hold header and metadata at most: what the platform reads of
/// each file for the save game list ([`read_info`]).
pub const MAX_INFO_BYTES: usize = 12 + 1 + 64 + 16 + MAX_META_BYTES as usize;
/// Edges of the maps a save game may have (ADR-020).
pub const MAP_EDGES: &[u16] = &[1536, 2048];
/// Most objects in a world and save game.
// TODO(fahrplan-ref: 3.1): enforced once objects exist in the world state.
pub const MAX_OBJECTS: usize = 1_000_000;
/// Most vehicles in a world and save game.
// TODO(fahrplan-ref: 3.4): enforced once vehicles exist in the world state.
pub const MAX_VEHICLES: usize = 50_000;

/// Codes of save and load errors (S-5 "Fehlercodes").
pub mod code {
    /// The file does not start with the identification.
    pub const BAD_MAGIC: &str = "badMagic";
    /// The file is shorter than its header or sections say.
    pub const TRUNCATED: &str = "truncated";
    /// The checksum does not match: the file is damaged.
    pub const CHECKSUM_MISMATCH: &str = "checksumMismatch";
    /// Saved by a newer version of the game.
    pub const VERSION_TOO_NEW: &str = "versionTooNew";
    /// An older format that cannot be converted (before 1.0: every older
    /// format).
    pub const MIGRATION_FAILED: &str = "migrationFailed";
    /// A data pack of the save game is not active.
    pub const PACK_MISSING: &str = "packMissing";
    /// A data pack is active with another major version.
    pub const PACK_VERSION_MISMATCH: &str = "packVersionMismatch";
    /// A size or count above its limit.
    pub const LIMIT_EXCEEDED: &str = "limitExceeded";
    /// The content breaks an invariant of the world state.
    pub const INVALID_STATE: &str = "invalidState";
    /// A field of the save request is out of range.
    pub const INVALID_ARGUMENT: &str = "invalidArgument";
    /// Writing failed (reported by the platform).
    pub const IO_ERROR: &str = "ioError";
    /// The disk is full (reported by the platform).
    pub const DISK_FULL: &str = "diskFull";
}

/// Prefix of the localisation keys of save and load errors.
pub const TEXT_PREFIX: &str = "save.error";

/// Reason of a save or load error.
#[must_use]
pub fn error(code: &str) -> Reason {
    Reason::new(TEXT_PREFIX, code)
}

fn limit(name: &str, max: f64) -> Reason {
    error(code::LIMIT_EXCEEDED)
        .with_text("limit", name)
        .with_number("max", max)
}

fn invalid(check: &str) -> Reason {
    error(code::INVALID_STATE).with_text("check", check)
}

/// How a game was saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum SaveKind {
    /// Saved by the player under a name.
    Manual,
    /// Quick save.
    Quick,
    /// Automatic save with rotation.
    Auto,
}

/// A data pack the save game was played with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct SavedPack {
    /// Pack id.
    pub id: String,
    /// Version of the pack (Semantic Versioning).
    pub version: String,
    /// Version of the pack format.
    pub format: u32,
}

/// Metadata of a save game, readable without the world state (S-5,
/// section 2 of the file); shown in the save game list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct SaveMeta {
    /// Name given by the player or the platform; only metadata, never a
    /// file name.
    pub name: String,
    /// How the game was saved.
    pub kind: SaveKind,
    /// Time of saving as ISO 8601 text, supplied by the platform.
    pub saved_at: String,
    /// Tick of the saved state.
    pub tick: Tick,
    /// Game day within the year, 1 … 12.
    pub day: u32,
    /// Game year, starting at 1.
    pub year: u32,
    /// Name of the airport.
    // TODO(fahrplan-ref: 5.1): the airport name (META-02) enters the world
    // state with the new-game settings; until then it is empty.
    pub airport_name: String,
    /// Size of the map.
    pub map: MapSize,
    /// Active data packs in load order.
    pub packs: Vec<SavedPack>,
}

/// What the platform hands over for saving.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveRequest {
    /// Name of the save game (at most [`MAX_NAME_CHARS`], no control
    /// characters).
    pub name: String,
    /// How the game is saved.
    pub kind: SaveKind,
    /// Time of saving as ISO 8601 text.
    pub saved_at: String,
    /// Preview image as PNG, or empty.
    pub preview: Vec<u8>,
}

/// Limits that depend on the game rather than on the file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// Allowed map edges; a map must be square with one of them.
    pub map_edges: &'static [u16],
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            map_edges: MAP_EDGES,
        }
    }
}

fn check_text(text: &str, max_chars: usize) -> bool {
    text.chars().count() <= max_chars && !text.chars().any(char::is_control)
}

fn check_saved_at(text: &str) -> bool {
    text.chars().count() <= MAX_SAVED_AT_CHARS
        && text
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '-' | ':' | '.' | 'T' | 'Z' | '+'))
}

/// Checks the fields of a save request.
///
/// # Errors
///
/// `invalidArgument` with the name of the field.
pub fn check_request(request: &SaveRequest) -> Result<(), Reason> {
    let field = |name: &str| error(code::INVALID_ARGUMENT).with_text("field", name);
    if !check_text(&request.name, MAX_NAME_CHARS) {
        return Err(field("name"));
    }
    if !check_saved_at(&request.saved_at) {
        return Err(field("savedAt"));
    }
    if request.preview.len() > MAX_PREVIEW_BYTES as usize {
        return Err(limit("preview", f64::from(MAX_PREVIEW_BYTES)));
    }
    container::check_preview(&request.preview)
}
