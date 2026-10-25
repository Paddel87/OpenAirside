/**
 * The handlers of `settings.*` and `lang.*` of the platform API S-6 (module
 * "Plattform", step 3.10): the player's settings with their file and the
 * texts of the loaded languages. No Electron import.
 */
import type { LanguageInfo } from "../generated/LanguageInfo.js";
import type { Settings } from "../generated/Settings.js";
import type { SavesRequest } from "./core-status.js";
import { errorCode, type Log } from "./files.js";
import { fail } from "./platform-error.js";
import { readSettingsFile, writeSettingsFile } from "./settings-file.js";
import { applySettings, defaultSettings, isLanguageCode, parseTexts } from "./settings.js";

/** What the service needs from the main process. */
export interface SettingsDeps {
  /** Path of `settings.json`. */
  readonly file: () => string;
  /** Languages of the last loaded packs (S-6 `lang.list`, ADR-031). */
  readonly languages: () => readonly LanguageInfo[];
  /** Sends a request to the core process and waits for its answer. */
  readonly request: (make: (request: number) => SavesRequest) => Promise<unknown>;
  /** Called when a change moved the autosave interval. */
  readonly onAutosaveInterval: () => void;
  readonly log: Log;
}

/** Handlers of `settings.*` and `lang.texts`. */
export interface SettingsService {
  /** Reads the settings file; the system languages give the defaults of a new installation. */
  load(preferredLanguages: readonly string[]): void;
  /** The current settings. */
  get(): Settings;
  /** S-6 `settings.set`. */
  set(patch: unknown): Promise<Settings>;
  /** S-6 `lang.texts`. */
  texts(arg: unknown): Promise<Record<string, string>>;
}

export function createSettingsService(deps: SettingsDeps): SettingsService {
  const { log } = deps;
  let settings: Settings = defaultSettings([]);
  /** Changes of `settings.json` one after the other, each on the result of the previous one. */
  let writing: Promise<unknown> = Promise.resolve();
  /**
   * Open `lang.texts` requests by language: a second call for the same
   * language shares the first one (finding 2 of the separate review of 2.6),
   * so at most one request per listed language is open.
   */
  const textsPending = new Map<string, Promise<Record<string, string>>>();

  return {
    load(preferredLanguages) {
      settings = readSettingsFile(deps.file(), defaultSettings(preferredLanguages), log);
    },

    get: () => settings,

    // Checks the update, writes the file, then applies it. Calls run one after
    // the other, each on the result of the previous one, so that overlapping
    // calls lose no change (finding 3 of the separate review of 2.6).
    set(patch) {
      const change = writing.then(async () => {
        const next = applySettings(settings, patch, deps.languages()) ?? fail("invalidArgument");
        try {
          await writeSettingsFile(deps.file(), next);
        } catch (error) {
          log("warn", `cannot save the settings: ${errorCode(error)}`);
          fail("ioError");
        }
        const intervalChanged = next.autosaveMinutes !== settings.autosaveMinutes;
        settings = next;
        if (intervalChanged) deps.onAutosaveInterval();
        log("info", `settings changed: ${JSON.stringify(next)}`);
        return next;
      });
      writing = change.catch(() => undefined);
      return change;
    },

    texts(arg) {
      if (!isLanguageCode(arg) || !deps.languages().some((l) => l.code === arg)) {
        fail("invalidArgument");
      }
      const open = textsPending.get(arg);
      if (open !== undefined) return open;
      const request = (async (): Promise<Record<string, string>> => {
        const value = await deps.request((n) => ({ type: "lang:texts", request: n, code: arg }));
        // `null`: the language vanished with a pack change since the list.
        if (value === null) fail("invalidArgument");
        return parseTexts(value) ?? fail("invalidAnswer");
      })();
      textsPending.set(arg, request);
      void request
        .finally(() => {
          textsPending.delete(arg);
        })
        .catch(() => undefined);
      return request;
    },
  };
}
