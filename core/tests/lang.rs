//! Language packs (S-4 `lang/`, FR-006, ADR-031): the texts of the base
//! pack cover every key the core produces, packs add languages and texts,
//! and faulty language files fail their pack as a whole without a crash.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use openairside_core::datapack::{
    self, LanguageInfo, LoadOutcome, PackError, PackReport, PackState, code, game_version,
};
use openairside_core::text::core_text_keys;

/// A temporary folder, removed on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "openairside-lang-{}-{name}-{n}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create temp dir");
        Self(path)
    }

    /// Writes `text` to `rel`, creating folders.
    fn write(&self, rel: &str, text: &str) {
        let path = self.0.join(rel);
        fs::create_dir_all(path.parent().expect("parent")).expect("create folders");
        fs::write(path, text).expect("write file");
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn base_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/base")
}

/// Writes a pack `id` that depends on nothing, with `files` as
/// `(path in the pack, content)`.
fn pack(dir: &TempDir, id: &str, files: &[(&str, &str)]) {
    dir.write(
        &format!("{id}/manifest.json"),
        &format!(
            r#"{{"id": "{id}", "name": "Pack {id}", "version": "1.0.0", "format": 1,
                "game": ">=0.0.0"}}"#
        ),
    );
    for (rel, text) in files {
        dir.write(&format!("{id}/{rel}"), text);
    }
}

fn load(user: &TempDir, active: &[&str]) -> LoadOutcome {
    let folders = datapack::discover(Some(&base_dir()), Some(&user.0));
    let active: Vec<String> = active.iter().map(|s| (*s).to_owned()).collect();
    datapack::load(&folders, &active, &game_version())
}

fn report<'a>(outcome: &'a LoadOutcome, id: &str) -> &'a PackReport {
    outcome
        .reports
        .iter()
        .find(|r| r.id.as_deref() == Some(id))
        .expect("report of the pack")
}

fn codes(report: &PackReport) -> Vec<&str> {
    report
        .errors
        .iter()
        .map(|e| e.reason.code.as_str())
        .collect()
}

fn first_error(report: &PackReport) -> &PackError {
    report.errors.first().expect("an error")
}

/// Names of the parameters `{name}` in a text.
fn placeholders(text: &str) -> BTreeSet<&str> {
    text.split('{')
        .skip(1)
        .filter_map(|part| part.split_once('}').map(|(name, _)| name))
        .collect()
}

#[test]
fn the_base_pack_ships_german_and_english() {
    let user = TempDir::new("base");
    let outcome = load(&user, &[]);
    let base = report(&outcome, "base");
    assert_eq!(base.state, PackState::Active, "{:?}", base.errors);
    let list = outcome.languages.list();
    let names: Vec<(&str, &str)> = list
        .iter()
        .map(|l| (l.code.as_str(), l.name.as_str()))
        .collect();
    assert_eq!(names, vec![("de", "Deutsch"), ("en", "English")]);
    assert_eq!(list[0].keys, list[1].keys);
    // Every text counts as an entry (S-4); the base pack has 3 objects.
    assert_eq!(base.entries, 3 + list[0].keys + list[1].keys);
}

#[test]
fn base_texts_cover_every_key_of_the_core_and_every_object_name() {
    let user = TempDir::new("cover");
    let outcome = load(&user, &[]);
    let mut wanted: BTreeSet<String> = core_text_keys().into_iter().collect();
    for id in outcome.catalog.object_ids() {
        let object = outcome.catalog.object(id).expect("object");
        wanted.insert(object.name.clone());
    }
    assert!(wanted.len() > 50, "{}", wanted.len());
    for language in ["de", "en"] {
        let texts = outcome.languages.texts(language).expect("language");
        let missing: Vec<&String> = wanted.iter().filter(|k| !texts.contains_key(*k)).collect();
        assert!(missing.is_empty(), "{language} misses {missing:?}");
    }
}

#[test]
fn german_and_english_have_the_same_keys_and_parameters() {
    let user = TempDir::new("same");
    let outcome = load(&user, &[]);
    let de = outcome.languages.texts("de").expect("de");
    let en = outcome.languages.texts("en").expect("en");
    assert_eq!(de.keys().collect::<Vec<_>>(), en.keys().collect::<Vec<_>>());
    for (key, text) in de {
        assert_eq!(placeholders(text), placeholders(&en[key]), "{key}");
    }
}

#[test]
fn a_pack_adds_a_language_and_texts_to_existing_ones() {
    let user = TempDir::new("add");
    pack(
        &user,
        "french",
        &[(
            "lang/fr.json",
            r#"{"languages": [
                {"id": "fr", "name": "Français", "texts": {"object.bench": "Banc"}},
                {"id": "de", "texts": {"mod.french.hello": "Hallo"}}
            ]}"#,
        )],
    );
    let outcome = load(&user, &["french"]);
    let french = report(&outcome, "french");
    assert_eq!(french.state, PackState::Active, "{:?}", french.errors);
    assert_eq!(french.entries, 2);
    let list = outcome.languages.list();
    assert_eq!(
        list.iter().map(|l| l.code.as_str()).collect::<Vec<_>>(),
        vec!["de", "en", "fr"]
    );
    assert_eq!(
        list[2],
        LanguageInfo {
            code: "fr".into(),
            name: "Français".into(),
            keys: 1
        }
    );
    let de = outcome.languages.texts("de").expect("de");
    assert_eq!(de["mod.french.hello"], "Hallo");
    assert_eq!(de["object.bench"], "Sitzbank");
    // Not active: no texts.
    let outcome = load(&user, &[]);
    assert!(outcome.languages.texts("fr").is_none());
}

