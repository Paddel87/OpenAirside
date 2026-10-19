//! Codes and localisation keys for errors and notices (`docs/architecture.md`
//! section 4, "Fehler und Texte"). The core never produces finished texts;
//! the presentation looks up `textKey` and inserts the parameters as text.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Value of a text parameter. Numbers stay numbers so that the presentation
/// can format them for the chosen language (FR-006).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(untagged)]
pub enum ParamValue {
    /// A number (finite, below 2^53).
    Number(f64),
    /// A plain text, never interpreted as markup.
    Text(String),
}

/// Every localisation key the core produces (FR-006): rejections of S-2,
/// the notice of S-3, query errors of S-7, pack errors of S-4 and save
/// errors of S-5. The base pack has a text for each in every language it
/// ships (`core/tests/lang.rs`).
#[must_use]
pub fn core_text_keys() -> Vec<String> {
    let mut keys = Vec::new();
    let mut add = |prefix: &str, codes: &[&str]| {
        keys.extend(codes.iter().map(|code| format!("{prefix}.{code}")));
    };
    add(
        crate::command::REJECT_TEXT_PREFIX,
        crate::command::code::ALL,
    );
    add("notice", &[crate::simulation::TICK_LIMIT]);
    add(crate::query::ERROR_TEXT_PREFIX, crate::query::code::ALL);
    add(
        crate::datapack::ERROR_TEXT_PREFIX,
        crate::datapack::code::ALL,
    );
    add(crate::savegame::TEXT_PREFIX, crate::savegame::code::ALL);
    keys
}

/// Parameters of a text, sorted by name for a deterministic order.
pub type Params = BTreeMap<String, ParamValue>;

/// Stable code plus localisation key of a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Reason {
    /// Stable identifier, e.g. `outOfMap`.
    pub code: String,
    /// Localisation key, e.g. `command.rejected.outOfMap`.
    pub text_key: String,
    /// Parameters for the text.
    pub params: Params,
}

impl Reason {
    /// Reason with the key `<prefix>.<code>` and no parameters.
    #[must_use]
    pub fn new(prefix: &str, code: &str) -> Self {
        Self {
            code: code.to_owned(),
            text_key: format!("{prefix}.{code}"),
            params: Params::new(),
        }
    }

    /// Adds a numeric parameter.
    #[must_use]
    pub fn with_number(mut self, name: &str, value: f64) -> Self {
        self.params
            .insert(name.to_owned(), ParamValue::Number(value));
        self
    }

    /// Adds a text parameter.
    #[must_use]
    pub fn with_text(mut self, name: &str, value: &str) -> Self {
        self.params
            .insert(name.to_owned(), ParamValue::Text(value.to_owned()));
        self
    }
}

/// Severity of a notice (S-3 `notice`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    /// Information; dropped first on overflow.
    Info,
    /// Warning; dropped after all `info` notices.
    Warning,
    /// Alert; never dropped.
    Alert,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reason_builds_key_from_prefix_and_code() {
        let reason = Reason::new("command.rejected", "areaTooLarge").with_number("max", 512.0);
        assert_eq!(reason.code, "areaTooLarge");
        assert_eq!(reason.text_key, "command.rejected.areaTooLarge");
        assert_eq!(reason.params.get("max"), Some(&ParamValue::Number(512.0)));
        let reason = reason.with_text("id", "base:bench");
        assert_eq!(
            reason.params.get("id"),
            Some(&ParamValue::Text("base:bench".into()))
        );
    }

    #[test]
    fn severities_are_ordered_by_importance() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Alert);
    }
}
