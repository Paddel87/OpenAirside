/**
 * Build tool (module "Oberfläche", step 3.1b): turns pointer gestures on the
 * playfield into the build commands `build`, `buildArea` and `demolish`
 * (S-2) and keeps their provisional drawing (S-2 "Vorläufiges Zeichnen",
 * NFR "Bauaktion im nächsten Bild"): the drawing appears with the gesture,
 * is marked with the command id, and goes away when the core refuses the
 * command (with the reason) or when a frame at or after the tick of its
 * acceptance was drawn. The tool checks nothing about the world (rule 002):
 * whether a building is allowed decides the core.
 */
import type { CatalogEntry } from "../src/generated/CatalogEntry.js";
import type { CellRect } from "../src/generated/CellRect.js";
import type { Command } from "../src/generated/Command.js";
import type { CellPointer, PreviewShape } from "./playfield.js";

/** Floor the tool builds on until floors can be built (step 5.2). */
export const BUILD_FLOOR = 0;
/** Largest area of one command in cells per edge (S-2 `areaTooLarge`); the drag stops growing there. */
export const MAX_AREA_EDGE = 512;

/** What the tool does with a gesture. */
export type ToolMode =
  | { readonly kind: "off" }
  | { readonly kind: "build"; readonly entry: CatalogEntry }
  | { readonly kind: "demolish" };

/** What the tool needs from the renderer. */
export interface ToolHost {
  /** Sends a command; returns its id, 0 if not connected. */
  send(command: Command): number;
  /** Draws the shapes of the provisional drawing, an empty list clears it. */
  preview(shapes: readonly PreviewShape[]): void;
  /** The mode changed. */
  modeChanged(mode: ToolMode): void;
}

interface Provisional {
  readonly shape: PreviewShape;
  /** Tick of the acceptance, once the core accepted the command. */
  acceptedTick?: number;
}

/** Footprint of a single object in cells at `rotation` (rotations 1 and 3 swap width and depth). */
export function footprint(entry: CatalogEntry, rotation: number): { w: number; d: number } {
  if (entry.placement.kind !== "single") return { w: 1, d: 1 };
  const w = entry.placement.width / 2;
  const d = entry.placement.depth / 2;
  return rotation % 2 === 1 ? { w: d, d: w } : { w, d };
}

/** The rectangle between two cells of a drag, both included, limited to {@link MAX_AREA_EDGE} cells per edge. */
export function dragRect(
  from: { readonly x: number; readonly y: number },
  to: { readonly x: number; readonly y: number },
): CellRect {
  const x0 = Math.min(from.x, to.x);
  const y0 = Math.min(from.y, to.y);
  return {
    floor: BUILD_FLOOR,
    x0,
    y0,
    x1: Math.min(Math.max(from.x, to.x) + 1, x0 + MAX_AREA_EDGE),
    y1: Math.min(Math.max(from.y, to.y) + 1, y0 + MAX_AREA_EDGE),
  };
}

/** The build tool. */
export class BuildTool {
  readonly #host: ToolHost;
  #mode: ToolMode = { kind: "off" };
  #rotation = 0;
  #start: { x: number; y: number } | undefined;
  #hover: { x: number; y: number } | undefined;
  readonly #sent = new Map<number, Provisional>();

  constructor(host: ToolHost) {
    this.#host = host;
  }

  /** Current mode. */
  get mode(): ToolMode {
    return this.#mode;
  }

  /** Rotation of the object to place, 0 … 3. */
  get rotation(): number {
    return this.#rotation;
  }

  /** Commands whose provisional drawing is still shown. */
  get pending(): number {
    return this.#sent.size;
  }

  /** Chooses what the tool does; `off` hands the pointer back to the camera. */
  setMode(mode: ToolMode): void {
    this.#mode = mode;
    this.#start = undefined;
    this.#hover = undefined;
    this.#host.modeChanged(mode);
    this.#draw();
  }

  /** Turns the object to place by 90° (key R). */
  rotate(): void {
    this.#rotation = (this.#rotation + 1) % 4;
    this.#draw();
  }

