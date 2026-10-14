/**
 * Reader of the table buffer of S-7 (layout version 1, core/src/query.rs).
 * Checks all lengths of the header against `byteLength` before it creates
 * views (S-7 "Leser im Renderer").
 */
import type { ColumnId } from "../src/generated/ColumnId.js";

/** Layout version this reader understands. */
export const TABLE_LAYOUT_VERSION = 1;
/** Header size of layout version 1. */
export const TABLE_HEADER_BYTES = 32;

/** Bytes per row of each column. */
export const COLUMN_WIDTH: Readonly<Record<ColumnId, number>> = {
  handle: 8,
  x: 4,
  y: 4,
  floor: 1,
  heading: 1,
  state: 2,
};

/** Header of a table answer. */
export interface TableHeader {
  readonly tick: number;
  readonly total: number;
  readonly offset: number;
  readonly rows: number;
  readonly dictionaryVersion: number;
}

/** A checked table: header plus one view per requested column. */
export interface Table {
  readonly header: TableHeader;
  readonly columns: ReadonlyMap<ColumnId, DataView>;
}

/** Result of {@link readTable}. */
export type TableResult =
  | { readonly ok: true; readonly table: Table }
  | { readonly ok: false; readonly error: "tooShort" | "layoutVersion" | "columns" | "length" };

/** Checks and reads a table buffer for the columns in request order. */
export function readTable(buf: ArrayBuffer, columns: readonly ColumnId[]): TableResult {
  if (buf.byteLength < TABLE_HEADER_BYTES) {
    return { ok: false, error: "tooShort" };
  }
  const head = new DataView(buf, 0, TABLE_HEADER_BYTES);
  if (
    head.getUint16(0, true) !== TABLE_LAYOUT_VERSION ||
    head.getUint16(2, true) !== TABLE_HEADER_BYTES
  ) {
    return { ok: false, error: "layoutVersion" };
  }
  if (head.getUint16(20, true) !== columns.length) {
    return { ok: false, error: "columns" };
  }
  const rows = head.getUint32(16, true);
  const views = new Map<ColumnId, DataView>();
  let offset = TABLE_HEADER_BYTES;
  for (const column of columns) {
    offset = Math.ceil(offset / 4) * 4;
    const length = rows * COLUMN_WIDTH[column];
    if (offset + length > buf.byteLength) {
      return { ok: false, error: "length" };
    }
    views.set(column, new DataView(buf, offset, length));
    offset += length;
  }
  if (offset !== buf.byteLength) {
    return { ok: false, error: "length" };
  }
  return {
    ok: true,
    table: {
      header: {
        tick: head.getUint32(4, true),
        total: head.getUint32(8, true),
        offset: head.getUint32(12, true),
        rows,
        dictionaryVersion: head.getUint32(24, true),
      },
      columns: views,
    },
  };
}

/** Value of `column` in `row` as number (handle: its index). */
export function cell(table: Table, column: ColumnId, row: number): number | undefined {
  const view = table.columns.get(column);
  if (view === undefined || row < 0 || row >= table.header.rows) {
    return undefined;
  }
  const at = row * COLUMN_WIDTH[column];
  switch (column) {
    case "handle":
      return view.getUint32(at, true);
    case "x":
    case "y":
      return view.getFloat32(at, true);
    case "floor":
      return view.getInt8(at);
    case "heading":
      return view.getUint8(at);
    case "state":
      return view.getUint16(at, true);
  }
}
