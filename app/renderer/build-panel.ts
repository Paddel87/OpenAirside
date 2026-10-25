/**
 * Panel of the build tool (module "Oberfläche", step 3.1b): buttons for new
 * game, the tool modes (one button per buildable entry of the catalog,
 * demolition, off) and the hint with the reason when the core refused a
 * build action. Replaced by the build menu of step 5.1; plain DOM like the
 * other panels until the React interface of step 3.6.
 */
import type { CatalogEntry } from "../src/generated/CatalogEntry.js";
import type { BuildTool } from "./build-tool.js";
import type { Localizer } from "./i18n.js";

/** Map sizes offered for a new game (S-6 `game.new`). */
export const NEW_GAME_MAPS = [1536, 2048] as const;

/** Entries the player can build: not replaced by another entry (the list keeps replaced ones for the indices). */
export function buildable(catalog: readonly CatalogEntry[]): CatalogEntry[] {
  return catalog.filter((entry) => entry.replacedBy === undefined);
}

/** The panel in `container`. */
export class BuildPanel {
  #catalog: readonly CatalogEntry[] = [];
  #hint = "";
  #hintTimer: ReturnType<typeof setTimeout> | undefined;

  constructor(
    private readonly doc: Document,
    private readonly container: HTMLElement,
    private readonly tool: BuildTool,
    private readonly i18n: Localizer,
    private readonly newGame: (map: (typeof NEW_GAME_MAPS)[number]) => void,
  ) {
    i18n.onChange(() => {
      this.render();
    });
    this.render();
  }

  /** The catalog of the session. */
  setCatalog(catalog: readonly CatalogEntry[]): void {
    this.#catalog = catalog;
    this.render();
  }

  /** Shows the reason of a refused action for a few seconds. */
  showHint(text: string): void {
    this.#hint = text;
    if (this.#hintTimer !== undefined) clearTimeout(this.#hintTimer);
    this.#hintTimer = setTimeout(() => {
      this.#hint = "";
      this.render();
    }, 6000);
    this.render();
  }

  /** Draws the panel again (mode or language changed). */
  render(): void {
    const { doc, i18n, tool } = this;
    const t = i18n.t.bind(i18n);
    const button = (label: string, active: boolean, run: () => void): HTMLButtonElement => {
      const element = doc.createElement("button");
      element.type = "button";
      element.textContent = label;
      element.classList.toggle("active", active);
      element.addEventListener("click", run);
      return element;
    };
    const row = doc.createElement("div");
    row.className = "tools";
    for (const map of NEW_GAME_MAPS) {
      row.append(
        button(t("ui.build.newGame", { map: String(map) }), false, () => {
          this.newGame(map);
        }),
      );
    }
    const mode = tool.mode;
    row.append(
      button(t("ui.build.off"), mode.kind === "off", () => {
        tool.setMode({ kind: "off" });
        this.render();
      }),
    );
    for (const entry of buildable(this.#catalog)) {
      const active = mode.kind === "build" && mode.entry.id === entry.id;
      row.append(
        button(t(entry.name), active, () => {
          tool.setMode({ kind: "build", entry });
          this.render();
        }),
      );
    }
    row.append(
      button(t("ui.build.demolish"), mode.kind === "demolish", () => {
        tool.setMode({ kind: "demolish" });
        this.render();
      }),
    );
    const note = doc.createElement("div");
    note.className = "note";
    note.textContent = this.#hint === "" ? t("ui.build.keys") : this.#hint;
    note.classList.toggle("hint", this.#hint !== "");
    this.container.replaceChildren(row, note);
    this.container.hidden = false;
  }
}
