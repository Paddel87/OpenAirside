/**
 * Playfield (module "Spielfeld"): top view with PixiJS over WebGL2. Shows
 * the resting world of the newest frames (floors, walls, zones, objects;
 * `world-view.ts`, S-1 layout 2, ADR-038), the provisional drawing of build
 * actions (S-2) and the persons as particles in one `ParticleContainer`
 * (spike 1.3: sprites are too slow at 50,000 objects). It loads the texture
 * atlases of the graphics pipeline (step 2.10). Display only (rule 002).
 *
 * Synchronous WebGL queries (`getParameter`, `getExtension`, `getError`,
 * `readPixels` and the like) only at start, never per frame or from a timer:
 * each blocks the main thread until the GPU process answers (step 3.14/3.15,
 * gpu-name.ts).
 */
import {
  Application,
  Container,
  Graphics,
  Particle,
  ParticleContainer,
  Rectangle,
  Sprite,
  Texture,
  UPDATE_PRIORITY,
} from "pixi.js";
// Replaces the eval-based particle upload, so that the CSP needs no 'unsafe-eval' (spike 1.3).
import "pixi.js/unsafe-eval";
import type { CatalogEntry } from "../src/generated/CatalogEntry.js";
import type { FrameFilter } from "../src/generated/FrameFilter.js";
import { type AtlasIndex, bundledIndex, chooseTier, lookup, textureBytes } from "./atlas.js";
import { type Camera, centerCamera, fitCamera, pan, visibleRect, zoomAt } from "./camera.js";
import { type Frame, type PersonRecord, readPerson } from "./frame-reader.js";
import { GpuName } from "./gpu-name.js";
import { densestSpot } from "./hotspot.js";
import type { AtlasInfo, View } from "./measure.js";
import { type WorldCamera, type WorldFigures, WorldView } from "./world-view.js";
import type { WorldResult } from "./world-reader.js";

/** Lowest and highest floor (BAU-07). */
const MIN_FLOOR = -2;
const MAX_FLOOR = 2;
/** Diameter of a person in cells. */
const PERSON_SIZE = 0.8;
/** Smallest diameter of a person on the screen in pixels, so that crowds stay visible in the overview. */
const MIN_PERSON_PIXELS = 2;
/** Colours of the provisional drawing (palette): area, object, demolition, refused. */
const PREVIEW_COLOURS = { build: 0x3f6fa8, demolish: 0xc8433b, refused: 0xe8b730 } as const;

/** Loaded atlas: the index, one texture per page of each tier and the page images for the tile sheet. */
interface Atlas {
  readonly index: AtlasIndex;
  readonly pages: readonly (readonly Texture[])[];
  readonly images: readonly (readonly HTMLImageElement[])[];
}

async function loadImage(url: string): Promise<HTMLImageElement> {
  const image = new Image();
  image.src = url;
  await image.decode();
  return image;
}

/** Loads the pages of the bundled atlas; `undefined` without atlas or if a page fails to load. */
async function loadAtlas(): Promise<Atlas | undefined> {
  const index = bundledIndex();
  if (index === undefined) {
    return undefined;
  }
  try {
    const images = await Promise.all(
      index.tiers.map((tier) => Promise.all(tier.pages.map((p) => loadImage(p.image)))),
    );
    return { index, images, pages: images.map((tier) => tier.map((i) => Texture.from(i))) };
  } catch {
    return undefined;
  }
}

function dotTexture(): Texture {
  const canvas = document.createElement("canvas");
  canvas.width = 16;
  canvas.height = 16;
  const ctx = canvas.getContext("2d");
  if (ctx !== null) {
    ctx.fillStyle = "#ffffff";
    ctx.beginPath();
    ctx.arc(8, 8, 7, 0, Math.PI * 2);
    ctx.fill();
  }
  return Texture.from(canvas);
}

/** A shape of the provisional drawing of a build action (S-2). */
export interface PreviewShape {
  /** What the action does: builds (area or object), demolishes, or was refused. */
  readonly kind: keyof typeof PREVIEW_COLOURS;
  /** Cells, half open. */
  readonly x0: number;
  readonly y0: number;
  readonly x1: number;
  readonly y1: number;
  /** Image name of the object for a single object; the area is drawn as a rectangle. */
  readonly object?: { readonly name: string; readonly rotation: number };
}

