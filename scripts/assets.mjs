#!/usr/bin/env node
// Checks the SVG sources of the graphics pipeline against the style guide (docs/stilleitfaden.md,
// step 2.10, ADR-008, ADR-033): allowed elements and attributes, colours from assets/palette.json,
// stroke widths, size in whole cells matching the footprint of the object, one graphic per object
// of the base pack and no graphic without an object. Rasterising is done by tools/assets (Rust).
//
// Voraussetzungen: Node 26.11.1 (project-context.md Abschnitt 3)
// Plattformen: Linux, macOS, Windows (reines Node-Skript)
// Exit-Codes: 0 = alle Quellen regelkonform, 1 = mindestens ein Verstoß (je Zeile gemeldet)
// Idempotenz und Reproduzierbarkeit: nur lesend; das Ergebnis hängt nur von den Dateien ab.

import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

/** SVG user units per cell (2 m, ADR-020); as UNITS_PER_CELL in tools/assets. */
export const UNITS_PER_CELL = 32;
/** Metres per cell (ADR-020). */
export const METRES_PER_CELL = 2;
/** Allowed stroke widths in user units. */
export const STROKE_WIDTHS = ["0.5", "1", "2"];
/** Allowed elements: plain shapes in groups. */
export const ELEMENTS = new Set([
  "svg",
  "g",
  "rect",
  "circle",
  "ellipse",
  "line",
  "polyline",
  "polygon",
  "path",
]);
/** Allowed attributes. */
export const ATTRIBUTES = new Set([
  "xmlns",
  "width",
  "height",
  "viewBox",
  "x",
  "y",
  "rx",
  "ry",
  "cx",
  "cy",
  "r",
  "x1",
  "y1",
  "x2",
  "y2",
  "d",
  "points",
  "transform",
  "fill",
  "fill-rule",
  "fill-opacity",
  "stroke",
  "stroke-width",
  "stroke-linecap",
  "stroke-linejoin",
  "stroke-opacity",
  "opacity",
]);

const TAG = /<(\/?)([A-Za-z][\w:-]*)([^>]*?)(\/?)>/g;
const ATTRIBUTE = /([A-Za-z_:][\w:.-]*)\s*=\s*"([^"]*)"/g;

/**
 * Checks one SVG source. `cells` is the expected size in cells `{ width, depth }`,
 * or undefined for the icon (square, any size). Returns the problems, empty if none.
 */
export function checkSvg(text, palette, cells) {
  const problems = [];
  const colours = new Set(Object.values(palette));
  const body = text.replace(/<!--[\s\S]*?-->/g, "");
  if (/<!|<\?(?!xml)/.test(body)) {
    problems.push("no DOCTYPE, CDATA or processing instructions");
  }
  let root;
  for (const match of body.matchAll(TAG)) {
    const [, closing, name = "", rest = ""] = match;
    if (closing === "/") {
      continue;
    }
    if (!ELEMENTS.has(name)) {
      problems.push(`element <${name}> is not allowed`);
      continue;
    }
    const attributes = new Map();
    for (const [, key = "", value = ""] of rest.matchAll(ATTRIBUTE)) {
      attributes.set(key, value);
    }
    if (rest.replace(ATTRIBUTE, "").trim() !== "") {
      problems.push(`<${name}>: attributes must be written as key="value"`);
    }
    for (const [key, value] of attributes) {
      if (!ATTRIBUTES.has(key)) {
        problems.push(`<${name}>: attribute ${key} is not allowed`);
      } else if ((key === "fill" || key === "stroke") && value !== "none" && !colours.has(value)) {
        problems.push(`<${name}>: colour ${value} is not in the palette`);
      } else if (key === "stroke-width" && !STROKE_WIDTHS.includes(value)) {
        problems.push(`<${name}>: stroke width ${value} is not one of ${STROKE_WIDTHS.join(", ")}`);
      }
    }
    if (name === "svg" && root === undefined) {
      root = attributes;
    }
  }
  if (root === undefined) {
    problems.push("no <svg> element");
    return problems;
  }
  const width = Number(root.get("width"));
  const height = Number(root.get("height"));
  if (!Number.isInteger(width) || !Number.isInteger(height) || width <= 0 || height <= 0) {
    problems.push("width and height must be positive whole numbers");
    return problems;
  }
  if (root.get("viewBox") !== `0 0 ${width} ${height}`) {
    problems.push(`viewBox must be "0 0 ${width} ${height}"`);
  }
  if (cells === undefined) {
    if (width !== height) {
      problems.push(`icon must be square, is ${width} × ${height}`);
    }
  } else {
    const expected = [cells.width * UNITS_PER_CELL, cells.depth * UNITS_PER_CELL];
    if (width !== expected[0] || height !== expected[1]) {
      problems.push(
        `size ${width} × ${height} units, expected ${expected[0]} × ${expected[1]} (${cells.width} × ${cells.depth} cells)`,
      );
    }
  }
  return problems;
}

