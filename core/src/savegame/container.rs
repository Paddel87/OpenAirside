//! Byte layout of the container `.oasave` (S-5, ADR-027). All numbers are
//! little-endian:
//!
//! | Bytes | Content |
//! |---|---|
//! | 6 | identification `OASAVE` |
//! | 2 | container version |
//! | 4 | world state format version |
//! | 1 + n | game version: length (1 … 64), ASCII text |
//! | 4 · 4 | lengths of metadata, preview, packed world state and unpacked world state |
//! | … | metadata (JSON), preview (PNG or empty), world state (postcard, Deflate) |
//! | 8 | checksum FNV-1a 64 over all bytes before it |
//!
//! Loading checks in the order of the contract: identification and
//! lengths, checksum, metadata, world state, invariants.

use super::state::{LoadedGame, SaveSnapshot, SavedState};
use super::{
    CONTAINER_VERSION, FORMAT_VERSION, Limits, MAGIC, MAX_FILE_BYTES, MAX_META_BYTES,
    MAX_NAME_CHARS, MAX_PREVIEW_BYTES, MAX_PREVIEW_HEIGHT, MAX_PREVIEW_WIDTH, MAX_UNPACKED_BYTES,
    SaveMeta, SaveRequest, check_request, check_saved_at, check_text, code, error, invalid, limit,
};
use crate::hash::StateHasher;
use crate::text::Reason;
use crate::time::GameTime;

/// Longest game version text in the header.
const MAX_GAME_VERSION: usize = 64;
/// Deflate level of the world state (ADR-027: level 6 in the measurement).
const LEVEL: u8 = 6;
const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

/// What the save game list needs of a file, read without the world state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveInfo {
    /// Game version that wrote the file.
    pub game_version: String,
    /// Metadata.
    pub meta: SaveMeta,
    /// Byte offset of the preview image in the file.
    pub preview_offset: u64,
    /// Length of the preview image; 0 if there is none.
    pub preview_len: u32,
}

impl SaveSnapshot {
    /// Builds the complete file of a save game.
    ///
    /// # Errors
    ///
    /// `invalidArgument` or `limitExceeded` for a request out of range;
    /// `limitExceeded` if the world state is larger than the limits.
    pub fn encode(
        &self,
        request: &SaveRequest,
        airport_name: &str,
        game_version: &str,
    ) -> Result<Vec<u8>, Reason> {
        check_request(request)?;
        if !check_text(airport_name, MAX_NAME_CHARS) {
            return Err(error(code::INVALID_ARGUMENT).with_text("field", "airportName"));
        }
        if game_version.is_empty()
            || game_version.len() > MAX_GAME_VERSION
            || !game_version.bytes().all(|b| b.is_ascii_graphic())
        {
            return Err(error(code::INVALID_ARGUMENT).with_text("field", "gameVersion"));
        }
        let time = GameTime::at(self.tick);
        let meta = SaveMeta {
            name: request.name.clone(),
            kind: request.kind,
            saved_at: request.saved_at.clone(),
            tick: self.tick,
            day: time.day,
            year: time.year,
            airport_name: airport_name.to_owned(),
            map: self.grid.map(),
            packs: self.packs.clone(),
        };
        let meta = serde_json::to_vec(&meta).map_err(|_| invalid("meta"))?;
        if meta.len() > MAX_META_BYTES as usize {
            return Err(limit("meta", f64::from(MAX_META_BYTES)));
        }
        let raw = postcard::to_stdvec(&self.to_saved()).map_err(|_| invalid("world"))?;
        let unpacked = u32::try_from(raw.len())
            .ok()
            .filter(|&n| n <= MAX_UNPACKED_BYTES)
            .ok_or_else(|| limit("unpacked", f64::from(MAX_UNPACKED_BYTES)))?;
        let world = miniz_oxide::deflate::compress_to_vec(&raw, LEVEL);
        let world_len =
            u32::try_from(world.len()).map_err(|_| limit("file", MAX_FILE_BYTES as f64))?;

        let mut out = Vec::with_capacity(64 + meta.len() + request.preview.len() + world.len());
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&CONTAINER_VERSION.to_le_bytes());
        out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
        out.push(u8::try_from(game_version.len()).unwrap_or(0));
        out.extend_from_slice(game_version.as_bytes());
        for len in [
            u32::try_from(meta.len()).unwrap_or(u32::MAX),
            u32::try_from(request.preview.len()).unwrap_or(u32::MAX),
            world_len,
            unpacked,
        ] {
            out.extend_from_slice(&len.to_le_bytes());
        }
        out.extend_from_slice(&meta);
        out.extend_from_slice(&request.preview);
        out.extend_from_slice(&world);
        if out.len() as u64 + 8 > MAX_FILE_BYTES {
            return Err(limit("file", MAX_FILE_BYTES as f64));
        }
        let checksum = checksum(&out);
        out.extend_from_slice(&checksum.to_le_bytes());
        Ok(out)
    }
}

