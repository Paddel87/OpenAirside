/**
 * Camera of the playfield: zoom in pixels per cell and the cell at the top
 * left corner. Pure arithmetic, so that the visible rectangle sent with
 * frame requests (S-1 filter) is testable without WebGL.
 */
import type { ViewRect } from "../src/generated/ViewRect.js";

/** Smallest and largest zoom in pixels per cell. */
export const MIN_ZOOM = 0.25;
export const MAX_ZOOM = 32;

/** Camera state. */
export interface Camera {
  /** Pixels per cell. */
  zoom: number;
  /** Cell at the left edge of the screen. */
  x: number;
  /** Cell at the top edge of the screen. */
  y: number;
}

/** Camera that shows the whole map of `width` × `height` cells on the screen. */
export function fitCamera(width: number, height: number, screenW: number, screenH: number): Camera {
  const zoom = clampZoom(Math.min(screenW / width, screenH / height));
  return {
    zoom,
    x: (width - screenW / zoom) / 2,
    y: (height - screenH / zoom) / 2,
  };
}

/** Camera with `zoom` that shows the cell (`x`, `y`) in the middle of the screen. */
export function centerCamera(
  zoom: number,
  x: number,
  y: number,
  screenW: number,
  screenH: number,
): Camera {
  const z = clampZoom(zoom);
  return { zoom: z, x: x - screenW / z / 2, y: y - screenH / z / 2 };
}

function clampZoom(zoom: number): number {
  return Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom));
}

/** Zooms by `factor` around the screen point (`px`, `py`), keeping that cell under it. */
export function zoomAt(camera: Camera, factor: number, px: number, py: number): Camera {
  const zoom = clampZoom(camera.zoom * factor);
  const cellX = camera.x + px / camera.zoom;
  const cellY = camera.y + py / camera.zoom;
  return { zoom, x: cellX - px / zoom, y: cellY - py / zoom };
}

/** Moves the camera by a drag of (`dx`, `dy`) pixels. */
export function pan(camera: Camera, dx: number, dy: number): Camera {
  return { zoom: camera.zoom, x: camera.x - dx / camera.zoom, y: camera.y - dy / camera.zoom };
}

/** Visible rectangle in cells for a screen of `screenW` × `screenH` pixels. */
export function visibleRect(camera: Camera, screenW: number, screenH: number): ViewRect {
  return {
    x0: camera.x,
    y0: camera.y,
    x1: camera.x + screenW / camera.zoom,
    y1: camera.y + screenH / camera.zoom,
  };
}
