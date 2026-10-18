//! File formats of a data pack (S-4): the manifest and the content files.
//!
//! These Rust types are the single source of the format: the loader reads
//! them with serde (stage 1, structure), the JSON Schema files for modders
//! are generated from them with schemars (feature `schema`, ADR-025) and the
//! TypeScript types with ts-rs (feature `ts`). Unknown fields are an error.
//! Stage 2 (values, ids, references) lives in `load.rs`.

use serde::{Deserialize, Serialize};

/// `manifest.json` of a data pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Manifest {
    /// Path or URL of the JSON Schema for editors; ignored by the game.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Identifier of the pack: 3 to 64 characters `a-z`, `0-9`, `_`, `-`.
    /// Entries are addressed as `<pack>:<entry>`.
    #[cfg_attr(feature = "schema", schemars(pattern(r"^[a-z0-9_-]{3,64}$")))]
    pub id: String,
    /// Display name of the pack.
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 4096)))]
    pub name: String,
    /// Version of the pack (Semantic Versioning, e.g. `1.2.0`).
    pub version: String,
    /// Version of the pack format (schema version); the current one is 1.
    #[cfg_attr(feature = "schema", schemars(range(min = 1)))]
    pub format: u32,
    /// Supported game versions as a range (e.g. `>=0.1.0, <2.0.0`).
    pub game: String,
    /// Packs this pack needs; they load before it.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

/// A pack another pack needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Dependency {
    /// Identifier of the needed pack.
    #[cfg_attr(feature = "schema", schemars(pattern(r"^[a-z0-9_-]{3,64}$")))]
    pub id: String,
    /// Accepted versions of the needed pack as a range (e.g. `^1.0`).
    pub version: String,
}

/// A content file in the folder `objects/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectsFile {
    /// Path or URL of the JSON Schema for editors; ignored by the game.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Object types of this file.
    pub objects: Vec<ObjectEntry>,
}

/// An object type: something the player builds (step 2.4 defines the
/// minimal fields; later steps add fields additively).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectEntry {
    /// Identifier within the pack: 1 to 64 characters `a-z`, `0-9`, `_`, `-`.
    #[cfg_attr(feature = "schema", schemars(pattern(r"^[a-z0-9_-]{1,64}$")))]
    pub id: String,
    /// Localisation key of the name (e.g. `object.bench`).
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 4096)))]
    pub name: String,
    /// How the object is placed.
    pub placement: Placement,
    /// Entry of another pack that this entry replaces (`<pack>:<entry>`);
    /// allowed only if this pack depends on that pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaces: Option<String>,
}

/// How an object type is placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum Placement {
    /// Drawn over an area of cells, such as floors, walls and zones
    /// (command `buildArea`).
    Area {},
    /// Placed as a single object with a fixed footprint (command `build`).
    Single {
        /// Width in metres along x at rotation 0: an even number from 2 to
        /// 1024 (2 m per cell, ADR-020; at most the command area of 512
        /// cells).
        #[cfg_attr(
            feature = "schema",
            schemars(range(min = 2, max = 1024), extend("multipleOf" = 2))
        )]
        width: u16,
        /// Depth in metres along y at rotation 0, like `width`.
        #[cfg_attr(
            feature = "schema",
            schemars(range(min = 2, max = 1024), extend("multipleOf" = 2))
        )]
        depth: u16,
    },
}