fn checksum(bytes: &[u8]) -> u64 {
    let mut hasher = StateHasher::new();
    hasher.bytes(bytes);
    hasher.finish()
}

/// Header of a file with the section lengths.
struct Header {
    game_version: Vec<u8>,
    /// Offset of the metadata.
    start: usize,
    meta: usize,
    preview: usize,
    world: usize,
    unpacked: usize,
}

impl Header {
    /// Length of the whole file according to the header.
    fn file_len(&self) -> u64 {
        (self.start + self.meta + self.preview + self.world) as u64 + 8
    }
}

/// Small cursor over the header bytes; running out means `truncated`.
struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], Reason> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.bytes.len());
        let end = end.ok_or_else(|| error(code::TRUNCATED))?;
        let out = &self.bytes[self.at..end];
        self.at = end;
        Ok(out)
    }

    fn u32(&mut self) -> Result<u32, Reason> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
}

/// Reads identification, versions and lengths from the start of a file of
/// `file_len` bytes.
fn header(bytes: &[u8], file_len: u64) -> Result<Header, Reason> {
    if bytes.len() < MAGIC.len() {
        return Err(if MAGIC.starts_with(bytes) {
            error(code::TRUNCATED)
        } else {
            error(code::BAD_MAGIC)
        });
    }
    if &bytes[..MAGIC.len()] != MAGIC {
        return Err(error(code::BAD_MAGIC));
    }
    if file_len > MAX_FILE_BYTES {
        return Err(limit("file", MAX_FILE_BYTES as f64));
    }
    let mut cursor = Cursor {
        bytes,
        at: MAGIC.len(),
    };
    let container = cursor.take(2)?;
    let container = u16::from_le_bytes([container[0], container[1]]);
    let format = cursor.u32()?;
    if container > CONTAINER_VERSION || format > FORMAT_VERSION {
        return Err(error(code::VERSION_TOO_NEW)
            .with_number("format", f64::from(format))
            .with_number("supported", f64::from(FORMAT_VERSION)));
    }
    if container < CONTAINER_VERSION || format < FORMAT_VERSION {
        return Err(error(code::MIGRATION_FAILED)
            .with_number("format", f64::from(format))
            .with_number("supported", f64::from(FORMAT_VERSION)));
    }
    let version_len = usize::from(cursor.take(1)?[0]);
    if version_len == 0 || version_len > MAX_GAME_VERSION {
        return Err(invalid("gameVersion"));
    }
    let game_version = cursor.take(version_len)?.to_vec();
    let meta = cursor.u32()?;
    let preview = cursor.u32()?;
    let world = cursor.u32()?;
    let unpacked = cursor.u32()?;
    if meta > MAX_META_BYTES {
        return Err(limit("meta", f64::from(MAX_META_BYTES)));
    }
    if preview > MAX_PREVIEW_BYTES {
        return Err(limit("preview", f64::from(MAX_PREVIEW_BYTES)));
    }
    if unpacked > MAX_UNPACKED_BYTES {
        return Err(limit("unpacked", f64::from(MAX_UNPACKED_BYTES)));
    }
    let header = Header {
        game_version,
        start: cursor.at,
        meta: meta as usize,
        preview: preview as usize,
        world: world as usize,
        unpacked: unpacked as usize,
    };
    // Every section length against the rest of the file.
    if header.file_len() > file_len {
        return Err(error(code::TRUNCATED));
    }
    if header.file_len() < file_len {
        return Err(error(code::CHECKSUM_MISMATCH));
    }
    Ok(header)
}

