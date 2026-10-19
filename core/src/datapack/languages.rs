//! Texts of the loaded packs by language (S-4 `lang/`, step 2.6, ADR-031).
//!
//! The simulation never reads texts; they go to the presentation over the
//! platform API S-6 (`lang.list`, `lang.texts`). Several packs add texts to
//! the same language; a key exists once per language.

use std::collections::BTreeMap;

use serde::Serialize;

/// Longest localisation key in characters.
pub const MAX_KEY_CHARS: usize = 128;

/// A language for the choice in the settings (S-6 `lang.list`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct LanguageInfo {
    /// Language code, e.g. `de`.
    pub code: String,
    /// Name of the language in itself, e.g. `Deutsch`.
    pub name: String,
    /// Number of texts.
    pub keys: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Language {
    name: String,
    texts: BTreeMap<String, String>,
}

/// Texts of all loaded packs, by language code. Sorted maps keep the order
/// deterministic.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Languages {
    by_code: BTreeMap<String, Language>,
}

impl Languages {
    /// No texts.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// The languages sorted by code.
    #[must_use]
    pub fn list(&self) -> Vec<LanguageInfo> {
        self.by_code
            .iter()
            .map(|(code, language)| LanguageInfo {
                code: code.clone(),
                name: language.name.clone(),
                keys: u32::try_from(language.texts.len()).unwrap_or(u32::MAX),
            })
            .collect()
    }

    /// The texts of the language `code`, sorted by key.
    #[must_use]
    pub fn texts(&self, code: &str) -> Option<&BTreeMap<String, String>> {
        self.by_code.get(code).map(|language| &language.texts)
    }

    /// Whether the language `code` already has a name.
    pub(super) fn is_named(&self, code: &str) -> bool {
        self.by_code.contains_key(code)
    }

    /// Whether the language `code` already has the text `key`.
    pub(super) fn has_text(&self, code: &str, key: &str) -> bool {
        self.by_code
            .get(code)
            .is_some_and(|language| language.texts.contains_key(key))
    }

    /// Adds the texts of one checked pack. The first name of a language
    /// stays; the caller has checked that new languages carry a name and
    /// that no key exists yet.
    pub(super) fn add(&mut self, code: String, name: Option<String>, texts: Vec<(String, String)>) {
        let language = self.by_code.entry(code).or_insert_with(|| Language {
            name: name.unwrap_or_default(),
            texts: BTreeMap::new(),
        });
        language.texts.extend(texts);
    }
}

/// Checks the naming rule of a language code (`[a-z]{2,3}`, optionally
/// `-` and `[a-z0-9]{2,8}`).
#[must_use]
pub fn is_language_code(code: &str) -> bool {
    let (language, region) = match code.split_once('-') {
        Some((language, region)) => (language, Some(region)),
        None => (code, None),
    };
    (2..=3).contains(&language.len())
        && language.bytes().all(|b| b.is_ascii_lowercase())
        && region.is_none_or(|r| {
            (2..=8).contains(&r.len())
                && r.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

/// Checks the naming rule of a localisation key (`[A-Za-z0-9_.-]{1,128}`).
#[must_use]
pub fn is_text_key(key: &str) -> bool {
    (1..=MAX_KEY_CHARS).contains(&key.len())
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_codes_follow_the_naming_rule() {
        for ok in ["de", "en", "fil", "pt-br", "zh-hans", "es-419"] {
            assert!(is_language_code(ok), "{ok}");
        }
        for bad in [
            "", "d", "deut", "DE", "de-", "de-x", "de-BR", "de_br", "de-br-x", "-de", "dé",
        ] {
            assert!(!is_language_code(bad), "{bad}");
        }
    }

    #[test]
    fn text_keys_follow_the_naming_rule() {
        assert!(is_text_key("save.error.checksumMismatch"));
        assert!(is_text_key("ui.saves-panel_title"));
        assert!(is_text_key(&"a".repeat(MAX_KEY_CHARS)));
        assert!(!is_text_key(""));
        assert!(!is_text_key(&"a".repeat(MAX_KEY_CHARS + 1)));
        assert!(!is_text_key("a b"));
        assert!(!is_text_key("a:b"));
        assert!(!is_text_key("ä"));
    }

    #[test]
    fn texts_merge_by_language_and_keep_the_first_name() {
        let mut languages = Languages::empty();
        let pair = |k: &str, v: &str| (k.to_owned(), v.to_owned());
        languages.add(
            "de".into(),
            Some("Deutsch".into()),
            vec![pair("a", "A"), pair("b", "B")],
        );
        languages.add("en".into(), Some("English".into()), vec![pair("a", "a")]);
        languages.add("de".into(), Some("German".into()), vec![pair("c", "C")]);
        assert!(languages.is_named("de"));
        assert!(!languages.is_named("fr"));
        assert!(languages.has_text("de", "c"));
        assert!(!languages.has_text("en", "c"));
        assert_eq!(
            languages.list(),
            vec![
                LanguageInfo {
                    code: "de".into(),
                    name: "Deutsch".into(),
                    keys: 3
                },
                LanguageInfo {
                    code: "en".into(),
                    name: "English".into(),
                    keys: 1
                },
            ]
        );
        assert_eq!(
            languages
                .texts("de")
                .and_then(|t| t.get("c"))
                .map(String::as_str),
            Some("C")
        );
        assert!(languages.texts("fr").is_none());
    }
}
