//! Graphics pipeline of OpenAirside (roadmap step 2.10, ADR-008, ADR-033).
//!
//! Rasterises the SVG sources of the base pack into texture atlases in
//! several resolutions and writes the atlas index the renderer bundles. The
//! image name of an object is its full object id (`base:bench`, ADR-032);
//! the neutral fallback graphic is named [`FALLBACK`]. Rasterises the
//! program icon as well. Rules for the sources: `docs/stilleitfaden.md`.
//!
//! The output is deterministic: the same sources give the same bytes on
//! every platform (rasterising on the CPU, sorted inputs, fixed packing).

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use resvg::tiny_skia::{IntSize, Pixmap, Transform};
use resvg::usvg;
use serde::Serialize;

/// SVG user units per cell (2 m, ADR-020).
pub const UNITS_PER_CELL: u32 = 32;
/// Resolutions of the atlases in pixels per cell, smallest first.
pub const TIERS: [u32; 3] = [8, 16, 32];
/// Largest edge of an atlas page in pixels; WebGL2 guarantees at least 2048.
pub const MAX_PAGE: u32 = 2048;
/// Image name of the neutral fallback graphic; no object id, because those always contain `:`.
pub const FALLBACK: &str = "missing";
/// Version of the atlas index; raised by every non-additive change.
pub const INDEX_VERSION: u32 = 1;
/// Edges of the rasterised program icon in pixels: installers and packages, window.
pub const ICON_SIZES: [u32; 2] = [1024, 256];
/// Largest SVG source in bytes; sources are our own, the limit only catches mistakes.
pub const MAX_SOURCE_BYTES: u64 = 1024 * 1024;

/// Error of the pipeline, with the file it concerns.
#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub file: PathBuf,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.file.display(), self.message)
    }
}

impl std::error::Error for Error {}

fn error(file: &Path, message: impl Into<String>) -> Error {
    Error {
        file: file.to_path_buf(),
        message: message.into(),
    }
}

/// Paths of the pipeline, relative to the repository root unless absolute.
#[derive(Debug, Clone)]
pub struct Layout {
    /// SVG sources of the base pack, one file per object: `<entry id>.svg`; outside the pack folder, so that S-4 is unchanged and the sources are not shipped.
    pub graphics: PathBuf,
    /// Pack id of the sources (`base`).
    pub pack: String,
    /// Fallback graphic.
    pub fallback: PathBuf,
    /// Program icon.
    pub icon: PathBuf,
    /// Output folder of the atlas pages (copied by Vite next to `index.html`).
    pub pages: PathBuf,
    /// URL prefix of the pages relative to `index.html`.
    pub pages_url: String,
    /// Output file of the atlas index (bundled by Vite).
    pub index: PathBuf,
    /// Output files of the icon, one per entry of [`ICON_SIZES`].
    pub icons: [PathBuf; 2],
}

impl Layout {
    /// The layout of the repository at `root`.
    pub fn repository(root: &Path) -> Self {
        Self {
            graphics: root.join("assets/graphics/base"),
            pack: "base".to_owned(),
            fallback: root.join("assets/missing.svg"),
            icon: root.join("assets/icon.svg"),
            pages: root.join("app/renderer/public/atlas"),
            pages_url: "atlas".to_owned(),
            index: root.join("app/renderer/atlas/atlas.json"),
            icons: [
                root.join("build/icon.png"),
                root.join("app/renderer/public/icon.png"),
            ],
        }
    }
}

/// A parsed SVG source.
#[derive(Debug)]
pub struct Source {
    pub name: String,
    pub file: PathBuf,
    tree: usvg::Tree,
}

impl Source {
    /// Size in SVG user units.
    pub fn units(&self) -> (u32, u32) {
        let size = self.tree.size();
        (size.width().round() as u32, size.height().round() as u32)
    }
}

/// Checks the naming rule of an entry id (`[a-z0-9_-]{1,64}`, as the pack loader).
pub fn valid_entry_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