/** A pointer event for the build tool, in cells. */
export interface CellPointer {
  readonly phase: "down" | "move" | "up" | "cancel";
  /** Cell under the pointer (may be outside the map). */
  readonly x: number;
  readonly y: number;
  /** Mouse button of `down` and `up` (0 = primary), −1 for a move. */
  readonly button: number;
}

/** Handler for pointer events; `true` consumes the gesture (no panning). */
export type PointerHandler = (event: CellPointer) => boolean;

/** The playfield with camera and floor selection. */
export class Playfield {
  readonly #app: Application;
  readonly #world = new Container();
  readonly #bounds = new Graphics();
  readonly #preview = new Container();
  /** Frames rendered so far. */
  #renders = 0;
  #previewWaiters: { since: number; resolve: (drawn: { frames: number; at: number }) => void }[] =
    [];
  readonly #view: WorldView;
  readonly #atlas: Atlas | undefined;
  readonly #textures = new Map<string, { texture: Texture; scale: number }>();
  #tier = -1;
  readonly #persons: ParticleContainer;
  readonly #pool: Particle[] = [];
  readonly #texture: Texture;
  readonly #person: PersonRecord = { x: 0, y: 0, floor: 0, heading: 0, group: 0, state: 0 };
  #camera: Camera = { zoom: 1, x: 0, y: 0 };
  #floor = 0;
  #map = { width: 0, height: 0 };
  #session = 0;
  #shown = 0;
  #pointer: PointerHandler | undefined;
  readonly #gpu: GpuName;

