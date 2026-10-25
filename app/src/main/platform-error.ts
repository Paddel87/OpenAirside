/**
 * Error form of the platform API S-6 (step 2.5): every call answers with
 * an envelope `{ ok, value }` or `{ ok: false, error }`, and the preload
 * rejects the promise with `error` as `{ code, textKey, params }`.
 * Electron passes only the message text of an error thrown in a handler,
 * hence the envelope (finding 9 of the separate review of step 2.4).
 *
 * Errors of the main process carry the text key `platform.error.<code>`;
 * reasons of the core (`save.error.<code>` …) pass through unchanged.
 */
import type { ParamValue } from "../generated/ParamValue.js";
import type { Reason } from "../generated/Reason.js";

/** Prefix of the text keys of errors raised by the main process. */
export const PLATFORM_TEXT_PREFIX = "platform.error";

/**
 * Every code the main process raises; the base pack has a text
 * `platform.error.<code>` for each in every language (app/renderer/i18n.test.ts,
 * checked against the sources in platform-error.test.ts).
 */
export const PLATFORM_CODES: readonly string[] = [
  "alreadyRunning",
  "busy",
  "coreExited",
  "diskFull",
  "internal",
  "invalidAnswer",
  "invalidArgument",
  "invalidId",
  "invalidState",
  "ioError",
  "limitExceeded",
  "notAnObject",
  "notReady",
  "notStarted",
  "timeout",
  "tooLarge",
  "unknownPack",
  "unknownSave",
  "untrustedSender",
];

/** Answer of an IPC handler of S-6. */
export type IpcAnswer =
  { readonly ok: true; readonly value: unknown } | { readonly ok: false; readonly error: Reason };

/** A refused S-6 call with its reason. */
export class PlatformFailure extends Error {
  constructor(readonly reason: Reason) {
    super(reason.code);
    this.name = "PlatformFailure";
  }
}

/** Reason of the main process with the key `platform.error.<code>`. */
export function platformReason(code: string, params: Record<string, ParamValue> = {}): Reason {
  return { code, textKey: `${PLATFORM_TEXT_PREFIX}.${code}`, params };
}

/** Throws a refusal with a reason of the main process. */
export function fail(code: string, params: Record<string, ParamValue> = {}): never {
  throw new PlatformFailure(platformReason(code, params));
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isParam(value: unknown): value is ParamValue {
  if (isRecord(value)) {
    // A localisation key `{ key }` (ADR-032, L-5): exactly one text member.
    return Object.keys(value).length === 1 && typeof value.key === "string";
  }
  return typeof value === "string" || (typeof value === "number" && Number.isFinite(value));
}

/**
 * Checks a reason that crossed a process boundary.
 *
 * @returns the reason, or `undefined` for an unexpected shape.
 */
export function parseReason(value: unknown): Reason | undefined {
  if (
    !isRecord(value) ||
    typeof value.code !== "string" ||
    typeof value.textKey !== "string" ||
    !isRecord(value.params)
  ) {
    return undefined;
  }
  const params: Record<string, ParamValue> = {};
  for (const [name, param] of Object.entries(value.params)) {
    if (!isParam(param)) return undefined;
    params[name] = param;
  }
  return { code: value.code, textKey: value.textKey, params };
}

/**
 * Reason in the message of an error of the native bridge (JSON text, see
 * bridge/src/lib.rs); any other message becomes `internal`.
 */
export function reasonOfBridgeError(error: unknown): Reason {
  const message = error instanceof Error ? error.message : String(error);
  try {
    return parseReason(JSON.parse(message)) ?? platformReason("internal");
  } catch {
    return platformReason("internal");
  }
}

/**
 * Runs an S-6 handler and wraps its result in the envelope. A refusal
 * becomes its reason; any other exception becomes `internal`, reported
 * through `onInternal` (its message may name paths and stays out of the
 * answer).
 */
export async function answer(
  run: () => unknown,
  onInternal: (error: unknown) => void,
): Promise<IpcAnswer> {
  try {
    return { ok: true, value: await run() };
  } catch (error) {
    if (error instanceof PlatformFailure) {
      return { ok: false, error: error.reason };
    }
    onInternal(error);
    return { ok: false, error: platformReason("internal") };
  }
}
