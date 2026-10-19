/**
 * Playfield (module "Spielfeld"): top view with PixiJS over WebGL2. Shows
 * the map bounds and the persons of the newest frame (S-1) as particles in
 * one `ParticleContainer` (spike 1.3: sprites are too slow at 50,000
 * objects). Since step 2.10 it loads the texture atlases of the graphics
 * pipeline and draws object sprites and zone overlays that the measurement
 * tool places (ADR-032: the way of the world display is open until 3.8).
 * Display only (rule 002).
 *
 * The grid itself (floors, walls, zones) is not drawn yet.
 * TODO(fahrplan-ref: 3.1): draw the grid with building in the grid.
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
} from "pixi.js";
// Replaces the eval-based particle upload, so that the CSP needs no 'unsafe-eval' (spike 1.3).
import "pixi.js/unsafe-eval";
import type { FrameFilter } from "../src/generated/FrameFilter.js";
import {
  type AtlasIndex,
  bundledIndex,
  chooseTier,
  imageNames,
  lookup,
  textureBytes,
} from "./atlas.js";
import { type Camera, centerCamera, fitCamera, pan, visibleRect, zoomAt } from "./camera.js";
import { type Frame, type PersonRecord, readPerson } from "./frame-reader.js";
import { densestSpot } from "./hotspot.js";
import type { MeasureScene, SceneInfo, View } from "./measure.js";

/** Lowest and highest floor (BAU-07). */
const MIN_FLOOR = -2;
const MAX_FLOOR = 2;
/** Diameter of a person in cells. */
const PERSON_SIZE = 0.8;
/** Smallest diameter of a person on the screen in pixels, so that crowds stay visible in the overview. */
const MIN_PERSON_PIXELS = 2;
/** Colours of the zone overlays (palette fabric_blue, plant, signal_yellow, signal_red) and their opacity. */
const ZONE_COLOURS = [0x3f6fa8, 0x5e9c4f, 0xe8b730, 0xc8433b];
const ZONE_ALPHA = 0.25;
/** Colour of an object drawn without any atlas (palette metal). */
const NO_ATLAS_TINT = 0x8a929e;

/** Loaded atlas: the index and one texture per page of each tier. */
interface Atlas {
  readonly index: AtlasIndex;
  readonly pages: readonly (readonly Texture[])[];
}

async function loadImage(url: string): Promise<Texture> {
  const image = new Image();
  image.src = url;
  await image.decode();
  return Texture.from(image);
}

