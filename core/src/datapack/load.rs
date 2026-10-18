//! Loading of data packs (S-4): manifests, dependencies, load order, content
//! check in two stages and the atomic transfer into the catalog.
//!
//! Order of the work:
//! 1. Every found folder: read and check `manifest.json` (stage 1 and 2).
//! 2. A second folder with an id already seen fails (`duplicatePack`).
//! 3. Wanted packs: the base pack, then the player's active ids in order.
//! 4. Dependencies of wanted packs must be wanted, loadable and in range;
//!    a failure spreads to every pack that needs the failed one.
//! 5. Load order: dependencies first, otherwise the player's order; packs
//!    left over form or need a cycle (`dependencyCycle`).
//! 6. Content of each pack in load order: stage 1 (limits, structure) and
//!    stage 2 (ids, values, `replaces`); only a pack without any error
//!    reaches the catalog, as a whole.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use semver::{Version, VersionReq};
use serde::de::DeserializeOwned;

use super::catalog::{Catalog, ObjectType};
use super::files::{ContentFile, Folder, Kind, MANIFEST};
use super::model::{Manifest, ObjectEntry, ObjectsFile, Placement};
use super::scan::{self, Limit, Limits};
use super::{
    FORMAT, MAX_DEPTH, MAX_ENTRIES, MAX_LIST_LEN, MAX_PACKS, MAX_STRING_CHARS, PackError,
    PackReport, PackSource, PackState, code, error, is_entry_id, is_pack_id, limit_error,
};
use crate::text::Reason;

/// Limits of a JSON document (S-4).
const LIMITS: Limits = Limits {
    depth: MAX_DEPTH,
    string_chars: MAX_STRING_CHARS,
    list_len: MAX_LIST_LEN,
};

/// Smallest and largest footprint edge in metres (2 m per cell, ADR-020;
/// at most the command area of 512 cells, S-2).
const FOOTPRINT_METRES: (u16, u16) = (2, 1024);

/// Longest error detail taken from the JSON parser, in characters.
const MAX_DETAIL_CHARS: usize = 512;

/// A folder that may hold a pack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackFolder {
    /// Path of the pack folder.
    pub path: PathBuf,
    /// Origin of the pack.
    pub source: PackSource,
}

/// Result of [`load`].
#[derive(Debug, Clone)]
pub struct LoadOutcome {
    /// Content of all loaded packs.
    pub catalog: Catalog,
    /// One report per found folder, in the order of the folders.
    pub reports: Vec<PackReport>,
}

/// The pack folders: `base` (the base pack folder itself) and every folder
/// in `user_dir`, sorted by name. Hidden entries and files are skipped; a
/// missing `user_dir` holds no packs. Links are kept, so that loading
/// reports them.
#[must_use]
pub fn discover(base: Option<&Path>, user_dir: Option<&Path>) -> Vec<PackFolder> {
    let mut out: Vec<PackFolder> = base
        .map(|path| PackFolder {
            path: path.to_path_buf(),
            source: PackSource::Base,
        })
        .into_iter()
        .collect();
    let Some(dir) = user_dir else {
        return out;
    };
    let Ok(read) = std::fs::read_dir(dir) else {
        return out;
    };
    let mut user: Vec<PathBuf> = read
        .filter_map(Result::ok)
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
        .filter(|entry| {
            std::fs::symlink_metadata(entry.path())
                .is_ok_and(|m| m.is_dir() || m.file_type().is_symlink())
        })
        .map(|entry| entry.path())
        .collect();
    user.sort();
    out.extend(user.into_iter().map(|path| PackFolder {
        path,
        source: PackSource::User,
    }));
    out
}

/// A manifest that passed both stages.
struct Valid {
    id: String,
    version: Version,
    dependencies: Vec<(String, VersionReq)>,
}

struct Pack {
    report: PackReport,
    folder: Option<Folder>,
    valid: Option<Valid>,
}

impl Pack {
    fn failed(&self) -> bool {
        self.report.state == PackState::Failed
    }
}

