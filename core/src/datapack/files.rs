//! Reading the files of a pack folder (S-4 "Pfade" and "Grenzen").
//!
//! The loader never takes a path from a pack: it enumerates the pack folder
//! itself, in a fixed order (sorted by name), and reads only `manifest.json`
//! and `*.json` files directly inside the folders of the known kinds.
//! Symbolic links, Windows reparse points (junctions), reserved and invalid
//! names are refused on every platform, so that a pack valid on one
//! platform is valid on all. Each file is resolved canonically and must lie
//! below the pack folder; it is read through the handle opened for the
//! check, after comparing that handle with the enumerated entry.

use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use super::{MAX_FILE_BYTES, MAX_FILES, MAX_PACK_BYTES, PackError, code, error, limit_error};
use crate::text::{ParamValue, Reason};

/// Name of the manifest file.
pub const MANIFEST: &str = "manifest.json";

/// Kinds of content with a defined format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Object types (`objects/`).
    Objects,
    /// Texts by language (`lang/`, step 2.6).
    Lang,
}

/// Kind folders of S-4 whose format a later step defines (ADR-025), with
/// that step.
const LATER_KINDS: [(&str, &str); 4] = [
    ("aircraft", "3.6"),
    ("airlines", "3.6"),
    ("vehicles", "3.4"),
    ("research", "5.6"),
];

/// A content file read from a pack.
#[derive(Debug)]
pub struct ContentFile {
    /// Kind of the content.
    pub kind: Kind,
    /// Path relative to the pack folder with `/`.
    pub file: String,
    /// The bytes of the file.
    pub bytes: Vec<u8>,
}

/// Bytes and files read so far from one pack.
#[derive(Debug, Default)]
struct Budget {
    bytes: u64,
    files: usize,
}

/// A checked pack folder: its canonical path.
pub struct Folder {
    root: PathBuf,
}

impl Folder {
    /// Checks the pack folder itself: a real directory, no link.
    ///
    /// # Errors
    ///
    /// Returns `invalidPath` or `ioError`.
    pub fn open(path: &Path) -> Result<Self, Reason> {
        let meta = fs::symlink_metadata(path).map_err(|_| error(code::IO_ERROR))?;
        if is_link(&meta) || !meta.is_dir() {
            return Err(error(code::INVALID_PATH));
        }
        let root = fs::canonicalize(path).map_err(|_| error(code::IO_ERROR))?;
        Ok(Self { root })
    }

    /// Reads `manifest.json`; `None` after reporting an error. The name must
    /// match exactly on every platform (`Manifest.json` is missing on all,
    /// not only on case-sensitive file systems).
    pub fn manifest(&self, errors: &mut Vec<PackError>) -> Option<Vec<u8>> {
        let Ok(found) = has_exact_entry(&self.root, MANIFEST) else {
            errors.push(PackError::file(MANIFEST, error(code::IO_ERROR)));
            return None;
        };
        if !found {
            errors.push(PackError::file(MANIFEST, error(code::MANIFEST_MISSING)));
            return None;
        }
        let path = self.root.join(MANIFEST);
        let Ok(meta) = fs::symlink_metadata(&path) else {
            errors.push(PackError::file(MANIFEST, error(code::IO_ERROR)));
            return None;
        };
        let mut budget = Budget::default();
        match self.read(&path, &meta, &mut budget) {
            Ok(bytes) => Some(bytes),
            Err(reason) => {
                errors.push(PackError::file(MANIFEST, reason));
                None
            }
        }
    }

    /// Enumerates and reads all content files. Stops at the first limit of
    /// the whole pack; other errors are collected and reading goes on.
    pub fn content(&self, errors: &mut Vec<PackError>) -> Vec<ContentFile> {
        let mut out = Vec::new();
        // The manifest counts towards the limits of the pack.
        let mut budget = Budget {
            bytes: fs::symlink_metadata(self.root.join(MANIFEST)).map_or(0, |m| m.len()),
            files: 1,
        };
        let Some(entries) = list(&self.root, "", errors) else {
            return out;
        };
        for (name, meta) in entries {
            if meta.is_file() {
                // Other files in the root (readme, licence) are not read.
                continue;
            }
            let kind = match name.as_str() {
                "objects" => Kind::Objects,
                "lang" => Kind::Lang,
                other => {
                    let reason = match LATER_KINDS.iter().find(|(k, _)| *k == other) {
                        Some((kind, step)) => error(code::KIND_NOT_SUPPORTED)
                            .with_text("kind", kind)
                            .with_text("step", step),
                        None => error(code::UNEXPECTED_ENTRY),
                    };
                    errors.push(PackError::file(&format!("{name}/"), reason));
                    continue;
                }
            };
            let Some(files) = list(&self.root.join(&name), &name, errors) else {
                continue;
            };
            for (file_name, file_meta) in files {
                let rel = format!("{name}/{file_name}");
                if file_meta.is_dir() {
                    errors.push(PackError::file(
                        &format!("{rel}/"),
                        error(code::UNEXPECTED_ENTRY),
                    ));
                    continue;
                }
                if !file_name.ends_with(".json") {
                    // `.JSON` would be read on no platform: say so instead
                    // of passing over it.
                    if file_name.to_ascii_lowercase().ends_with(".json") {
                        errors.push(PackError::file(&rel, error(code::UNEXPECTED_ENTRY)));
                    }
                    continue;
                }
                if budget.files >= MAX_FILES {
                    errors.push(PackError::pack(limit_error("files", as_u32(MAX_FILES))));
                    return out;
                }
                budget.files += 1;
                let path = self.root.join(&name).join(&file_name);
                match self.read(&path, &file_meta, &mut budget) {
                    Ok(bytes) => out.push(ContentFile {
                        kind,
                        file: rel,
                        bytes,
                    }),
                    Err(reason) => {
                        let stop = reason.params.get("limit")
                            == Some(&ParamValue::Text("packBytes".into()));
                        errors.push(PackError::file(&rel, reason));
                        if stop {
                            return out;
                        }
                    }
                }
            }
        }
        out
    }

