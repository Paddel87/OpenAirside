//! File formats of a data pack (S-4): the manifest and the content files.
//!
//! These Rust types are the single source of the format: the loader reads
//! them with serde (stage 1, structure), the JSON Schema files for modders
//! are generated from them with schemars (feature `schema`, ADR-025) and the
//! TypeScript types with ts-rs (feature `ts`). Unknown fields are an error.
//! Stage 2 (values, ids, references) lives in `load.rs`.

use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::build::grid::{AreaKind, COST_NORMAL};

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
/// minimal fields; later steps add fields additively; `effect` and `blocks`
/// came with ADR-042, `work` and the effect `deliveryPoint` with ADR-045).
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
    #[cfg_attr(feature = "schema", schemars(with = "Placement"))]
    pub placement: PlacementSpec,
    /// What the object does: required for `area` (`floor` or `wall`,
    /// ADR-042); for `single` only `deliveryPoint` (ADR-045), else absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Option<Effect>"))]
    pub effect: Option<EffectSpec>,
    /// Whether a `single` object keeps persons from walking over its cells;
    /// default `true`, not allowed for `area` (ADR-042) and for a
    /// `deliveryPoint`, on which its workers stand (ADR-045).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<bool>,
    /// Work of the construction workers in game minutes, 1 to 1440: for
    /// `area` per cell, for `single` per object (ADR-045). Default 1 for a
    /// floor, 3 for a wall, 10 for a `single` object; not allowed for a
    /// `deliveryPoint`, which stands at once.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 1440)))]
    pub work: Option<u16>,
    /// Entry of another pack that this entry replaces (`<pack>:<entry>`);
    /// allowed only if this pack depends on that pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaces: Option<String>,
}

/// `effect` as read from the file: a plain struct for the same reason as
/// [`PlacementSpec`]. Stage 2 turns it into [`Effect`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EffectSpec {
    /// `floor`, `wall` or `deliveryPoint`.
    pub kind: EffectKind,
    /// What the floor is used for, only for `floor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_kind: Option<AreaKind>,
    /// Cost factor of the floor in tenths, only for `floor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<u8>,
    /// Construction workers of the company, only for `deliveryPoint`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<u8>,
}

/// Kind of an [`EffectSpec`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EffectKind {
    /// See [`Effect::Floor`].
    Floor,
    /// See [`Effect::Wall`].
    Wall,
    /// See [`Effect::DeliveryPoint`].
    DeliveryPoint,
}

/// What an object type does: an `area` type to the cells it is drawn over
/// (`floor`, `wall`), a `single` type as a whole (`deliveryPoint`). Further
/// kinds (zone, stairs, marking) come with the steps that build them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum Effect {
    /// Walkable floor: the cell shows this object type as floor covering.
    Floor {
        /// What the floor is used for, which decides who may walk on it
        /// (ADR-037); default `publicWalk`.
        #[serde(default)]
        area_kind: AreaKind,
        /// Cost factor in tenths, 1 to 40; default 10 (normal floor).
        #[serde(default = "default_floor_cost")]
        #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 40)))]
        cost: u8,
    },
    /// Wall: the cell cannot be walked on and shows this object type as
    /// wall.
    Wall {},
    /// Delivery point of a construction company (ADR-045), only for
    /// `single`: built at once, also on terrain; its construction workers
    /// fetch material here and carry it to the construction sites.
    DeliveryPoint {
        /// Construction workers of the company, 1 to 64; default 10.
        #[serde(default = "default_workers")]
        #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 64)))]
        workers: u8,
    },
}

fn default_floor_cost() -> u8 {
    COST_NORMAL
}

