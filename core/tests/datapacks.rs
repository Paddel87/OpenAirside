//! Data pack loader (S-4, FR-044): valid packs, load order, `replaces`, and
//! faulty packs that fail as a whole without a crash while the others load.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use openairside_core::datapack::{
    self, BASE_PACK_ID, Catalog, LoadOutcome, MAX_LIST_LEN, PackFolder, PackReport, PackSource,
    PackState, code, game_version, model::Placement,
};
use openairside_core::text::ParamValue;

/// A temporary folder, removed on drop (no extra crate, ADR-025).
struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "openairside-datapacks-{}-{name}-{n}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create temp dir");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
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

fn manifest(id: &str, version: &str, deps: &[(&str, &str)]) -> String {
    let deps: Vec<String> = deps
        .iter()
        .map(|(id, range)| format!(r#"{{"id": "{id}", "version": "{range}"}}"#))
        .collect();
    format!(
        r#"{{"id": "{id}", "name": "Pack {id}", "version": "{version}", "format": 1,
            "game": ">=0.0.0", "dependencies": [{}]}}"#,
        deps.join(", ")
    )
}

fn objects(entries: &[&str]) -> String {
    format!(r#"{{"objects": [{}]}}"#, entries.join(", "))
}

fn area(id: &str) -> String {
    format!(r#"{{"id": "{id}", "name": "object.{id}", "placement": {{"kind": "area"}}}}"#)
}

/// Writes a pack `id` with the given object entries into `dir/folder`.
fn pack(dir: &TempDir, folder: &str, manifest_text: &str, entries: &[&str]) {
    dir.write(&format!("{folder}/manifest.json"), manifest_text);
    dir.write(&format!("{folder}/objects/main.json"), &objects(entries));
}

/// The base pack of the repository plus the user packs in `user`.
fn load_with_base(user: &TempDir, active: &[&str]) -> LoadOutcome {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/base");
    let folders = datapack::discover(Some(&base), Some(user.path()));
    let active: Vec<String> = active.iter().map(|s| (*s).to_owned()).collect();
    datapack::load(&folders, &active, &game_version())
}

/// Only the user packs (no base pack).
fn load_user(user: &TempDir, active: &[&str]) -> LoadOutcome {
    let folders = datapack::discover(None, Some(user.path()));
    let active: Vec<String> = active.iter().map(|s| (*s).to_owned()).collect();
    datapack::load(&folders, &active, &game_version())
}

fn report<'a>(outcome: &'a LoadOutcome, folder: &str) -> &'a PackReport {
    outcome
        .reports
        .iter()
        .find(|r| r.folder == folder)
        .unwrap_or_else(|| panic!("no report for {folder}"))
}

fn codes(report: &PackReport) -> Vec<&str> {
    report
        .errors
        .iter()
        .map(|e| e.reason.code.as_str())
        .collect()
}

fn ids(catalog: &Catalog) -> Vec<&str> {
    catalog.object_ids().collect()
}

#[test]
fn base_pack_of_the_repository_loads() {
    let user = TempDir::new("base");
    let outcome = load_with_base(&user, &[]);
    let base = &outcome.reports[0];
    assert_eq!(base.source, PackSource::Base);
    assert_eq!(base.id.as_deref(), Some(BASE_PACK_ID));
    assert_eq!(base.state, PackState::Active, "{:?}", base.errors);
    assert_eq!(base.entries, 3);
    assert!(!outcome.catalog.base_failed());
    assert_eq!(
        ids(&outcome.catalog),
        ["base:bench", "base:floor_concrete", "base:wall_concrete"]
    );
    let bench = outcome.catalog.object("base:bench").expect("bench");
    assert_eq!(bench.placement, Placement::Single { width: 4, depth: 2 });
    assert_eq!(bench.name, "object.bench");
}