    /// Reads one enumerated file through a checked handle.
    fn read(&self, path: &Path, listed: &Metadata, budget: &mut Budget) -> Result<Vec<u8>, Reason> {
        let io_error = |_| error(code::IO_ERROR);
        // Only regular files: opening a FIFO or device would block or never end.
        if !listed.is_file() {
            return Err(error(code::INVALID_PATH));
        }
        let canonical = fs::canonicalize(path).map_err(io_error)?;
        if !canonical.starts_with(&self.root) {
            return Err(error(code::INVALID_PATH));
        }
        let file = open_no_follow(path).map_err(io_error)?;
        let meta = file.metadata().map_err(io_error)?;
        if !meta.is_file() || is_link(&meta) || !same_file(&meta, listed) || has_other_links(&meta)
        {
            return Err(error(code::INVALID_PATH));
        }
        let remaining = MAX_PACK_BYTES.saturating_sub(budget.bytes);
        check_size(meta.len(), remaining)?;
        let mut bytes = Vec::new();
        // Read at most one byte more than allowed: the file may grow after
        // the size check.
        file.take(MAX_FILE_BYTES.min(remaining) + 1)
            .read_to_end(&mut bytes)
            .map_err(io_error)?;
        let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        check_size(len, remaining)?;
        budget.bytes += len;
        Ok(bytes)
    }
}

fn check_size(len: u64, remaining: u64) -> Result<(), Reason> {
    if len > MAX_FILE_BYTES {
        return Err(limit_error("fileBytes", as_u32(MAX_FILE_BYTES)));
    }
    if len > remaining {
        return Err(limit_error("packBytes", as_u32(MAX_PACK_BYTES)));
    }
    Ok(())
}

fn as_u32<T: TryInto<u32>>(value: T) -> u32 {
    value.try_into().unwrap_or(u32::MAX)
}