fn game_version(header: &Header) -> Result<String, Reason> {
    let text = std::str::from_utf8(&header.game_version).map_err(|_| invalid("gameVersion"))?;
    if !text.bytes().all(|b| b.is_ascii_graphic()) {
        return Err(invalid("gameVersion"));
    }
    Ok(text.to_owned())
}

fn meta(bytes: &[u8]) -> Result<SaveMeta, Reason> {
    let meta: SaveMeta = serde_json::from_slice(bytes).map_err(|_| invalid("meta"))?;
    let texts_ok = check_text(&meta.name, MAX_NAME_CHARS)
        && check_text(&meta.airport_name, MAX_NAME_CHARS)
        && check_saved_at(&meta.saved_at);
    if !texts_ok || meta.packs.len() > crate::datapack::MAX_PACKS {
        return Err(invalid("meta"));
    }
    Ok(meta)
}

/// Checks a preview image: empty, or a PNG of at most
/// [`MAX_PREVIEW_WIDTH`] × [`MAX_PREVIEW_HEIGHT`] pixels (header only; the
/// image itself is decoded by the presentation).
///
/// # Errors
///
/// `invalidState` for data that is not a PNG, `limitExceeded` for a larger
/// image.
pub(super) fn check_preview(png: &[u8]) -> Result<(), Reason> {
    if png.is_empty() {
        return Ok(());
    }
    // Signature, length of the first chunk, "IHDR", width, height.
    if png.len() < 24 || &png[..8] != PNG_SIGNATURE || &png[12..16] != b"IHDR" {
        return Err(invalid("preview"));
    }
    let width = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
    let height = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
    if width == 0 || height == 0 {
        return Err(invalid("preview"));
    }
    if width > MAX_PREVIEW_WIDTH || height > MAX_PREVIEW_HEIGHT {
        return Err(limit("previewSize", f64::from(MAX_PREVIEW_WIDTH)));
    }
    Ok(())
}

/// Reads header and metadata for the save game list. `prefix` is the start
/// of the file (at least header and metadata, at most
/// [`super::MAX_INFO_BYTES`] are needed), `file_len` its full length. The
/// checksum is not checked: damage outside header and metadata shows when
/// the game is loaded (`checksumMismatch`).
///
/// # Errors
///
/// The code of the first failed check.
pub fn read_info(prefix: &[u8], file_len: u64) -> Result<SaveInfo, Reason> {
    let header = header(prefix, file_len)?;
    let game_version = game_version(&header)?;
    let meta_bytes = Cursor {
        bytes: prefix,
        at: header.start,
    }
    .take(header.meta)?;
    let meta = meta(meta_bytes)?;
    Ok(SaveInfo {
        game_version,
        meta,
        preview_offset: (header.start + header.meta) as u64,
        preview_len: u32::try_from(header.preview).unwrap_or(0),
    })
}

/// Reads and checks a complete save game file.
///
/// # Errors
///
/// The code of the first failed check, in the order of S-5.
pub fn decode(bytes: &[u8], limits: &Limits) -> Result<LoadedGame, Reason> {
    let header = header(bytes, bytes.len() as u64)?;
    let body_end = bytes.len() - 8;
    let stored = u64::from_le_bytes(
        bytes[body_end..]
            .try_into()
            .map_err(|_| error(code::TRUNCATED))?,
    );
    if checksum(&bytes[..body_end]) != stored {
        return Err(error(code::CHECKSUM_MISMATCH));
    }
    game_version(&header)?;
    let meta_end = header.start + header.meta;
    let preview_end = meta_end + header.preview;
    let meta = meta(&bytes[header.start..meta_end])?;
    check_preview(&bytes[meta_end..preview_end])?;
    let raw = miniz_oxide::inflate::decompress_to_vec_with_limit(
        &bytes[preview_end..body_end],
        header.unpacked,
    )
    .map_err(|_| invalid("world"))?;
    if raw.len() != header.unpacked {
        return Err(invalid("world"));
    }
    let (state, rest): (SavedState, &[u8]) =
        postcard::take_from_bytes(&raw).map_err(|_| invalid("world"))?;
    if !rest.is_empty() {
        return Err(invalid("world"));
    }
    if meta.tick != state.tick() || meta.map != state.map() {
        return Err(invalid("meta"));
    }
    // Structural checks now, so that a game that cannot become a session
    // fails before the platform starts loading.
    let restored = super::state::restore(state, limits.map_edges)?;
    Ok(LoadedGame { meta, restored })
}