/// Parses an SVG source and checks the rules of the style guide that the
/// rasteriser can see: no text, no embedded or linked images, whole cells.
pub fn parse(file: &Path, name: &str, whole_cells: bool) -> Result<Source, Error> {
    let size = fs::metadata(file)
        .map_err(|e| error(file, format!("cannot read: {e}")))?
        .len();
    if size > MAX_SOURCE_BYTES {
        return Err(error(
            file,
            format!("{size} bytes, at most {MAX_SOURCE_BYTES}"),
        ));
    }
    let data = fs::read(file).map_err(|e| error(file, format!("cannot read: {e}")))?;
    let text = std::str::from_utf8(&data).map_err(|_| error(file, "not UTF-8"))?;
    for banned in ["<text", "<image", "<script", "<foreignObject", "href="] {
        if text.contains(banned) {
            return Err(error(file, format!("`{banned}` is not allowed")));
        }
    }
    // No resource folder: nothing outside the file is ever read.
    let options = usvg::Options {
        resources_dir: None,
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_data(&data, &options)
        .map_err(|e| error(file, format!("invalid SVG: {e}")))?;
    if contains_foreign(tree.root()) {
        return Err(error(file, "text and images are not allowed"));
    }
    let source = Source {
        name: name.to_owned(),
        file: file.to_path_buf(),
        tree,
    };
    let (w, h) = source.units();
    if w == 0 || h == 0 {
        return Err(error(file, "empty size"));
    }
    if whole_cells && (w % UNITS_PER_CELL != 0 || h % UNITS_PER_CELL != 0) {
        return Err(error(
            file,
            format!("size {w} × {h} is not a multiple of {UNITS_PER_CELL} units (one cell)"),
        ));
    }
    Ok(source)
}

fn contains_foreign(group: &usvg::Group) -> bool {
    group.children().iter().any(|node| match node {
        usvg::Node::Group(g) => contains_foreign(g),
        usvg::Node::Path(_) => false,
        usvg::Node::Image(_) | usvg::Node::Text(_) => true,
    })
}

/// Rasterises `source` so that its size becomes `width` × `height` pixels.
pub fn rasterise(source: &Source, width: u32, height: u32) -> Result<Pixmap, Error> {
    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| error(&source.file, format!("cannot allocate {width} × {height}")))?;
    let size = source.tree.size();
    let transform =
        Transform::from_scale(width as f32 / size.width(), height as f32 / size.height());
    resvg::render(&source.tree, transform, &mut pixmap.as_mut());
    Ok(pixmap)
}

/// Reads the sources: the fallback first, then the object graphics sorted by name.
pub fn read_sources(layout: &Layout) -> Result<Vec<Source>, Error> {
    let mut sources = vec![parse(&layout.fallback, FALLBACK, true)?];
    let entries = fs::read_dir(&layout.graphics)
        .map_err(|e| error(&layout.graphics, format!("cannot read: {e}")))?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| error(&layout.graphics, format!("cannot read: {e}")))?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("svg") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| error(&path, "file name is not UTF-8"))?;
        if !valid_entry_id(stem) {
            return Err(error(
                &path,
                "file name must be an entry id ([a-z0-9_-]{1,64})",
            ));
        }
        files.push((format!("{}:{stem}", layout.pack), path));
    }
    files.sort();
    for (name, path) in files {
        sources.push(parse(&path, &name, true)?);
    }
    Ok(sources)
}

/// Place of a rectangle in a page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Result of packing: per input the page and the place of its content.
#[derive(Debug, PartialEq, Eq)]
pub struct Packing {
    /// Width and height of each page, trimmed to the content.
    pub pages: Vec<(u32, u32)>,
    /// Per input: page index and inner frame (without the 1 px border).
    pub places: Vec<(usize, Frame)>,
}

