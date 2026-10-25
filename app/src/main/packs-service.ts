/**
 * The handlers of `packs.*` and `lang.list` of the platform API S-6 (module
 * "Plattform", step 3.10): the pack reports and languages of the core
 * process, the player's saved choice and the call `packs.setActive` with
 * its open request. No Electron import.
 */
import type { LanguageInfo } from "../generated/LanguageInfo.js";
import type { PackReport } from "../generated/PackReport.js";
import type { PackRequest } from "./core-status.js";
import { errorCode, type Log } from "./files.js";
import type { OperationLock } from "./operation-lock.js";
import { choosableIds, parseActiveIds } from "./packs.js";
import { writeActivePacksFile } from "./packs-file.js";
import { fail, PlatformFailure } from "./platform-error.js";

/** Longest wait for the core's answer to `packs.setActive`. */
export const PACKS_TIMEOUT_MS = 60_000;

/** Codes of a refused `packs.setActive` passed on to the renderer. */
const PACK_CODES: ReadonlySet<string> = new Set([
  "invalidArgument",
  "invalidId",
  "limitExceeded",
  "unknownPack",
  "busy",
  "notReady",
  "ioError",
  "timeout",
  "coreExited",
  "invalidAnswer",
]);

/** Answer of the core process to `packs:setActive`. */
export type PackAnswer =
  | { type: "packs"; request: number; packs: PackReport[]; languages: LanguageInfo[] }
  | { type: "packsError"; request: number; message: string };

/** What the service needs from the main process. */
export interface PacksDeps {
  readonly lock: OperationLock;
  /** Path of `packs-active.json`. */
  readonly file: () => string;
  /** Sends a message to the core process; `undefined` if there is none. */
  readonly send: (message: PackRequest) => void;
  /** True while the core process is ready for requests. */
  readonly coreReady: () => boolean;
  readonly log: Log;
}

/** Handlers of `packs.*` and `lang.list`. */
export interface PacksService {
  /** The core process reports its packs and languages (at start). */
  setReports(packs: PackReport[], languages: LanguageInfo[]): void;
  /** S-6 `packs.list`. */
  reports(): PackReport[];
  /** S-6 `lang.list`. */
  languages(): LanguageInfo[];
  /** S-6 `packs.setActive`. Rejects with a code as message. */
  setActive(arg: unknown): Promise<PackReport[]>;
  /** Settles the open `packs.setActive` call if `answer` belongs to it. */
  settle(answer: PackAnswer): void;
  /** The open `packs.setActive` call, if any (by request number). */
  pendingRequest(): number | undefined;
}

export function createPacksService(deps: PacksDeps): PacksService {
  let packReports: PackReport[] = [];
  let languageList: LanguageInfo[] = [];
  /** Open `packs.setActive` call; a second one is refused with `busy` (S-6). */
  let pending:
    | {
        readonly request: number;
        resolve: (reports: PackReport[]) => void;
        reject: (error: Error) => void;
      }
    | undefined;
  /** Number of the last `packs.setActive` request to the core process. */
  let requestNumber = 0;

  function settle(answer: PackAnswer): void {
    const open = pending;
    if (open?.request !== answer.request) {
      // Late answer to an earlier request (after its time limit).
      return;
    }
    pending = undefined;
    if (answer.type === "packs") {
      packReports = answer.packs;
      languageList = answer.languages;
      open.resolve(answer.packs);
    } else {
      open.reject(new Error(answer.message));
    }
  }

  async function change(ids: string[]): Promise<PackReport[]> {
    if (pending !== undefined) fail("busy");
    if (!deps.coreReady()) fail("notReady");
    requestNumber += 1;
    const request = requestNumber;
    const answer = new Promise<PackReport[]>((resolve, reject) => {
      const timer = setTimeout(() => {
        settle({ type: "packsError", request, message: "timeout" });
      }, PACKS_TIMEOUT_MS);
      pending = {
        request,
        resolve: (reports) => {
          clearTimeout(timer);
          resolve(reports);
        },
        reject: (error) => {
          clearTimeout(timer);
          reject(error);
        },
      };
    });
    // The answer may fail (core exit, time limit) while the file is written.
    answer.catch(() => undefined);
    try {
      await writeActivePacksFile(deps.file(), ids);
    } catch (error) {
      settle({ type: "packsError", request, message: "ioError" });
      deps.log("warn", `cannot save the chosen packs: ${errorCode(error)}`);
      return answer;
    }
    deps.send({ type: "packs:setActive", request, ids });
    return answer;
  }

  return {
    setReports(packs, languages) {
      packReports = packs;
      languageList = languages;
    },
    reports: () => packReports,
    languages: () => languageList,
    settle,
    pendingRequest: () => pending?.request,

    // Checks the ids, saves the choice (a restarted core process starts with
    // it) and lets the core reload the packs; the choice applies from the next
    // session on.
    async setActive(arg) {
      const ids = parseActiveIds(arg);
      if (ids === undefined) fail("invalidArgument");
      const choosable = choosableIds(packReports);
      if (!ids.every((id) => choosable.has(id))) fail("unknownPack");
      return deps.lock
        .exclusive("packs", () => change(ids))
        .catch((error: unknown) => {
          // The core host answers with a code as message; only known codes pass.
          if (error instanceof PlatformFailure) throw error;
          const code = error instanceof Error ? error.message : "";
          fail(PACK_CODES.has(code) ? code : "internal");
        });
    },
  };
}
