/**
 * Simple save game list with previews and the notice after a crash of the
 * core (step 2.5, point 8 of the review of S-5/S-6): a minimal panel until
 * the game menu of step 5.1. Previews come from `saves.preview` as `blob:`
 * URLs (CSP `img-src blob:`, S-5). Texts come from the localizer (step
 * 2.6) and are set as `textContent` only; console lines stay English.
 */
import type { SaveMeta } from "../src/generated/SaveMeta.js";
import type { Localizer } from "./i18n.js";
import type { OpenAirsideApi, SaveEntry } from "./platform.js";
import { describeError } from "./platform.js";
import { savedAtText } from "./saves.js";

/** Label of a save game for the player: its name, or its kind for unnamed ones. */
export function localSaveLabel(meta: SaveMeta, i18n: Localizer): string {
  if (meta.name !== "") return meta.name;
  if (meta.kind === "quick") return i18n.t("ui.saves.quick");
  if (meta.kind === "auto") return i18n.t("ui.saves.auto");
  return i18n.t("ui.saves.unnamed");
}

/** A button of a panel or notice. */
interface Action {
  readonly label: string;
  readonly run: () => void;
}

function button(doc: Document, action: Action): HTMLButtonElement {
  const element = doc.createElement("button");
  element.type = "button";
  element.textContent = action.label;
  element.addEventListener("click", action.run);
  return element;
}

/** The save game list in `container` (hidden while closed). */
export class SavesPanel {
  private urls: string[] = [];
  private entries: readonly SaveEntry[] = [];

  constructor(
    private readonly doc: Document,
    private readonly container: HTMLElement,
    private readonly api: OpenAirsideApi,
    private readonly write: (line: string) => void,
    private readonly i18n: Localizer,
  ) {
    container.hidden = true;
    // A new language renders an open list again (no restart, FR-006).
    i18n.onChange(() => {
      if (!this.container.hidden) void this.show(this.entries);
    });
  }

  /** Reads the list and shows it; previews load one after the other. */
  async open(): Promise<void> {
    let entries: SaveEntry[];
    try {
      entries = await this.api.saves.list();
    } catch (error) {
      this.write(`saves failed: ${describeError(error)}`);
      return;
    }
    await this.show(entries);
  }

  /** Shows `entries` (from `saves.list()`); previews load one after the other. */
  async show(entries: readonly SaveEntry[]): Promise<void> {
    this.entries = entries;
    this.render(entries);
    for (const entry of entries) {
      if (entry.state !== "ok" || this.container.hidden) continue;
      await this.preview(entry.id);
    }
  }

  close(): void {
    this.container.hidden = true;
    this.container.replaceChildren();
    for (const url of this.urls) URL.revokeObjectURL(url);
    this.urls = [];
  }

  private render(entries: readonly SaveEntry[]): void {
    this.close();
    const doc = this.doc;
    const t = this.i18n.t.bind(this.i18n);
    const title = doc.createElement("h2");
    title.textContent = t("ui.saves.title", { count: entries.length });
    const list = doc.createElement("ul");
    for (const entry of entries) {
      const item = doc.createElement("li");
      item.dataset.id = entry.id;
      const image = doc.createElement("img");
      image.alt = "";
      const text = doc.createElement("span");
      if (entry.state === "ok") {
        text.textContent = t("ui.saves.entry", {
          label: localSaveLabel(entry.meta, this.i18n),
          savedAt: savedAtText(entry.meta.savedAt),
          year: entry.meta.year,
          day: entry.meta.day,
        });
        item.append(
          image,
          text,
          button(doc, {
            label: t("ui.saves.load"),
            run: () => {
              void this.load(entry.id);
            },
          }),
        );
      } else {
        text.textContent = t("ui.saves.damaged", { reason: this.i18n.reason(entry.error) });
        item.append(image, text);
      }
      item.append(
        button(doc, {
          label: t("ui.saves.delete"),
          run: () => {
            void this.remove(entry.id);
          },
        }),
      );
      list.append(item);
    }
    this.container.append(
      title,
      list,
      button(doc, {
        label: t("ui.close"),
        run: () => {
          this.close();
        },
      }),
    );
    this.container.hidden = false;
  }

  private async preview(id: string): Promise<void> {
    try {
      const png = await this.api.saves.preview(id);
      if (png.byteLength === 0) return;
      const url = URL.createObjectURL(new Blob([png], { type: "image/png" }));
      this.urls.push(url);
      const image = this.container.querySelector(`li[data-id="${id}"] img`);
      if (image instanceof HTMLImageElement) image.src = url;
    } catch (error) {
      this.write(`preview ${id} failed: ${describeError(error)}`);
    }
  }

  private async load(id: string): Promise<void> {
    this.close();
    await loadSave(this.api, id, this.write);
  }

  private async remove(id: string): Promise<void> {
    try {
      await this.api.saves.delete(id);
      this.write(`deleted ${id}`);
    } catch (error) {
      this.write(`delete failed: ${describeError(error)}`);
    }
    await this.open();
  }
}

/** Loads a save game and reports the result in the console. */
export async function loadSave(
  api: OpenAirsideApi,
  id: string,
  write: (line: string) => void,
): Promise<void> {
  write(`loading ${id} …`);
  try {
    const meta = await api.saves.load(id);
    write(`loaded ${id} (tick ${String(meta.tick)})`);
  } catch (error) {
    write(`load failed: ${describeError(error)}`);
  }
}

/** A notice with buttons in `container`, e.g. after a crash of the core. */
export function showNotice(
  doc: Document,
  container: HTMLElement,
  text: string,
  actions: readonly Action[],
  dismissLabel: string,
): void {
  const message = doc.createElement("p");
  message.textContent = text;
  const close = (): void => {
    container.hidden = true;
    container.replaceChildren();
  };
  container.replaceChildren(
    message,
    ...actions.map((a) =>
      button(doc, {
        label: a.label,
        run: () => {
          close();
          a.run();
        },
      }),
    ),
    button(doc, { label: dismissLabel, run: close }),
  );
  container.hidden = false;
}
