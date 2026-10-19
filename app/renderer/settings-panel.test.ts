import { describe, expect, it } from "vitest";
import { Localizer } from "./i18n.js";
import { refusalText, settingsPatch } from "./settings-panel.js";

describe("settings panel", () => {
  it("turns the form into an update within the bounds", () => {
    expect(settingsPatch({ language: "de", minutes: " 15 ", count: "20" })).toEqual({
      language: "de",
      autosaveMinutes: 15,
      autosaveCount: 20,
    });
    for (const [minutes, count] of [
      ["0", "5"],
      ["61", "5"],
      ["10", "21"],
      ["1.5", "5"],
      ["", "5"],
      ["10", "-1"],
      ["1e1", "5"],
    ]) {
      expect(
        settingsPatch({ language: "de", minutes: minutes ?? "", count: count ?? "" }),
      ).toBeUndefined();
    }
  });

  it("shows a refusal of the platform in the chosen language", () => {
    const i18n = new Localizer("de");
    expect(
      refusalText(
        { code: "invalidArgument", textKey: "platform.error.invalidArgument", params: {} },
        i18n,
      ),
    ).toBe("Die Anfrage enthält einen ungültigen Wert.");
    expect(refusalText(new Error("x"), i18n)).toBe("Die Einstellungen ließen sich nicht ändern.");
  });
});
