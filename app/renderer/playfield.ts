/**
 * Playfield (module "Spielfeld"): top view with PixiJS over WebGL2. Shows
 * the map bounds and the persons of the newest frame (S-1) as particles in
 * one `ParticleContainer` (spike 1.3: sprites are too slow at 50,000
 * objects). Display only (rule 002).
 *
 * The grid itself (floors, walls, zones) is not drawn yet.
 * TODO(fahrplan-ref: 3.1): draw the grid with building in the grid.
 */
import { Application, Container, Graphics, Particle, ParticleContainer, Texture } from "pixi.js";
// Replaces the eval-based particle upload, so that the CSP needs no 'unsafe-eval' (spike 1.3).
import "pixi.js/unsafe-eval";
import type { FrameFilter } from "../src/generated/FrameFilter.js";
import { type Camera, fitCamera, pan, visibleRect, zoomAt } from "./camera.js";
import { type Frame, type PersonRecord, readPerson } from "./frame-reader.js";

/** Lowest and highest floor (BAU-07). */
const MIN_FLOOR = -2;
const MAX_FLOOR = 2;
/** Diameter of a person in cells. */
const PERSON_SIZE = 0.8;
/** Smallest diameter of a person on the screen in pixels, so that crowds stay visible in the overview. */
const MIN_PERSON_PIXELS = 2;

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
  readonly #persons: ParticleContainer;
  readonly #pool: Particle[] = [];
  readonly #texture: Texture;
  readonly #person: PersonRecord = { x: 0, y: 0, floor: 0, heading: 0 };
  #camera: Camera = { zoom: 1, x: 0, y: 0 };
  #floor = 0;
  #map = { width: 0, height: 0 };
  #shown = 0;

  private constructor(app: Application) {
    this.#app = app;
    this.#texture = dotTexture();
    this.#persons = new ParticleContainer({
      texture: this.#texture,
      dynamicProperties: { position: true },
    });
    this.#world.addChild(this.#bounds, this.#persons);
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
    const field = new Playfield(app);
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

  #setCamera(camera: Camera): void {
    const zoomed = camera.zoom !== this.#camera.zoom;
    this.#camera = camera;
    this.#world.scale.set(camera.zoom);
    this.#world.position.set(-camera.x * camera.zoom, -camera.y * camera.zoom);
    if (zoomed) {
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
