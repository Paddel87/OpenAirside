import { describe, expect, it } from "vitest";
import {
  type AtlasIndex,
  bundledIndex,
  chooseTier,
  imageNames,
  INDEX_VERSION,
  lookup,
  parseAtlasIndex,
  textureBytes,
} from "./atlas.js";

function index(): AtlasIndex {
  const page = (
    px: number,
    n: number,
    frames: Record<string, { x: number; y: number; w: number; h: number }>,
  ) => ({
    image: `atlas/atlas-${String(px)}-${String(n)}.png`,
    width: 64,
    height: 32,
    frames,
  });
  return {
    version: INDEX_VERSION,
    unitsPerCell: 32,
    fallback: "missing",
    tiers: [
      {
        pxPerCell: 8,
        pages: [
          page(8, 0, {
            missing: { x: 1, y: 1, w: 8, h: 8 },
            "base:bench": { x: 11, y: 1, w: 16, h: 8 },
          }),
        ],
      },
      {
        pxPerCell: 16,
        pages: [
          page(16, 0, { missing: { x: 1, y: 1, w: 16, h: 16 } }),
          page(16, 1, { "base:bench": { x: 1, y: 1, w: 32, h: 16 } }),
        ],
      },
      { pxPerCell: 32, pages: [page(32, 0, {})] },
    ],
  };
}

describe("parseAtlasIndex", () => {
  it("accepts the index of the build", () => {
    const value = JSON.parse(JSON.stringify(index())) as unknown;
    expect(parseAtlasIndex(value)).toEqual(index());
  });

  it("rejects other versions and malformed parts", () => {
    const bad: unknown[] = [
      undefined,
      [],
      { ...index(), version: 2 },
      { ...index(), unitsPerCell: 0 },
      { ...index(), fallback: 1 },
      { ...index(), tiers: [] },
      { ...index(), tiers: [index().tiers[1], index().tiers[0]] },
      {
        ...index(),
        tiers: [{ pxPerCell: 8, pages: [{ ...index().tiers[0]?.pages[0], image: "../x.png" }] }],
      },
      {
        ...index(),
        tiers: [
          { pxPerCell: 8, pages: [{ ...index().tiers[0]?.pages[0], image: "https://x/a.png" }] },
        ],
      },
      {
        ...index(),
        tiers: [
          {
            pxPerCell: 8,
            pages: [{ ...index().tiers[0]?.pages[0], frames: { a: { x: -1, y: 0, w: 1, h: 1 } } }],
          },
        ],
      },
      { ...index(), tiers: [{ pxPerCell: 8, pages: "x" }] },
    ];
    for (const value of bad) {
      expect(parseAtlasIndex(value)).toBeUndefined();
    }
  });
});

describe("chooseTier", () => {
  it("takes the smallest resolution with one image pixel per screen pixel", () => {
    expect(chooseTier(index(), 0.5, 1)).toBe(0);
    expect(chooseTier(index(), 8, 1)).toBe(0);
    expect(chooseTier(index(), 8, 2)).toBe(1);
    expect(chooseTier(index(), 20, 1)).toBe(2);
    expect(chooseTier(index(), 32, 2)).toBe(2);
  });
});

describe("lookup", () => {
  it("finds an image on any page of the tier", () => {
    expect(lookup(index(), 1, "base:bench")).toEqual({
      page: 1,
      frame: { x: 1, y: 1, w: 32, h: 16 },
      fallback: false,
    });
  });

  it("falls back to the neutral image, and to nothing without it", () => {
    expect(lookup(index(), 0, "base:unknown")).toEqual({
      page: 0,
      frame: { x: 1, y: 1, w: 8, h: 8 },
      fallback: true,
    });
    expect(lookup(index(), 2, "base:bench")).toBeUndefined();
    expect(lookup(index(), 9, "base:bench")).toBeUndefined();
  });
});

describe("imageNames and textureBytes", () => {
  it("list the object images and the texture memory of a tier", () => {
    expect(imageNames(index())).toEqual(["base:bench"]);
    expect(textureBytes(index(), 1)).toBe(2 * 64 * 32 * 4);
    expect(textureBytes(index(), 9)).toBe(0);
  });
});

describe("bundledIndex", () => {
  it("is the index of the build or nothing", () => {
    const bundled = bundledIndex();
    if (bundled !== undefined) {
      expect(bundled.fallback).toBe("missing");
      expect(bundled.tiers.map((t) => t.pxPerCell)).toEqual([8, 16, 32]);
    }
  });
});
