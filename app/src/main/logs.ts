/**
 * Log file of the game (S-6 `logs.*`, step 2.5): one file in the folder
 * `logs` of the user data folder with rotation over 5 files of 10 MB,
 * throttled entries of the renderer (at most 100 per second, at most 2,000
 * characters, line breaks and control characters masked) and the warnings
 * of the main process. Level `info` in releases, `debug` in development
 * builds (`project-context.md` section 8).
 *
 * File access goes through {@link LogFs}, so that rotation and throttling
 * are testable without a disk.
 */

/** Folder of the log files in the user data folder. */
export const LOGS_DIR = "logs";
/** Name of the current log file; older ones end in `.1` … `.4`. */
export const LOG_FILE = "openairside.log";
/** Log files kept, the current one included. */
export const LOG_FILES = 5;
/** Size from which the current file is rotated. */
export const LOG_FILE_BYTES = 10 * 1024 * 1024;
/** Most entries of the renderer per second. */
export const MAX_ENTRIES_PER_SECOND = 100;
/** Longest message of the renderer in characters. */
export const MAX_MESSAGE_CHARS = 2000;

/** Levels in rising order. */
export const LEVELS = ["debug", "info", "warn", "error"] as const;
/** Level of a log entry. */
export type LogLevel = (typeof LEVELS)[number];

/** File operations of the log; synchronous, the entries are small. */
export interface LogFs {
  /** Size of `file` in bytes, 0 if it does not exist. */
  size(file: string): number;
  append(file: string, text: string): void;
  /** Renames `from` to `to`, replacing `to`; nothing if `from` is missing. */
  rename(from: string, to: string): void;
  /** Removes `file`; nothing if it is missing. */
  remove(file: string): void;
}

/** Checks a level given by the renderer. */
export function isLogLevel(value: unknown): value is LogLevel {
  return typeof value === "string" && (LEVELS as readonly string[]).includes(value);
}

/**
 * Masks line breaks and control characters, so that one entry stays one
 * line and cannot forge others: `\n`, `\r`, `\t` as such, others as `\uXXXX`.
 */
export function maskMessage(message: string): string {
  let out = "";
  for (const char of message) {
    const code = char.codePointAt(0) ?? 0;
    if (char === "\n") out += "\\n";
    else if (char === "\r") out += "\\r";
    else if (char === "\t") out += "\\t";
    else if (code <= 0x1f || (code >= 0x7f && code <= 0x9f)) {
      out += `\\u${code.toString(16).padStart(4, "0")}`;
    } else out += char;
  }
  return out;
}

/**
 * Checks the arguments of `logs.write(level, message)`.
 *
 * @returns level and message, or `undefined` if invalid.
 */
export function parseLogArgs(
  level: unknown,
  message: unknown,
): { level: LogLevel; message: string } | undefined {
  if (!isLogLevel(level) || typeof message !== "string") return undefined;
  if (Array.from(message).length > MAX_MESSAGE_CHARS) return undefined;
  return { level, message };
}

/** The log file with rotation and a throttle for the renderer. */
export class LogFile {
  private windowStart = -Infinity;
  private inWindow = 0;
  private dropped = 0;

  /**
   * @param file path of the current log file; rotated files sit next to it.
   * @param minLevel entries below it are skipped.
   */
  constructor(
    private readonly fs: LogFs,
    private readonly file: string,
    private readonly minLevel: LogLevel,
    private readonly now: () => Date = () => new Date(),
  ) {}

  /** Writes an entry of the main process; never throws. */
  write(level: LogLevel, source: "main" | "core" | "renderer", message: string): void {
    if (LEVELS.indexOf(level) < LEVELS.indexOf(this.minLevel)) return;
    this.append(
      `${this.now().toISOString()} ${level.toUpperCase()} ${source} ${maskMessage(message)}\n`,
    );
  }

  /**
   * Writes an entry of the renderer at real time `nowMs`; returns `false`
   * if the throttle dropped it (the number of dropped entries is logged
   * with the next accepted one).
   */
  writeThrottled(level: LogLevel, message: string, nowMs: number): boolean {
    if (nowMs - this.windowStart >= 1000) {
      this.windowStart = nowMs;
      this.inWindow = 0;
    }
    if (this.inWindow >= MAX_ENTRIES_PER_SECOND) {
      this.dropped += 1;
      return false;
    }
    this.inWindow += 1;
    if (this.dropped > 0) {
      this.write("warn", "main", `${String(this.dropped)} renderer log entries dropped`);
      this.dropped = 0;
    }
    this.write(level, "renderer", message);
    return true;
  }

  private append(line: string): void {
    try {
      if (this.fs.size(this.file) + Buffer.byteLength(line) > LOG_FILE_BYTES) {
        this.rotate();
      }
      this.fs.append(this.file, line);
    } catch {
      // Logging must never stop the game; a full disk shows elsewhere.
    }
  }

  private rotate(): void {
    this.fs.remove(`${this.file}.${String(LOG_FILES - 1)}`);
    for (let n = LOG_FILES - 2; n >= 1; n -= 1) {
      this.fs.rename(`${this.file}.${String(n)}`, `${this.file}.${String(n + 1)}`);
    }
    this.fs.rename(this.file, `${this.file}.1`);
  }
}
