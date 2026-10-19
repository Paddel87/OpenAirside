/**
 * Renderer entry of OpenAirside: connects to the core over the port of the
 * platform (S-6), shows the playfield (PixiJS) and the debug panel with
 * console, performance display and measurement series (FR-009, step 2.7),
 * plus the simple save game list, quick save (F5) and the offer to load the
 * last autosave after a crash of the core (step 2.5), and since step 2.6
 * the language of the interface (switched without restart) and the
 * minimal settings panel. Console and performance display stay English.
 * The interface panels (React) follow from roadmap step 3.6 on.
 */
import type { PackReport } from "../src/generated/PackReport.js";
import type { TickStats } from "../src/generated/TickStats.js";
import { type ClientHandlers, CoreClient } from "./core-client.js";
import { basePackLoaded, formatPacks, parseConsoleLine, TABLE_COLUMNS } from "./console.js";
import { FALLBACK_LANGUAGE, loadTexts, Localizer } from "./i18n.js";
import { defaultConfig, MeasureSeries, type SeriesHost } from "./measure.js";
import { PerfMeter } from "./perf.js";
import { describeError, onCorePort } from "./platform.js";
import { formatSaves, latestAutosave } from "./saves.js";
import { loadSave, SavesPanel, showNotice } from "./saves-panel.js";
import { Playfield } from "./playfield.js";
import { SettingsPanel } from "./settings-panel.js";
import { cell, readTable } from "./table-reader.js";

