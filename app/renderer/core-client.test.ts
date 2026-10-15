import { describe, expect, it, vi } from "vitest";
import type { ClientMessage } from "../src/generated/ClientMessage.js";
import type { Query } from "../src/generated/Query.js";
import { type ClientHandlers, CoreClient, FRAME_TIMEOUT_MS, PROTOCOL } from "./core-client.js";
import { HEADER_BYTES } from "./frame-reader.js";

function setup() {
  const sent: ClientMessage[] = [];
  const handlers = {
    welcome: vi.fn(),
    incompatible: vi.fn(),
    sessionStarted: vi.fn(),
    events: vi.fn(),
    frame: vi.fn(),
    queryResult: vi.fn(),
    queryError: vi.fn(),
    stats: vi.fn(),
    dropped: vi.fn(),
  } satisfies ClientHandlers;
  let time = 0;
  const client = new CoreClient(
    {
      postMessage: (m) => {
        sent.push(m);
      },
    },
    handlers,
    () => time,
  );
  return {
    client,
    sent,
    handlers,
    advance: (ms: number) => {
      time += ms;
    },
  };
}

const core = { name: "openairside-core", version: "0.0.0" };

function frameBuf(session: number): ArrayBuffer {
  const buf = new ArrayBuffer(HEADER_BYTES);
  const v = new DataView(buf);
  v.setUint16(0, 1, true);
  v.setUint16(2, 32, true);
  v.setUint32(4, session, true);
  v.setInt8(24, 127);
  v.setUint8(25, 6);
  return buf;
}

function record(seq: number) {
  return { seq, tick: 1, event: { kind: "commandAccepted", id: seq, tick: 1 } };
}

