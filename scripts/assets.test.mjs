import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import { checkRepository, checkSvg, footprint } from "./assets.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const PALETTE = { outline: "#262a30", metal: "#8a929e" };
const ONE_CELL = { width: 1, depth: 1 };

function svg(body, width = 32, height = 32) {
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">${body}</svg>`;
}

describe("checkSvg", () => {
  it("accepts plain shapes in palette colours", () => {
    const text = svg(
      '<!-- base:x; CC0 --><g><rect x="1" y="1" width="30" height="30" fill="#8a929e" stroke="#262a30" stroke-width="1" /><path d="M0 0 L32 32" fill="none" stroke="#262a30" stroke-width="0.5"/></g>',
    );
    expect(checkSvg(text, PALETTE, ONE_CELL)).toEqual([]);
  });

  it("rejects foreign elements, attributes, colours and stroke widths", () => {
    const text = svg(
      '<text x="1">A</text><image href="a.png"/><rect style="fill:red" fill="#ff0000" stroke-width="1.5" class=\'a\'/>',
    );
    expect(checkSvg(text, PALETTE, ONE_CELL)).toEqual([
      "element <text> is not allowed",
      "element <image> is not allowed",
      '<rect>: attributes must be written as key="value"',
      "<rect>: attribute style is not allowed",
      "<rect>: colour #ff0000 is not in the palette",
      "<rect>: stroke width 1.5 is not one of 0.5, 1, 2",
    ]);
  });

  it("rejects DOCTYPE and entities", () => {
    const text = `<!DOCTYPE svg [<!ENTITY a "b">]>${svg("")}`;
    expect(checkSvg(text, PALETTE, ONE_CELL)).toContain(
      "no DOCTYPE, CDATA or processing instructions",
    );
  });

  it("checks the size against the footprint", () => {
    expect(checkSvg(svg("", 64, 32), PALETTE, { width: 2, depth: 1 })).toEqual([]);
    expect(checkSvg(svg("", 32, 32), PALETTE, { width: 2, depth: 1 })).toEqual([
      "size 32 × 32 units, expected 64 × 32 (2 × 1 cells)",
    ]);
    expect(
      checkSvg(
        '<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 16 16"></svg>',
        PALETTE,
        ONE_CELL,
      ),
    ).toEqual(['viewBox must be "0 0 32 32"']);
    expect(checkSvg('<svg width="3.5" height="32"></svg>', PALETTE, ONE_CELL)).toEqual([
      "width and height must be positive whole numbers",
    ]);
    expect(checkSvg("<g/>", PALETTE, ONE_CELL)).toEqual(["no <svg> element"]);
  });

  it("requires a square icon of any size", () => {
    expect(checkSvg(svg("", 100, 100), PALETTE, undefined)).toEqual([]);
    expect(checkSvg(svg("", 100, 50), PALETTE, undefined)).toEqual([
      "icon must be square, is 100 × 50",
    ]);
  });
});

describe("footprint", () => {
  it("converts metres to cells and treats area objects as one tile", () => {
    expect(footprint({ placement: { kind: "single", width: 4, depth: 2 } })).toEqual({
      width: 2,
      depth: 1,
    });
    expect(footprint({ placement: { kind: "area" } })).toEqual(ONE_CELL);
    expect(footprint({})).toEqual(ONE_CELL);
  });
});

describe("checkRepository", () => {
  const dirs = [];
  afterEach(() => {
    for (const dir of dirs.splice(0)) {
      rmSync(dir, { recursive: true, force: true });
    }
  });

  function fakeRepo() {
    const root = mkdtempSync(join(tmpdir(), "openairside-graphics-"));
    dirs.push(root);
    const write = (file, text) => {
      mkdirSync(dirname(join(root, file)), { recursive: true });
      writeFileSync(join(root, file), text);
    };
    write("assets/palette.json", JSON.stringify({ outline: "#262a30", metal: "#8A929E" }));
    write("docs/stilleitfaden.md", "| outline | #262a30 |");
    write("assets/missing.svg", svg('<rect width="32" height="32" fill="#262a30"/>'));
    write("assets/icon.svg", svg("", 64, 32));
    write(
      "content/base/objects/a.json",
      JSON.stringify({
        objects: [
          { id: "bench", placement: { kind: "single", width: 4, depth: 2 } },
          { id: "floor" },
        ],
      }),
    );
    write("content/base/objects/notes.txt", "ignored");
    write("assets/graphics/base/bench.svg", svg("", 32, 32));
    write("assets/graphics/base/orphan.svg", svg(""));
    return root;
  }

  it("reports every problem with its file", () => {
    expect(checkRepository(fakeRepo())).toEqual([
      "assets/palette.json: metal must be written as #rrggbb in lower case",
      "docs/stilleitfaden.md: palette colour metal #8A929E is missing",
      "assets/icon.svg: icon must be square, is 64 × 32",
      "assets/graphics/base/bench.svg: size 32 × 32 units, expected 64 × 32 (2 × 1 cells)",
      "assets/graphics/base/floor.svg: missing for object base:floor",
      "assets/graphics/base/orphan.svg: no object base:orphan in content/base/objects",
    ]);
  });

  it("finds no problem in the sources of this repository", () => {
    expect(checkRepository(ROOT)).toEqual([]);
  });
});