  /** A new session began: nothing of the old one stays on the screen (S-2). */
  reset(): void {
    this.#sent.clear();
    this.#start = undefined;
    this.#hover = undefined;
    this.#draw();
  }

  /** Handles a pointer event; `true` if the tool takes the gesture. */
  pointer(event: CellPointer): boolean {
    const mode = this.#mode;
    if (mode.kind === "off") return false;
    const cell = { x: event.x, y: event.y };
    switch (event.phase) {
      case "move":
        this.#hover = cell;
        this.#draw();
        return this.#start !== undefined;
      case "down":
        // The primary button builds; the others pan the camera.
        if (event.button !== 0) return false;
        this.#start = cell;
        this.#hover = cell;
        this.#draw();
        return true;
      case "cancel":
        this.#start = undefined;
        this.#draw();
        return true;
      case "up": {
        const start = this.#start;
        this.#start = undefined;
        if (start === undefined) return false;
        this.#issue(mode, start, cell);
        this.#draw();
        return true;
      }
    }
  }

  /** The core accepted command `id` at `tick`. */
  accepted(id: number, tick: number): void {
    const pending = this.#sent.get(id);
    if (pending !== undefined) pending.acceptedTick = tick;
  }

  /** The core refused command `id`: its drawing goes away. */
  rejected(id: number): void {
    if (this.#sent.delete(id)) this.#draw();
  }

  /** A frame of `tick` was drawn: drawings accepted up to this tick are in the world now. */
  frameDrawn(tick: number): void {
    let changed = false;
    for (const [id, pending] of this.#sent) {
      if (pending.acceptedTick !== undefined && tick >= pending.acceptedTick) {
        this.#sent.delete(id);
        changed = true;
      }
    }
    if (changed) this.#draw();
  }

  #issue(
    mode: Exclude<ToolMode, { kind: "off" }>,
    start: { x: number; y: number },
    end: { x: number; y: number },
  ): void {
    if (mode.kind === "demolish") {
      const area = dragRect(start, end);
      this.#send({ kind: "demolish", area }, { kind: "demolish", ...bounds(area) });
      return;
    }
    const { entry } = mode;
    if (entry.placement.kind === "area") {
      const area = dragRect(start, end);
      this.#send({ kind: "buildArea", object: entry.id, area }, { kind: "build", ...bounds(area) });
      return;
    }
    const at = { x: end.x, y: end.y, floor: BUILD_FLOOR };
    const { w, d } = footprint(entry, this.#rotation);
    this.#send(
      { kind: "build", object: entry.id, at, rotation: this.#rotation as 0 | 1 | 2 | 3 },
      {
        kind: "build",
        x0: end.x,
        y0: end.y,
        x1: end.x + w,
        y1: end.y + d,
        object: { name: entry.id, rotation: this.#rotation },
      },
    );
  }

  #send(command: Command, shape: PreviewShape): void {
    const id = this.#host.send(command);
    if (id !== 0) this.#sent.set(id, { shape });
  }

  /** Shapes of the gesture in progress or the hovering pointer. */
  #gesture(): PreviewShape | undefined {
    const mode = this.#mode;
    const hover = this.#hover;
    if (mode.kind === "off" || hover === undefined) return undefined;
    const start = this.#start;
    if (mode.kind === "demolish") {
      return { kind: "demolish", ...bounds(dragRect(start ?? hover, hover)) };
    }
    if (mode.entry.placement.kind === "area") {
      return { kind: "build", ...bounds(dragRect(start ?? hover, hover)) };
    }
    const { w, d } = footprint(mode.entry, this.#rotation);
    return {
      kind: "build",
      x0: hover.x,
      y0: hover.y,
      x1: hover.x + w,
      y1: hover.y + d,
      object: { name: mode.entry.id, rotation: this.#rotation },
    };
  }

  #draw(): void {
    const shapes = [...this.#sent.values()].map((p) => p.shape);
    const gesture = this.#gesture();
    if (gesture !== undefined) shapes.push(gesture);
    this.#host.preview(shapes);
  }
}

function bounds(area: CellRect): Pick<PreviewShape, "x0" | "y0" | "x1" | "y1"> {
  return { x0: area.x0, y0: area.y0, x1: area.x1, y1: area.y1 };
}
