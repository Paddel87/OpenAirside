//! JSON Schema files of the pack format for modders (S-4 "Schema für
//! Modder", ADR-025): generated from the Rust types of `model.rs` with
//! schemars, so that editors show errors while a pack is written. The files
//! are checked in under `content/schema/` and compared in the CI
//! (`npm run check:schema`).

use schemars::schema_for;
use serde_json::Value;

use super::model::{LangFile, Manifest, ObjectsFile};

/// File name and schema of every file of the pack format.
#[must_use]
pub fn schemas() -> Vec<(&'static str, Value)> {
    vec![
        ("manifest.schema.json", Value::from(schema_for!(Manifest))),
        ("objects.schema.json", Value::from(schema_for!(ObjectsFile))),
        ("lang.schema.json", Value::from(schema_for!(LangFile))),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes the schemas to `$SCHEMA_EXPORT_DIR` if set (used by
    /// `scripts/gen-schema.mjs`); always checks their main properties.
    #[test]
    fn export_schemas() {
        let schemas = schemas();
        for (name, schema) in &schemas {
            assert_eq!(
                schema["$schema"], "https://json-schema.org/draft/2020-12/schema",
                "{name}"
            );
            assert_eq!(schema["additionalProperties"], false, "{name}");
        }
        let manifest = &schemas[0].1;
        assert_eq!(
            manifest["properties"]["id"]["pattern"],
            "^[a-z0-9_-]{3,64}$"
        );
        let lang = &schemas[2].1;
        let entry = &lang["$defs"]["LanguageEntry"];
        assert_eq!(
            entry["properties"]["id"]["pattern"],
            "^[a-z]{2,3}(-[a-z0-9]{2,8})?$"
        );
        assert_eq!(entry["properties"]["texts"]["type"], "object");
        if let Some(dir) = std::env::var_os("SCHEMA_EXPORT_DIR") {
            let dir = std::path::PathBuf::from(dir);
            std::fs::create_dir_all(&dir).expect("create export dir");
            for (name, schema) in schemas {
                let text = serde_json::to_string_pretty(&schema).expect("serialise schema");
                std::fs::write(dir.join(name), text + "\n").expect("write schema");
            }
        }
    }
}
