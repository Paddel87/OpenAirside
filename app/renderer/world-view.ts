/**
 * Spike 3.8 (throw-away code on branch `erkundung/3-8-messaufbau`, never
 * merged; docs/research/3.8-weltdarstellung.md): the resting world in the
 * playfield. Reads the world section of a frame (W-A) or the answers of the
 * query `blocks` (W-B) and draws floors, walls and zones in one of two ways:
 *
 * - Z-1: per 512 × 512 cells one data texture (one texel per cell) and a
 *   fragment shader that picks the tile from a sheet, or its mean colour in
 *   the simplified overview.
 * - Z-2: per visible block a texture rendered once in the current tier
 *   (cached, least recently used out), over a map of mean colours.
 *
 * Objects are sprites per block, hidden below 4 device pixels per cell
 * unless "show all" is set. Display only (rule 002).
 */
import {
  BufferImageSource,
  Container,
  Geometry,
  GlProgram,
  Mesh,
  type Renderer,
  RenderTexture,
  Shader,
  Sprite,
  Texture,
  UniformGroup,
} from "pixi.js";

/** Edge of a block in cells (core grid). */
export const BLOCK = 32;
/** Edge of a data texture in cells. */
export const TILE = 512;
/** Device pixels per cell below which the overview is simplified. */
export const SIMPLE_BELOW_PX = 4;
/** Most blocks rendered into textures per frame (Z-2). */
export const Z2_BLOCKS_PER_FRAME = 6;
/** Texture memory of the Z-2 block cache, bytes. */
export const Z2_CACHE_BYTES = 256 * 1024 * 1024;
/** Most `blocks` queries in flight (W-B). */
export const WB_IN_FLIGHT = 4;
/** Blocks per `blocks` query (core `MAX_QUERY_BLOCKS`). */
export const WB_QUERY_BLOCKS = 256;

const FLOORS = 5;
const RECORD_HEADER = 12;
const SECTION_HEADER = 16;
const ZONE_RGB: readonly [number, number, number][] = [
  [0.25, 0.44, 0.66],
  [0.37, 0.61, 0.31],
  [0.91, 0.72, 0.19],
  [0.78, 0.26, 0.23],
];
const ZONE_ALPHA = 0.25;

/** Draw way. */
export type DrawWay = "z1" | "z2";

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
  /** Atlas tier in pixels per cell. */
  readonly tierPx: number;
}

