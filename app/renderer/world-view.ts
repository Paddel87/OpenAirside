/**
 * The resting world in the playfield (module "Spielfeld", step 3.1b,
 * ADR-038): floors, walls, zones and objects from the world section of the
 * frames (S-1 layout 2). Floors and walls are drawn as Z-1: a data texture
 * of 512 × 512 cells per tile with one texel per cell, and a fragment
 * shader that picks the tile of the floor covering or wall type from a tile
 * sheet (`tile-sheet.ts`), or its mean colour in the simplified overview.
 * Objects are sprites per block, hidden in the simplified overview unless
 * "all details" is set. Display only (rule 002).
 *
 * Synchronous WebGL queries never happen here (step 3.14/3.15): textures are
 * uploaded through PixiJS, the sheet is drawn on a 2D canvas.
 */
import {
  BufferImageSource,
  Container,
  Geometry,
  GlProgram,
  Mesh,
  Shader,
  Sprite,
  Texture,
  UniformGroup,
} from "pixi.js";
import type { CatalogEntry } from "../src/generated/CatalogEntry.js";
import type { WorldSection } from "./frame-reader.js";
import { renderSheet, type SheetContext } from "./tile-sheet.js";
import { BLOCK, readBlocks, type WorldBlock, type WorldResult } from "./world-reader.js";
import {
  blockInView,
  placeOf,
  SHEET_TILES,
  SlotTable,
  TILE,
  tileKey,
  writeBlockTexels,
} from "./world-texels.js";

/** Device pixels per cell below which the overview is simplified (ADR-038). */
export const SIMPLE_BELOW_PX = 4;
/** Opacity of the zone tint over a tile. */
const ZONE_ALPHA = 0.25;
/** Blocks around the view for which objects stay visible when panning. */
const OBJECT_MARGIN_CELLS = BLOCK;

/** What the world view needs from the playfield (atlas and tier). */
export interface WorldDeps {
  /** Texture of the image `name` in the tier in use and its scale from pixels to cells. */
  objectTexture(name: string): { texture: Texture; scale: number };
  /** Draws the image `name` of the tier in use into the square tile at (`dx`, `dy`) with `px` pixels; `false` if there is none. */
  drawTile(
    ctx: CanvasRenderingContext2D,
    name: string,
    dx: number,
    dy: number,
    px: number,
  ): boolean;
}

/** Camera as the world view needs it. */
export interface WorldCamera {
  readonly floor: number;
  /** Visible rectangle in cells. */
  readonly x0: number;
  readonly y0: number;
  readonly x1: number;
  readonly y1: number;
  /** Device pixels per cell. */
  readonly px: number;
  /** Atlas tier in pixels per cell, 0 without atlas. */
  readonly tierPx: number;
}

/** Figures for the measurement. */
export interface WorldFigures {
  /** Blocks received in this session. */
  readonly blocks: number;
  /** Bytes of world sections received. */
  readonly bytes: number;
  /** Changed blocks the core still has to send. */
  readonly pending: number;
  readonly objects: number;
  /** Data textures. */
  readonly tiles: number;
  /** Estimated texture memory of the world display, MB. */
  readonly textureMb: number;
  /** Copies of the data textures in the renderer's memory, MB. */
  readonly cpuMb: number;
  /** Slots of the tile sheet in use. */
  readonly slots: number;
}

const VERTEX = `#version 300 es
in vec2 aPosition;
in vec2 aUV;
out vec2 vCell;
uniform mat3 uProjectionMatrix;
uniform mat3 uWorldTransformMatrix;
uniform mat3 uTransformMatrix;
void main() {
  mat3 mvp = uProjectionMatrix * uWorldTransformMatrix * uTransformMatrix;
  gl_Position = vec4((mvp * vec3(aPosition, 1.0)).xy, 0.0, 1.0);
  vCell = aUV;
}`;

const FRAGMENT = `#version 300 es
precision highp float;
in vec2 vCell;
uniform sampler2D uData;
uniform sampler2D uSheet;
uniform sampler2D uMean;
uniform float uSimple;
uniform float uInset;
out vec4 fragColor;
void main() {
  ivec2 c = clamp(ivec2(floor(vCell)), ivec2(0), ivec2(${String(TILE - 1)}));
  vec4 d = texelFetch(uData, c, 0);
  int floorSlot = int(d.r * 255.0 + 0.5);
  int wallSlot = int(d.g * 255.0 + 0.5);
  int zone = int(d.b * 255.0 + 0.5);
  int idx = wallSlot > 0 ? wallSlot : floorSlot;
  if (idx == 0) discard;
  vec4 col;
  if (uSimple > 0.5) {
    col = texelFetch(uMean, ivec2(idx % ${String(SHEET_TILES)}, idx / ${String(SHEET_TILES)}), 0);
  } else {
    vec2 sub = clamp(fract(vCell), uInset, 1.0 - uInset);
    col = texture(uSheet, (vec2(float(idx % ${String(SHEET_TILES)}), float(idx / ${String(SHEET_TILES)})) + sub) / ${String(SHEET_TILES)}.0);
  }
  vec3 zoneColour = zone == 1 ? vec3(0.37, 0.61, 0.31) : zone == 2 ? vec3(0.91, 0.72, 0.19) : vec3(0.78, 0.26, 0.23);
  float zoneAlpha = zone > 0 ? ${String(ZONE_ALPHA)} : 0.0;
  fragColor = vec4(mix(col.rgb, zoneColour, zoneAlpha), 1.0);
}`;

