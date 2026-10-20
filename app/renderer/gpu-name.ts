/**
 * Name of the GPU for the performance display and the measurement protocol
 * (module "Spielfeld", step 3.15). Display only (rule 002).
 *
 * Rule: synchronous WebGL queries (`getParameter`, `getExtension`, `getError`,
 * `readPixels` and the like) run only at start, never while the game runs.
 * Each one is a round trip to the GPU process that blocks the main thread
 * until the GPU process answers; under ANGLE/D3D11 that cost one or two
 * display ticks whenever the GPU process held a flush (step 3.14,
 * docs/research/3.14-spaete-bilder.md).
 */

/** The part of a WebGL context that the name needs. */
export interface GpuNameContext {
  readonly RENDERER: number;
  getExtension(
    name: "WEBGL_debug_renderer_info",
  ): { readonly UNMASKED_RENDERER_WEBGL: number } | null;
  getParameter(name: number): unknown;
}

/** Access to the context of the canvas, WebGL2 first. */
export interface GpuNameSource {
  webgl2(): GpuNameContext | null;
  webgl(): GpuNameContext | null;
  /** Name of the PixiJS renderer, used if the canvas has no WebGL context. */
  readonly rendererName: string;
}

/** Reads the name once when created; later accesses do not touch the context. */
export class GpuName {
  readonly #value: string;

  constructor(source: GpuNameSource) {
    this.#value = readGpuName(source);
  }

  /** For example "WebGL2 ANGLE (AMD, ...)". */
  get value(): string {
    return this.#value;
  }
}

function readGpuName(source: GpuNameSource): string {
  // On the canvas of PixiJS this returns the context PixiJS created (same type), not a new one.
  const gl2 = source.webgl2();
  const gl = gl2 ?? source.webgl();
  if (gl === null) {
    return `renderer ${source.rendererName}`;
  }
  const ext = gl.getExtension("WEBGL_debug_renderer_info");
  const name = String(gl.getParameter(ext === null ? gl.RENDERER : ext.UNMASKED_RENDERER_WEBGL));
  return `${gl2 === null ? "WebGL1" : "WebGL2"} ${name}`;
}