/** Loads the pages of the bundled atlas; `undefined` without atlas or if a page fails to load. */
async function loadAtlas(): Promise<Atlas | undefined> {
  const index = bundledIndex();
  if (index === undefined) {
    return undefined;
  }
  try {
    const pages = await Promise.all(
      index.tiers.map((tier) => Promise.all(tier.pages.map((p) => loadImage(p.image)))),
    );
    return { index, pages };
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

/** The playfield with camera and floor selection. */
export class Playfield {
  readonly #app: Application;
  readonly #world = new Container();
  readonly #bounds = new Graphics();
  readonly #objects = new Container();
  readonly #zones = new Graphics();
  readonly #objectNames: string[] = [];
  readonly #atlas: Atlas | undefined;
  readonly #textures = new Map<string, { texture: Texture; scale: number }>();
  #tier = -1;
  readonly #persons: ParticleContainer;
  readonly #pool: Particle[] = [];
  readonly #texture: Texture;
  readonly #person: PersonRecord = { x: 0, y: 0, floor: 0, heading: 0 };
  #camera: Camera = { zoom: 1, x: 0, y: 0 };
  #floor = 0;
  #map = { width: 0, height: 0 };
  #shown = 0;

  private constructor(app: Application, atlas: Atlas | undefined) {
    this.#app = app;
    this.#atlas = atlas;
    this.#texture = dotTexture();
    this.#persons = new ParticleContainer({
      texture: this.#texture,
      dynamicProperties: { position: true },
    });
    // Static layer under a moving camera: its own render group keeps the
    // transforms of the sprites when only the camera changes.
    this.#objects.isRenderGroup = true;
    this.#world.addChild(this.#bounds, this.#objects, this.#zones, this.#persons);
    app.stage.addChild(this.#world);
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

  /** Name of the GPU, for the measurement protocol. */
  get gpu(): string {
    // Returns the context PixiJS created on this canvas (same type), not a new one.
    const canvas = this.#app.canvas;
    const gl2 = canvas.getContext("webgl2");
    const gl = gl2 ?? canvas.getContext("webgl");
    if (gl === null) {
      return `renderer ${this.#app.renderer.name}`;
    }
    const ext = gl.getExtension("WEBGL_debug_renderer_info");
    const name = String(gl.getParameter(ext === null ? gl.RENDERER : ext.UNMASKED_RENDERER_WEBGL));
    return `${gl2 === null ? "WebGL1" : "WebGL2"} ${name}`;
  }

  /** `true` if the atlas of the build was loaded. */
  get hasAtlas(): boolean {
    return this.#atlas !== undefined;
  }

  /** Image names of the atlas without the fallback; empty without atlas. */
  imageNames(): readonly string[] {
    return this.#atlas === undefined ? [] : imageNames(this.#atlas.index);
  }

  /** Resolution drawn with now in pixels per cell, 0 without atlas. */
  tierPx(): number {
    return this.#atlas?.index.tiers[this.#tier]?.pxPerCell ?? 0;
  }

  /** Draws the objects and zones of `scene` instead of the previous ones (step 2.10). */
  scene(scene: MeasureScene): SceneInfo {
    for (const child of this.#objects.removeChildren()) {
      child.destroy();
    }
    this.#objectNames.length = 0;
    let fallbacks = 0;
    for (const object of scene.objects) {
      const sprite = new Sprite();
      sprite.position.set(object.x, object.y);
      this.#objects.addChild(sprite);
      this.#objectNames.push(object.name);
      if (
        this.#atlas === undefined ||
        lookup(this.#atlas.index, 0, object.name)?.fallback !== false
      ) {
        fallbacks += 1;
      }
    }
    this.#tier = -1;
    this.#applyTier();
    this.#zones.clear();
    for (const zone of scene.zones) {
      this.#zones.rect(zone.x, zone.y, zone.w, zone.h).fill({
        color: ZONE_COLOURS[zone.colour % ZONE_COLOURS.length] ?? NO_ATLAS_TINT,
        alpha: ZONE_ALPHA,
      });
    }
    const index = this.#atlas?.index;
    return {
      objects: scene.objects.length,
      fallbacks,
      zones: scene.zones.length,
      tiers: index?.tiers.map((t) => t.pxPerCell) ?? [],
      pages: index?.tiers.reduce((sum, t) => sum + t.pages.length, 0) ?? 0,
      textureMbMax:
        index === undefined
          ? 0
          : Math.round((textureBytes(index, index.tiers.length - 1) / 1048576) * 100) / 100,
    };
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

  /** Draws the persons of `frame`. */
  show(frame: Frame): void {
    const { mapWidth, mapHeight, persons } = frame.header;
    if (mapWidth !== this.#map.width || mapHeight !== this.#map.height) {
      this.#map = { width: mapWidth, height: mapHeight };
      this.#bounds.clear().rect(0, 0, mapWidth, mapHeight).fill(0x1d232b);
      const { width, height } = this.#app.screen;
      this.#setCamera(fitCamera(mapWidth, mapHeight, width, height));
    }
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
  }

  /** Scale of the particle texture for the current zoom. */
  #personScale(): number {
    const cells = Math.max(PERSON_SIZE, MIN_PERSON_PIXELS / this.#camera.zoom);
    return cells / this.#texture.width;
  }

  /** Gives every object sprite the texture of the tier for the current zoom, if the tier changed. */
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
    const sprites = this.#objects.children as Sprite[];
    for (let i = 0; i < sprites.length; i += 1) {
      const sprite = sprites[i];
      if (sprite !== undefined) {
        const { texture, scale } = this.#objectTexture(this.#objectNames[i] ?? "");
        sprite.texture = texture;
        sprite.tint = atlas === undefined ? NO_ATLAS_TINT : 0xffffff;
        sprite.scale.set(scale);
      }
    }
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
    canvas.addEventListener("pointerdown", (e) => {
      drag = { x: e.clientX, y: e.clientY };
      canvas.setPointerCapture(e.pointerId);
    });
    canvas.addEventListener("pointermove", (e) => {
      if (drag !== undefined) {
        this.#setCamera(pan(this.#camera, e.clientX - drag.x, e.clientY - drag.y));
        drag = { x: e.clientX, y: e.clientY };
      }
    });
    canvas.addEventListener("pointerup", () => {
      drag = undefined;
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
