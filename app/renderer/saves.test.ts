import { describe, expect, it } from "vitest";
import type { SaveMeta } from "../src/generated/SaveMeta.js";
import type { SaveEntry } from "./platform.js";
import { formatEntry, formatSaves, latestAutosave, saveLabel, savedAtText } from "./saves.js";

const meta = (kind: SaveMeta["kind"], savedAt: string, name = ""): SaveMeta => ({
  name,
  kind,
  savedAt,
  tick: 300,
  day: 2,
  year: 1,
  airportName: "",
  map: { width: 2048, height: 2048 },
  packs: [],
});

const ok = (id: string, m: SaveMeta): SaveEntry => ({
  id,
  state: "ok",
  gameVersion: "0.0.0",
  meta: m,
});
const damaged: SaveEntry = {
  id: "auto-3",
  state: "damaged",
  error: { code: "truncated", textKey: "save.error.truncated", params: {} },
};

describe("save game list in the renderer", () => {
  it("labels saves by name or kind", () => {
    expect(saveLabel(meta("manual", "", "Hub"))).toBe("Hub");
    expect(saveLabel(meta("manual", ""))).toBe("(unnamed)");
    expect(saveLabel(meta("quick", ""))).toBe("quick save");
    expect(saveLabel(meta("auto", ""))).toBe("autosave");
  });

  it("shortens the saving time", () => {
    expect(savedAtText("2026-10-19T11:52:31.123Z")).toBe("2026-10-19 11:52");
    expect(savedAtText("later")).toBe("later");
  });

  it("formats console lines", () => {
    expect(formatSaves([])).toEqual(["saves: none"]);
    expect(formatEntry(ok("quick", meta("quick", "2026-10-19T11:52:31Z")))).toBe(
      'quick quick "quick save" – 2026-10-19 11:52, year 1 day 2, tick 300',
    );
    expect(formatSaves([damaged])).toEqual(["saves: 1", "  auto-3 damaged: truncated"]);
  });

  it("offers the newest readable autosave", () => {
    expect(latestAutosave([])).toBeUndefined();
    expect(
      latestAutosave([
        ok("auto-1", meta("auto", "2026-10-19T10:00:00Z")),
        damaged,
        ok("quick", meta("quick", "2026-10-19T12:00:00Z")),
        ok("auto-2", meta("auto", "2026-10-19T11:00:00Z")),
      ]),
    ).toBe("auto-2");
  });
});
