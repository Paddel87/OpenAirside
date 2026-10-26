import { describe, expect, it } from "vitest";
import type { CatalogEntry } from "../src/generated/CatalogEntry.js";
import type { Command } from "../src/generated/Command.js";
import { buildable } from "./build-panel.js";
import { BuildTool, dragRect, footprint, MAX_AREA_EDGE, type ToolMode } from "./build-tool.js";
import type { CellPointer, PreviewShape } from "./playfield.js";

const floor: CatalogEntry = {
  kind: "object",
  id: "base:floor_concrete",
  name: "object.floor_concrete",
  placement: { kind: "area" },
};
const bench: CatalogEntry = {
  kind: "object",
  id: "base:bench",
  name: "object.bench",
  placement: { kind: "single", width: 4, depth: 2 },
};

function setup(accept = true) {
  const sent: Command[] = [];
  const drawn: (readonly PreviewShape[])[] = [];
  const modes: ToolMode[] = [];
  let nextId = 1;
  const tool = new BuildTool({
    send: (command) => {
      sent.push(command);
      if (!accept) return 0;
      const id = nextId;
      nextId += 1;
      return id;
    },
    preview: (shapes) => {
      drawn.push(shapes);
    },
    modeChanged: (mode) => {
      modes.push(mode);
    },
  });
  const pointer = (phase: CellPointer["phase"], x: number, y: number, button = 0): boolean =>
    tool.pointer({ phase, x, y, button: phase === "move" ? -1 : button });
  return { tool, sent, drawn, modes, pointer };
}

describe("footprint and dragRect", () => {
  it("turns width and depth with the odd rotations and counts 2 m per cell", () => {
    expect(footprint(bench, 0)).toEqual({ w: 2, d: 1 });
    expect(footprint(bench, 1)).toEqual({ w: 1, d: 2 });
    expect(footprint(bench, 2)).toEqual({ w: 2, d: 1 });
    expect(footprint(bench, 3)).toEqual({ w: 1, d: 2 });
    expect(footprint(floor, 1)).toEqual({ w: 1, d: 1 });
  });

  it("includes both cells of a drag in any direction and limits the area to the command size", () => {
    expect(dragRect({ x: 5, y: 7 }, { x: 5, y: 7 })).toEqual({
      floor: 0,
      x0: 5,
      y0: 7,
      x1: 6,
      y1: 8,
    });
    expect(dragRect({ x: 9, y: 2 }, { x: 4, y: 6 })).toMatchObject({ x0: 4, y0: 2, x1: 10, y1: 7 });
    const huge = dragRect({ x: 0, y: 0 }, { x: 5000, y: 600 });
    expect(huge.x1 - huge.x0).toBe(MAX_AREA_EDGE);
    expect(huge.y1 - huge.y0).toBe(MAX_AREA_EDGE);
  });
});

