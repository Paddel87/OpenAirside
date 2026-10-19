/**
 * Localisation of the user interface (module "Lokalisierung", FR-006,
 * step 2.6, ADR-031): looks up texts by key in the chosen language, falls
 * back to English and then to the key, and inserts parameters `{name}` as
 * plain text (numbers formatted for the language). Callers put the result
 * into `textContent`, never into HTML (S-3 "Texte").
 *
 * Texts come from the packs over S-6 `lang.texts`; the language files of
 * the base pack are also bundled, so that messages can be shown before the
 * core is ready or after it stopped.
 */
import baseDe from "../../content/base/lang/de.json" with { type: "json" };
import baseEn from "../../content/base/lang/en.json" with { type: "json" };
import type { ParamValue } from "../src/generated/ParamValue.js";
import type { Reason } from "../src/generated/Reason.js";

/** Language of the fallback texts. */
export const FALLBACK_LANGUAGE = "en";

/** Texts of one language by key. */
export type TextMap = ReadonlyMap<string, string>;

/** Shape of a language file of a pack (S-4 `lang/`). */
interface LangFile {
  readonly languages: readonly { readonly id: string; readonly texts: Record<string, string> }[];
}

/** Texts of `code` in a language file. */
export function textsOfFile(file: LangFile, code: string): TextMap {
  const out = new Map<string, string>();
  for (const language of file.languages) {
    if (language.id !== code) continue;
    for (const [key, text] of Object.entries(language.texts)) out.set(key, text);
  }
  return out;
}

/** Texts of the base pack bundled into the renderer, by language code. */
export const BUNDLED: ReadonlyMap<string, TextMap> = new Map([
  ["de", textsOfFile(baseDe, "de")],
  ["en", textsOfFile(baseEn, "en")],
]);

/** Texts from `lang.texts` (a plain object, checked by the main process). */
export function textMap(texts: Readonly<Record<string, string>>): TextMap {
  return new Map(Object.entries(texts));
}

/**
 * Inserts `params` into `text`: `{name}` becomes the parameter as text,
 * numbers formatted for `language`; unknown placeholders stay as written.
 */
export function format(
  text: string,
  params: Readonly<Record<string, ParamValue | undefined>>,
  language: string,
): string {
  let numbers: Intl.NumberFormat;
  try {
    numbers = new Intl.NumberFormat(language);
  } catch {
    numbers = new Intl.NumberFormat(FALLBACK_LANGUAGE);
  }
  return text.replace(/\{([A-Za-z0-9_]+)\}/g, (whole, name: string) => {
    const value = Object.hasOwn(params, name) ? params[name] : undefined;
    if (value === undefined) return whole;
    return typeof value === "number" ? numbers.format(value) : value;
  });
}

/** Looks up texts in the chosen language with English as fallback. */
export class Localizer {
  private listeners: (() => void)[] = [];
  private reported = new Set<string>();

  /**
   * @param onMissing called once per key that has no text in any language
   *   (e.g. to write a debug line into the log file).
   */
  constructor(
    private language: string,
    private primary: TextMap = BUNDLED.get(language) ?? new Map(),
    private fallback: TextMap = BUNDLED.get(FALLBACK_LANGUAGE) ?? new Map(),
    private readonly onMissing: (key: string) => void = () => undefined,
  ) {}

  /** Code of the current language. */
  get code(): string {
    return this.language;
  }

  /**
   * Switches to `language` with its texts and the English fallback texts;
   * registered views render again (no restart, FR-006).
   */
  use(language: string, primary: TextMap, fallback: TextMap = this.fallback): void {
    this.language = language;
    this.primary = primary;
    this.fallback = fallback;
    for (const listener of this.listeners) listener();
  }

  /** Calls `listener` after every switch of the language. */
  onChange(listener: () => void): void {
    this.listeners.push(listener);
  }

  /** Text of `key` with `params` inserted. */
  t(key: string, params: Readonly<Record<string, ParamValue | undefined>> = {}): string {
    const text = this.primary.get(key) ?? this.fallback.get(key);
    if (text === undefined) {
      if (!this.reported.has(key)) {
        this.reported.add(key);
        this.onMissing(key);
      }
      return key;
    }
    return format(text, params, this.language);
  }

  /** Text of a reason `{ code, textKey, params }` of the core or the platform. */
  reason(reason: Reason): string {
    return this.t(reason.textKey, reason.params);
  }
}

/** The part of S-6 that hands out texts. */
export interface TextSource {
  texts(code: string): Promise<Record<string, string>>;
}

/**
 * Texts of `code` and the English fallback: from the packs over
 * `lang.texts` where the core answers, else the bundled base texts (core
 * not ready or stopped, language unknown).
 */
export async function loadTexts(
  source: TextSource | undefined,
  code: string,
): Promise<{ readonly primary: TextMap; readonly fallback: TextMap }> {
  const fetch = async (language: string): Promise<TextMap> => {
    try {
      if (source !== undefined) return textMap(await source.texts(language));
    } catch {
      // Falls back to the bundled texts below.
    }
    return BUNDLED.get(language) ?? new Map<string, string>();
  };
  const fallback = await fetch(FALLBACK_LANGUAGE);
  const primary = code === FALLBACK_LANGUAGE ? fallback : await fetch(code);
  return { primary, fallback };
}
