import { describe, expect, it } from "vitest";
import { densestSpot } from "./hotspot.js";

describe("densestSpot", () => {
  it("finds the block with the most persons", () => {
    const points = [
      { x: 1, y: 1 },
      { x: 40, y: 70 },
      { x: 41, y: 71 },
      { x: 47, y: 79 },
      { x: -5, y: 3 },
      { x: 9999, y: 3 },
    ];
    expect(densestSpot(points, 128, 128)).toEqual({ x: 40, y: 72 });
  });

  it("falls back to the centre of the map", () => {
    expect(densestSpot([], 2048, 1536)).toEqual({ x: 1024, y: 768 });
  });
});