/// Loads the packs in `folders`: the base packs always, user packs if their
/// id is in `active` (in that order). `game` is the running game version.
#[must_use]
pub fn load(folders: &[PackFolder], active: &[String], game: &Version) -> LoadOutcome {
    let mut packs: Vec<Pack> = folders
        .iter()
        .enumerate()
        .map(|(index, folder)| open(folder, index, game))
        .collect();
    reject_duplicates(&mut packs);
    let wanted = wanted(&packs, active);
    check_dependencies(&mut packs, &wanted);
    let order = load_order(&mut packs, &wanted);
    let mut catalog = Catalog::empty();
    for index in order {
        load_content(&mut packs, index, &mut catalog);
    }
    let base_failed = packs
        .iter()
        .any(|p| p.report.source == PackSource::Base && p.report.state != PackState::Active);
    catalog.set_base_failed(base_failed);
    LoadOutcome {
        catalog,
        reports: packs.into_iter().map(|p| p.report).collect(),
    }
}

/// The running game version as Semantic Versioning.
#[must_use]
pub fn game_version() -> Version {
    Version::parse(crate::VERSION).unwrap_or(Version::new(0, 0, 0))
}

fn open(folder: &PackFolder, index: usize, game: &Version) -> Pack {
    let name = folder
        .path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut report = PackReport::new(name, folder.source);
    if index >= MAX_PACKS {
        report.fail(PackError::pack(limit_error("packs", as_u32(MAX_PACKS))));
        return Pack {
            report,
            folder: None,
            valid: None,
        };
    }
    let dir = match Folder::open(&folder.path) {
        Ok(dir) => dir,
        Err(reason) => {
            report.fail(PackError::pack(reason));
            return Pack {
                report,
                folder: None,
                valid: None,
            };
        }
    };
    let mut errors = Vec::new();
    let valid = dir
        .manifest(&mut errors)
        .and_then(|bytes| parse::<Manifest>(MANIFEST, &bytes, &mut errors))
        .and_then(|manifest| check_manifest(manifest, game, &mut report, &mut errors));
    for err in errors {
        report.fail(err);
    }
    Pack {
        valid: if report.state == PackState::Failed {
            None
        } else {
            valid
        },
        report,
        folder: Some(dir),
    }
}

/// Stage 1 of one file: limits, then structure with JSON path.
fn parse<T: DeserializeOwned>(file: &str, bytes: &[u8], errors: &mut Vec<PackError>) -> Option<T> {
    // A byte order mark is allowed in UTF-8 (RFC 8259 permits ignoring it).
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    if let Err(violation) = scan::scan(bytes, LIMITS) {
        let (limit, max) = match violation.limit {
            Limit::Depth => ("depth", MAX_DEPTH),
            Limit::StringChars => ("stringChars", MAX_STRING_CHARS),
            Limit::ListLen => ("listLength", MAX_LIST_LEN),
        };
        errors.push(PackError {
            file: Some(file.to_owned()),
            line: Some(violation.line),
            column: Some(violation.column),
            path: Some(violation.path),
            reason: limit_error(limit, as_u32(max)),
        });
        return None;
    }
    let mut de = serde_json::Deserializer::from_slice(bytes);
    let result = serde_path_to_error::deserialize::<_, T>(&mut de)
        .map_err(|err| {
            let path = err.path().to_string();
            (Some(path), err.into_inner())
        })
        .and_then(|value| de.end().map(|()| value).map_err(|err| (None, err)));
    match result {
        Ok(value) => Some(value),
        Err((path, err)) => {
            let code = match err.classify() {
                serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                    code::SYNTAX_ERROR
                }
                serde_json::error::Category::Data => code::INVALID_STRUCTURE,
                serde_json::error::Category::Io => code::IO_ERROR,
            };
            let place = |n: usize| (n > 0).then(|| as_u32(n));
            errors.push(PackError {
                file: Some(file.to_owned()),
                line: place(err.line()),
                column: place(err.column()),
                path: if code == code::SYNTAX_ERROR {
                    None
                } else {
                    path
                },
                reason: error(code).with_text("detail", &detail(&err)),
            });
            None
        }
    }
}

/// Message of the parser without its position (the error carries it).
fn detail(err: &serde_json::Error) -> String {
    let text = err.to_string();
    let message = text
        .rsplit_once(" at line ")
        .map_or(text.as_str(), |(m, _)| m);
    message.chars().take(MAX_DETAIL_CHARS).collect()
}

