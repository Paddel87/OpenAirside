/**
 * Renderer entry of OpenAirside: connects to the core over the port of the
 * platform (S-6), shows the playfield (PixiJS) and the debug panel with
 * console and performance display (FR-009). The interface panels (React)
 * follow from roadmap step 3.6 on.
 */
import { type ClientHandlers, CoreClient } from "./core-client.js";
import { parseConsoleLine, TABLE_COLUMNS } from "./console.js";
import { PerfMeter } from "./perf.js";
import { onCorePort } from "./platform.js";
import { Playfield } from "./playfield.js";
import { cell, readTable } from "./table-reader.js";

const root = document.getElementById("root");
const hud = document.getElementById("hud");
const log = document.getElementById("log");
const form = document.getElementById("console");
const input = document.getElementById("console-input");

function write(line: string): void {
  if (log === null) return;
  const entry = document.createElement("div");
  entry.textContent = line;
  log.append(entry);
  while (log.childElementCount > 200) log.firstElementChild?.remove();
  log.scrollTop = log.scrollHeight;
}

// The port may arrive while the playfield is still starting; register at once.
const ports: MessagePort[] = [];
let portArrived: ((port: MessagePort) => void) | undefined;
onCorePort(window, window, (port) => {
  if (portArrived === undefined) {
    ports.push(port);
  } else {
    portArrived(port);
  }
});

async function start(): Promise<void> {
  if (root === null || !(form instanceof HTMLFormElement) || !(input instanceof HTMLInputElement)) {
    return;
  }
  const playfield = await Playfield.create(root);
  const perf = new PerfMeter();
  let requestedAt = 0;
  let tick = 0;
  let personsTotal = 0;
  let lag = "";
  let client: CoreClient | undefined;
  const queriesSent = new Map<number, number>();
  let states: readonly string[] = [];

  const handlers: ClientHandlers = {
    welcome: (core, session) => {
      root.dataset.session = String(session);
      write(`core ${core.name} ${core.version}, session ${String(session)}`);
    },
    incompatible: (protocol) => {
      write(`incompatible core (protocol ${String(protocol)}) – please reinstall the game`);
    },
    sessionStarted: (session, at) => {
      root.dataset.session = String(session);
      write(`session ${String(session)} at tick ${String(at)}`);
    },
    events: (_session, events) => {
      for (const { event } of events) {
        switch (event.kind) {
          case "commandAccepted":
            write(`#${String(event.id)} ok (tick ${String(event.tick)})`);
            break;
          case "commandRejected":
            write(`#${String(event.id)} rejected: ${event.reason.code}`);
            break;
          case "simulationLag":
            lag = `lag ${String(event.behindTicks)} ticks`;
            break;
          case "simulationLagCleared":
            lag = "";
            break;
          case "speedChanged":
            write(`speed ${event.speed}${event.reason === undefined ? "" : ` (${event.reason})`}`);
            break;
          default:
            break;
        }
      }
    },
    frame: (frame) => {
      tick = frame.header.tick;
      personsTotal = frame.header.personsTotal;
      root.dataset.tick = String(tick);
      playfield.show(frame);
      perf.frame(performance.now(), performance.now() - requestedAt);
    },
    queryResult: (id, answer) => {
      const ms = performance.now() - (queriesSent.get(id) ?? performance.now());
      queriesSent.delete(id);
      if (answer.dictionary !== undefined) {
        states = answer.dictionary.entries;
        return;
      }
      if (answer.buf === undefined) return;
      const result = readTable(answer.buf, TABLE_COLUMNS);
      if (!result.ok) {
        write(`table rejected: ${result.error}`);
        return;
      }
      const { header } = result.table;
      write(
        `table: ${String(header.rows)} of ${String(header.total)} rows at tick ${String(header.tick)} in ${ms.toFixed(1)} ms`,
      );
      // The console shows at most 10 rows; the measurement needs only the timing.
      for (let row = 0; row < Math.min(header.rows, 10); row += 1) {
        const value = (column: (typeof TABLE_COLUMNS)[number]): number =>
          cell(result.table, column, row) ?? NaN;
        write(
          `  #${String(value("handle"))} (${value("x").toFixed(2)}, ${value("y").toFixed(2)}) floor ${String(value("floor"))} ${states[value("state")] ?? "?"}`,
        );
      }
    },
    queryError: (id, code) => {
      queriesSent.delete(id);
      write(`query #${String(id)} failed: ${code}`);
    },
    dropped: (reason) => {
      write(`dropped message: ${reason}`);
    },
  };

  const connect = (port: MessagePort): void => {
    const current = new CoreClient(port, handlers);
    client = current;
    states = [];
    port.onmessage = (event: MessageEvent) => {
      if (client === current) current.handle(event.data);
    };
    port.start();
    current.start();
  };
  portArrived = connect;
  for (const port of ports.splice(0)) connect(port);
  window.openairside?.onCoreExit((code) => {
    write(`core process ended (code ${String(code)}); restarting`);
  });

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    const line = input.value;
    input.value = "";
    const result = parseConsoleLine(line);
    if (result.kind === "command") {
      const id = client?.send(result.command) ?? 0;
      write(id === 0 ? "not connected" : `#${String(id)} ${line}`);
    } else if (result.kind === "query") {
      if (states.length === 0) client?.query({ kind: "dictionary", table: "debugPersons" });
      const id = client?.query(result.query) ?? 0;
      if (id === 0) write("not connected or query still running");
      else queriesSent.set(id, performance.now());
    } else {
      write(result.text);
    }
  });
  window.addEventListener("keydown", (event) => {
    if (event.target === input) return;
    if (event.key === "PageUp") playfield.changeFloor(1);
    if (event.key === "PageDown") playfield.changeFloor(-1);
  });

  const loop = (): void => {
    if (client?.requestFrame(playfield.filter()) === true) {
      requestedAt = performance.now();
    }
    requestAnimationFrame(loop);
  };
  requestAnimationFrame(loop);

  setInterval(() => {
    const s = perf.snapshot(performance.now());
    if (hud !== null) {
      hud.textContent = [
        `${String(s.fps)} fps · frame ${s.rttMs.toFixed(1)} ms (max ${s.rttMaxMs.toFixed(1)})`,
        `tick ${String(tick)} · floor ${String(playfield.floor)} · persons ${String(playfield.shown)}/${String(personsTotal)}`,
        `${playfield.gpu}${lag === "" ? "" : ` · ${lag}`}${client === undefined ? "" : ` · gaps ${String(client.gaps)}`}`,
      ].join("\n");
    }
  }, 250);

  root.dataset.ready = "true";
}

void start();