describe("BuildTool", () => {
  it("leaves the pointer to the camera while it is off", () => {
    const { pointer, sent } = setup();
    expect(pointer("down", 1, 1)).toBe(false);
    expect(pointer("move", 1, 1)).toBe(false);
    expect(sent).toEqual([]);
  });

  it("builds an area with a drag and shows its provisional drawing until the frame of its tick", () => {
    const { tool, sent, drawn, pointer } = setup();
    tool.setMode({ kind: "build", entry: floor });
    expect(pointer("down", 10, 20)).toBe(true);
    expect(pointer("move", 14, 22)).toBe(true);
    // The drag shows where the area will be.
    expect(drawn.at(-1)).toEqual([{ kind: "build", x0: 10, y0: 20, x1: 15, y1: 23 }]);
    expect(pointer("up", 14, 22)).toBe(true);
    expect(sent).toEqual([
      {
        kind: "buildArea",
        object: "base:floor_concrete",
        area: { floor: 0, x0: 10, y0: 20, x1: 15, y1: 23 },
      },
    ]);
    // The hovering pointer's shape comes after the provisional one of the command.
    expect(tool.pending).toBe(1);
    expect(drawn.at(-1)?.[0]).toEqual({ kind: "build", x0: 10, y0: 20, x1: 15, y1: 23 });
    // Accepted for tick 5: it stays until a frame of tick 5 or later was drawn.
    tool.accepted(1, 5);
    tool.frameDrawn(4);
    expect(tool.pending).toBe(1);
    tool.frameDrawn(5);
    expect(tool.pending).toBe(0);
  });

  it("removes the drawing of a refused command at once", () => {
    const { tool, drawn, pointer } = setup();
    tool.setMode({ kind: "build", entry: floor });
    pointer("down", 1, 1);
    pointer("up", 3, 3);
    expect(tool.pending).toBe(1);
    tool.rejected(1);
    expect(tool.pending).toBe(0);
    // Only the hovering shape of the pointer is left.
    expect(drawn.at(-1)?.every((s) => s.x0 === 3 && s.y0 === 3)).toBe(true);
    tool.rejected(99);
    expect(tool.pending).toBe(0);
  });

  it("places an object at the cell under the pointer with the rotation, turned by R", () => {
    const { tool, sent, drawn, pointer } = setup();
    tool.setMode({ kind: "build", entry: bench });
    pointer("move", 30, 40);
    expect(drawn.at(-1)).toEqual([
      {
        kind: "build",
        x0: 30,
        y0: 40,
        x1: 32,
        y1: 41,
        object: { name: "base:bench", rotation: 0 },
      },
    ]);
    tool.rotate();
    expect(tool.rotation).toBe(1);
    pointer("down", 30, 40);
    pointer("up", 30, 40);
    expect(sent).toEqual([
      { kind: "build", object: "base:bench", at: { x: 30, y: 40, floor: 0 }, rotation: 1 },
    ]);
    // The drawing of the object has the turned footprint.
    expect(drawn.at(-1)?.[0]).toMatchObject({ x1: 31, y1: 42, object: { rotation: 1 } });
    for (let i = 0; i < 3; i += 1) tool.rotate();
    expect(tool.rotation).toBe(0);
  });

  it("demolishes an area with a drag", () => {
    const { tool, sent, drawn, pointer } = setup();
    tool.setMode({ kind: "demolish" });
    pointer("down", 8, 8);
    expect(drawn.at(-1)?.[0]?.kind).toBe("demolish");
    pointer("up", 6, 9);
    expect(sent).toEqual([{ kind: "demolish", area: { floor: 0, x0: 6, y0: 8, x1: 9, y1: 10 } }]);
  });

  it("lets the other buttons pan, cancels a drag and forgets everything with a new session", () => {
    const { tool, sent, pointer } = setup();
    tool.setMode({ kind: "demolish" });
    expect(pointer("down", 1, 1, 2)).toBe(false);
    expect(pointer("down", 1, 1, 0)).toBe(true);
    expect(pointer("cancel", 1, 1)).toBe(true);
    expect(pointer("up", 5, 5)).toBe(false);
    expect(sent).toEqual([]);
    pointer("down", 1, 1);
    pointer("up", 2, 2);
    expect(tool.pending).toBe(1);
    tool.reset();
    expect(tool.pending).toBe(0);
  });

  it("keeps no drawing for a command that was not sent", () => {
    const { tool, pointer } = setup(false);
    tool.setMode({ kind: "demolish" });
    pointer("down", 1, 1);
    pointer("up", 2, 2);
    expect(tool.pending).toBe(0);
  });

  it("announces mode changes", () => {
    const { tool, modes } = setup();
    tool.setMode({ kind: "demolish" });
    tool.setMode({ kind: "off" });
    expect(modes.map((m) => m.kind)).toEqual(["demolish", "off"]);
  });
});

describe("buildable", () => {
  it("leaves out the entries another entry replaced", () => {
    const replaced: CatalogEntry = { ...floor, id: "base:old", replacedBy: "mod:new" };
    expect(buildable([replaced, floor, bench]).map((e) => e.id)).toEqual([
      "base:floor_concrete",
      "base:bench",
    ]);
  });
});
