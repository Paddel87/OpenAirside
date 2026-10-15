/**
 * Client of the port to the core in the renderer (S-1, S-2, S-3; ADR-021):
 * handshake, session, command ids, at most one frame request in flight and
 * acknowledgement of events. Incoming messages are checked before use; the
 * renderer decides nothing about the game state (rule 002).
 */
import type { ClientMessage } from "../src/generated/ClientMessage.js";
import type { Command } from "../src/generated/Command.js";
import type { CoreInfo } from "../src/generated/CoreInfo.js";
import type { Detail } from "../src/generated/Detail.js";
import type { Dictionary } from "../src/generated/Dictionary.js";
import type { EventRecord } from "../src/generated/EventRecord.js";
import type { FrameFilter } from "../src/generated/FrameFilter.js";
import type { Query } from "../src/generated/Query.js";
import type { TickStats } from "../src/generated/TickStats.js";
import { type Frame, readFrame } from "./frame-reader.js";

/** Protocol version of this renderer (core/src/port.rs `PROTOCOL`). */
export const PROTOCOL = 1;
/** A frame request without answer for this long is given up. */
export const FRAME_TIMEOUT_MS = 1000;

/** The part of a `MessagePort` the client needs. */
export interface PortLike {
  postMessage(message: ClientMessage): void;
}

/** Callbacks of the client. */
export interface ClientHandlers {
  welcome(core: CoreInfo, session: number): void;
  incompatible(coreProtocol: number): void;
  sessionStarted(session: number, tick: number): void;
  /** Events in stream order; `session` may be an ended one (answers sent before a switch). */
  events(session: number, events: readonly EventRecord[]): void;
  frame(frame: Frame): void;
  /** Answer of query `id`: a table buffer, a dictionary or a detail (S-7). */
  queryResult(id: number, answer: QueryAnswer): void;
  /** Query `id` failed with an S-7 error code. */
  queryError(id: number, code: string): void;
  /** Figures of the tick at `tick`, about once per second (step 2.7). */
  stats(tick: number, stats: TickStats): void;
  /** A message was malformed; counted, never thrown. */
  dropped(reason: string): void;
}

/** Answer of a query; exactly one field is set. */
export interface QueryAnswer {
  readonly buf?: ArrayBuffer;
  readonly dictionary?: Dictionary;
  readonly detail?: Detail;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isCount(value: unknown): value is number {
  return typeof value === "number" && Number.isInteger(value) && value >= 0;
}

function isEventRecord(value: unknown): value is EventRecord {
  return (
    isRecord(value) &&
    isCount(value.seq) &&
    isCount(value.tick) &&
    isRecord(value.event) &&
    typeof value.event.kind === "string"
  );
}

const STATS_COUNTS = [
  "ticksPerSecond",
  "stepMeanUs",
  "stepP99Us",
  "stepMaxUs",
  "behindTicks",
  "threads",
  "backgroundThreads",
] as const;

function isTickStats(value: unknown): value is TickStats {
  if (!isRecord(value) || typeof value.recomputing !== "boolean") {
    return false;
  }
  if (!STATS_COUNTS.every((key) => isCount(value[key]))) {
    return false;
  }
  const last = value.lastRecompute;
  return last === undefined || (isRecord(last) && isCount(last.revision) && isCount(last.millis));
}

function eventsOf(value: unknown): EventRecord[] | undefined {
  if (!Array.isArray(value)) {
    return undefined;
  }
  return value.every(isEventRecord) ? value : undefined;
}

/** Protocol state of the renderer side of the port. */
export class CoreClient {
  #session = 0;
  #nextId = 1;
  #lastSeq = 0;
  #frameSince: number | undefined;
  #gaps = 0;
  #nextQuery = 1;
  /** Table queries in flight by panel (S-7 "Drosselung"). */
  readonly #panels = new Map<number, number>();
  readonly #queryPanel = new Map<number, number>();

  constructor(
    private readonly port: PortLike,
    private readonly handlers: ClientHandlers,
    private readonly now: () => number = () => performance.now(),
  ) {}

  /** Session of the handshake; 0 before `welcome`. */
  get session(): number {
    return this.#session;
  }

  /** Gaps seen in the `seq` numbering (lost events, S-3). */
  get gaps(): number {
    return this.#gaps;
  }

  /** Sends `hello`. */
  start(): void {
    this.port.postMessage({ type: "hello", protocol: PROTOCOL });
  }