/** Figures for the measurement. */
export interface WorldFigures {
  readonly blocks: number;
  readonly bytes: number;
  readonly pending: number;
  readonly objects: number;
  /** Estimated texture memory of the world display, MB. */
  readonly textureMb: number;
  /** CPU copies of the world display, MB. */
  readonly cpuMb: number;
  readonly z2Cached: number;
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
out vec4 fragColor;
void main() {
  ivec2 c = clamp(ivec2(floor(vCell)), ivec2(0), ivec2(${String(TILE - 1)}));
  vec4 d = texelFetch(uData, c, 0);
  int cov = int(d.r * 255.0 + 0.5);
  int wall = int(d.g * 255.0 + 0.5);
  int zone = int(d.a * 255.0 + 0.5);
  int idx = wall > 0 ? wall : cov;
  if (idx == 255) discard;
  vec4 col;
  if (uSimple > 0.5) {
    col = texelFetch(uMean, ivec2(idx % 16, idx / 16), 0);
  } else {
    vec2 sub = clamp(fract(vCell), 0.02, 0.98);
    col = texture(uSheet, (vec2(float(idx % 16), float(idx / 16)) + sub) / 16.0);
  }
  vec3 zc = zone == 1 ? vec3(0.37, 0.61, 0.31) : zone == 2 ? vec3(0.91, 0.72, 0.19)
    : zone == 3 ? vec3(0.78, 0.26, 0.23) : vec3(0.25, 0.44, 0.66);
  fragColor = vec4(mix(col.rgb, zc, ${String(ZONE_ALPHA)}), 1.0);
}`;

/** Base colour of tile `i` as RGB 0–255 (synthetic, palette-like). */
function tileColour(i: number): [number, number, number] {
  if (i >= 200 && i < 204) return [74 + (i - 200) * 6, 120 + (i - 200) * 4, 62];
  if (i >= 128 && i < 144) return [70 + (i - 128) * 3, 72, 80];
  const zone = Math.floor(i / 8);
  const v = (i % 8) * 10;
  return [
    ([190, 180, 200, 170][zone % 4] ?? 180) + v,
    ([190, 200, 180, 170][zone % 4] ?? 180) + v,
    ([200, 170, 160, 190][zone % 4] ?? 180) - v,
  ];
}

/** Tile sheet of 16 × 16 tiles at `px` pixels each, plus its mean colours. */
function makeSheet(px: number): { sheet: Texture; mean: Texture; means: Uint8Array } {
  const canvas = document.createElement("canvas");
  canvas.width = 16 * px;
  canvas.height = 16 * px;
  const ctx = canvas.getContext("2d");
  const means = new Uint8Array(256 * 4);
  for (let i = 0; i < 256; i += 1) {
    const [r, g, b] = tileColour(i);
    means.set([r, g, b, 255], i * 4);
    if (ctx === null) continue;
    const x = (i % 16) * px;
    const y = Math.floor(i / 16) * px;
    ctx.fillStyle = `rgb(${String(r)},${String(g)},${String(b)})`;
    ctx.fillRect(x, y, px, px);
    ctx.fillStyle = "rgba(0,0,0,0.12)";
    if (i >= 128 && i < 144) {
      ctx.fillRect(x, y + px / 2, px, Math.max(1, px / 16));
    } else if (i >= 200) {
      for (let k = 0; k < px; k += 2) ctx.fillRect(x + ((k * 7) % px), y + k, 1, 1);
    } else {
      ctx.fillRect(x, y, px, Math.max(1, px / 16));
      ctx.fillRect(x, y, Math.max(1, px / 16), px);
    }
  }
  const sheet = Texture.from(canvas);
  sheet.source.scaleMode = "nearest";
  const mean = new Texture({
    source: new BufferImageSource({ resource: means, width: 16, height: 16, scaleMode: "nearest" }),
  });
  return { sheet, mean, means };
}

interface DataTile {
  readonly data: Uint8Array;
  readonly texture: Texture;
  readonly mesh: Mesh<Geometry, Shader>;
  readonly minimap: Uint8Array;
  readonly minimapTexture: Texture;
  readonly minimapSprite: Sprite;
  dirty: boolean;
}

interface CachedBlock {
  readonly sprite: Sprite;
  readonly texture: RenderTexture;
  readonly bytes: number;
  used: number;
}

/** The resting world of one session. */
export class WorldView {
  readonly layer = new Container();
  readonly #renderer: Renderer;
  readonly #objectTexture: (name: string) => { texture: Texture; scale: number };
  readonly #tiles = new Map<number, DataTile>();
  readonly #objects = new Map<number, Container>();
  readonly #received = new Set<number>();
  readonly #updatedAt = new Map<number, number>();
  readonly #queue = new Set<number>();
  readonly #inFlight = new Map<number, number>();
  readonly #z2 = new Map<number, CachedBlock>();
  readonly #objectLayer = new Container();
  readonly #tileLayer = new Container();
  readonly #z2Layer = new Container();
  #z2Bytes = 0;
  #sheets = new Map<number, ReturnType<typeof makeSheet>>();
  #program: GlProgram | undefined;
  #uniforms = new UniformGroup({ uSimple: { value: 0, type: "f32" } });
  #pending = 0;
  #bytes = 0;
  #objectCount = 0;
  #width = 0;
  #height = 0;
  #tierPx = 0;
  draw: DrawWay = "z1";
  showAll = false;

  constructor(
    renderer: Renderer,
    objectTexture: (name: string) => { texture: Texture; scale: number },
  ) {
    this.#renderer = renderer;
    this.#objectTexture = objectTexture;
    this.layer.addChild(this.#tileLayer, this.#z2Layer, this.#objectLayer);
  }

  /** Forgets everything (new session or new map). */
  reset(width: number, height: number): void {
    for (const tile of this.#tiles.values()) {
      tile.mesh.shader?.destroy();
      tile.mesh.destroy();
      tile.texture.destroy(true);
      tile.minimapSprite.destroy();
      tile.minimapTexture.destroy(true);
    }
    this.#tiles.clear();
    for (const c of this.#objects.values()) c.destroy({ children: true });
    this.#objects.clear();
    this.#clearZ2();
    this.#received.clear();
    this.#updatedAt.clear();
    this.#queue.clear();
    this.#inFlight.clear();
    this.#pending = 0;
    this.#bytes = 0;
    this.#objectCount = 0;
    this.#width = width;
    this.#height = height;
  }

  get #blocksX(): number {
    return Math.ceil(this.#width / BLOCK);
  }

  get #perFloor(): number {
    return this.#blocksX * Math.ceil(this.#height / BLOCK);
  }

  /** Reads the world section starting at `offset` of `buf`. */
  applySection(buf: ArrayBuffer, offset: number, now: number): void {
    const view = new DataView(buf, offset);
    if (view.byteLength < SECTION_HEADER) return;
    const bytes = view.getUint32(0, true);
    const blocks = view.getUint32(4, true);
    this.#pending = view.getUint32(8, true);
    const mode = view.getUint32(12, true);
    if (bytes > view.byteLength) return;
    this.#bytes += bytes;
    if (mode === 2) {
      for (let k = 0; k < blocks; k += 1) {
        this.#queue.add(view.getUint32(SECTION_HEADER + k * 4, true));
      }
      return;
    }
    this.#readRecords(view, SECTION_HEADER, bytes, blocks, now);
  }

  /** `true` if `id` is one of the `blocks` queries in flight. */
  ownsQuery(id: number): boolean {
    return this.#inFlight.has(id);
  }

  /** Answer of a `blocks` query. */
  applyAnswer(id: number, buf: ArrayBuffer, now: number): void {
    this.#inFlight.delete(id);
    const view = new DataView(buf);
    if (view.byteLength < 4) return;
    this.#bytes += view.byteLength;
    this.#readRecords(view, 4, view.byteLength, view.getUint32(0, true), now);
  }

  /** W-B: sends queries for queued blocks, visible ones first. */
  fetch(camera: WorldCamera, query: (blocks: number[]) => number): void {
    if (this.#queue.size === 0 || this.#inFlight.size >= WB_IN_FLIGHT) return;
    const all = [...this.#queue];
    all.sort((a, b) => Number(!this.#visible(a, camera)) - Number(!this.#visible(b, camera)));
    while (this.#inFlight.size < WB_IN_FLIGHT && all.length > 0) {
      const chunk = all.splice(0, WB_QUERY_BLOCKS);
      const id = query(chunk);
      if (id === 0) return;
      for (const b of chunk) this.#queue.delete(b);
      this.#inFlight.set(id, chunk.length);
    }
  }

  #readRecords(view: DataView, start: number, end: number, count: number, now: number): void {
    let at = start;
    for (let k = 0; k < count && at + RECORD_HEADER <= end; k += 1) {
      const index = view.getUint32(at, true);
      const encoding = view.getUint8(at + 4);
      const palette = view.getUint8(at + 5);
      const objects = view.getUint16(at + 6, true);
      const payload = view.getUint32(at + 8, true);
      const body = at + RECORD_HEADER;
      if (body + payload > end) return;
      if (encoding !== 255) {
        this.#applyBlock(index, view, body, encoding, palette, objects, payload);
      }
      this.#received.add(index);
      this.#updatedAt.set(index, now);
      at = body + payload;
    }
  }

  #applyBlock(
    index: number,
    view: DataView,
    body: number,
    encoding: number,
    palette: number,
    objects: number,
    payload: number,
  ): void {
    const floorIndex = Math.floor(index / this.#perFloor);
    const inFloor = index % this.#perFloor;
    const bx = (inFloor % this.#blocksX) * BLOCK;
    const by = Math.floor(inFloor / this.#blocksX) * BLOCK;
    const tileKey = floorIndex * 1024 + Math.floor(by / TILE) * 32 + Math.floor(bx / TILE);
    const tile = this.#tile(tileKey, floorIndex, Math.floor(bx / TILE), Math.floor(by / TILE));
    const value = (k: number): number => body + k * 8;
    let cellsEnd: number;
    const write = (cell: number, src: number): void => {
      const x = (bx % TILE) + (cell % BLOCK);
      const y = (by % TILE) + Math.floor(cell / BLOCK);
      const o = (y * TILE + x) * 4;
      const covering = view.getUint16(src + 4, true);
      const wall = view.getUint16(src + 6, true);
      const r = covering === 0xffff ? 255 : covering & 0xff;
      const g = wall & 0xff;
      const zone = view.getUint8(src + 1) & 3;
      tile.data[o] = r;
      tile.data[o + 1] = g;
      tile.data[o + 2] = view.getUint8(src);
      tile.data[o + 3] = zone;
      const idx = g > 0 ? g : r;
      const [zr, zg, zb] = ZONE_RGB[zone] ?? [0, 0, 0];
      const m = this.#sheet(8).means;
      tile.minimap[o] =
        idx === 255 ? 0 : (m[idx * 4] ?? 0) * (1 - ZONE_ALPHA) + zr * 255 * ZONE_ALPHA;
      tile.minimap[o + 1] =
        idx === 255 ? 0 : (m[idx * 4 + 1] ?? 0) * (1 - ZONE_ALPHA) + zg * 255 * ZONE_ALPHA;
      tile.minimap[o + 2] =
        idx === 255 ? 0 : (m[idx * 4 + 2] ?? 0) * (1 - ZONE_ALPHA) + zb * 255 * ZONE_ALPHA;
      tile.minimap[o + 3] = idx === 255 ? 0 : 255;
    };
    if (encoding === 3) {
      for (let c = 0; c < BLOCK * BLOCK; c += 1) write(c, value(c));
      cellsEnd = body + BLOCK * BLOCK * 8;
    } else {
      const indices = body + palette * 8;
      for (let c = 0; c < BLOCK * BLOCK; c += 1) {
        const p = encoding === 0 ? 0 : (view.getUint8(indices + (c >> 1)) >> ((c & 1) * 4)) & 15;
        write(c, value(p));
      }
      cellsEnd = indices + (encoding === 0 ? 0 : (BLOCK * BLOCK) / 2);
    }
    cellsEnd = Math.ceil(cellsEnd / 4) * 4;
    tile.dirty = true;
    const cached = this.#z2.get(index);
    if (cached !== undefined) this.#dropZ2(index, cached);
    this.#setObjects(index, floorIndex, bx, by, view, cellsEnd, Math.min(objects, payload));
  }

  #setObjects(
    index: number,
    floorIndex: number,
    bx: number,
    by: number,
    view: DataView,
    at: number,
    count: number,
  ): void {
    const old = this.#objects.get(index);
    if (old !== undefined) {
      this.#objectCount -= old.children.length;
      old.destroy({ children: true });
      this.#objects.delete(index);
    }
    if (count === 0) return;
    const box = new Container();
    box.label = String(floorIndex);
    for (let k = 0; k < count && at + 6 * k + 6 <= view.byteLength; k += 1) {
      const o = at + k * 6;
      const graphic = view.getUint16(o, true);
      const sprite = new Sprite();
      sprite.label = graphicName(graphic);
      sprite.position.set(bx + view.getUint8(o + 2), by + view.getUint8(o + 3));
      box.addChild(sprite);
    }
    this.#objectCount += box.children.length;
    this.#applyTierTo(box);
    this.#objects.set(index, box);
    this.#objectLayer.addChild(box);
  }

  #applyTierTo(box: Container): void {
    for (const child of box.children as Sprite[]) {
      const { texture, scale } = this.#objectTexture(child.label);
      child.texture = texture;
      child.scale.set(scale);
    }
  }

  #sheet(px: number): ReturnType<typeof makeSheet> {
    let s = this.#sheets.get(px);
    if (s === undefined) {
      s = makeSheet(px);
      this.#sheets.set(px, s);
    }
    return s;
  }

  #tile(key: number, floorIndex: number, tx: number, ty: number): DataTile {
    const existing = this.#tiles.get(key);
    if (existing !== undefined) return existing;
    const data = new Uint8Array(TILE * TILE * 4).fill(255);
    for (let i = 1; i < data.length; i += 4) data[i] = 0;
    const texture = new Texture({
      source: new BufferImageSource({
        resource: data,
        width: TILE,
        height: TILE,
        scaleMode: "nearest",
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
    const sheet = this.#sheet(Math.max(8, this.#tierPx));
    const shader = new Shader({
      glProgram: this.#glProgram(),
      resources: {
        uData: texture.source,
        uSheet: sheet.sheet.source,
        uMean: sheet.mean.source,
        params: this.#uniforms,
      },
    });
    const mesh = new Mesh({ geometry, shader });
    mesh.label = String(floorIndex);
    const minimap = new Uint8Array(TILE * TILE * 4);
    const minimapTexture = new Texture({
      source: new BufferImageSource({
        resource: minimap,
        width: TILE,
        height: TILE,
        scaleMode: "nearest",
      }),
    });
    const minimapSprite = new Sprite(minimapTexture);
    minimapSprite.position.set(x0, y0);
    minimapSprite.label = String(floorIndex);
    this.#tileLayer.addChild(mesh);
    this.#z2Layer.addChildAt(minimapSprite, 0);
    const tile = { data, texture, mesh, minimap, minimapTexture, minimapSprite, dirty: true };
    this.#tiles.set(key, tile);
    return tile;
  }

  #glProgram(): GlProgram {
    this.#program ??= GlProgram.from({ vertex: VERTEX, fragment: FRAGMENT, name: "world-z1" });
    return this.#program;
  }

  #visible(index: number, c: WorldCamera): boolean {
    const floorIndex = Math.floor(index / this.#perFloor);
    if (floorIndex !== c.floor + 2) return false;
    const inFloor = index % this.#perFloor;
    const x = (inFloor % this.#blocksX) * BLOCK;
    const y = Math.floor(inFloor / this.#blocksX) * BLOCK;
    return x + BLOCK >= c.x0 && x <= c.x1 && y + BLOCK >= c.y0 && y <= c.y1;
  }

  /** `true` if every ground floor block in view was received. */
  visibleComplete(c: WorldCamera): boolean {
    if (this.#width === 0) return false;
    const x0 = Math.max(0, Math.floor(c.x0 / BLOCK));
    const y0 = Math.max(0, Math.floor(c.y0 / BLOCK));
    const x1 = Math.min(this.#blocksX - 1, Math.floor(c.x1 / BLOCK));
    const y1 = Math.min(Math.ceil(this.#height / BLOCK) - 1, Math.floor(c.y1 / BLOCK));
    const base = (c.floor + 2) * this.#perFloor;
    for (let y = y0; y <= y1; y += 1) {
      for (let x = x0; x <= x1; x += 1) {
        if (!this.#received.has(base + y * this.#blocksX + x)) return false;
      }
    }
    return true;
  }

  /** `true` once nothing waits in the core or in the query queue. */
  complete(): boolean {
    return (
      this.#received.size > 0 &&
      this.#pending === 0 &&
      this.#queue.size === 0 &&
      this.#inFlight.size === 0
    );
  }

  /** Time the block holding cell (x, y) of `floor` was last updated, or undefined. */
  updatedAt(floor: number, x: number, y: number): number | undefined {
    const index =
      (floor + 2) * this.#perFloor + Math.floor(y / BLOCK) * this.#blocksX + Math.floor(x / BLOCK);
    return this.#updatedAt.get(index);
  }

  /** Prepares the next picture for `camera`. */
  update(camera: WorldCamera): void {
    const simple = camera.px < SIMPLE_BELOW_PX;
    if (camera.tierPx !== this.#tierPx) {
      this.#tierPx = camera.tierPx;
      this.#clearZ2();
      for (const box of this.#objects.values()) this.#applyTierTo(box);
      const sheet = this.#sheet(Math.max(8, camera.tierPx));
      for (const tile of this.#tiles.values()) {
        const shader = tile.mesh.shader;
        if (shader !== null) {
          shader.resources.uSheet = sheet.sheet.source;
          shader.resources.uMean = sheet.mean.source;
        }
      }
    }
    this.#uniforms.uniforms.uSimple = simple ? 1 : 0;
    const floorLabel = String(camera.floor + 2);
    const z1 = this.draw === "z1";
    this.#tileLayer.visible = z1;
    this.#z2Layer.visible = !z1;
    for (const tile of this.#tiles.values()) {
      const shown = tile.mesh.label === floorLabel;
      tile.mesh.visible = shown;
      tile.minimapSprite.visible = shown;
      if (tile.dirty) {
        if (z1) tile.texture.source.update();
        else tile.minimapTexture.source.update();
        tile.dirty = false;
      }
    }
    for (const [index, box] of this.#objects) {
      box.visible =
        box.label === floorLabel && (this.showAll || !simple) && this.#visible(index, camera);
    }
    if (!z1) this.#updateZ2(camera, simple);
  }

  #updateZ2(camera: WorldCamera, simple: boolean): void {
    for (const cached of this.#z2.values()) cached.sprite.visible = false;
    if (simple) return;
    const px = Math.max(8, camera.tierPx);
    const now = performance.now();
    let rendered = 0;
    const x0 = Math.max(0, Math.floor(camera.x0 / BLOCK));
    const y0 = Math.max(0, Math.floor(camera.y0 / BLOCK));
    const x1 = Math.min(this.#blocksX - 1, Math.floor(camera.x1 / BLOCK));
    const y1 = Math.min(Math.ceil(this.#height / BLOCK) - 1, Math.floor(camera.y1 / BLOCK));
    const base = (camera.floor + 2) * this.#perFloor;
    for (let by = y0; by <= y1; by += 1) {
      for (let bx = x0; bx <= x1; bx += 1) {
        const index = base + by * this.#blocksX + bx;
        let cached = this.#z2.get(index);
        if (cached === undefined) {
          if (rendered >= Z2_BLOCKS_PER_FRAME || !this.#received.has(index)) continue;
          cached = this.#renderBlock(index, bx * BLOCK, by * BLOCK, px);
          rendered += 1;
        }
        cached.used = now;
        cached.sprite.visible = true;
      }
    }
    while (this.#z2Bytes > Z2_CACHE_BYTES) {
      let oldest: [number, CachedBlock] | undefined;
      for (const entry of this.#z2)
        if (oldest === undefined || entry[1].used < oldest[1].used) oldest = entry;
      if (oldest === undefined) break;
      this.#dropZ2(oldest[0], oldest[1]);
    }
  }

  #renderBlock(index: number, x: number, y: number, px: number): CachedBlock {
    const floorIndex = Math.floor(index / this.#perFloor);
    const tile = this.#tiles.get(
      floorIndex * 1024 + Math.floor(y / TILE) * 32 + Math.floor(x / TILE),
    );
    const sheet = this.#sheet(px);
    const box = new Container();
    const frames: Texture[] = [];
    for (let i = 0; i < 256; i += 1) {
      frames.push(
        new Texture({
          source: sheet.sheet.source,
          frame: { x: (i % 16) * px, y: Math.floor(i / 16) * px, width: px, height: px } as never,
        }),
      );
    }
    for (let c = 0; c < BLOCK * BLOCK; c += 1) {
      const cx = (x % TILE) + (c % BLOCK);
      const cy = (y % TILE) + Math.floor(c / BLOCK);
      const o = (cy * TILE + cx) * 4;
      const r = tile?.data[o] ?? 255;
      const g = tile?.data[o + 1] ?? 0;
      const idx = g > 0 ? g : r;
      if (idx === 255) continue;
      const [zr, zg, zb] = ZONE_RGB[(tile?.data[o + 3] ?? 0) & 3] ?? [0, 0, 0];
      const s = new Sprite(frames[idx]);
      s.position.set((c % BLOCK) * px, Math.floor(c / BLOCK) * px);
      const t = (v: number): number => Math.round((1 - ZONE_ALPHA + ZONE_ALPHA * v) * 255);
      s.tint = (t(zr) << 16) | (t(zg) << 8) | t(zb);
      box.addChild(s);
    }
    const texture = RenderTexture.create({ width: BLOCK * px, height: BLOCK * px });
    this.#renderer.render({ container: box, target: texture });
    box.destroy({ children: true });
    for (const f of frames) f.destroy(false);
    const sprite = new Sprite(texture);
    sprite.position.set(x, y);
    sprite.scale.set(1 / px);
    this.#z2Layer.addChild(sprite);
    const cached = { sprite, texture, bytes: BLOCK * px * BLOCK * px * 4, used: 0 };
    this.#z2.set(index, cached);
    this.#z2Bytes += cached.bytes;
    return cached;
  }

  #dropZ2(index: number, cached: CachedBlock): void {
    cached.sprite.destroy();
    cached.texture.destroy(true);
    this.#z2Bytes -= cached.bytes;
    this.#z2.delete(index);
  }

  #clearZ2(): void {
    for (const [index, cached] of [...this.#z2]) this.#dropZ2(index, cached);
  }

  /** Figures for the measurement. */
  figures(): WorldFigures {
    const tileBytes = TILE * TILE * 4;
    let sheets = 0;
    for (const [px] of this.#sheets) sheets += 16 * px * 16 * px * 4;
    const z1 = this.draw === "z1";
    const tex = this.#tiles.size * tileBytes + sheets + (z1 ? 0 : this.#z2Bytes);
    return {
      blocks: this.#received.size,
      bytes: this.#bytes,
      pending: this.#pending + this.#queue.size,
      objects: this.#objectCount,
      textureMb: Math.round((tex / 1048576) * 10) / 10,
      cpuMb: Math.round(((this.#tiles.size * tileBytes * 2) / 1048576) * 10) / 10,
      z2Cached: this.#z2.size,
    };
  }
}

/** Atlas name of generated graphic `n` (scripts in spikes/3.8-weltdarstellung). */
export function graphicName(n: number): string {
  return `base:spike_g_${String(n).padStart(3, "0")}`;
}

/** Number of floors (for the tests of the reader). */
export const WORLD_FLOORS = FLOORS;