#[test]
fn user_packs_load_only_when_active_and_after_their_dependencies() {
    let user = TempDir::new("order");
    pack(
        &user,
        "a-first",
        &manifest("addon", "1.0.0", &[("library", "^2.1")]),
        &[&area("hall")],
    );
    pack(
        &user,
        "b-second",
        &manifest("library", "2.3.0", &[("base", "*")]),
        &[&area("tile")],
    );
    pack(
        &user,
        "c-idle",
        &manifest("idle", "1.0.0", &[]),
        &[&area("x")],
    );
    // The player lists the add-on before its library.
    let outcome = load_with_base(&user, &["addon", "library"]);
    for folder in ["a-first", "b-second"] {
        let r = report(&outcome, folder);
        assert_eq!(r.state, PackState::Active, "{folder}: {:?}", r.errors);
    }
    assert_eq!(report(&outcome, "c-idle").state, PackState::Inactive);
    assert!(outcome.catalog.object("addon:hall").is_some());
    assert!(outcome.catalog.object("library:tile").is_some());
    assert!(outcome.catalog.object("idle:x").is_none());
}

#[test]
fn replaces_redirects_the_target_to_the_replacing_entry() {
    let user = TempDir::new("replace");
    let wide = r#"{"id": "wide_bench", "name": "object.wide_bench",
        "placement": {"kind": "single", "width": 6, "depth": 2}, "replaces": "base:bench"}"#;
    pack(
        &user,
        "benches",
        &manifest("benches", "1.0.0", &[("base", "^0")]),
        &[wide],
    );
    let outcome = load_with_base(&user, &["benches"]);
    assert_eq!(report(&outcome, "benches").state, PackState::Active);
    let bench = outcome.catalog.object("base:bench").expect("bench");
    assert_eq!(bench.id, "benches:wide_bench");
}

#[test]
fn replace_errors_and_conflicts_fail_the_later_pack() {
    let user = TempDir::new("replace-errors");
    let replace = |id: &str, target: &str| {
        format!(
            r#"{{"id": "{id}", "name": "object.{id}", "placement": {{"kind": "area"}},
                "replaces": "{target}"}}"#
        )
    };
    pack(
        &user,
        "one",
        &manifest("one", "1.0.0", &[("base", "*")]),
        &[&replace("a", "base:floor_concrete")],
    );
    pack(
        &user,
        "two",
        &manifest("two", "1.0.0", &[("base", "*")]),
        &[&replace("b", "base:floor_concrete")],
    );
    pack(
        &user,
        "three",
        &manifest("three", "1.0.0", &[]),
        &[&replace("c", "base:wall_concrete")],
    );
    pack(
        &user,
        "four",
        &manifest("four", "1.0.0", &[("base", "*")]),
        &[&replace("d", "base:missing"), &replace("e", "nocolon")],
    );
    let outcome = load_with_base(&user, &["one", "two", "three", "four"]);
    assert_eq!(report(&outcome, "one").state, PackState::Active);
    let two = report(&outcome, "two");
    assert_eq!(codes(two), [code::REPLACE_CONFLICT]);
    assert_eq!(
        two.errors[0].reason.params.get("by"),
        Some(&ParamValue::Text("one:a".into()))
    );
    assert_eq!(two.errors[0].path.as_deref(), Some("objects[0].replaces"));
    assert_eq!(codes(report(&outcome, "three")), [code::REPLACE_TARGET]);
    assert_eq!(
        codes(report(&outcome, "four")),
        [code::REPLACE_TARGET, code::INVALID_ID]
    );
    // Nothing of the failed packs reached the catalog.
    assert!(outcome.catalog.object("two:b").is_none());
    assert_eq!(
        outcome
            .catalog
            .object("base:floor_concrete")
            .map(|o| o.id.as_str()),
        Some("one:a")
    );
}