interface DataTile {
  readonly floorIndex: number;
  readonly data: Uint8Array;
  readonly texture: Texture;
  readonly mesh: Mesh<Geometry, Shader>;
  dirty: boolean;
}

interface ObjectItem {
  readonly sprite: Sprite;
  /** Catalog value (catalog index plus 1) of the type. */
  readonly type: number;
  /** Cell of the object in the block and its rotation. */
  readonly x: number;
  readonly y: number;
  readonly rotation: number;
}

interface ObjectBox {
  readonly container: Container;
  readonly floorIndex: number;
  /** North-west corner of the block in cells. */
  readonly x: number;
  readonly y: number;
  readonly items: ObjectItem[];
}

interface Sheet {
  readonly px: number;
  readonly canvas: HTMLCanvasElement;
  readonly ctx: CanvasRenderingContext2D;
  readonly texture: Texture;
}

/** The resting world of one session. */
export class WorldView {
  readonly layer = new Container();
  readonly #deps: WorldDeps;
  readonly #tileLayer = new Container();
  readonly #objectLayer = new Container();
  readonly #tiles = new Map<number, DataTile>();
  readonly #objects = new Map<number, ObjectBox>();
  readonly #received = new Set<number>();
  readonly #updatedAt = new Map<number, number>();
  readonly #slots = new SlotTable();
  readonly #meanData = new Uint8Array(SHEET_TILES * SHEET_TILES * 4);
  readonly #means = new Texture({
    source: new BufferImageSource({
      resource: this.#meanData,
      width: SHEET_TILES,
      height: SHEET_TILES,
      scaleMode: "nearest",
    }),
  });
  readonly #uniforms = new UniformGroup({
    uSimple: { value: 0, type: "f32" },
    uInset: { value: 0.02, type: "f32" },
  });
  #program: GlProgram | undefined;
  #sheet: Sheet | undefined;
  #sheetDirty = true;
  #catalog: readonly CatalogEntry[] = [];
  /** Largest side of a single object in cells, for the margin of the object culling. */
  #extent = 1;
  #width = 0;
  #height = 0;
  #pending = 0;
  #bytes = 0;
  #objectCount = 0;
  /** `true` to draw the objects also in the simplified overview ("all details", ADR-038). */
  showAll = false;

  constructor(deps: WorldDeps) {
    this.#deps = deps;
    this.layer.addChild(this.#tileLayer, this.#objectLayer);
  }

  /** Forgets everything of the old session (new session or new map). */
  reset(width: number, height: number): void {
    for (const tile of this.#tiles.values()) {
      tile.mesh.shader?.destroy();
      tile.mesh.destroy();
      tile.texture.destroy(true);
    }
    this.#tiles.clear();
    for (const box of this.#objects.values()) box.container.destroy({ children: true });
    this.#objects.clear();
    this.#received.clear();
    this.#updatedAt.clear();
    this.#slots.clear();
    this.#sheetDirty = true;
    this.#pending = 0;
    this.#bytes = 0;
    this.#objectCount = 0;
    this.#width = width;
    this.#height = height;
  }

  /** Map size the view was reset for. */
  get mapSize(): { width: number; height: number } {
    return { width: this.#width, height: this.#height };
  }

  /** The catalog of the session (S-7): names of the types behind the values of the blocks. */
  setCatalog(catalog: readonly CatalogEntry[]): void {
    this.#catalog = catalog;
    this.#extent = 1;
    for (const entry of catalog) {
      if (entry.placement.kind === "single") {
        this.#extent = Math.max(
          this.#extent,
          Math.ceil(Math.max(entry.placement.width, entry.placement.depth) / 2),
        );
      }
    }
    this.#sheetDirty = true;
    // Objects that arrived before the catalog know their type only by number.
    for (const box of this.#objects.values()) this.#nameSprites(box);
  }

  /** Reads the world section of a frame; blocks outside the map are ignored. */
  apply(section: WorldSection, now: number): WorldResult {
    this.#pending = section.pending;
    this.#bytes += section.bytes;
    const before = this.#slots.size;
    const result = readBlocks(section, (block) => {
      this.#applyBlock(block, now);
    });
    if (this.#slots.size !== before) this.#sheetDirty = true;
    return result;
  }

  #applyBlock(block: WorldBlock, now: number): void {
    const place = placeOf(block.index, this.#width, this.#height);
    if (place === undefined) return;
    const key = tileKey(place.floorIndex, place.x, place.y);
    const tile =
      this.#tiles.get(key) ??
      this.#addTile(key, place.floorIndex, Math.floor(place.x / TILE), Math.floor(place.y / TILE));
    writeBlockTexels(tile.data, place.x, place.y, block, this.#slots);
    tile.dirty = true;
    this.#received.add(block.index);
    this.#updatedAt.set(block.index, now);
    this.#setObjects(block, place.floorIndex, place.x, place.y);
  }

  #setObjects(block: WorldBlock, floorIndex: number, bx: number, by: number): void {
    const old = this.#objects.get(block.index);
    if (old !== undefined) {
      this.#objectCount -= old.items.length;
      old.container.destroy({ children: true });
      this.#objects.delete(block.index);
    }
    if (block.objects.length === 0) return;
    const container = new Container();
    const items: ObjectItem[] = block.objects.map((o) => {
      const sprite = new Sprite();
      sprite.anchor.set(0.5);
      sprite.rotation = (o.rotation * Math.PI) / 2;
      container.addChild(sprite);
      return { sprite, type: o.type, x: o.x, y: o.y, rotation: o.rotation };
    });
    const box: ObjectBox = { container, floorIndex, x: bx, y: by, items };
    this.#nameSprites(box);
    this.#objectCount += items.length;
    this.#objects.set(block.index, box);
    this.#objectLayer.addChild(container);
  }

  /**
   * Places the sprites of `box` and gives them the image of their type;
   * footprint and name come from the catalog (a 1 × 1 cell and the
   * fallback image until it is known).
   */
  #nameSprites(box: ObjectBox): void {
    for (const item of box.items) {
      const entry = this.#catalog[item.type - 1];
      const single = entry?.placement.kind === "single" ? entry.placement : undefined;
      const [w, d] = single === undefined ? [1, 1] : [single.width / 2, single.depth / 2];
      const [fw, fd] = item.rotation % 2 === 1 ? [d, w] : [w, d];
      item.sprite.position.set(box.x + item.x + fw / 2, box.y + item.y + fd / 2);
      const { texture, scale } = this.#deps.objectTexture(entry?.id ?? "");
      item.sprite.texture = texture;
      item.sprite.scale.set(scale);
    }
  }

  /** The atlas tier changed: objects and the sheet take their images from the new tier. */
  retier(): void {
    this.#sheetDirty = true;
    for (const box of this.#objects.values()) this.#nameSprites(box);
  }

  #glProgram(): GlProgram {
    this.#program ??= GlProgram.from({ vertex: VERTEX, fragment: FRAGMENT, name: "world-z1" });
    return this.#program;
  }

  #addTile(key: number, floorIndex: number, tx: number, ty: number): DataTile {
    const data = new Uint8Array(TILE * TILE * 4);
    const texture = new Texture({
      source: new BufferImageSource({
        resource: data,
        width: TILE,
        height: TILE,
        scaleMode: "nearest",
        alphaMode: "no-premultiply-alpha",
      }),
    });
    const x0 = tx * TILE;
    const y0 = ty * TILE;
    const geometry = new Geometry({
      attributes: {
        aPosition: new Float32Array([x0, y0, x0 + TILE, y0, x0 + TILE, y0 + TILE, x0, y0 + TILE]),
        aUV: new Float32Array([0, 0, TILE, 0, TILE, TILE, 0, TILE]),
      },
      indexBuffer: new Uint32Array([0, 1, 2, 0, 2, 3]),
    });
    const shader = new Shader({
      glProgram: this.#glProgram(),
      resources: {
        uData: texture.source,
        uSheet: (this.#sheet?.texture ?? Texture.WHITE).source,
        uMean: this.#means.source,
        params: this.#uniforms,
      },
    });
    const mesh = new Mesh({ geometry, shader });
    this.#tileLayer.addChild(mesh);
    const tile: DataTile = { floorIndex, data, texture, mesh, dirty: true };
    this.#tiles.set(key, tile);
    return tile;
  }

  /** Draws the tile sheet for `px` pixels per tile (the atlas tier, at least 8). */
  #renderSheet(px: number): void {
    if (this.#sheet?.px !== px) {
      const old = this.#sheet;
      const canvas = document.createElement("canvas");
      canvas.width = SHEET_TILES * px;
      canvas.height = SHEET_TILES * px;
      const ctx = canvas.getContext("2d", { willReadFrequently: true });
      if (ctx === null) return;
      const texture = Texture.from(canvas);
      texture.source.scaleMode = "linear";
      this.#sheet = { px, canvas, ctx, texture };
      for (const tile of this.#tiles.values()) {
        const shader = tile.mesh.shader;
        if (shader !== null) shader.resources.uSheet = texture.source;
      }
      old?.texture.destroy(true);
      // Half a texel inside the tile, so that linear filtering does not bleed from the neighbour.
      this.#uniforms.uniforms.uInset = 0.5 / px;
    }
    const sheet = this.#sheet;
    const names = (value: number): string | undefined => this.#catalog[value - 1]?.id;
    const result = renderSheet(
      sheet.ctx as unknown as SheetContext,
      px,
      this.#slots,
      names,
      (name, dx, dy) => this.#deps.drawTile(sheet.ctx, name, dx, dy, px),
    );
    this.#meanData.set(result.means);
    this.#means.source.update();
    sheet.texture.source.update();
  }

  /** `true` if block `index` is in view (objects keep a margin of the largest object). */
  #inView(index: number, c: WorldCamera, margin: number): boolean {
    return blockInView(index, this.#width, this.#height, c.floor, c, margin);
  }

  /** Prepares the next picture for `camera`. */
  update(camera: WorldCamera): void {
    const simple = camera.px < SIMPLE_BELOW_PX;
    if (this.#sheetDirty || this.#sheet === undefined) {
      this.#renderSheet(Math.max(8, camera.tierPx));
      this.#sheetDirty = false;
    } else if (this.#sheet.px !== Math.max(8, camera.tierPx)) {
      this.#renderSheet(Math.max(8, camera.tierPx));
    }
    this.#uniforms.uniforms.uSimple = simple ? 1 : 0;
    const floor = camera.floor + 2;
    for (const tile of this.#tiles.values()) {
      tile.mesh.visible = tile.floorIndex === floor;
      if (tile.dirty) {
        tile.texture.source.update();
        tile.dirty = false;
      }
    }
    const show = this.showAll || !simple;
    const margin = OBJECT_MARGIN_CELLS + this.#extent;
    for (const [index, box] of this.#objects) {
      box.container.visible =
        show && box.floorIndex === floor && this.#inView(index, camera, margin);
    }
  }

  /** `true` if every block of the shown floor in view was received. */
  visibleComplete(c: WorldCamera): boolean {
    if (this.#width === 0) return false;
    const base = (c.floor + 2) * Math.ceil(this.#width / BLOCK) * Math.ceil(this.#height / BLOCK);
    const bx = Math.ceil(this.#width / BLOCK);
    const x0 = Math.max(0, Math.floor(c.x0 / BLOCK));
    const y0 = Math.max(0, Math.floor(c.y0 / BLOCK));
    const x1 = Math.min(bx - 1, Math.floor(c.x1 / BLOCK));
    const y1 = Math.min(Math.ceil(this.#height / BLOCK) - 1, Math.floor(c.y1 / BLOCK));
    for (let y = y0; y <= y1; y += 1) {
      for (let x = x0; x <= x1; x += 1) {
        if (!this.#received.has(base + y * bx + x)) return false;
      }
    }
    return true;
  }

  /** `true` once the core has nothing more to send. */
  complete(): boolean {
    return this.#received.size > 0 && this.#pending === 0;
  }

  /** Time the block holding cell (`x`, `y`) of `floor` was last updated, or `undefined`. */
  updatedAt(floor: number, x: number, y: number): number | undefined {
    const bx = Math.ceil(this.#width / BLOCK);
    const perFloor = bx * Math.ceil(this.#height / BLOCK);
    return this.#updatedAt.get(
      (floor + 2) * perFloor + Math.floor(y / BLOCK) * bx + Math.floor(x / BLOCK),
    );
  }

  /** Figures for the measurement. */
  figures(): WorldFigures {
    const tileBytes = TILE * TILE * 4;
    const sheet =
      this.#sheet === undefined ? 0 : SHEET_TILES * SHEET_TILES * this.#sheet.px ** 2 * 4;
    return {
      blocks: this.#received.size,
      bytes: this.#bytes,
      pending: this.#pending,
      objects: this.#objectCount,
      tiles: this.#tiles.size,
      textureMb: Math.round(((this.#tiles.size * tileBytes + sheet) / 1048576) * 10) / 10,
      cpuMb: Math.round(((this.#tiles.size * tileBytes) / 1048576) * 10) / 10,
      slots: this.#slots.size,
    };
  }
}
