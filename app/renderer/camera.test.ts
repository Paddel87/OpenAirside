import { describe, expect, it } from "vitest";
import { fitCamera, MAX_ZOOM, MIN_ZOOM, pan, visibleRect, zoomAt } from "./camera.js";

describe("camera", () => {
  it("fits the map and centres it", () => {
    const camera = fitCamera(2048, 2048, 1024, 512);
    expect(camera.zoom).toBe(0.25);
    expect(visibleRect(camera, 1024, 512)).toEqual({ x0: -1024, y0: 0, x1: 3072, y1: 2048 });
  });

  it("keeps the cell under the pointer while zooming", () => {
    const camera = { zoom: 4, x: 100, y: 50 };
    const zoomed = zoomAt(camera, 2, 40, 80);
    expect(zoomed.zoom).toBe(8);
    expect(zoomed.x + 40 / zoomed.zoom).toBeCloseTo(camera.x + 40 / camera.zoom);
    expect(zoomed.y + 80 / zoomed.zoom).toBeCloseTo(camera.y + 80 / camera.zoom);
  });

  it("clamps the zoom", () => {
    expect(zoomAt({ zoom: 30, x: 0, y: 0 }, 4, 0, 0).zoom).toBe(MAX_ZOOM);
    expect(zoomAt({ zoom: 0.3, x: 0, y: 0 }, 0.1, 0, 0).zoom).toBe(MIN_ZOOM);
  });

  it("pans opposite to the drag", () => {
    expect(pan({ zoom: 2, x: 10, y: 10 }, 20, -4)).toEqual({ zoom: 2, x: 0, y: 12 });
  });
});