  private constructor(app: Application, atlas: Atlas | undefined) {
    this.#app = app;
    this.#atlas = atlas;
    const canvas = app.canvas;
    this.#gpu = new GpuName({
      webgl2: () => canvas.getContext("webgl2"),
      webgl: () => canvas.getContext("webgl"),
      rendererName: app.renderer.name,
    });
    this.#texture = dotTexture();
    this.#persons = new ParticleContainer({
      texture: this.#texture,
      dynamicProperties: { position: true },
    });
    this.#view = new WorldView({
      objectTexture: (name) => this.#objectTexture(name),
      drawTile: (ctx, name, dx, dy, px) => this.#drawTile(ctx, name, dx, dy, px),
    });
    this.#world.addChild(this.#bounds, this.#view.layer, this.#preview, this.#persons);
    app.stage.addChild(this.#world);
    this.#applyTier();
    app.ticker.add(() => {
      this.#view.update(this.#worldCamera());
    });
    // After the render of the application (priority LOW): counts rendered frames for previewDrawn().
    app.ticker.add(
      () => {
        this.#renders += 1;
        const waiting = this.#previewWaiters;
        if (waiting.length === 0) return;
        this.#previewWaiters = [];
        const at = performance.now();
        for (const w of waiting) w.resolve({ frames: this.#renders - w.since, at });
      },
      undefined,
      UPDATE_PRIORITY.UTILITY,
    );
  }

  /**
   * Resolves after the first frame rendered since the call, which holds the provisional drawing
   * set before it (for the measurement series, NFR "Bauaktion im nächsten Bild").
   */
  previewDrawn(): Promise<{ frames: number; at: number }> {
    return new Promise((resolve) => {
      this.#previewWaiters.push({ since: this.#renders, resolve });
    });
  }

  /** Creates the playfield in `parent`. */
  static async create(parent: HTMLElement): Promise<Playfield> {
    const app = new Application();
    await app.init({
      preference: "webgl",
      resizeTo: parent,
      antialias: false,
      background: 0x111317,
      resolution: window.devicePixelRatio,
      autoDensity: true,
    });
    parent.append(app.canvas);
    const field = new Playfield(app, await loadAtlas());
    field.#bindInput(app.canvas);
    return field;
  }

  /**
   * Name of the GPU, for the performance display and the measurement protocol.
   * Read once at start: no synchronous WebGL queries while the game runs (step 3.15, see gpu-name.ts).
   */
  get gpu(): string {
    return this.#gpu.value;
  }

  /** `true` if the atlas of the build was loaded. */
  get hasAtlas(): boolean {
    return this.#atlas !== undefined;
  }

  /** Resolution drawn with now in pixels per cell, 0 without atlas. */
  tierPx(): number {
    return this.#atlas?.index.tiers[this.#tier]?.pxPerCell ?? 0;
  }

  /** The atlas of the build for the measurement result. */
  atlas(): AtlasInfo {
    const index = this.#atlas?.index;
    return {
      tiers: index?.tiers.map((t) => t.pxPerCell) ?? [],
      pages: index?.tiers.reduce((sum, t) => sum + t.pages.length, 0) ?? 0,
      textureMbMax:
        index === undefined
          ? 0
          : Math.round((textureBytes(index, index.tiers.length - 1) / 1048576) * 100) / 100,
    };
  }

  /** The catalog of the session (S-7): needed to name the objects and floors of the blocks. */
  setCatalog(catalog: readonly CatalogEntry[]): void {
    this.#view.setCatalog(catalog);
  }

  /** `true` draws the objects also in the simplified overview ("all details", ADR-038). */
  set details(all: boolean) {
    this.#view.showAll = all;
  }

  /** Figures of the world display for the measurement. */
  worldFigures(): WorldFigures {
    return this.#view.figures();
  }

  /** `true` if every block of the shown floor in view was received. */
  worldVisibleComplete(): boolean {
    return this.#view.visibleComplete(this.#worldCamera());
  }

  /** `true` once the core has nothing more to send of the resting world. */
  worldComplete(): boolean {
    return this.#view.complete();
  }

  /** Time the block holding cell (`x`, `y`) of the shown floor was last updated, or `undefined`. */
  worldUpdatedAt(x: number, y: number): number | undefined {
    return this.#view.updatedAt(this.#floor, x, y);
  }

  /** Shown floor. */
  get floor(): number {
    return this.#floor;
  }

  /** Persons drawn from the newest frame. */
  get shown(): number {
    return this.#shown;
  }

  /** Filter of the next frame request: shown floor and visible rectangle. */
  filter(): FrameFilter {
    const { width, height } = this.#app.screen;
    return { floor: this.#floor, view: visibleRect(this.#camera, width, height) };
  }

  /** Shows the floor above (`+1`) or below (`-1`). */
  changeFloor(step: number): void {
    this.#floor = Math.min(MAX_FLOOR, Math.max(MIN_FLOOR, this.#floor + step));
  }

  /** Points the camera for the measurement series (step 2.7). */
  view(view: View): void {
    this.#floor = Math.min(MAX_FLOOR, Math.max(MIN_FLOOR, view.floor));
    const { width, height } = this.#app.screen;
    if (view.kind === "overview") {
      this.#setCamera(fitCamera(this.#map.width, this.#map.height, width, height));
      return;
    }
    const x = view.x + Math.cos(view.angle) * view.radius;
    const y = view.y + Math.sin(view.angle) * view.radius;
    this.#setCamera(centerCamera(view.zoom, x, y, width, height));
  }

  /** Centre of the densest 16 × 16 cells among the persons drawn now. */
  hotspot(): { x: number; y: number } {
    return densestSpot(
      this.#pool.slice(0, this.#shown).map((p) => ({ x: p.x, y: p.y })),
      this.#map.width,
      this.#map.height,
    );
  }

  /** Sets the handler of the pointer events in cells, or removes it with `undefined` (build tool). */
  onPointer(handler: PointerHandler | undefined): void {
    this.#pointer = handler;
  }

  /** Cell under the client position (`clientX`, `clientY`). */
  cellAt(clientX: number, clientY: number): { x: number; y: number } {
    const rect = this.#app.canvas.getBoundingClientRect();
    return {
      x: Math.floor(this.#camera.x + (clientX - rect.left) / this.#camera.zoom),
      y: Math.floor(this.#camera.y + (clientY - rect.top) / this.#camera.zoom),
    };
  }

  /** Draws the provisional drawing of build actions (S-2); an empty list clears it. */
  setPreview(shapes: readonly PreviewShape[]): void {
    for (const child of this.#preview.removeChildren()) {
      child.destroy({ children: true });
    }
    for (const shape of shapes) {
      const colour = PREVIEW_COLOURS[shape.kind];
      const w = shape.x1 - shape.x0;
      const h = shape.y1 - shape.y0;
      if (shape.object !== undefined) {
        const { texture, scale } = this.#objectTexture(shape.object.name);
        const sprite = new Sprite(texture);
        sprite.anchor.set(0.5);
        sprite.scale.set(scale);
        sprite.rotation = (shape.object.rotation * Math.PI) / 2;
        sprite.position.set(shape.x0 + w / 2, shape.y0 + h / 2);
        sprite.alpha = 0.7;
        this.#preview.addChild(sprite);
      }
      const frame = new Graphics();
      frame
        .rect(shape.x0, shape.y0, w, h)
        .fill({ color: colour, alpha: shape.object === undefined ? 0.35 : 0.15 })
        .stroke({ color: colour, width: 2 / this.#camera.zoom, alpha: 0.9 });
      this.#preview.addChild(frame);
    }
  }

  /**
   * Shows the world and the persons of `frame`. A new session or a new map
   * size starts the world display afresh (the core starts the transfer
   * afresh, S-1). Returns the result of reading the world section.
   */
  show(frame: Frame, now: number): WorldResult {
    const { session, mapWidth, mapHeight, persons } = frame.header;
    const resized = mapWidth !== this.#map.width || mapHeight !== this.#map.height;
    if (session !== this.#session || resized) {
      this.#session = session;
      this.#map = { width: mapWidth, height: mapHeight };
      this.#view.reset(mapWidth, mapHeight);
      this.setPreview([]);
    }
    if (resized) {
      this.#bounds.clear().rect(0, 0, mapWidth, mapHeight).fill(0x1d232b);
      const { width, height } = this.#app.screen;
      this.#setCamera(fitCamera(mapWidth, mapHeight, width, height));
    }
    const result = this.#view.apply(frame.world, now);
    while (this.#pool.length < persons) {
      const scale = this.#personScale();
      this.#pool.push(
        new Particle({
          texture: this.#texture,
          scaleX: scale,
          scaleY: scale,
          anchorX: 0.5,
          anchorY: 0.5,
          tint: 0xf2a541,
        }),
      );
    }
    for (let i = 0; i < persons; i += 1) {
      const p = readPerson(frame, i, this.#person);
      const particle = this.#pool[i];
      if (particle !== undefined) {
        particle.x = p.x;
        particle.y = p.y;
      }
    }
    if (this.#shown !== persons) {
      this.#persons.particleChildren = this.#pool.slice(0, persons);
      this.#persons.update();
      this.#shown = persons;
    }
    return result;
  }

  /** Camera for the world view. */
  #worldCamera(): WorldCamera {
    const { width, height } = this.#app.screen;
    const rect = visibleRect(this.#camera, width, height);
    return {
      floor: this.#floor,
      ...rect,
      px: this.#camera.zoom * this.#app.renderer.resolution,
      tierPx: this.tierPx(),
    };
  }

  /** Scale of the particle texture for the current zoom. */
  #personScale(): number {
    const cells = Math.max(PERSON_SIZE, MIN_PERSON_PIXELS / this.#camera.zoom);
    return cells / this.#texture.width;
  }

  /** Takes the images of the tier for the current zoom, if the tier changed. */
  #applyTier(): void {
    const atlas = this.#atlas;
    const tier =
      atlas === undefined
        ? 0
        : chooseTier(atlas.index, this.#camera.zoom, this.#app.renderer.resolution);
    if (tier === this.#tier) {
      return;
    }
    this.#tier = tier;
    const old = [...this.#textures.values()];
    this.#textures.clear();
    this.#view.retier();
    // Views into the atlas pages only; the pages themselves stay.
    for (const { texture } of old) {
      if (texture !== Texture.WHITE) {
        texture.destroy(false);
      }
    }
  }

  /** Texture of `name` in the current tier and its scale to cells (cached per tier). */
  #objectTexture(name: string): { texture: Texture; scale: number } {
    const cached = this.#textures.get(name);
    if (cached !== undefined) {
      return cached;
    }
    const atlas = this.#atlas;
    const hit = atlas === undefined ? undefined : lookup(atlas.index, this.#tier, name);
    const page = hit === undefined ? undefined : atlas?.pages[this.#tier]?.[hit.page];
    let entry: { texture: Texture; scale: number };
    if (hit === undefined || page === undefined || atlas === undefined) {
      // No atlas built: one neutral cell (Texture.WHITE is 1 × 1 px).
      entry = { texture: Texture.WHITE, scale: 1 };
    } else {
      const { x, y, w, h } = hit.frame;
      entry = {
        texture: new Texture({ source: page.source, frame: new Rectangle(x, y, w, h) }),
        scale: 1 / (atlas.index.tiers[this.#tier]?.pxPerCell ?? 1),
      };
    }
    this.#textures.set(name, entry);
    return entry;
  }

  /** Draws the image `name` of the current tier into a tile of the sheet; `false` for an unknown image. */
  #drawTile(
    ctx: CanvasRenderingContext2D,
    name: string,
    dx: number,
    dy: number,
    px: number,
  ): boolean {
    const atlas = this.#atlas;
    if (atlas === undefined) {
      return false;
    }
    const hit = lookup(atlas.index, this.#tier, name);
    const image = hit === undefined ? undefined : atlas.images[this.#tier]?.[hit.page];
    if (hit === undefined || hit.fallback || image === undefined) {
      return false;
    }
    const { x, y, w, h } = hit.frame;
    ctx.drawImage(image, x, y, w, h, dx, dy, px, px);
    return true;
  }

  #setCamera(camera: Camera): void {
    const zoomed = camera.zoom !== this.#camera.zoom;
    this.#camera = camera;
    this.#world.scale.set(camera.zoom);
    this.#world.position.set(-camera.x * camera.zoom, -camera.y * camera.zoom);
    if (zoomed) {
      this.#applyTier();
      const scale = this.#personScale();
      for (const particle of this.#pool) {
        particle.scaleX = scale;
        particle.scaleY = scale;
      }
      // Scale is a static property of the container; upload it again.
      this.#persons.update();
    }
  }

  #bindInput(canvas: HTMLCanvasElement): void {
    let drag: { x: number; y: number } | undefined;
    let consumed = false;
    const emit = (phase: CellPointer["phase"], e: PointerEvent): boolean => {
      if (this.#pointer === undefined) return false;
      const { x, y } = this.cellAt(e.clientX, e.clientY);
      return this.#pointer({ phase, x, y, button: phase === "move" ? -1 : e.button });
    };
    canvas.addEventListener("pointerdown", (e) => {
      canvas.setPointerCapture(e.pointerId);
      consumed = emit("down", e);
      if (!consumed) drag = { x: e.clientX, y: e.clientY };
    });
    canvas.addEventListener("pointermove", (e) => {
      if (consumed) {
        emit("move", e);
      } else if (drag !== undefined) {
        this.#setCamera(pan(this.#camera, e.clientX - drag.x, e.clientY - drag.y));
        drag = { x: e.clientX, y: e.clientY };
      } else {
        // A hovering pointer: the tool may show where it would build.
        emit("move", e);
      }
    });
    const release = (e: PointerEvent, phase: "up" | "cancel"): void => {
      if (consumed) emit(phase, e);
      consumed = false;
      drag = undefined;
    };
    canvas.addEventListener("pointerup", (e) => {
      release(e, "up");
    });
    canvas.addEventListener("pointercancel", (e) => {
      release(e, "cancel");
    });
    canvas.addEventListener(
      "wheel",
      (e) => {
        e.preventDefault();
        const rect = canvas.getBoundingClientRect();
        const factor = e.deltaY < 0 ? 1.25 : 0.8;
        this.#setCamera(zoomAt(this.#camera, factor, e.clientX - rect.left, e.clientY - rect.top));
      },
      { passive: false },
    );
  }
}
