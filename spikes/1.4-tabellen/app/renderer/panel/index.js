// Spike 1.4 – flight table panel: React 19, TanStack Table 9 (sorting), own
// row virtualization with a fixed row height. Throwaway code.
//
// Display logic only (rule 002): the panel shows the table the core sends and
// sorts it for display; it decides nothing about the game state.
//
// Data path: the core encodes the table column by column (panel-core). The
// panel keeps typed array views on the newest buffer; rows are small objects
// { i } whose accessors read from these views. Each update passes a new row
// array, so TanStack rebuilds the row models (including the sort) once per
// update. Rendering is synchronous (flushSync) so that the cost of an update
// can be measured in one piece.

import { createElement as h, useReducer, useRef, useLayoutEffect } from "react";
import { flushSync } from "react-dom";
import { createRoot } from "react-dom/client";
import {
  createColumnHelper,
  createSortedRowModel,
  rowSortingFeature,
  sortFn_text,
  tableFeatures,
  useTable,
} from "@tanstack/react-table";

const ROW_H = 22; // px, fixed for the virtualization
const OVERSCAN = 6; // rows above and below the visible area

const features = tableFeatures({
  rowSortingFeature,
  sortedRowModel: createSortedRowModel(),
  sortFns: { text: sortFn_text },
});

/** Shared state between the measurement loop (main.js) and React. */
const S = {
  dict: null,
  cols: null, // typed array views on the newest table buffer
  rows: 0,
  rowObjs: [],
  data: [],
  sorting: [{ id: "onBoard", desc: true }],
  virtual: true,
  visible: true,
  rerender: null,
  modelMs: 0, // time of table.getRowModel() in the last render
  domRows: 0, // rows in the DOM after the last render
  scroller: null,
};

function decode(buf) {
  const dv = new DataView(buf);
  const rows = dv.getUint32(4, true);
  const ctors = { 1: Uint8Array, 2: Uint16Array, 4: Int32Array };
  const signed = { delay: Int16Array };
  const cols = {};
  let pos = S.dict.headerBytes;
  for (const [name, bytes] of S.dict.columns) {
    const Ctor = signed[name] ?? ctors[bytes];
    cols[name] = new Ctor(buf, pos, rows);
    pos += Math.ceil((rows * bytes) / 4) * 4;
  }
  return { rows, cols, tick: dv.getFloat64(8, true) };
}

// ---------------------------------------------------------------- formatting

const pad2 = (n) => String(n).padStart(2, "0");
const hhmm = (min) => `${pad2(Math.floor(min / 60) % 24)}:${pad2(min % 60)}`;
function remaining(s) {
  const a = Math.abs(s);
  const t = `${Math.floor(a / 3600)}:${pad2(Math.floor(a / 60) % 60)}:${pad2(a % 60)}`;
  return s < 0 ? `−${t}` : t;
}

const helper = createColumnHelper();
const c = () => S.cols;
const columns = helper.columns([
  helper.accessor(
    (r) => `${S.dict.airlines[c().airline[r.i]][0]} ${c().flightNo[r.i]}`,
    { id: "flight", header: "Flug", sortFn: "text", size: 72 },
  ),
  helper.accessor((r) => S.dict.airlines[c().airline[r.i]][1], {
    id: "airline",
    header: "Airline",
    sortFn: "text",
    size: 118,
  }),
  helper.accessor((r) => S.dict.places[c().place[r.i]], {
    id: "place",
    header: "Ziel/Herkunft",
    sortFn: "text",
    size: 104,
  }),
  helper.accessor((r) => c().kind[r.i], {
    id: "kind",
    header: "Art",
    cell: (x) => (x.getValue() ? "Ank." : "Abfl."),
    size: 44,
  }),
  helper.accessor((r) => c().sched[r.i], {
    id: "sched",
    header: "Plan",
    cell: (x) => hhmm(x.getValue()),
    size: 46,
  }),
  helper.accessor((r) => c().delay[r.i], {
    id: "delay",
    header: "Versp.",
    cell: (x) => (x.getValue() ? `+${x.getValue()}` : "–"),
    size: 48,
  }),
  helper.accessor((r) => c().gate[r.i], {
    id: "gate",
    header: "Gate",
    cell: (x) => `G${x.getValue()}`,
    size: 40,
  }),
  helper.accessor((r) => c().status[r.i], {
    id: "status",
    header: "Status",
    cell: (x) => S.dict.status[x.getValue()],
    size: 84,
  }),
  helper.accessor((r) => c().onBoard[r.i], {
    id: "onBoard",
    header: "PAX",
    cell: (x) => `${x.getValue()}/${c().booked[x.row.original.i]}`,
    size: 64,
  }),
  helper.accessor((r) => c().bags[r.i], {
    id: "bags",
    header: "Gepäck",
    size: 52,
  }),
  helper.accessor((r) => c().remaining[r.i], {
    id: "remaining",
    header: "Restzeit",
    cell: (x) => remaining(x.getValue()),
    size: 70,
  }),
]);
const gridTemplate = columns.map((col) => `${col.size}px`).join(" ");
const tableWidth = columns.reduce((a, col) => a + col.size, 0);

// ---------------------------------------------------------------- components

function Row({ row, top, table }) {
  return h(
    "div",
    {
      className: "row",
      style:
        top === undefined ? undefined : { transform: `translateY(${top}px)` },
    },
    row
      .getAllCells()
      .map((cell) =>
        h(
          "div",
          { key: cell.id, className: "cell" },
          h(table.FlexRender, { cell }),
        ),
      ),
  );
}