  /** Sends a command and returns its id, or 0 before the handshake. */
  send(command: Command): number {
    if (this.#session === 0) {
      return 0;
    }
    const id = this.#nextId;
    this.#nextId += 1;
    this.port.postMessage({ type: "command", session: this.#session, id, command });
    return id;
  }

  /**
   * Sends a query and returns its id, or 0 before the handshake or while a
   * table query of the same panel is on its way (one per panel, S-7).
   */
  query(query: Query): number {
    if (this.#session === 0) {
      return 0;
    }
    if (query.kind === "table" && this.#panels.has(query.panel)) {
      return 0;
    }
    const id = this.#nextQuery;
    this.#nextQuery += 1;
    if (query.kind === "table") {
      this.#panels.set(query.panel, id);
      this.#queryPanel.set(id, query.panel);
    }
    this.port.postMessage({ type: "query", session: this.#session, id, query });
    return id;
  }

  /**
   * Asks for the newest frame unless one is on its way. Returns `true` if a
   * request was sent.
   */
  requestFrame(filter: FrameFilter): boolean {
    if (this.#session === 0) {
      return false;
    }
    const now = this.now();
    if (this.#frameSince !== undefined && now - this.#frameSince < FRAME_TIMEOUT_MS) {
      return false;
    }
    this.#frameSince = now;
    this.port.postMessage({
      type: "frameRequest",
      session: this.#session,
      filter,
      ack: this.#lastSeq,
    });
    return true;
  }

  /** Handles a message from the core. */
  handle(data: unknown): void {
    if (!isRecord(data) || typeof data.type !== "string") {
      this.handlers.dropped("unreadable");
      return;
    }
    switch (data.type) {
      case "welcome":
        if (isCount(data.session) && isRecord(data.core)) {
          this.#begin(data.session);
          this.handlers.welcome(data.core as unknown as CoreInfo, data.session);
          return;
        }
        break;
      case "incompatible":
        if (isCount(data.protocol)) {
          this.handlers.incompatible(data.protocol);
          return;
        }
        break;
      case "sessionStarted":
        if (isCount(data.session) && isCount(data.tick)) {
          this.#begin(data.session);
          this.handlers.sessionStarted(data.session, data.tick);
          return;
        }
        break;
      case "events": {
        const events = eventsOf(data.events);
        if (isCount(data.session) && events !== undefined) {
          this.#deliver(data.session, events);
          if (data.session === this.#session && events.length > 0) {
            this.port.postMessage({
              type: "eventsAck",
              session: this.#session,
              seq: this.#lastSeq,
            });
          }
          return;
        }
        break;
      }
      case "queryResult":
        if (isCount(data.id) && data.session === this.#session) {
          this.#settle(data.id);
          const answer: QueryAnswer =
            data.buf instanceof ArrayBuffer
              ? { buf: data.buf }
              : isRecord(data.dictionary)
                ? { dictionary: data.dictionary as unknown as Dictionary }
                : isRecord(data.detail)
                  ? { detail: data.detail as unknown as Detail }
                  : {};
          this.handlers.queryResult(data.id, answer);
          return;
        }
        break;
      case "queryError":
        if (isCount(data.id) && typeof data.code === "string" && data.session === this.#session) {
          this.#settle(data.id);
          this.handlers.queryError(data.id, data.code);
          return;
        }
        break;
      case "coreStats":
        if (data.session === this.#session && isCount(data.tick) && isTickStats(data.stats)) {
          this.handlers.stats(data.tick, data.stats);
          return;
        }
        break;
      case "frame": {
        const events = eventsOf(data.events);
        if (isCount(data.session) && events !== undefined && data.buf instanceof ArrayBuffer) {
          this.#frameSince = undefined;
          this.#deliver(data.session, events);
          const result = readFrame(data.buf);
          if (!result.ok) {
            this.handlers.dropped(`frame: ${result.error}`);
            return;
          }
          this.handlers.frame(result.frame);
          return;
        }
        break;
      }
    }
    this.handlers.dropped(data.type);
  }

  #settle(id: number): void {
    const panel = this.#queryPanel.get(id);
    if (panel !== undefined) {
      this.#queryPanel.delete(id);
      this.#panels.delete(panel);
    }
  }

  #begin(session: number): void {
    this.#session = session;
    this.#nextId = 1;
    this.#nextQuery = 1;
    this.#panels.clear();
    this.#queryPanel.clear();
    this.#lastSeq = 0;
    this.#frameSince = undefined;
  }

  #deliver(session: number, events: readonly EventRecord[]): void {
    if (session === this.#session) {
      for (const record of events) {
        if (record.seq !== this.#lastSeq + 1) {
          this.#gaps += 1;
        }
        this.#lastSeq = record.seq;
      }
    }
    if (events.length > 0) {
      this.handlers.events(session, events);
    }
  }
}