#[test]
fn syntax_and_structure_errors_name_file_line_column_and_path() {
    let user = TempDir::new("structure");
    user.write("syntax/manifest.json", &manifest("syntax", "1.0.0", &[]));
    user.write(
        "syntax/objects/broken.json",
        "{\n  \"objects\": [\n    {\"id\": }\n  ]\n}",
    );
    pack(
        &user,
        "unknown",
        &manifest("unknown", "1.0.0", &[]),
        &[r#"{"id": "a", "name": "n", "placement": {"kind": "area"}, "colour": 3}"#],
    );
    pack(
        &user,
        "wrongtype",
        &manifest("wrongtype", "1.0.0", &[]),
        &[r#"{"id": "a", "name": "n", "placement": {"kind": "single", "width": 2.5, "depth": 2}}"#],
    );
    let outcome = load_user(&user, &["syntax", "unknown", "wrongtype"]);
    let syntax = report(&outcome, "syntax");
    assert_eq!(codes(syntax), [code::SYNTAX_ERROR]);
    let err = &syntax.errors[0];
    assert_eq!(err.file.as_deref(), Some("objects/broken.json"));
    assert_eq!((err.line, err.column), (Some(3), Some(12)));
    let unknown = &report(&outcome, "unknown").errors[0];
    assert_eq!(unknown.reason.code, code::INVALID_STRUCTURE);
    assert_eq!(unknown.path.as_deref(), Some("objects[0].colour"));
    assert!(matches!(
        unknown.reason.params.get("detail"),
        Some(ParamValue::Text(t)) if t.contains("unknown field `colour`")
    ));
    let wrong = &report(&outcome, "wrongtype").errors[0];
    assert_eq!(wrong.reason.code, code::INVALID_STRUCTURE);
    assert!(
        wrong
            .path
            .as_deref()
            .is_some_and(|p| p.starts_with("objects[0].placement"))
    );
}

#[test]
fn stage_two_checks_ids_values_and_duplicates_across_files() {
    let user = TempDir::new("stage2");
    user.write("p/manifest.json", &manifest("ppp", "1.0.0", &[]));
    user.write("p/objects/a.json", &objects(&[&area("dup"), &area("Bad")]));
    user.write(
        "p/objects/b.json",
        &objects(&[
            &area("dup"),
            r#"{"id": "odd", "name": " ", "placement": {"kind": "single", "width": 3, "depth": 2048}}"#,
        ]),
    );
    let outcome = load_user(&user, &["ppp"]);
    let r = report(&outcome, "p");
    assert_eq!(r.state, PackState::Failed);
    let found: Vec<(&str, Option<&str>, Option<&str>)> = r
        .errors
        .iter()
        .map(|e| (e.reason.code.as_str(), e.file.as_deref(), e.path.as_deref()))
        .collect();
    assert_eq!(
        found,
        [
            (
                code::INVALID_ID,
                Some("objects/a.json"),
                Some("objects[1].id")
            ),
            (
                code::DUPLICATE_ID,
                Some("objects/b.json"),
                Some("objects[0].id")
            ),
            (
                code::INVALID_VALUE,
                Some("objects/b.json"),
                Some("objects[1].name")
            ),
            (
                code::INVALID_VALUE,
                Some("objects/b.json"),
                Some("objects[1].placement.width")
            ),
            (
                code::INVALID_VALUE,
                Some("objects/b.json"),
                Some("objects[1].placement.depth")
            ),
        ]
    );
    assert_eq!(outcome.catalog.object_count(), 0);
}

#[test]
fn manifest_errors_fail_the_pack() {
    let user = TempDir::new("manifest");
    let cases = [
        (
            "bad-id",
            r#"{"id": "X", "name": "n", "version": "1.0.0", "format": 1, "game": "*"}"#,
            code::INVALID_ID,
        ),
        (
            "bad-version",
            r#"{"id": "bad", "name": "n", "version": "1", "format": 1, "game": "*"}"#,
            code::INVALID_VERSION,
        ),
        (
            "bad-range",
            r#"{"id": "bad", "name": "n", "version": "1.0.0", "format": 1, "game": "~>1"}"#,
            code::INVALID_VERSION,
        ),
        (
            "newer",
            r#"{"id": "newer", "name": "n", "version": "1.0.0", "format": 2, "game": "*"}"#,
            code::FORMAT_TOO_NEW,
        ),
        (
            "zero",
            r#"{"id": "zero", "name": "n", "version": "1.0.0", "format": 0, "game": "*"}"#,
            code::FORMAT_UNSUPPORTED,
        ),
        (
            "future-game",
            r#"{"id": "future", "name": "n", "version": "1.0.0", "format": 1, "game": ">=9.0.0"}"#,
            code::GAME_VERSION,
        ),
        (
            "self",
            r#"{"id": "self", "name": "n", "version": "1.0.0", "format": 1, "game": "*", "dependencies": [{"id": "self", "version": "*"}]}"#,
            code::DEPENDENCY_CYCLE,
        ),
        (
            "missing-field",
            r#"{"id": "miss", "name": "n", "version": "1.0.0", "game": "*"}"#,
            code::INVALID_STRUCTURE,
        ),
    ];
    for (folder, text, _) in cases {
        user.write(&format!("{folder}/manifest.json"), text);
    }
    fs::create_dir_all(user.path().join("no-manifest/objects")).expect("folder");
    let outcome = load_user(&user, &[]);
    for (folder, _, expected) in cases {
        let r = report(&outcome, folder);
        assert_eq!(r.state, PackState::Failed, "{folder}");
        assert_eq!(codes(r), [expected], "{folder}");
        assert_eq!(r.errors[0].file.as_deref(), Some("manifest.json"));
    }
    assert_eq!(
        codes(report(&outcome, "no-manifest")),
        [code::MANIFEST_MISSING]
    );
}

#[test]
fn dependency_problems_fail_the_pack_and_its_dependents() {
    let user = TempDir::new("deps");
    let simple = |dir: &str, id: &str, deps: &[(&str, &str)]| {
        pack(&user, dir, &manifest(id, "1.0.0", deps), &[&area("x")]);
    };
    simple("missing", "needs-ghost", &[("ghost", "*")]);
    simple("inactive", "needs-idle", &[("idle", "*")]);
    simple("idle", "idle", &[]);
    simple("version", "needs-new", &[("idle-two", ">=2")]);
    simple("idle-two", "idle-two", &[]);
    simple("cycle-a", "cyc-a", &[("cyc-b", "*")]);
    simple("cycle-b", "cyc-b", &[("cyc-a", "*")]);
    simple("behind-cycle", "behind", &[("cyc-a", "*")]);
    user.write("broken/manifest.json", &manifest("broken", "1.0.0", &[]));
    user.write("broken/objects/main.json", "not json");
    simple("on-broken", "on-broken", &[("broken", "*")]);
    simple("chain", "chain", &[("needs-ghost", "*")]);
    let active = [
        "needs-ghost",
        "needs-idle",
        "needs-new",
        "idle-two",
        "cyc-a",
        "cyc-b",
        "behind",
        "broken",
        "on-broken",
        "chain",
    ];
    let outcome = load_user(&user, &active);
    let expect = |folder: &str, wanted: &str| {
        let r = report(&outcome, folder);
        assert_eq!(r.state, PackState::Failed, "{folder}");
        assert_eq!(codes(r), [wanted], "{folder}");
    };
    expect("missing", code::DEPENDENCY_MISSING);
    expect("inactive", code::DEPENDENCY_INACTIVE);
    expect("version", code::DEPENDENCY_VERSION);
    expect("cycle-a", code::DEPENDENCY_CYCLE);
    expect("cycle-b", code::DEPENDENCY_CYCLE);
    expect("behind-cycle", code::DEPENDENCY_FAILED);
    expect("broken", code::SYNTAX_ERROR);
    expect("on-broken", code::DEPENDENCY_FAILED);
    expect("chain", code::DEPENDENCY_FAILED);
    assert_eq!(report(&outcome, "idle").state, PackState::Inactive);
    assert_eq!(report(&outcome, "idle-two").state, PackState::Active);
    assert_eq!(ids(&outcome.catalog), ["idle-two:x"]);
}

#[test]
fn a_second_folder_with_the_same_id_fails() {
    let user = TempDir::new("duplicate");
    pack(
        &user,
        "copy-a",
        &manifest("same", "1.0.0", &[]),
        &[&area("x")],
    );
    pack(
        &user,
        "copy-b",
        &manifest("same", "2.0.0", &[]),
        &[&area("x")],
    );
    pack(
        &user,
        "fake-base",
        &manifest("base", "9.0.0", &[]),
        &[&area("x")],
    );
    let outcome = load_with_base(&user, &["same", "base"]);
    assert_eq!(report(&outcome, "copy-a").state, PackState::Active);
    assert_eq!(codes(report(&outcome, "copy-b")), [code::DUPLICATE_PACK]);
    assert_eq!(codes(report(&outcome, "fake-base")), [code::DUPLICATE_PACK]);
    assert!(!outcome.catalog.base_failed());
}

#[test]
fn unknown_folders_later_kinds_and_nested_folders_are_refused() {
    let user = TempDir::new("folders");
    pack(&user, "p", &manifest("ppp", "1.0.0", &[]), &[&area("x")]);
    user.write("p/object/typo.json", "{}");
    user.write("p/aircraft/jets.json", "{}");
    user.write("p/objects/nested/deep.json", "{}");
    // Ignored: hidden entries, files in the root, non-JSON files.
    user.write("p/.DS_Store", "x");
    user.write("p/objects/.hidden.json", "not json");
    user.write("p/README.md", "# Pack");
    user.write("p/objects/notes.txt", "x");
    let outcome = load_user(&user, &["ppp"]);
    let r = report(&outcome, "p");
    let found: Vec<(&str, Option<&str>)> = r
        .errors
        .iter()
        .map(|e| (e.reason.code.as_str(), e.file.as_deref()))
        .collect();
    assert_eq!(
        found,
        [
            (code::KIND_NOT_SUPPORTED, Some("aircraft/")),
            (code::UNEXPECTED_ENTRY, Some("object/")),
            (code::UNEXPECTED_ENTRY, Some("objects/nested/")),
        ]
    );
    assert_eq!(
        r.errors[0].reason.params.get("step"),
        Some(&ParamValue::Text("3.6".into()))
    );
}

#[test]
fn json_limits_are_checked_before_parsing() {
    let user = TempDir::new("limits");
    let deep = format!("{}{}", "[".repeat(40), "]".repeat(40));
    user.write("deep/manifest.json", &manifest("deep", "1.0.0", &[]));
    user.write(
        "deep/objects/main.json",
        &format!(r#"{{"objects": {deep}}}"#),
    );
    let long_name = "n".repeat(4_097);
    pack(
        &user,
        "long",
        &manifest("long", "1.0.0", &[]),
        &[&format!(
            r#"{{"id": "a", "name": "{long_name}", "placement": {{"kind": "area"}}}}"#
        )],
    );
    let many: Vec<String> = (0..=MAX_LIST_LEN).map(|i| area(&format!("e{i}"))).collect();
    let many: Vec<&str> = many.iter().map(String::as_str).collect();
    pack(&user, "many", &manifest("many", "1.0.0", &[]), &many);
    let outcome = load_user(&user, &["deep", "long", "many"]);
    let limit = |folder: &str| {
        let r = report(&outcome, folder);
        assert_eq!(codes(r), [code::LIMIT_EXCEEDED], "{folder}");
        match r.errors[0].reason.params.get("limit") {
            Some(ParamValue::Text(name)) => name.clone(),
            other => panic!("{other:?}"),
        }
    };
    assert_eq!(limit("deep"), "depth");
    assert_eq!(limit("long"), "stringChars");
    assert_eq!(limit("many"), "listLength");
    let many = &report(&outcome, "many").errors[0];
    assert_eq!(many.path.as_deref(), Some("objects[65536]"));
}

#[test]
fn entry_and_file_limits_of_a_pack() {
    let user = TempDir::new("pack-limits");
    // 100,001 entries in two files (each list within 65,536).
    user.write("entries/manifest.json", &manifest("entries", "1.0.0", &[]));
    for (file, range) in [("a", 0..50_000), ("b", 50_000..100_001)] {
        let list: Vec<String> = range.map(|i| area(&format!("e{i}"))).collect();
        let list: Vec<&str> = list.iter().map(String::as_str).collect();
        user.write(&format!("entries/objects/{file}.json"), &objects(&list));
    }
    // A file above 16 MB.
    user.write("big/manifest.json", &manifest("big", "1.0.0", &[]));
    let padding = " ".repeat(16 * 1024 * 1024);
    user.write(
        "big/objects/big.json",
        &format!(r#"{{"objects": []{padding}}}"#),
    );
    let outcome = load_user(&user, &["entries", "big"]);
    let entries = report(&outcome, "entries");
    assert_eq!(codes(entries), [code::LIMIT_EXCEEDED]);
    assert_eq!(entries.errors[0].file.as_deref(), Some("objects/b.json"));
    let big = report(&outcome, "big");
    assert_eq!(codes(big), [code::LIMIT_EXCEEDED]);
    assert_eq!(
        big.errors[0].reason.params.get("limit"),
        Some(&ParamValue::Text("fileBytes".into()))
    );
}

#[test]
fn at_most_256_packs_are_read() {
    let user = TempDir::new("count");
    for i in 0..257 {
        user.write(
            &format!("p{i:03}/manifest.json"),
            &manifest(&format!("p{i:03}"), "1.0.0", &[]),
        );
    }
    let outcome = load_user(&user, &[]);
    assert_eq!(outcome.reports.len(), 257);
    assert_eq!(outcome.reports[255].state, PackState::Inactive);
    assert_eq!(codes(&outcome.reports[256]), [code::LIMIT_EXCEEDED]);
}

#[test]
fn byte_order_mark_and_schema_reference_are_accepted() {
    let user = TempDir::new("bom");
    user.write(
        "p/manifest.json",
        &format!("\u{feff}{}", manifest("bom", "1.0.0", &[])),
    );
    user.write(
        "p/objects/main.json",
        &format!(
            r#"{{"$schema": "../../schema/objects.schema.json", "objects": [{}]}}"#,
            area("x")
        ),
    );
    let outcome = load_user(&user, &["bom"]);
    assert_eq!(
        report(&outcome, "p").state,
        PackState::Active,
        "{:?}",
        report(&outcome, "p").errors
    );
}

#[test]
fn a_faulty_base_pack_is_reported_and_the_game_starts_without_it() {
    let base = TempDir::new("bad-base");
    base.write("manifest.json", "{");
    let folders = vec![PackFolder {
        path: base.path().to_path_buf(),
        source: PackSource::Base,
    }];
    let outcome = datapack::load(&folders, &[], &game_version());
    assert!(outcome.catalog.base_failed());
    assert_eq!(outcome.catalog.object_count(), 0);
    assert_eq!(codes(&outcome.reports[0]), [code::SYNTAX_ERROR]);
    // A missing base pack folder is reported the same way.
    let gone = vec![PackFolder {
        path: base.path().join("gone"),
        source: PackSource::Base,
    }];
    let outcome = datapack::load(&gone, &[], &game_version());
    assert!(outcome.catalog.base_failed());
    assert_eq!(codes(&outcome.reports[0]), [code::IO_ERROR]);
}

#[test]
fn reports_serialise_for_the_presentation() {
    let user = TempDir::new("serialise");
    user.write("p/manifest.json", "{");
    let outcome = load_user(&user, &[]);
    let json = serde_json::to_value(&outcome.reports[0]).expect("serialise");
    assert_eq!(json["folder"], "p");
    assert_eq!(json["source"], "user");
    assert_eq!(json["state"], "failed");
    assert_eq!(
        json["errors"][0]["reason"]["textKey"],
        "datapack.error.syntaxError"
    );
    assert_eq!(json["errorsOmitted"], 0);
}

#[cfg(unix)]
#[test]
fn links_are_refused() {
    use std::os::unix::fs::symlink;
    let user = TempDir::new("links");
    let outside = TempDir::new("outside");
    outside.write("secret.json", &objects(&[&area("x")]));
    pack(&user, "p", &manifest("ppp", "1.0.0", &[]), &[&area("y")]);
    symlink(
        outside.path().join("secret.json"),
        user.path().join("p/objects/link.json"),
    )
    .expect("symlink");
    pack(
        &user,
        "real",
        &manifest("real", "1.0.0", &[]),
        &[&area("y")],
    );
    symlink(user.path().join("real"), user.path().join("zz-linked-pack")).expect("symlink");
    let outcome = load_user(&user, &["ppp", "real"]);
    let r = report(&outcome, "p");
    assert_eq!(codes(r), [code::INVALID_PATH]);
    assert_eq!(r.errors[0].file.as_deref(), Some("objects/link.json"));
    assert_eq!(
        codes(report(&outcome, "zz-linked-pack")),
        [code::INVALID_PATH]
    );
    assert_eq!(report(&outcome, "real").state, PackState::Active);
}

#[cfg(unix)]
#[test]
fn reserved_names_are_refused_on_every_platform() {
    let user = TempDir::new("reserved");
    pack(&user, "p", &manifest("ppp", "1.0.0", &[]), &[&area("y")]);
    user.write("p/objects/nul.json", &objects(&[&area("z")]));
    user.write("p/objects/a:b.json", &objects(&[&area("z")]));
    let outcome = load_user(&user, &["ppp"]);
    let r = report(&outcome, "p");
    assert_eq!(codes(r), [code::INVALID_PATH, code::INVALID_PATH]);
}