/** Footprint in cells of an object entry of a content file; area objects are one tile. */
export function footprint(entry) {
  const placement = entry.placement ?? {};
  if (placement.kind === "single") {
    return { width: placement.width / METRES_PER_CELL, depth: placement.depth / METRES_PER_CELL };
  }
  return { width: 1, depth: 1 };
}

/** Reads the object entries of the base pack: id → footprint. */
export function baseObjects(root) {
  const dir = join(root, "content/base/objects");
  const objects = new Map();
  for (const file of readdirSync(dir)
    .filter((f) => f.endsWith(".json"))
    .sort()) {
    const content = JSON.parse(readFileSync(join(dir, file), "utf8"));
    for (const entry of content.objects ?? []) {
      objects.set(entry.id, footprint(entry));
    }
  }
  return objects;
}

/** Checks all sources of the repository at `root`; returns lines `file: problem`. */
export function checkRepository(root) {
  const palette = JSON.parse(readFileSync(join(root, "assets/palette.json"), "utf8"));
  const guide = readFileSync(join(root, "docs/stilleitfaden.md"), "utf8");
  const lines = [];
  for (const [name, colour] of Object.entries(palette)) {
    if (!/^#[0-9a-f]{6}$/.test(colour)) {
      lines.push(`assets/palette.json: ${name} must be written as #rrggbb in lower case`);
    }
    if (!guide.includes(colour)) {
      lines.push(`docs/stilleitfaden.md: palette colour ${name} ${colour} is missing`);
    }
  }
  const check = (file, cells) => {
    for (const problem of checkSvg(readFileSync(join(root, file), "utf8"), palette, cells)) {
      lines.push(`${file}: ${problem}`);
    }
  };
  check("assets/missing.svg", { width: 1, depth: 1 });
  check("assets/icon.svg", undefined);
  const objects = baseObjects(root);
  const graphics = new Set(
    readdirSync(join(root, "assets/graphics/base"))
      .filter((f) => f.endsWith(".svg"))
      .map((f) => f.slice(0, -4)),
  );
  for (const [id, cells] of [...objects].sort(([a], [b]) => a.localeCompare(b))) {
    if (graphics.has(id)) {
      check(`assets/graphics/base/${id}.svg`, cells);
    } else {
      lines.push(`assets/graphics/base/${id}.svg: missing for object base:${id}`);
    }
  }
  for (const id of [...graphics].sort()) {
    if (!objects.has(id)) {
      lines.push(`assets/graphics/base/${id}.svg: no object base:${id} in content/base/objects`);
    }
  }
  return lines;
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const root = join(dirname(fileURLToPath(import.meta.url)), "..");
  const lines = checkRepository(root);
  for (const line of lines) {
    console.error(line);
  }
  console.log(
    lines.length === 0
      ? "graphics: all sources follow the style guide"
      : `graphics: ${lines.length} problems`,
  );
  process.exitCode = lines.length === 0 ? 0 : 1;
}