/// Packs rectangles into pages of at most `max` × `max` pixels in shelves,
/// tallest first, with a 1 px border around each rectangle (repeated edge
/// pixels against bleeding under linear filtering). Deterministic for the
/// same input order.
pub fn pack(sizes: &[(u32, u32)], max: u32) -> Result<Packing, usize> {
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    // Stable sort: equal sizes keep their input order.
    order.sort_by(|&a, &b| {
        sizes[b]
            .1
            .cmp(&sizes[a].1)
            .then(sizes[b].0.cmp(&sizes[a].0))
    });
    let mut pages: Vec<(u32, u32)> = Vec::new();
    let mut places = vec![
        (
            0,
            Frame {
                x: 0,
                y: 0,
                w: 0,
                h: 0
            }
        );
        sizes.len()
    ];
    // Cursor of the current shelf: x, top of the shelf, height of the shelf.
    let (mut x, mut top, mut shelf) = (0u32, 0u32, 0u32);
    for i in order {
        let (w, h) = sizes[i];
        let (bw, bh) = (w + 2, h + 2);
        if bw > max || bh > max {
            return Err(i);
        }
        if pages.is_empty() {
            pages.push((0, 0));
        }
        if x + bw > max {
            top += shelf;
            x = 0;
            shelf = 0;
        }
        if top + bh > max {
            pages.push((0, 0));
            top = 0;
            x = 0;
            shelf = 0;
        }
        let page = pages.len() - 1;
        places[i] = (
            page,
            Frame {
                x: x + 1,
                y: top + 1,
                w,
                h,
            },
        );
        let extent = &mut pages[page];
        extent.0 = extent.0.max(x + bw);
        extent.1 = extent.1.max(top + bh);
        x += bw;
        shelf = shelf.max(bh);
    }
    Ok(Packing { pages, places })
}

/// Copies `sprite` into `page` (both premultiplied RGBA) at `frame` and
/// repeats its edge pixels into the 1 px border.
pub fn blit(page: &mut Pixmap, sprite: &Pixmap, frame: Frame) {
    let pw = page.width() as usize;
    let (sw, sh) = (sprite.width() as usize, sprite.height() as usize);
    let src = sprite.data();
    let dst = page.data_mut();
    // Rows -1 ..= sh and columns -1 ..= sw relative to the frame, clamped into the sprite.
    for row in 0..sh + 2 {
        let sy = row.saturating_sub(1).min(sh - 1);
        let dy = frame.y as usize + row - 1;
        for col in 0..sw + 2 {
            let sx = col.saturating_sub(1).min(sw - 1);
            let dx = frame.x as usize + col - 1;
            let s = (sy * sw + sx) * 4;
            let d = (dy * pw + dx) * 4;
            dst[d..d + 4].copy_from_slice(&src[s..s + 4]);
        }
    }
}

/// One page of a tier in the index.
#[derive(Debug, Serialize, PartialEq)]
pub struct PageIndex {
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub frames: BTreeMap<String, Frame>,
}

/// One resolution in the index.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TierIndex {
    pub px_per_cell: u32,
    pub pages: Vec<PageIndex>,
}

/// The atlas index the renderer bundles.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AtlasIndex {
    pub version: u32,
    pub units_per_cell: u32,
    pub fallback: String,
    pub tiers: Vec<TierIndex>,
}

/// Summary of a run.
#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    pub sources: usize,
    pub pages: usize,
    pub bytes: u64,
}

fn write(file: &Path, data: &[u8]) -> Result<(), Error> {
    if let Some(dir) = file.parent() {
        fs::create_dir_all(dir).map_err(|e| error(dir, format!("cannot create: {e}")))?;
    }
    fs::write(file, data).map_err(|e| error(file, format!("cannot write: {e}")))
}

fn encode(file: &Path, pixmap: &Pixmap) -> Result<Vec<u8>, Error> {
    pixmap
        .encode_png()
        .map_err(|e| error(file, format!("cannot encode PNG: {e}")))
}