/// Construction workers of a delivery point without `workers` (ADR-045).
pub const DEFAULT_WORKERS: u8 = 10;
/// Most construction workers of one delivery point.
pub const MAX_WORKERS: u8 = crate::build::orders::MAX_WORKERS;
/// Longest work of one object type in game minutes.
pub const MAX_WORK: u16 = 1440;
/// Work of a floor per cell without `work`, in game minutes (ADR-045).
pub const DEFAULT_FLOOR_WORK: u16 = 1;
/// Work of a wall per cell without `work`, in game minutes (ADR-045).
pub const DEFAULT_WALL_WORK: u16 = 3;
/// Work of a single object without `work`, in game minutes (ADR-045).
pub const DEFAULT_SINGLE_WORK: u16 = 10;

fn default_workers() -> u8 {
    DEFAULT_WORKERS
}

/// `placement` as read from the file: a plain struct, so that errors name
/// the exact field with its position (serde buffers an internally tagged
/// enum and loses the JSON path). Stage 2 turns it into [`Placement`]; the
/// schema for modders shows [`Placement`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlacementSpec {
    /// `area` or `single`.
    pub kind: PlacementKind,
    /// Width in metres, only for `single`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    /// Depth in metres, only for `single`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<u16>,
}

/// Kind of a [`PlacementSpec`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlacementKind {
    /// See [`Placement::Area`].
    Area,
    /// See [`Placement::Single`].
    Single,
}

/// How an object type is placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
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

/// A content file in the folder `lang/` (step 2.6, ADR-031).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LangFile {
    /// Path or URL of the JSON Schema for editors; ignored by the game.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Texts of one or more languages.
    pub languages: Vec<LanguageEntry>,
}

/// Texts of one language. Several packs may add texts to the same language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LanguageEntry {
    /// Language code: 2 or 3 letters `a-z`, optionally `-` and 2 to 8
    /// characters `a-z`, `0-9` (e.g. `de`, `en`, `pt-br`).
    #[cfg_attr(
        feature = "schema",
        schemars(pattern(r"^[a-z]{2,3}(-[a-z0-9]{2,8})?$"))
    )]
    pub id: String,
    /// Name of the language in itself (e.g. `Deutsch`); required in the
    /// first loaded pack that adds texts to this language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 4096)))]
    pub name: Option<String>,
    /// Texts by localisation key (key: 1 to 128 characters `A-Z`, `a-z`,
    /// `0-9`, `_`, `.`, `-`). Parameters are written as `{name}`.
    #[cfg_attr(
        feature = "schema",
        schemars(with = "std::collections::BTreeMap<String, String>")
    )]
    pub texts: Texts,
}

/// Texts of a [`LanguageEntry`] in the order of the file. Unlike a map,
/// a key written twice is kept, so that stage 2 reports it
/// (`duplicateId`) instead of the later text winning silently.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Texts(pub Vec<(String, String)>);

impl Serialize for Texts {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(k, v)| (k, v)))
    }
}

impl<'de> Deserialize<'de> for Texts {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct TextsVisitor;

        impl<'de> Visitor<'de> for TextsVisitor {
            type Value = Texts;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an object of texts by key")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Texts, A::Error> {
                let mut out = Vec::new();
                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    out.push((key, value));
                }
                Ok(Texts(out))
            }
        }

        deserializer.deserialize_map(TextsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_keep_their_order_and_round_trip() {
        let file: LangFile = serde_json::from_str(
            r#"{"languages": [{"id": "de", "name": "Deutsch", "texts": {"b": "2", "a": "1", "b": "3"}}]}"#,
        )
        .expect("parse");
        let texts = &file.languages[0].texts;
        let keys: Vec<&str> = texts.0.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, ["b", "a", "b"]);
        let json = serde_json::to_string(&file).expect("serialise");
        assert_eq!(
            json,
            r#"{"languages":[{"id":"de","name":"Deutsch","texts":{"b":"2","a":"1","b":"3"}}]}"#
        );
    }

    #[test]
    fn texts_must_be_an_object() {
        let err =
            serde_json::from_str::<LangFile>(r#"{"languages": [{"id": "de", "texts": ["x"]}]}"#)
                .expect_err("a list is not an object");
        assert!(
            err.to_string().contains("an object of texts by key"),
            "{err}"
        );
    }
}