/// Stage 2 of the manifest.
fn check_manifest(
    manifest: Manifest,
    game: &Version,
    report: &mut PackReport,
    errors: &mut Vec<PackError>,
) -> Option<Valid> {
    let at = |path: &str, reason: Reason| PackError {
        path: Some(path.to_owned()),
        ..PackError::file(MANIFEST, reason)
    };
    let before = errors.len();
    if is_pack_id(&manifest.id) {
        report.id = Some(manifest.id.clone());
    } else {
        errors.push(at("id", error(code::INVALID_ID)));
    }
    if manifest.name.trim().is_empty() {
        errors.push(at("name", error(code::INVALID_VALUE)));
    } else {
        report.name = Some(manifest.name.clone());
    }
    let version = Version::parse(&manifest.version);
    match &version {
        Ok(_) => report.version = Some(manifest.version.clone()),
        Err(_) => errors.push(at("version", error(code::INVALID_VERSION))),
    }
    if manifest.format == 0 {
        errors.push(at("format", error(code::FORMAT_UNSUPPORTED)));
    } else if manifest.format > FORMAT {
        errors.push(at(
            "format",
            error(code::FORMAT_TOO_NEW)
                .with_number("format", f64::from(manifest.format))
                .with_number("supported", f64::from(FORMAT)),
        ));
    }
    match VersionReq::parse(&manifest.game) {
        Ok(range) if !range.matches(game) => errors.push(at(
            "game",
            error(code::GAME_VERSION)
                .with_text("range", &manifest.game)
                .with_text("game", &game.to_string()),
        )),
        Ok(_) => {}
        Err(_) => errors.push(at("game", error(code::INVALID_VERSION))),
    }
    let mut dependencies = Vec::new();
    let mut seen = BTreeSet::new();
    for (i, dep) in manifest.dependencies.iter().enumerate() {
        let path = format!("dependencies[{i}]");
        if !is_pack_id(&dep.id) {
            errors.push(at(&format!("{path}.id"), error(code::INVALID_ID)));
            continue;
        }
        if dep.id == manifest.id {
            errors.push(at(&path, error(code::DEPENDENCY_CYCLE)));
            continue;
        }
        if !seen.insert(dep.id.as_str()) {
            errors.push(at(&path, error(code::INVALID_VALUE)));
            continue;
        }
        match VersionReq::parse(&dep.version) {
            Ok(range) => dependencies.push((dep.id.clone(), range)),
            Err(_) => errors.push(at(&format!("{path}.version"), error(code::INVALID_VERSION))),
        }
    }
    if errors.len() > before {
        return None;
    }
    Some(Valid {
        id: manifest.id,
        version: version.ok()?,
        dependencies,
    })
}

fn reject_duplicates(packs: &mut [Pack]) {
    let mut seen = BTreeSet::new();
    for pack in packs.iter_mut() {
        if let Some(valid) = &pack.valid
            && !seen.insert(valid.id.clone())
        {
            let reason = error(code::DUPLICATE_PACK).with_text("id", &valid.id);
            pack.report.fail(PackError::pack(reason));
            pack.valid = None;
        }
    }
}

/// Indices of the wanted packs in the player's order: base packs first.
fn wanted(packs: &[Pack], active: &[String]) -> Vec<usize> {
    let mut out: Vec<usize> = packs
        .iter()
        .enumerate()
        .filter(|(_, p)| p.report.source == PackSource::Base && p.valid.is_some())
        .map(|(i, _)| i)
        .collect();
    for id in active {
        let found = packs.iter().position(|p| {
            p.report.source == PackSource::User && p.valid.as_ref().is_some_and(|v| &v.id == id)
        });
        if let Some(index) = found
            && !out.contains(&index)
        {
            out.push(index);
        }
    }
    out
}