/// Builds atlases, index and icon for `sources` into the paths of `layout`.
pub fn build(layout: &Layout, sources: &[Source], icon: &Source) -> Result<Report, Error> {
    // Old pages of a previous run must not remain next to the new ones.
    if layout.pages.exists() {
        fs::remove_dir_all(&layout.pages)
            .map_err(|e| error(&layout.pages, format!("cannot clear: {e}")))?;
    }
    let mut tiers = Vec::new();
    let mut pages_written = 0;
    let mut bytes = 0u64;
    for px in TIERS {
        let sprites = sources
            .iter()
            .map(|s| {
                let (w, h) = s.units();
                rasterise(s, w * px / UNITS_PER_CELL, h * px / UNITS_PER_CELL)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let sizes: Vec<(u32, u32)> = sprites.iter().map(|p| (p.width(), p.height())).collect();
        let packing = pack(&sizes, MAX_PAGE).map_err(|i| {
            error(
                &sources[i].file,
                format!("larger than an atlas page of {MAX_PAGE} px at {px} px per cell"),
            )
        })?;
        let mut pixmaps = Vec::new();
        for &(w, h) in &packing.pages {
            let size = IntSize::from_wh(w, h).ok_or_else(|| error(&layout.pages, "empty page"))?;
            pixmaps.push(
                Pixmap::new(size.width(), size.height())
                    .ok_or_else(|| error(&layout.pages, format!("cannot allocate {w} × {h}")))?,
            );
        }
        let mut index: Vec<PageIndex> = packing
            .pages
            .iter()
            .enumerate()
            .map(|(n, &(width, height))| PageIndex {
                image: format!("{}/atlas-{px}-{n}.png", layout.pages_url),
                width,
                height,
                frames: BTreeMap::new(),
            })
            .collect();
        for (i, &(page, frame)) in packing.places.iter().enumerate() {
            blit(&mut pixmaps[page], &sprites[i], frame);
            index[page].frames.insert(sources[i].name.clone(), frame);
        }
        for (n, pixmap) in pixmaps.iter().enumerate() {
            let file = layout.pages.join(format!("atlas-{px}-{n}.png"));
            let png = encode(&file, pixmap)?;
            bytes += png.len() as u64;
            write(&file, &png)?;
            pages_written += 1;
        }
        tiers.push(TierIndex {
            px_per_cell: px,
            pages: index,
        });
    }
    let index = AtlasIndex {
        version: INDEX_VERSION,
        units_per_cell: UNITS_PER_CELL,
        fallback: FALLBACK.to_owned(),
        tiers,
    };
    let mut json = serde_json::to_string_pretty(&index)
        .map_err(|e| error(&layout.index, format!("cannot serialise: {e}")))?;
    json.push('\n');
    write(&layout.index, json.as_bytes())?;
    for (edge, file) in ICON_SIZES.iter().zip(&layout.icons) {
        let png = encode(file, &rasterise(icon, *edge, *edge)?)?;
        write(file, &png)?;
    }
    Ok(Report {
        sources: sources.len(),
        pages: pages_written,
        bytes,
    })
}

/// Runs the whole pipeline for `layout`.
pub fn run(layout: &Layout) -> Result<Report, Error> {
    let sources = read_sources(layout)?;
    let icon = parse(&layout.icon, "icon", false)?;
    let (w, h) = icon.units();
    if w != h {
        return Err(error(
            &layout.icon,
            format!("icon must be square, is {w} × {h}"),
        ));
    }
    build(layout, &sources, &icon)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECT: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="32" viewBox="0 0 64 32"><rect x="0" y="0" width="64" height="32" fill="#ff0000"/></svg>"##;

    /// Fresh empty folder below the system temp folder.
    fn temp(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("openairside-assets-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
        path
    }

    fn square(units: u32, color: &str) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{units}" height="{units}" viewBox="0 0 {units} {units}"><rect width="{units}" height="{units}" fill="{color}"/></svg>"#
        )
    }

    fn repo(name: &str) -> (PathBuf, Layout) {
        let root = temp(name);
        file(&root, "assets/missing.svg", &square(32, "#808080"));
        file(&root, "assets/icon.svg", &square(100, "#2060a0"));
        file(&root, "assets/graphics/base/bench.svg", RECT);
        file(
            &root,
            "assets/graphics/base/floor_concrete.svg",
            &square(32, "#c0c0c0"),
        );
        file(&root, "assets/graphics/base/notes.txt", "ignored");
        let layout = Layout::repository(&root);
        (root, layout)
    }

    #[test]
    fn entry_ids_follow_the_loader_rule() {
        assert!(valid_entry_id("bench"));
        assert!(valid_entry_id("floor_concrete-2"));
        assert!(!valid_entry_id(""));
        assert!(!valid_entry_id("Bench"));
        assert!(!valid_entry_id("base:bench"));
        assert!(!valid_entry_id(&"a".repeat(65)));
    }

    #[test]
    fn rejects_text_images_links_and_odd_sizes() {
        let dir = temp("reject");
        let cases = [
            (
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32"><text x="1" y="9">A</text></svg>"#,
                "<text",
            ),
            (
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32"><image width="32" height="32"/></svg>"#,
                "<image",
            ),
            (
                r##"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32"><use href="#a"/></svg>"##,
                "href=",
            ),
            (
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32"><script/></svg>"#,
                "<script",
            ),
            (&square(40, "#000000"), "multiple of 32"),
            ("<svg", "invalid SVG"),
        ];
        for (n, (content, expected)) in cases.iter().enumerate() {
            let path = file(&dir, &format!("case{n}.svg"), content);
            let err = parse(&path, "x", true).expect_err("must fail");
            assert!(err.message.contains(expected), "case {n}: {}", err.message);
            assert_eq!(err.file, path);
        }
        let big = file(&dir, "big.svg", &" ".repeat(MAX_SOURCE_BYTES as usize + 1));
        assert!(
            parse(&big, "x", true)
                .unwrap_err()
                .message
                .contains("at most")
        );
        let missing = dir.join("none.svg");
        assert!(
            parse(&missing, "x", true)
                .unwrap_err()
                .message
                .contains("cannot read")
        );
        // Any size is fine where whole cells are not required (icon).
        let icon = file(&dir, "icon.svg", &square(40, "#000000"));
        assert_eq!(parse(&icon, "icon", false).unwrap().units(), (40, 40));
    }

    #[test]
    fn rasterises_to_the_requested_size() {
        let dir = temp("raster");
        let source = parse(&file(&dir, "r.svg", RECT), "r", true).unwrap();
        let pixmap = rasterise(&source, 16, 8).unwrap();
        assert_eq!((pixmap.width(), pixmap.height()), (16, 8));
        // Opaque red in the middle (premultiplied RGBA).
        let mid = (4 * 16 + 8) * 4;
        assert_eq!(&pixmap.data()[mid..mid + 4], &[255, 0, 0, 255]);
    }

    #[test]
    fn packs_without_overlap_and_within_pages() {
        let sizes = vec![(30, 10), (10, 30), (50, 50), (20, 20), (20, 20), (60, 5)];
        let packing = pack(&sizes, 64).unwrap();
        let rects: Vec<(usize, Frame)> = packing.places.clone();
        for (i, &(page, f)) in rects.iter().enumerate() {
            assert_eq!((f.w, f.h), sizes[i]);
            let (pw, ph) = packing.pages[page];
            assert!(f.x >= 1 && f.y >= 1 && f.x + f.w < pw && f.y + f.h < ph);
            assert!(pw <= 64 && ph <= 64);
            for &(other_page, g) in &rects[i + 1..] {
                if other_page != page {
                    continue;
                }
                // The 1 px borders must not overlap either.
                let apart = f.x + f.w + 2 <= g.x
                    || g.x + g.w + 2 <= f.x
                    || f.y + f.h + 2 <= g.y
                    || g.y + g.h + 2 <= f.y;
                assert!(apart, "{f:?} overlaps {g:?}");
            }
        }
        assert!(packing.pages.len() >= 2, "64 px pages cannot hold all");
        assert_eq!(pack(&sizes, 64).unwrap(), packing, "deterministic");
        assert_eq!(pack(&[(10, 10), (63, 1)], 64), Err(1));
        assert_eq!(pack(&[], 64).unwrap().pages.len(), 0);
    }

    #[test]
    fn blit_repeats_the_edges() {
        let mut sprite = Pixmap::new(2, 1).unwrap();
        sprite
            .data_mut()
            .copy_from_slice(&[10, 0, 0, 255, 20, 0, 0, 255]);
        let mut page = Pixmap::new(4, 3).unwrap();
        blit(
            &mut page,
            &sprite,
            Frame {
                x: 1,
                y: 1,
                w: 2,
                h: 1,
            },
        );
        let red: Vec<u8> = page.data().chunks(4).map(|p| p[0]).collect();
        assert_eq!(red, vec![10, 10, 20, 20, 10, 10, 20, 20, 10, 10, 20, 20]);
    }

    #[test]
    fn builds_atlases_index_and_icon() {
        let (root, layout) = repo("build");
        // A page left over from an earlier run with more sources disappears.
        file(&layout.pages, "atlas-8-9.png", "old");
        let report = run(&layout).unwrap();
        assert_eq!(report.sources, 3);
        assert_eq!(report.pages, TIERS.len());
        assert!(!layout.pages.join("atlas-8-9.png").exists());
        let index: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&layout.index).unwrap()).unwrap();
        assert_eq!(index["version"], INDEX_VERSION);
        assert_eq!(index["fallback"], FALLBACK);
        assert_eq!(index["tiers"][0]["pxPerCell"], 8);
        let page = &index["tiers"][2]["pages"][0];
        assert_eq!(page["image"], "atlas/atlas-32-0.png");
        assert_eq!(page["frames"]["base:bench"]["w"], 64);
        assert_eq!(page["frames"]["base:bench"]["h"], 32);
        assert_eq!(page["frames"]["base:floor_concrete"]["w"], 32);
        assert_eq!(page["frames"][FALLBACK]["h"], 32);
        assert_eq!(
            index["tiers"][0]["pages"][0]["frames"]["base:bench"]["w"],
            16
        );
        for px in TIERS {
            let png = fs::read(layout.pages.join(format!("atlas-{px}-0.png"))).unwrap();
            assert_eq!(&png[1..4], b"PNG");
        }
        for (edge, icon) in ICON_SIZES.iter().zip(&layout.icons) {
            let png = Pixmap::load_png(icon).unwrap();
            assert_eq!((png.width(), png.height()), (*edge, *edge));
        }
        // Same sources, same bytes.
        let first = fs::read(layout.pages.join("atlas-16-0.png")).unwrap();
        let json = fs::read(&layout.index).unwrap();
        run(&layout).unwrap();
        assert_eq!(
            fs::read(layout.pages.join("atlas-16-0.png")).unwrap(),
            first
        );
        assert_eq!(fs::read(&layout.index).unwrap(), json);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn reports_bad_inputs_with_their_file() {
        let (root, layout) = repo("bad");
        // Not an entry id on any file system (an upper-case variant of an existing file would be
        // the same file on macOS and Windows).
        let wrong = file(&root, "assets/graphics/base/bench.old.svg", RECT);
        assert_eq!(run(&layout).unwrap_err().file, wrong);
        fs::remove_file(&wrong).unwrap();

        let icon = file(&root, "assets/icon.svg", RECT);
        assert!(run(&layout).unwrap_err().message.contains("square"));
        file(&root, "assets/icon.svg", &square(100, "#2060a0"));

        let huge = file(
            &root,
            "assets/graphics/base/huge.svg",
            &square(32 * 64, "#000000"),
        );
        let err = run(&layout).unwrap_err();
        assert_eq!(err.file, huge);
        assert!(err.message.contains("atlas page"));
        fs::remove_file(&huge).unwrap();

        fs::remove_file(&icon).unwrap();
        assert!(run(&layout).unwrap_err().message.contains("cannot read"));
        let mut gone = layout.clone();
        gone.graphics = root.join("nowhere");
        assert_eq!(read_sources(&gone).err().unwrap().file, gone.graphics);
        let _ = fs::remove_dir_all(root);
    }
}