function Panel() {
  const [, force] = useReducer((x) => x + 1, 0);
  S.rerender = force;
  const scrollRef = useRef(null);
  const [, setScroll] = useReducer((x) => x + 1, 0);
  const table = useTable({
    features,
    columns,
    data: S.data,
    state: { sorting: S.sorting },
    onSortingChange: (updater) => {
      S.sorting = typeof updater === "function" ? updater(S.sorting) : updater;
      force();
    },
    getRowId: (r) => String(r.i),
  });
  useLayoutEffect(() => {
    S.scroller = scrollRef.current;
  });

  const t0 = performance.now();
  const rows = S.cols ? table.getRowModel().rows : [];
  S.modelMs = performance.now() - t0;

  let body;
  if (S.virtual) {
    const el = scrollRef.current;
    const top = el ? el.scrollTop : 0;
    const height = el ? el.clientHeight : 600;
    const first = Math.max(0, Math.floor(top / ROW_H) - OVERSCAN);
    const last = Math.min(
      rows.length,
      Math.ceil((top + height) / ROW_H) + OVERSCAN,
    );
    const visible = [];
    for (let k = first; k < last; k++) {
      visible.push(
        h(Row, { key: rows[k].id, row: rows[k], top: k * ROW_H, table }),
      );
    }
    S.domRows = visible.length;
    body = h(
      "div",
      { className: "inner", style: { height: rows.length * ROW_H } },
      visible,
    );
  } else {
    S.domRows = rows.length;
    body = h(
      "div",
      { className: "flow" },
      rows.map((row) => h(Row, { key: row.id, row, table })),
    );
  }

  return h(
    "div",
    { className: "panel", style: { width: tableWidth + 18 } },
    h(
      "div",
      { className: "title" },
      `Flugmonitor – ${S.rows.toLocaleString("de-DE")} Flüge`,
      h(
        "span",
        { className: "sub" },
        S.virtual ? " (virtualisiert)" : " (alle Zeilen im DOM)",
      ),
    ),
    h(
      "div",
      { className: "head", style: { gridTemplateColumns: gridTemplate } },
      table.getHeaderGroups()[0].headers.map((header) =>
        h(
          "div",
          {
            key: header.id,
            className: "hcell",
            onClick: header.column.getToggleSortingHandler(),
          },
          h(table.FlexRender, { header }),
          { asc: " ▲", desc: " ▼" }[header.column.getIsSorted()] ?? "",
        ),
      ),
    ),
    h(
      "div",
      {
        className: "scroller",
        ref: scrollRef,
        onScroll: S.virtual ? setScroll : undefined,
      },
      body,
    ),
  );
}

const STYLE = `
.panel { position: absolute; right: 0; top: 0; bottom: 0; display: flex; flex-direction: column;
  background: rgba(20, 24, 30, 0.92); border-left: 1px solid #39414c; font: 12px/1 system-ui, sans-serif;
  color: #e8eaed; font-variant-numeric: tabular-nums; }
.title { padding: 8px 10px; font-weight: 600; }
.title .sub { font-weight: 400; color: #9aa4b0; }
.head, .row { display: grid; grid-template-columns: ${gridTemplate}; }
.head { background: #252b33; border-bottom: 1px solid #39414c; }
.hcell { padding: 5px 4px; cursor: pointer; user-select: none; white-space: nowrap; overflow: hidden; }
.scroller { flex: 1; overflow-y: auto; overflow-x: hidden; contain: strict; }
.inner { position: relative; }
.inner .row { position: absolute; left: 0; right: 0; top: 0; }
.row { height: ${ROW_H}px; align-items: center; border-bottom: 1px solid #2a3038; }
.cell { padding: 0 4px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
`;

// ---------------------------------------------------------------- API for main.js

/**
 * Mounts the panel into `container`. `dict` is the dictionary from panel-core.
 * Returns the API the measurement loop uses.
 */
export function createPanel(container, dict) {
  S.dict = dict;
  const style = document.createElement("style");
  style.textContent = STYLE;
  document.head.append(style);
  const root = createRoot(container);
  flushSync(() => root.render(h(Panel)));
  const rerender = () => flushSync(() => S.rerender());
  return {
    /** Shows a new table buffer; returns { totalMs, modelMs, domRows }. */
    setData(buf) {
      const t0 = performance.now();
      const { rows, cols } = decode(buf);
      if (S.rowObjs.length !== rows) {
        S.rowObjs = Array.from({ length: rows }, (_, i) => ({ i }));
      }
      S.rows = rows;
      S.cols = cols;
      S.data = S.rowObjs.slice(); // new identity: TanStack rebuilds the models
      rerender();
      return {
        totalMs: performance.now() - t0,
        modelMs: S.modelMs,
        domRows: S.domRows,
      };
    },
    setSorting(sorting) {
      S.sorting = sorting;
      rerender();
    },
    setVirtual(on) {
      S.virtual = on;
      rerender();
    },
    get virtual() {
      return S.virtual;
    },
    setVisible(on) {
      S.visible = on;
      container.style.display = on ? "" : "none";
    },
    get visible() {
      return S.visible;
    },
    /** Scrolls by `px`, wrapping at the end (measurement series). */
    scrollBy(px) {
      const el = S.scroller;
      if (!el) return;
      const max = el.scrollHeight - el.clientHeight;
      el.scrollTop = el.scrollTop + px >= max ? 0 : el.scrollTop + px;
    },
    scrollTo(px) {
      if (S.scroller) S.scroller.scrollTop = px;
    },
  };
}