#[test]
fn a_key_of_another_pack_fails_the_whole_pack() {
    let user = TempDir::new("dup");
    pack(
        &user,
        "clash",
        &[(
            "lang/de.json",
            r#"{"languages": [
                {"id": "it", "name": "Italiano", "texts": {"object.bench": "Panchina"}},
                {"id": "de", "texts": {"object.bench": "Bank"}}
            ]}"#,
        )],
    );
    let outcome = load(&user, &["clash"]);
    let clash = report(&outcome, "clash");
    assert_eq!(clash.state, PackState::Failed);
    assert_eq!(codes(clash), vec![code::DUPLICATE_ID]);
    let error = first_error(clash);
    assert_eq!(error.file.as_deref(), Some("lang/de.json"));
    assert_eq!(
        error.path.as_deref(),
        Some("languages[1].texts.object.bench")
    );
    assert_eq!(error.line, Some(3));
    // Nothing of the failed pack is loaded, not even its valid language.
    assert!(outcome.languages.texts("it").is_none());
    assert_eq!(
        outcome.languages.texts("de").expect("de")["object.bench"],
        "Sitzbank"
    );
}

#[test]
fn a_key_written_twice_in_one_file_is_reported() {
    let user = TempDir::new("twice");
    pack(
        &user,
        "twice",
        &[(
            "lang/x.json",
            r#"{"languages": [{"id": "nl", "name": "Nederlands",
                "texts": {"a.b": "een", "a.b": "twee"}}]}"#,
        )],
    );
    let outcome = load(&user, &["twice"]);
    let twice = report(&outcome, "twice");
    assert_eq!(codes(twice), vec![code::DUPLICATE_ID]);
    assert!(outcome.languages.texts("nl").is_none());
}

#[test]
fn invalid_language_entries_are_reported_with_their_place() {
    let user = TempDir::new("invalid");
    pack(
        &user,
        "faulty",
        &[(
            "lang/a.json",
            r#"{"languages": [
                {"id": "DE", "name": "x", "texts": {}},
                {"id": "sv", "texts": {"ok": "ja"}},
                {"id": "fi", "name": " ", "texts": {}},
                {"id": "da", "name": "Dansk", "texts": {"bad key": "x", "empty": " "}}
            ]}"#,
        )],
    );
    let outcome = load(&user, &["faulty"]);
    let faulty = report(&outcome, "faulty");
    assert_eq!(faulty.state, PackState::Failed);
    let found: Vec<(&str, Option<&str>)> = faulty
        .errors
        .iter()
        .map(|e| (e.reason.code.as_str(), e.path.as_deref()))
        .collect();
    assert_eq!(
        found,
        vec![
            (code::INVALID_ID, Some("languages[0].id")),
            (code::INVALID_STRUCTURE, Some("languages[1]")),
            (code::INVALID_VALUE, Some("languages[2].name")),
            (code::INVALID_ID, Some("languages[3].texts.bad key")),
            (code::INVALID_VALUE, Some("languages[3].texts.empty")),
        ]
    );
    assert!(faulty.errors.iter().all(|e| e.line.is_some()));
}

#[test]
fn a_new_language_named_in_an_earlier_file_of_the_pack_needs_no_name() {
    let user = TempDir::new("named");
    pack(
        &user,
        "split",
        &[
            (
                "lang/a.json",
                r#"{"languages": [{"id": "pl", "name": "Polski", "texts": {"x.a": "a"}}]}"#,
            ),
            (
                "lang/b.json",
                r#"{"languages": [{"id": "pl", "texts": {"x.b": "b"}}]}"#,
            ),
        ],
    );
    let outcome = load(&user, &["split"]);
    let split = report(&outcome, "split");
    assert_eq!(split.state, PackState::Active, "{:?}", split.errors);
    assert_eq!(outcome.languages.texts("pl").map(|t| t.len()), Some(2));
}

#[test]
fn the_structure_of_language_files_is_strict() {
    let user = TempDir::new("strict");
    pack(
        &user,
        "strict",
        &[(
            "lang/a.json",
            r#"{"languages": [{"id": "cs", "name": "Čeština", "texts": {"a": 1}}]}"#,
        )],
    );
    pack(
        &user,
        "extra",
        &[(
            "lang/a.json",
            r#"{"languages": [{"id": "cs", "name": "Čeština", "texts": {}, "rtl": true}]}"#,
        )],
    );
    let outcome = load(&user, &["strict", "extra"]);
    for id in ["strict", "extra"] {
        let r = report(&outcome, id);
        assert_eq!(codes(r), vec![code::INVALID_STRUCTURE], "{id}");
    }
}
