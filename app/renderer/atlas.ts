/**
 * Texture atlases of the graphics pipeline (step 2.10, ADR-008, ADR-033):
 * the index written by `tools/assets`, the choice of the resolution and the
 * lookup of an image by its full object id (`base:bench`, ADR-032) with the
 * neutral fallback for objects without an image. Display only (rule 002).
 *
 * The index is bundled by Vite; the pages are loaded as images below
 * `index.html` (`img-src 'self'`), so no `fetch` and no change of the CSP.
 */

/** Version of the index this renderer reads (`INDEX_VERSION` in tools/assets). */
export const INDEX_VERSION = 1;

/** Place of an image in a page, in pixels. */
export interface AtlasFrame {
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
}

/** One page of a resolution. */
export interface AtlasPage {
  /** URL relative to `index.html`. */
  readonly image: string;
  readonly width: number;
  readonly height: number;
  readonly frames: Readonly<Record<string, AtlasFrame>>;
}

/** One resolution. */
export interface AtlasTier {
  readonly pxPerCell: number;
  readonly pages: readonly AtlasPage[];
}

/** The atlas index. */
export interface AtlasIndex {
  readonly version: number;
  readonly unitsPerCell: number;
  readonly fallback: string;
  /** Ascending by `pxPerCell`. */
  readonly tiers: readonly AtlasTier[];
}

/** Where an image is: page index and frame, and whether the fallback was used. */
export interface AtlasHit {
  readonly page: number;
  readonly frame: AtlasFrame;
  readonly fallback: boolean;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isCount(value: unknown): value is number {
  return typeof value === "number" && Number.isInteger(value) && value >= 0;
}

function isFrame(value: unknown): value is AtlasFrame {
  return (
    isRecord(value) && isCount(value.x) && isCount(value.y) && isCount(value.w) && isCount(value.h)
  );
}

function isPage(value: unknown): value is AtlasPage {
  return (
    isRecord(value) &&
    typeof value.image === "string" &&
    /^[a-z0-9/_.-]+\.png$/.test(value.image) &&
    !value.image.includes("..") &&
    isCount(value.width) &&
    isCount(value.height) &&
    isRecord(value.frames) &&
    Object.values(value.frames).every(isFrame)
  );
}

/**
 * Checks the bundled index. Our own build writes it, so a mismatch is a
 * build error; the game then draws without atlas (`undefined`).
 */
export function parseAtlasIndex(value: unknown): AtlasIndex | undefined {
  if (
    !isRecord(value) ||
    value.version !== INDEX_VERSION ||
    !isCount(value.unitsPerCell) ||
    value.unitsPerCell === 0 ||
    typeof value.fallback !== "string" ||
    !Array.isArray(value.tiers) ||
    value.tiers.length === 0
  ) {
    return undefined;
  }
  let last = 0;
  for (const tier of value.tiers as unknown[]) {
    if (
      !isRecord(tier) ||
      !isCount(tier.pxPerCell) ||
      tier.pxPerCell <= last ||
      !Array.isArray(tier.pages) ||
      !(tier.pages as unknown[]).every(isPage)
    ) {
      return undefined;
    }
    last = tier.pxPerCell;
  }
  return value as unknown as AtlasIndex;
}

/**
 * Index of the tier to draw with at `zoom` screen points per cell and
 * `pixelRatio` pixels per point: the smallest with at least one image pixel
 * per screen pixel, else the largest.
 */
export function chooseTier(index: AtlasIndex, zoom: number, pixelRatio: number): number {
  const needed = zoom * pixelRatio;
  const found = index.tiers.findIndex((t) => t.pxPerCell >= needed);
  return found === -1 ? index.tiers.length - 1 : found;
}

/** Finds `name` in tier `tier`, else the fallback; `undefined` only if both are missing. */
export function lookup(index: AtlasIndex, tier: number, name: string): AtlasHit | undefined {
  const pages = index.tiers[tier]?.pages ?? [];
  for (const fallback of [false, true]) {
    const wanted = fallback ? index.fallback : name;
    for (let page = 0; page < pages.length; page += 1) {
      const frame = pages[page]?.frames[wanted];
      if (frame !== undefined) {
        return { page, frame, fallback };
      }
    }
  }
  return undefined;
}

/** Image names of the index without the fallback, sorted. */
export function imageNames(index: AtlasIndex): string[] {
  const names = new Set<string>();
  for (const page of index.tiers[0]?.pages ?? []) {
    for (const name of Object.keys(page.frames)) {
      if (name !== index.fallback) {
        names.add(name);
      }
    }
  }
  return [...names].sort();
}

/** Bytes of all pages of `tier` as RGBA textures on the graphics card. */
export function textureBytes(index: AtlasIndex, tier: number): number {
  return (index.tiers[tier]?.pages ?? []).reduce((sum, p) => sum + p.width * p.height * 4, 0);
}

/** The index bundled by the build, `undefined` without `npm run build:assets`. */
export function bundledIndex(): AtlasIndex | undefined {
  const found = import.meta.glob<unknown>("./atlas/atlas.json", { eager: true, import: "default" });
  return parseAtlasIndex(Object.values(found)[0]);
}