/// Whether `dir` holds an entry named exactly `name` (looked up in the
/// listing, so that case-insensitive file systems behave like the others).
/// At most [`MAX_FILES`] entries are looked at.
fn has_exact_entry(dir: &Path, name: &str) -> io::Result<bool> {
    for entry in fs::read_dir(dir)?.take(MAX_FILES) {
        if entry?.file_name() == name {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Entries of a folder sorted by name, with the metadata of the entry itself
/// (links are not followed). Hidden entries (`.` at the start, e.g.
/// `.DS_Store`) are skipped. Links and invalid names are reported; `None`
/// if the folder cannot be read or holds more than [`MAX_FILES`] entries
/// (counted before anything is kept, so that memory stays bounded).
fn list(dir: &Path, rel: &str, errors: &mut Vec<PackError>) -> Option<Vec<(String, Metadata)>> {
    let place = |name: &str| {
        if rel.is_empty() {
            name.to_owned()
        } else {
            format!("{rel}/{name}")
        }
    };
    let Ok(read) = fs::read_dir(dir) else {
        errors.push(PackError::file(&place(""), error(code::IO_ERROR)));
        return None;
    };
    let mut out = Vec::new();
    for (count, entry) in read.enumerate() {
        if count >= MAX_FILES {
            let reason = limit_error("files", as_u32(MAX_FILES));
            errors.push(if rel.is_empty() {
                PackError::pack(reason)
            } else {
                PackError::file(&format!("{rel}/"), reason)
            });
            return None;
        }
        let Ok(entry) = entry else {
            errors.push(PackError::file(&place(""), error(code::IO_ERROR)));
            continue;
        };
        let Ok(name) = entry.file_name().into_string() else {
            let lossy = entry.file_name().to_string_lossy().into_owned();
            errors.push(PackError::file(&place(&lossy), error(code::INVALID_PATH)));
            continue;
        };
        if name.starts_with('.') {
            continue;
        }
        let Ok(meta) = fs::symlink_metadata(entry.path()) else {
            errors.push(PackError::file(&place(&name), error(code::IO_ERROR)));
            continue;
        };
        if is_link(&meta) || !is_valid_name(&name) {
            errors.push(PackError::file(&place(&name), error(code::INVALID_PATH)));
            continue;
        }
        out.push((name, meta));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Some(out)
}

/// A name that is valid on Windows, macOS and Linux: no reserved device
/// name, no characters `<>:"/\|?*` or control characters (`:` would open an
/// alternate data stream on NTFS), no trailing dot or space, no bidirectional
/// controls (they would disguise the name in error messages).
#[must_use]
pub fn is_valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 255
        && !name.ends_with('.')
        && !name.ends_with(' ')
        && !name.chars().any(|c| {
            c.is_control()
                || matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*')
                || matches!(
                    c,
                    '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'
                )
        })
        && !is_reserved(name)
}

/// Reserved device names of Windows, also with an extension (`nul.json`),
/// including `COM¹`–`COM³`, `LPT¹`–`LPT³`, `CONIN$` and `CONOUT$`.
fn is_reserved(name: &str) -> bool {
    let stem = name
        .split('.')
        .next()
        .unwrap_or("")
        .trim_end()
        .to_uppercase();
    if matches!(
        stem.as_str(),
        "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$"
    ) {
        return true;
    }
    let Some(rest) = stem
        .strip_prefix("COM")
        .or_else(|| stem.strip_prefix("LPT"))
    else {
        return false;
    };
    let mut chars = rest.chars();
    matches!(
        (chars.next(), chars.next()),
        (Some('0'..='9' | '¹' | '²' | '³'), None)
    )
}

/// A file with further hard links: it may be a file outside the pack under
/// another name (Unix; `std` gives no link count on Windows).
#[cfg(unix)]
fn has_other_links(meta: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    meta.nlink() > 1
}

#[cfg(not(unix))]
fn has_other_links(_meta: &Metadata) -> bool {
    false
}

/// A symbolic link or, on Windows, any reparse point (junctions included).
fn is_link(meta: &Metadata) -> bool {
    meta.file_type().is_symlink() || is_reparse_point(meta)
}

#[cfg(windows)]
fn is_reparse_point(meta: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    /// `FILE_ATTRIBUTE_REPARSE_POINT`.
    const REPARSE_POINT: u32 = 0x400;
    meta.file_attributes() & REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn is_reparse_point(_meta: &Metadata) -> bool {
    false
}

/// Opens a file for reading without following a link at its place where
/// the platform allows it (Windows: `FILE_FLAG_OPEN_REPARSE_POINT`). On
/// Unix, [`same_file`] detects a link swapped in after the enumeration.
fn open_no_follow(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        /// `FILE_FLAG_OPEN_REPARSE_POINT`.
        const OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(OPEN_REPARSE_POINT);
    }
    options.open(path)
}

/// The opened handle is the enumerated entry (same device and inode).
#[cfg(unix)]
fn same_file(opened: &Metadata, listed: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    opened.dev() == listed.dev() && opened.ino() == listed.ino()
}

/// Without a stable file id in `std` on this platform, the reparse-point
/// check of the opened handle replaces the comparison.
#[cfg(not(unix))]
fn same_file(_opened: &Metadata, _listed: &Metadata) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_valid_on_all_platforms() {
        assert!(is_valid_name("objects"));
        assert!(is_valid_name("seats.json"));
        assert!(is_valid_name("Ä ü.json"));
        for bad in [
            "",
            "nul.json",
            "CON",
            "com1.json",
            "LPT9",
            "aux .json",
            "a:b",
            "x?.json",
            "trail.",
            "trail ",
            "tab\t",
            "COM¹.json",
            "lpt³",
            "conin$",
            "CONOUT$.json",
            "evil\u{202e}nosj.json",
        ] {
            assert!(!is_valid_name(bad), "{bad:?} must be refused");
        }
        assert!(is_valid_name("console.json"));
        assert!(is_valid_name("com10.json"));
        assert!(!is_valid_name(&"a".repeat(256)));
    }

    #[test]
    fn sizes_are_checked_against_file_and_pack_limits() {
        assert!(check_size(MAX_FILE_BYTES, MAX_PACK_BYTES).is_ok());
        let err = check_size(MAX_FILE_BYTES + 1, MAX_PACK_BYTES).unwrap_err();
        assert_eq!(
            err.params.get("limit"),
            Some(&ParamValue::Text("fileBytes".into()))
        );
        let err = check_size(10, 9).unwrap_err();
        assert_eq!(
            err.params.get("limit"),
            Some(&ParamValue::Text("packBytes".into()))
        );
    }

    #[test]
    fn later_kinds_name_their_step() {
        assert!(
            LATER_KINDS
                .iter()
                .any(|(k, s)| *k == "aircraft" && *s == "3.6")
        );
    }
}