describe("CoreClient", () => {
  it("does nothing before the handshake", () => {
    const { client, sent } = setup();
    expect(client.send({ kind: "skipNight" })).toBe(0);
    expect(client.requestFrame({})).toBe(false);
    client.start();
    expect(sent).toEqual([{ type: "hello", protocol: PROTOCOL }]);
  });

  it("numbers commands per session and restarts them on a new session", () => {
    const { client, sent, handlers } = setup();
    client.handle({ type: "welcome", protocol: 1, core, session: 2 });
    expect(handlers.welcome).toHaveBeenCalledWith(core, 2);
    expect(client.send({ kind: "setSpeed", speed: "x3" })).toBe(1);
    expect(client.send({ kind: "skipNight" })).toBe(2);
    expect(sent.at(-1)).toEqual({
      type: "command",
      session: 2,
      id: 2,
      command: { kind: "skipNight" },
    });
    client.handle({ type: "sessionStarted", session: 3, tick: 0 });
    expect(handlers.sessionStarted).toHaveBeenCalledWith(3, 0);
    expect(client.send({ kind: "skipNight" })).toBe(1);
    expect(client.session).toBe(3);
  });

  it("keeps one frame request in flight and acknowledges with the next", () => {
    const { client, sent, handlers, advance } = setup();
    client.handle({ type: "welcome", protocol: 1, core, session: 1 });
    expect(client.requestFrame({ floor: 0 })).toBe(true);
    expect(client.requestFrame({ floor: 0 })).toBe(false);
    client.handle({
      type: "frame",
      session: 1,
      tick: 5,
      events: [record(1), record(2)],
      buf: frameBuf(1),
    });
    expect(handlers.events).toHaveBeenCalledWith(1, [record(1), record(2)]);
    expect(handlers.frame).toHaveBeenCalledOnce();
    expect(client.requestFrame({})).toBe(true);
    expect(sent.at(-1)).toEqual({ type: "frameRequest", session: 1, filter: {}, ack: 2 });
    // A lost answer does not block forever.
    advance(FRAME_TIMEOUT_MS);
    expect(client.requestFrame({})).toBe(true);
  });

  it("acknowledges events that come without a frame and counts gaps", () => {
    const { client, sent, handlers } = setup();
    client.handle({ type: "welcome", protocol: 1, core, session: 1 });
    client.handle({ type: "events", session: 1, events: [record(1), record(3)] });
    expect(sent.at(-1)).toEqual({ type: "eventsAck", session: 1, seq: 3 });
    expect(client.gaps).toBe(1);
    // Answers of an ended session are delivered but not acknowledged.
    client.handle({ type: "events", session: 0, events: [record(9)] });
    expect(handlers.events).toHaveBeenLastCalledWith(0, [record(9)]);
    expect(sent.at(-1)).toEqual({ type: "eventsAck", session: 1, seq: 3 });
    client.handle({ type: "events", session: 1, events: [] });
    expect(handlers.events).toHaveBeenCalledTimes(2);
  });

  it("reports incompatible cores", () => {
    const { client, handlers } = setup();
    client.handle({ type: "incompatible", protocol: 2 });
    expect(handlers.incompatible).toHaveBeenCalledWith(2);
  });

  it.each([
    null,
    "frame",
    { type: 3 },
    { type: "welcome", session: -1, core },
    { type: "incompatible" },
    { type: "sessionStarted", session: 1 },
    { type: "events", session: 1, events: [{ seq: 1 }] },
    { type: "events", session: 1, events: "x" },
    { type: "frame", session: 1, events: [], buf: new Uint8Array(32) },
    { type: "other" },
  ])("drops malformed message %j", (message) => {
    const { client, handlers } = setup();
    client.handle(message);
    expect(handlers.dropped).toHaveBeenCalledOnce();
  });

  it("drops frames with a broken buffer", () => {
    const { client, handlers } = setup();
    client.handle({ type: "welcome", protocol: 1, core, session: 1 });
    client.handle({ type: "frame", session: 1, tick: 1, events: [], buf: new ArrayBuffer(4) });
    expect(handlers.dropped).toHaveBeenCalledWith("frame: tooShort");
    expect(handlers.frame).not.toHaveBeenCalled();
  });

  it("keeps one table query per panel in flight", () => {
    const { client, sent, handlers } = setup();
    const table: Query = {
      kind: "table",
      panel: 3,
      table: "debugPersons",
      columns: ["x"],
      filter: [],
      sort: [],
      window: { offset: 0, limit: 5 },
    };
    expect(client.query(table)).toBe(0);
    client.handle({ type: "welcome", protocol: 1, core, session: 1 });
    expect(client.query(table)).toBe(1);
    expect(client.query(table)).toBe(0);
    expect(client.query({ kind: "dictionary", table: "debugPersons" })).toBe(2);
    expect(sent.at(-1)).toEqual({
      type: "query",
      session: 1,
      id: 2,
      query: { kind: "dictionary", table: "debugPersons" },
    });
    const buf = new ArrayBuffer(32);
    client.handle({ type: "queryResult", session: 1, id: 1, buf });
    expect(handlers.queryResult).toHaveBeenLastCalledWith(1, { buf });
    expect(client.query(table)).toBe(3);
    const dictionary = { version: 1, entries: ["walking"] };
    client.handle({ type: "queryResult", session: 1, id: 2, dictionary });
    expect(handlers.queryResult).toHaveBeenLastCalledWith(2, { dictionary });
    const detail = { kind: "person", x: 1, y: 2, floor: 0, state: "walking" };
    client.handle({ type: "queryResult", session: 1, id: 4, detail });
    expect(handlers.queryResult).toHaveBeenLastCalledWith(4, { detail });
    client.handle({ type: "queryResult", session: 1, id: 5 });
    expect(handlers.queryResult).toHaveBeenLastCalledWith(5, {});
    client.handle({ type: "queryError", session: 1, id: 3, code: "unknownTable", textKey: "x" });
    expect(handlers.queryError).toHaveBeenCalledWith(3, "unknownTable");
    expect(client.query(table)).toBe(4);
    // Answers of an ended session are dropped.
    client.handle({ type: "queryError", session: 0, id: 4, code: "x" });
    expect(handlers.dropped).toHaveBeenLastCalledWith("queryError");
    client.handle({ type: "sessionStarted", session: 2, tick: 0 });
    expect(client.query(table)).toBe(1);
  });

  it("passes the tick figures of the own session and drops malformed ones", () => {
    const { client, handlers } = setup();
    const stats = {
      ticksPerSecond: 180,
      stepMeanUs: 400,
      stepP99Us: 900,
      stepMaxUs: 1200,
      behindTicks: 0,
      recomputing: false,
      lastRecompute: { revision: 2, millis: 1100 },
      threads: 2,
      backgroundThreads: 5,
    };
    client.handle({ type: "coreStats", session: 1, tick: 9, stats });
    expect(handlers.stats).not.toHaveBeenCalled();
    expect(handlers.dropped).toHaveBeenLastCalledWith("coreStats");
    client.handle({ type: "welcome", protocol: 1, core, session: 1 });
    client.handle({ type: "coreStats", session: 1, tick: 9, stats });
    expect(handlers.stats).toHaveBeenLastCalledWith(9, stats);
    const withoutRecompute: Partial<typeof stats> = { ...stats };
    delete withoutRecompute.lastRecompute;
    client.handle({ type: "coreStats", session: 1, tick: 10, stats: withoutRecompute });
    expect(handlers.stats).toHaveBeenLastCalledWith(10, withoutRecompute);
    for (const bad of [
      { ...stats, stepP99Us: -1 },
      { ...stats, recomputing: "no" },
      { ...stats, lastRecompute: { revision: 1 } },
      null,
    ]) {
      client.handle({ type: "coreStats", session: 1, tick: 11, stats: bad });
    }
    expect(handlers.stats).toHaveBeenCalledTimes(2);
  });
});