fn check_dependencies(packs: &mut [Pack], wanted: &[usize]) {
    // Repeat until no further pack fails: a failure spreads to dependents.
    loop {
        let mut changed = false;
        for &index in wanted {
            if packs[index].failed() {
                continue;
            }
            let Some(valid) = &packs[index].valid else {
                continue;
            };
            let problem = valid.dependencies.iter().find_map(|(id, range)| {
                let failed_with_id = packs
                    .iter()
                    .any(|p| p.failed() && p.report.id.as_deref() == Some(id.as_str()));
                match packs
                    .iter()
                    .position(|p| p.valid.as_ref().is_some_and(|v| &v.id == id))
                {
                    Some(dep) if packs[dep].failed() => Some(failed_dependency(id)),
                    Some(dep) if !wanted.contains(&dep) => {
                        Some(error(code::DEPENDENCY_INACTIVE).with_text("id", id))
                    }
                    Some(dep) => {
                        let version = &packs[dep].valid.as_ref()?.version;
                        (!range.matches(version)).then(|| {
                            error(code::DEPENDENCY_VERSION)
                                .with_text("id", id)
                                .with_text("version", &version.to_string())
                                .with_text("range", &range.to_string())
                        })
                    }
                    None if failed_with_id => Some(failed_dependency(id)),
                    None => Some(error(code::DEPENDENCY_MISSING).with_text("id", id)),
                }
            });
            if let Some(reason) = problem {
                packs[index].report.fail(PackError::file(MANIFEST, reason));
                changed = true;
            }
        }
        if !changed {
            return;
        }
    }
}

fn failed_dependency(id: &str) -> Reason {
    error(code::DEPENDENCY_FAILED).with_text("id", id)
}

/// Wanted packs in load order: a pack comes after its dependencies, else in
/// the order of `wanted`. Packs in or behind a cycle fail.
fn load_order(packs: &mut [Pack], wanted: &[usize]) -> Vec<usize> {
    let deps_of = |packs: &[Pack], index: usize| -> Vec<usize> {
        packs[index].valid.as_ref().map_or_else(Vec::new, |v| {
            v.dependencies
                .iter()
                .filter_map(|(id, _)| {
                    packs
                        .iter()
                        .position(|p| p.valid.as_ref().is_some_and(|w| &w.id == id))
                })
                .collect()
        })
    };
    let mut left: Vec<usize> = wanted
        .iter()
        .copied()
        .filter(|&i| !packs[i].failed())
        .collect();
    let mut order = Vec::new();
    while let Some(pos) = left
        .iter()
        .position(|&i| deps_of(packs, i).iter().all(|d| order.contains(d)))
    {
        order.push(left.remove(pos));
    }
    let edges: HashMap<usize, Vec<usize>> = left.iter().map(|&i| (i, deps_of(packs, i))).collect();
    for &index in &left {
        let reason = if reaches(&edges, index, index) {
            error(code::DEPENDENCY_CYCLE)
        } else {
            let blocker = edges[&index]
                .iter()
                .find(|d| left.contains(d))
                .and_then(|&d| packs[d].report.id.clone())
                .unwrap_or_default();
            failed_dependency(&blocker)
        };
        packs[index].report.fail(PackError::file(MANIFEST, reason));
    }
    order
}

/// Whether `target` is reachable from the dependencies of `from`.
fn reaches(edges: &HashMap<usize, Vec<usize>>, from: usize, target: usize) -> bool {
    let mut stack: Vec<usize> = edges.get(&from).cloned().unwrap_or_default();
    let mut seen = BTreeSet::new();
    while let Some(node) = stack.pop() {
        if node == target {
            return true;
        }
        if seen.insert(node) {
            stack.extend(edges.get(&node).into_iter().flatten().copied());
        }
    }
    false
}

fn load_content(packs: &mut [Pack], index: usize, catalog: &mut Catalog) {
    let Some(valid) = &packs[index].valid else {
        return;
    };
    // A dependency may have failed in its content stage.
    let dependencies: Vec<String> = valid
        .dependencies
        .iter()
        .map(|(id, _)| id.clone())
        .collect();
    let failed = dependencies.iter().find(|id| {
        !packs.iter().any(|p| {
            p.report.state == PackState::Active && p.report.id.as_deref() == Some(id.as_str())
        })
    });
    if let Some(id) = failed {
        let reason = failed_dependency(id);
        packs[index].report.fail(PackError::file(MANIFEST, reason));
        return;
    }
    let pack_id = valid.id.clone();
    let Some(folder) = &packs[index].folder else {
        return;
    };
    let mut errors = Vec::new();
    let files = folder.content(&mut errors);
    let mut stage = Stage {
        pack: &pack_id,
        dependencies: &dependencies,
        catalog: &*catalog,
        ids: BTreeSet::new(),
        objects: Vec::new(),
        replaces: Vec::new(),
        entries: 0,
        errors,
    };
    stage.check_files(files);
    let Stage {
        objects,
        replaces,
        entries,
        errors,
        ..
    } = stage;
    let report = &mut packs[index].report;
    if !errors.is_empty() {
        for err in errors {
            report.fail(err);
        }
        return;
    }
    for object in objects {
        catalog.insert(object);
    }
    for (target, by) in replaces {
        catalog.replace(target, by);
    }
    report.state = PackState::Active;
    report.entries = as_u32(entries);
}

