import { describe, expect, it } from "vitest";
import { GpuName, type GpuNameContext, type GpuNameSource } from "./gpu-name.js";

const RENDERER = 0x1f01;
const UNMASKED_RENDERER_WEBGL = 0x9246;

/** Fake context that counts its synchronous queries. */
function context(withDebugInfo: boolean): GpuNameContext & { queries: number } {
  const gl = {
    RENDERER,
    queries: 0,
    getExtension(): { readonly UNMASKED_RENDERER_WEBGL: number } | null {
      gl.queries += 1;
      return withDebugInfo ? { UNMASKED_RENDERER_WEBGL } : null;
    },
    getParameter(name: number): unknown {
      gl.queries += 1;
      return name === UNMASKED_RENDERER_WEBGL ? "ANGLE (AMD Radeon)" : "WebKit WebGL";
    },
  };
  return gl;
}

function source(
  gl2: GpuNameContext | null,
  gl1: GpuNameContext | null,
): GpuNameSource & { lookups: number } {
  const s = {
    rendererName: "webgl",
    lookups: 0,
    webgl2: () => {
      s.lookups += 1;
      return gl2;
    },
    webgl: () => {
      s.lookups += 1;
      return gl1;
    },
  };
  return s;
}

describe("GpuName", () => {
  it("does not query the context again on a second access", () => {
    const gl = context(true);
    const src = source(gl, null);
    const name = new GpuName(src);
    const queries = gl.queries;
    const lookups = src.lookups;
    expect(name.value).toBe("WebGL2 ANGLE (AMD Radeon)");
    expect(name.value).toBe("WebGL2 ANGLE (AMD Radeon)");
    expect(gl.queries).toBe(queries);
    expect(src.lookups).toBe(lookups);
    // One getExtension and one getParameter, both at creation.
    expect(queries).toBe(2);
    expect(lookups).toBe(1);
  });

  it("falls back to the masked name without the debug extension", () => {
    expect(new GpuName(source(context(false), null)).value).toBe("WebGL2 WebKit WebGL");
  });

  it("names WebGL1 if there is no WebGL2 context", () => {
    expect(new GpuName(source(null, context(true))).value).toBe("WebGL1 ANGLE (AMD Radeon)");
  });

  it("names the PixiJS renderer without any WebGL context", () => {
    expect(new GpuName(source(null, null)).value).toBe("renderer webgl");
  });
});
