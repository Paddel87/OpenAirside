/**
 * Settings of the player (S-6 `settings.get`/`settings.set`, step 2.6,
 * ADR-031) and the languages of the loaded packs (`lang.list`,
 * `lang.texts`): checking, defaults and the file `settings.json`.
 *
 * The file and every call from the renderer are untrusted input: a broken
 * or foreign value falls back to its default, a call with one is refused.
 * The bounds mirror the constants of core/src/settings.rs.
 */
import type { LanguageInfo } from "../generated/LanguageInfo.js";
import type { Settings } from "../generated/Settings.js";

/** File of the settings in the data folder. */
export const SETTINGS_FILE = "settings.json";
/** Largest settings file that is read (a few dozen bytes are expected). */
export const MAX_SETTINGS_BYTES = 64 * 1024;
/** Bounds of the autosave interval in minutes (core/src/settings.rs). */
export const AUTOSAVE_MINUTES = { min: 1, max: 60, default: 10 } as const;
/** Bounds of the number of autosave files (core/src/settings.rs, S-5). */
export const AUTOSAVE_COUNT = { min: 1, max: 20, default: 5 } as const;
/** Language when the system language has no texts. */
export const FALLBACK_LANGUAGE = "en";
/** Languages of the base pack, offered on the first start (FR-006). */
export const BASE_LANGUAGES: readonly string[] = ["de", "en"];

const LANGUAGE_CODE = /^[a-z]{2,3}(?:-[a-z0-9]{2,8})?$/;
const TEXT_KEY = /^[A-Za-z0-9_.-]{1,128}$/;

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** Whether `value` follows the naming rule of a language code (S-4 `lang/`). */
export function isLanguageCode(value: unknown): value is string {
  return typeof value === "string" && LANGUAGE_CODE.test(value);
}

function inRange(value: unknown, bounds: { min: number; max: number }): value is number {
  return (
    typeof value === "number" &&
    Number.isInteger(value) &&
    value >= bounds.min &&
    value <= bounds.max
  );
}

/**
 * Language of a new installation: the first preferred system language
 * whose primary subtag is a base language (`de-AT` → `de`), else English.
 */
export function systemLanguage(preferred: readonly string[]): string {
  for (const tag of preferred) {
    const primary = tag.toLowerCase().split(/[-_]/)[0] ?? "";
    if (BASE_LANGUAGES.includes(primary)) return primary;
  }
  return FALLBACK_LANGUAGE;
}

/** Settings of a new installation. */
export function defaultSettings(preferred: readonly string[]): Settings {
  return {
    language: systemLanguage(preferred),
    autosaveMinutes: AUTOSAVE_MINUTES.default,
    autosaveCount: AUTOSAVE_COUNT.default,
  };
}

/**
 * Settings from the parsed file: each valid field is taken, each missing
 * or broken one falls back to `defaults`; `valid` says whether all were
 * taken (else the main process logs a warning).
 */
export function settingsFromFile(
  value: unknown,
  defaults: Settings,
): { settings: Settings; valid: boolean } {
  if (!isRecord(value)) return { settings: defaults, valid: false };
  const language = isLanguageCode(value.language) ? value.language : defaults.language;
  const autosaveMinutes = inRange(value.autosaveMinutes, AUTOSAVE_MINUTES)
    ? value.autosaveMinutes
    : defaults.autosaveMinutes;
  const autosaveCount = inRange(value.autosaveCount, AUTOSAVE_COUNT)
    ? value.autosaveCount
    : defaults.autosaveCount;
  const valid =
    language === value.language &&
    autosaveMinutes === value.autosaveMinutes &&
    autosaveCount === value.autosaveCount &&
    Object.keys(value).length === 3;
  return { settings: { language, autosaveMinutes, autosaveCount }, valid };
}

/**
 * Applies a partial update of `settings.set`: only the known fields, each
 * checked; the language must be one of `languages`.
 *
 * @returns the new settings, or `undefined` for an invalid update.
 */
export function applySettings(
  current: Settings,
  patch: unknown,
  languages: readonly LanguageInfo[],
): Settings | undefined {
  if (!isRecord(patch)) return undefined;
  const known = new Set(["language", "autosaveMinutes", "autosaveCount"]);
  if (Object.keys(patch).some((key) => !known.has(key))) return undefined;
  const next: Settings = { ...current };
  if ("language" in patch) {
    if (!isLanguageCode(patch.language)) return undefined;
    if (!languages.some((l) => l.code === patch.language)) return undefined;
    next.language = patch.language;
  }
  if ("autosaveMinutes" in patch) {
    if (!inRange(patch.autosaveMinutes, AUTOSAVE_MINUTES)) return undefined;
    next.autosaveMinutes = patch.autosaveMinutes;
  }
  if ("autosaveCount" in patch) {
    if (!inRange(patch.autosaveCount, AUTOSAVE_COUNT)) return undefined;
    next.autosaveCount = patch.autosaveCount;
  }
  return next;
}

/** Checks the language list of the core process (S-6 `lang.list`). */
export function parseLanguageList(value: unknown): LanguageInfo[] | undefined {
  if (!Array.isArray(value)) return undefined;
  const out: LanguageInfo[] = [];
  for (const item of value as unknown[]) {
    if (
      !isRecord(item) ||
      !isLanguageCode(item.code) ||
      typeof item.name !== "string" ||
      !inRange(item.keys, { min: 0, max: Number.MAX_SAFE_INTEGER })
    ) {
      return undefined;
    }
    out.push({ code: item.code, name: item.name, keys: item.keys });
  }
  return out;
}

/** Checks the texts of one language from the core process (S-6 `lang.texts`). */
export function parseTexts(value: unknown): Record<string, string> | undefined {
  if (!isRecord(value)) return undefined;
  const out: Record<string, string> = Object.create(null) as Record<string, string>;
  for (const [key, text] of Object.entries(value)) {
    if (!TEXT_KEY.test(key) || typeof text !== "string") return undefined;
    out[key] = text;
  }
  return out;
}