/// Entries of one pack checked so far, not yet in the catalog.
struct Stage<'a> {
    pack: &'a str,
    dependencies: &'a [String],
    catalog: &'a Catalog,
    ids: BTreeSet<String>,
    objects: Vec<ObjectType>,
    replaces: Vec<(String, String)>,
    entries: usize,
    errors: Vec<PackError>,
}

impl Stage<'_> {
    fn check_files(&mut self, files: Vec<ContentFile>) {
        for file in files {
            match file.kind {
                Kind::Objects => {
                    let Some(parsed) =
                        parse::<ObjectsFile>(&file.file, &file.bytes, &mut self.errors)
                    else {
                        continue;
                    };
                    for (i, entry) in parsed.objects.into_iter().enumerate() {
                        self.entries += 1;
                        if self.entries > MAX_ENTRIES {
                            let reason = limit_error("entries", as_u32(MAX_ENTRIES));
                            self.errors.push(PackError::file(&file.file, reason));
                            return;
                        }
                        self.check_object(&file.file, &format!("objects[{i}]"), entry);
                    }
                }
            }
        }
    }

    fn at(&mut self, file: &str, path: &str, reason: Reason) {
        self.errors.push(PackError {
            path: Some(path.to_owned()),
            ..PackError::file(file, reason)
        });
    }

    fn check_object(&mut self, file: &str, path: &str, entry: ObjectEntry) {
        let before = self.errors.len();
        if !is_entry_id(&entry.id) {
            self.at(file, &format!("{path}.id"), error(code::INVALID_ID));
        }
        let full = format!("{}:{}", self.pack, entry.id);
        if is_entry_id(&entry.id) && !self.ids.insert(full.clone()) {
            let reason = error(code::DUPLICATE_ID).with_text("id", &full);
            self.at(file, &format!("{path}.id"), reason);
        }
        if entry.name.trim().is_empty() {
            self.at(file, &format!("{path}.name"), error(code::INVALID_VALUE));
        }
        if let Placement::Single { width, depth } = entry.placement {
            for (field, metres) in [("width", width), ("depth", depth)] {
                let (min, max) = FOOTPRINT_METRES;
                if metres < min || metres > max || metres % 2 != 0 {
                    let reason = error(code::INVALID_VALUE)
                        .with_number("min", f64::from(min))
                        .with_number("max", f64::from(max));
                    self.at(file, &format!("{path}.placement.{field}"), reason);
                }
            }
        }
        if let Some(target) = &entry.replaces {
            self.check_replace(file, &format!("{path}.replaces"), target, &full);
        }
        if self.errors.len() == before {
            self.objects.push(ObjectType {
                id: full,
                name: entry.name,
                placement: entry.placement,
            });
        }
    }

    fn check_replace(&mut self, file: &str, path: &str, target: &str, by: &str) {
        let Some((pack, entry)) = target.split_once(':') else {
            self.at(file, path, error(code::INVALID_ID));
            return;
        };
        if !is_pack_id(pack) || !is_entry_id(entry) {
            self.at(file, path, error(code::INVALID_ID));
            return;
        }
        if !self.dependencies.iter().any(|d| d == pack) || !self.catalog.contains(target) {
            let reason = error(code::REPLACE_TARGET).with_text("target", target);
            self.at(file, path, reason);
            return;
        }
        let earlier = self
            .catalog
            .replacement_of(target)
            .map(str::to_owned)
            .or_else(|| {
                self.replaces
                    .iter()
                    .find(|(t, _)| t == target)
                    .map(|(_, b)| b.clone())
            });
        if let Some(earlier) = earlier {
            let reason = error(code::REPLACE_CONFLICT)
                .with_text("target", target)
                .with_text("by", &earlier);
            self.at(file, path, reason);
            return;
        }
        self.replaces.push((target.to_owned(), by.to_owned()));
    }
}

fn as_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
