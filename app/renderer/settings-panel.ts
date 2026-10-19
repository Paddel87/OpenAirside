/**
 * Minimal settings panel (step 2.6, ADR-031): language, autosave interval
 * and number of autosaves over S-6 `settings.*`; replaced by the settings
 * menu of step 5.1. The platform checks every value; the panel only offers
 * the bounds and shows a refusal as text.
 */
import type { LanguageInfo } from "../src/generated/LanguageInfo.js";
import type { Settings } from "../src/generated/Settings.js";
import type { Localizer } from "./i18n.js";
import { isPlatformError, type OpenAirsideApi } from "./platform.js";

/** Bounds offered by the inputs (checked again by the main process). */
export const MINUTES_RANGE = { min: 1, max: 60 } as const;
export const COUNT_RANGE = { min: 1, max: 20 } as const;

/**
 * The update of `settings.set` from the values of the form; `undefined`
 * if a number is not a whole number in its range.
 */
export function settingsPatch(values: {
  readonly language: string;
  readonly minutes: string;
  readonly count: string;
}): Partial<Settings> | undefined {
  const whole = (text: string, range: { min: number; max: number }): number | undefined => {
    if (!/^\d{1,3}$/.test(text.trim())) return undefined;
    const value = Number(text.trim());
    return value >= range.min && value <= range.max ? value : undefined;
  };
  const autosaveMinutes = whole(values.minutes, MINUTES_RANGE);
  const autosaveCount = whole(values.count, COUNT_RANGE);
  if (autosaveMinutes === undefined || autosaveCount === undefined) return undefined;
  return { language: values.language, autosaveMinutes, autosaveCount };
}

/** Text of a refused call: the reason of the platform, else its message. */
export function refusalText(error: unknown, i18n: Localizer): string {
  if (isPlatformError(error)) return i18n.reason(error);
  return i18n.t("ui.settings.failed");
}

/** The settings panel in `container` (hidden while closed). */
export class SettingsPanel {
  /** While saving, the panel renders itself once at the end. */
  private saving = false;

  constructor(
    private readonly doc: Document,
    private readonly container: HTMLElement,
    private readonly api: OpenAirsideApi,
    private readonly i18n: Localizer,
    /** Applies a new language to the whole interface (no restart). */
    private readonly applyLanguage: (code: string) => Promise<void>,
  ) {
    container.hidden = true;
    i18n.onChange(() => {
      if (!this.container.hidden && !this.saving) void this.open();
    });
  }

  /** Reads settings and languages and shows the form. */
  async open(message = ""): Promise<void> {
    let settings: Settings;
    let languages: LanguageInfo[];
    try {
      [settings, languages] = await Promise.all([this.api.settings.get(), this.api.lang.list()]);
    } catch (error) {
      this.render(undefined, [], refusalText(error, this.i18n));
      return;
    }
    this.render(settings, languages, message);
  }

  close(): void {
    this.container.hidden = true;
    this.container.replaceChildren();
  }

  private render(
    settings: Settings | undefined,
    languages: readonly LanguageInfo[],
    message: string,
  ): void {
    const doc = this.doc;
    const t = this.i18n.t.bind(this.i18n);
    const title = doc.createElement("h2");
    title.textContent = t("ui.settings.title");
    const form = doc.createElement("form");
    const status = doc.createElement("p");
    status.className = "status";
    status.textContent = message;
    const label = (key: string, input: HTMLElement): HTMLLabelElement => {
      const element = doc.createElement("label");
      const text = doc.createElement("span");
      text.textContent = t(key);
      element.append(text, input);
      return element;
    };
    const language = doc.createElement("select");
    language.name = "language";
    for (const info of languages) {
      const option = doc.createElement("option");
      option.value = info.code;
      option.textContent = `${info.name} (${info.code})`;
      option.selected = info.code === settings?.language;
      language.append(option);
    }
    const number = (name: string, range: { min: number; max: number }, value?: number) => {
      const input = doc.createElement("input");
      input.type = "number";
      input.name = name;
      input.min = String(range.min);
      input.max = String(range.max);
      input.step = "1";
      input.value = value === undefined ? "" : String(value);
      return input;
    };
    const minutes = number("minutes", MINUTES_RANGE, settings?.autosaveMinutes);
    const count = number("count", COUNT_RANGE, settings?.autosaveCount);
    const apply = doc.createElement("button");
    apply.type = "submit";
    apply.textContent = t("ui.settings.apply");
    apply.disabled = settings === undefined;
    const close = doc.createElement("button");
    close.type = "button";
    close.textContent = t("ui.close");
    close.addEventListener("click", () => {
      this.close();
    });
    form.append(
      label("ui.settings.language", language),
      label("ui.settings.autosaveMinutes", minutes),
      label("ui.settings.autosaveCount", count),
      apply,
      close,
    );
    form.addEventListener("submit", (event) => {
      event.preventDefault();
      const patch = settingsPatch({
        language: language.value,
        minutes: minutes.value,
        count: count.value,
      });
      if (patch === undefined) {
        status.textContent = t("ui.settings.invalid", {
          minutesMax: MINUTES_RANGE.max,
          countMax: COUNT_RANGE.max,
        });
        return;
      }
      void this.save(patch, status);
    });
    this.container.replaceChildren(title, form, status);
    this.container.hidden = false;
  }

  private async save(patch: Partial<Settings>, status: HTMLElement): Promise<void> {
    let saved: Settings;
    try {
      saved = await this.api.settings.set(patch);
    } catch (error) {
      status.textContent = refusalText(error, this.i18n);
      return;
    }
    if (saved.language !== this.i18n.code) {
      this.saving = true;
      try {
        await this.applyLanguage(saved.language);
      } finally {
        this.saving = false;
      }
    }
    // Renders the form again, in the new language if it changed.
    await this.open(this.i18n.t("ui.settings.saved"));
  }
}