const root = document.getElementById("root");
const hud = document.getElementById("hud");
const log = document.getElementById("log");
const form = document.getElementById("console");
const input = document.getElementById("console-input");
const savesBox = document.getElementById("saves");
const noticeBox = document.getElementById("notice");
const settingsBox = document.getElementById("settings");
const settingsButton = document.getElementById("settings-button");

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
  let stats: TickStats | undefined;
  let series: MeasureSeries | undefined;

  const showPacks = (reports: readonly PackReport[]): void => {
    for (const line of formatPacks(reports)) write(line);
  };
  // S-4: a faulty base pack leads to a message at start instead of a crash.
  // Checked once the core answers, i.e. after it has loaded the packs.
  const checkBasePack = (): void => {
    window.openairside?.packs
      .list()
      .then((reports) => {
        const loaded = basePackLoaded(reports);
        root.dataset.basePack = loaded ? "loaded" : "failed";
        if (!loaded) {
          write("base pack failed to load – the game has no content:");
          showPacks(reports.filter((r) => r.source === "base"));
        }
      })
      .catch((error: unknown) => {
        write(`packs: ${error instanceof Error ? error.message : String(error)}`);
      });
  };

  const handlers: ClientHandlers = {
    welcome: (core, session) => {
      root.dataset.session = String(session);
      write(`core ${core.name} ${core.version}, session ${String(session)}`);
      checkBasePack();
      // The core is ready: texts of all loaded packs (S-6 `lang.texts`).
      reloadLanguage();
    },
    incompatible: (protocol) => {
      write(`incompatible core (protocol ${String(protocol)}) – please reinstall the game`);
    },
    sessionStarted: (session, at) => {
      root.dataset.session = String(session);
      write(`session ${String(session)} at tick ${String(at)}`);
      series?.onSession(session);
    },
    events: (session, events) => {
      series?.onEvents(performance.now(), session, events);
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
      const now = performance.now();
      const rtt = now - requestedAt;
      perf.frame(now, rtt);
      series?.onFrame(now, frame.header, rtt);
    },
    stats: (_tick, figures) => {
      stats = figures;
      series?.onStats(figures);
    },
    queryResult: (id, answer) => {
      const now = performance.now();
      if (answer.dictionary !== undefined) {
        states = answer.dictionary.entries;
        return;
      }
      const sentAt = queriesSent.get(id);
      if (sentAt === undefined) {
        // Not a console query: one of the measurement series.
        series?.onQuery(now, id);
        return;
      }
      queriesSent.delete(id);
      const ms = now - sentAt;
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
      series?.onQuery(performance.now(), id);
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
  const api = window.openairside;
  // FR-006: English until the settings are read; texts of the packs once the core answers.
  const i18n = new Localizer(FALLBACK_LANGUAGE, undefined, undefined, (key) => {
    void api?.logs.write("debug", `missing text ${key}`).catch(() => undefined);
  });
  // Only the latest call applies: texts loading for an earlier language never win.
  let wantedLanguage = FALLBACK_LANGUAGE;
  let languageCall = 0;
  const applyLanguage = async (code: string): Promise<void> => {
    wantedLanguage = code;
    languageCall += 1;
    const call = languageCall;
    const { primary, fallback } = await loadTexts(api?.lang, code);
    if (call !== languageCall) return;
    document.documentElement.lang = code;
    root.dataset.language = code;
    i18n.use(code, primary, fallback);
  };
  const reloadLanguage = (): void => {
    void applyLanguage(wantedLanguage);
  };
  if (api !== undefined) {
    api.settings.get().then(
      (settings) => applyLanguage(settings.language),
      (error: unknown) => {
        write(`settings failed: ${describeError(error)}`);
      },
    );
  }
  const panel =
    api === undefined || savesBox === null
      ? undefined
      : new SavesPanel(document, savesBox, api, write, i18n);
  const settingsPanel =
    api === undefined || settingsBox === null
      ? undefined
      : new SettingsPanel(document, settingsBox, api, i18n, applyLanguage);
  if (settingsButton instanceof HTMLButtonElement) {
    const label = (): void => {
      settingsButton.textContent = i18n.t("ui.settings.open");
    };
    label();
    i18n.onChange(label);
    settingsButton.addEventListener("click", () => {
      void settingsPanel?.open();
    });
  }
  const save = (name: string, kind: "manual" | "quick"): void => {
    if (api === undefined) {
      write("save: not connected");
      return;
    }
    api.saves.save({ name, kind }).then(
      ({ id }) => {
        write(`saved ${id}`);
      },
      (error: unknown) => {
        write(`save failed: ${describeError(error)}`);
      },
    );
  };
  const loadLastAutosave = async (): Promise<void> => {
    if (api === undefined) return;
    try {
      const id = latestAutosave(await api.saves.list());
      if (id === undefined) write("no autosave to load");
      else await loadSave(api, id, write);
    } catch (error) {
      write(`load failed: ${describeError(error)}`);
    }
  };
  api?.onCoreExit((code, restarting) => {
    write(`core process ended (code ${String(code)})${restarting ? "; restarting" : ""}`);
    void api.logs
      .write("error", `core process ended (code ${String(code)})`)
      .catch(() => undefined);
    if (noticeBox === null) return;
    // S-6 "Absturz des Kerns" (ADR-029): offer the last autosave, or point to the log file.
    const t = i18n.t.bind(i18n);
    if (restarting) {
      showNotice(
        document,
        noticeBox,
        t("ui.notice.coreRestarting"),
        [{ label: t("ui.notice.loadAutosave"), run: () => void loadLastAutosave() }],
        t("ui.notice.dismiss"),
      );
    } else {
      showNotice(
        document,
        noticeBox,
        t("ui.notice.coreStopped"),
        [
          {
            label: t("ui.notice.openLogFolder"),
            run: () =>
              void api.logs.openFolder().catch((error: unknown) => {
                write(`log folder: ${describeError(error)}`);
              }),
          },
        ],
        t("ui.notice.dismiss"),
      );
    }
  });

  const host: SeriesHost = {
    now: () => performance.now(),
    sleep: (ms) =>
      new Promise((resolve) => {
        setTimeout(resolve, ms);
      }),
    command: (command) => client?.send(command) ?? 0,
    query: (query) => client?.query(query) ?? 0,
    view: (view) => {
      playfield.view(view);
    },
    hotspot: () => playfield.hotspot(),
    status: write,
  };
  try {
    // Long tasks (> 50 ms) of the renderer's main thread, for the series.
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) series?.onLongTask(entry.duration);
    }).observe({ type: "longtask", buffered: false });
  } catch {
    write("long task observer not available");
  }

  const measure = async (minutes: number): Promise<void> => {
    const api = window.openairside;
    if (api === undefined || client === undefined) {
      write("measure: not connected");
      return;
    }
    await api.measure.start();
    const run = new MeasureSeries(host, defaultConfig(minutes));
    run.onSession(client.session);
    series = run;
    try {
      const result = await run.run();
      const file = await api.measure.report({
        series: result,
        renderer: {
          gpu: playfield.gpu,
          screenWidth: window.screen.width,
          screenHeight: window.screen.height,
          devicePixelRatio: window.devicePixelRatio,
          windowWidth: window.innerWidth,
          windowHeight: window.innerHeight,
        },
      });
      write(`measure: done, result saved to ${file}`);
      root.dataset.measure = "done";
    } finally {
      series = undefined;
    }
  };

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    const line = input.value;
    input.value = "";
    const result = parseConsoleLine(line);
    if (series !== undefined) {
      write("measurement running – please wait");
      return;
    }
    if (result.kind === "measure") {
      write(`measure: ${String(result.minutes)} min – do not touch, keep the window visible`);
      measure(result.minutes).catch((error: unknown) => {
        write(`measure failed: ${describeError(error)}`);
        root.dataset.measure = "failed";
      });
    } else if (result.kind === "packs" || result.kind === "setPacks") {
      const api = window.openairside;
      if (api === undefined) {
        write("packs: not connected");
        return;
      }
      const call = result.kind === "packs" ? api.packs.list() : api.packs.setActive(result.ids);
      call
        .then((reports) => {
          showPacks(reports);
          if (result.kind === "setPacks") {
            write("packs: chosen packs apply from the next session (e.g. scenario hall)");
            // Languages and texts follow the newly loaded packs (ADR-031).
            reloadLanguage();
          }
        })
        .catch((error: unknown) => {
          write(`packs failed: ${describeError(error)}`);
        });
    } else if (result.kind === "saves") {
      if (api === undefined || panel === undefined) {
        write("saves: not connected");
        return;
      }
      api.saves.list().then(
        (entries) => {
          for (const line of formatSaves(entries)) write(line);
          void panel.show(entries);
        },
        (error: unknown) => {
          write(`saves failed: ${describeError(error)}`);
        },
      );
    } else if (result.kind === "save") {
      save(result.name, result.saveKind);
    } else if (result.kind === "load") {
      if (api === undefined) write("load: not connected");
      else void loadSave(api, result.id, write);
    } else if (result.kind === "deleteSave") {
      api?.saves.delete(result.id).then(
        () => {
          write(`deleted ${result.id}`);
        },
        (error: unknown) => {
          write(`delete failed: ${describeError(error)}`);
        },
      );
    } else if (result.kind === "languages") {
      api?.lang.list().then(
        (languages) => {
          for (const l of languages) {
            const current = l.code === i18n.code ? " (current)" : "";
            write(`lang ${l.code} ${l.name} – ${String(l.keys)} texts${current}`);
          }
        },
        (error: unknown) => {
          write(`lang failed: ${describeError(error)}`);
        },
      );
    } else if (result.kind === "setLanguage") {
      api?.settings.set({ language: result.code }).then(
        async (settings) => {
          await applyLanguage(settings.language);
          write(`language ${settings.language}`);
        },
        (error: unknown) => {
          write(`lang failed: ${describeError(error)}`);
        },
      );
    } else if (result.kind === "settings") {
      if (settingsPanel === undefined) write("settings: not connected");
      else void settingsPanel.open();
    } else if (result.kind === "command") {
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
    // Quick save also while typing in the console.
    if (event.key === "F5") {
      event.preventDefault();
      save("", "quick");
      return;
    }
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
        stats === undefined
          ? "tick figures pending"
          : `${String(stats.ticksPerSecond)} ticks/s · step ${(stats.stepMeanUs / 1000).toFixed(2)} ms (p99 ${(stats.stepP99Us / 1000).toFixed(2)}) · recompute ${stats.recomputing ? "running" : stats.lastRecompute === undefined ? "–" : `${(stats.lastRecompute.millis / 1000).toFixed(2)} s`}`,
        `${playfield.gpu}${lag === "" ? "" : ` · ${lag}`}${client === undefined ? "" : ` · gaps ${String(client.gaps)}`}`,
      ].join("\n");
    }
  }, 250);

  root.dataset.ready = "true";
}

void start();
