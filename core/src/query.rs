//! Query service S-7 (`docs/architecture.md` section 4, ADR-021): tables
//! with filter, sorting and window computed in the core, a dictionary for
//! text columns and detail queries. Queries are answered from the snapshot
//! of the newest frame handed to the renderer, so that table and playfield
//! match, and outside the simulation step.
//!
//! Phase 2 has one table, `debugPersons`, for the debug panel and the
//! performance test; tables of the game (flights, staff, finances) follow
//! from roadmap step 3.6 on and are additive.
//!
//! Table layout version 1, little endian. Header of [`TABLE_HEADER_BYTES`]
//! bytes:
//!
//! | Offset | Type | Field |
//! |---|---|---|
//! | 0 | u16 | layout version ([`TABLE_LAYOUT_VERSION`]) |
//! | 2 | u16 | header size in bytes |
//! | 4 | u32 | tick of the answered state |
//! | 8 | u32 | matching rows in total |
//! | 12 | u32 | first row of the window |
//! | 16 | u32 | rows in the window (n) |
//! | 20 | u16 | column count |
//! | 22 | u16 | reserved, 0 |
//! | 24 | u32 | dictionary version |
//! | 28 | u32 | reserved, 0 |
//!
//! Then one block per requested column, in request order, each starting at
//! a 4-byte boundary: `handle` 8 bytes per row (u32 index, u32 generation),
//! `x` and `y` f32, `floor` i8, `heading` u8, `state` u16 (dictionary
//! index).

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::frame::Snapshot;
use crate::geometry::Handle;

/// Version of the table layout.
pub const TABLE_LAYOUT_VERSION: u16 = 1;
/// Size of the table header in bytes.
pub const TABLE_HEADER_BYTES: usize = 32;
/// Most rows of a window.
pub const MAX_LIMIT: u16 = 1000;
/// Most columns of a query.
pub const MAX_COLUMNS: usize = 64;
/// Most filters of a query.
pub const MAX_FILTERS: usize = 16;
/// Most sort criteria of a query.
pub const MAX_SORTS: usize = 4;
/// Version of the dictionary of `debugPersons`.
pub const PERSONS_DICTIONARY_VERSION: u32 = 1;
/// Texts of the column `state` of `debugPersons`.
pub const PERSON_STATES: [&str; 4] = ["walking", "stairs", "arrived", "unreachable"];

/// Tables of S-7; the ids are sent as text, so that unknown ids can be
/// answered with `unknownTable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum TableId {
    /// Persons of the newest frame (debug panel, performance test).
    DebugPersons,
}

/// Columns of `debugPersons`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum ColumnId {
    /// Handle of the row (index, session as generation).
    Handle,
    /// Position east in cells.
    X,
    /// Position south in cells.
    Y,
    /// Floor.
    Floor,
    /// Direction of the current step.
    Heading,
    /// Text from the dictionary: walking, stairs, arrived, unreachable.
    State,
}

impl ColumnId {
    fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "handle" => Self::Handle,
            "x" => Self::X,
            "y" => Self::Y,
            "floor" => Self::Floor,
            "heading" => Self::Heading,
            "state" => Self::State,
            _ => return None,
        })
    }

    fn width(self) -> usize {
        match self {
            Self::Handle => 8,
            Self::X | Self::Y => 4,
            Self::Floor | Self::Heading => 1,
            Self::State => 2,
        }
    }
}

/// Comparison of a filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum FilterOp {
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
    /// Less than.
    Lt,
    /// Less than or equal.
    Le,
    /// Greater than.
    Gt,
    /// Greater than or equal.
    Ge,
}

/// A filter on one column; text columns compare their dictionary index.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    /// Column id.
    pub column: String,
    /// Comparison.
    pub op: FilterOp,
    /// Value to compare with; must be finite.
    pub value: f64,
}

/// A sort criterion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    /// Column id.
    pub column: String,
    /// Largest first.
    pub descending: bool,
}

/// Rows to return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Window {
    /// First row.
    pub offset: u32,
    /// Number of rows, at most 1,000.
    pub limit: u16,
}

/// A query of S-7.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Query {
    /// Rows of a table.
    Table {
        /// Panel that asks; a renderer keeps one query per panel in flight.
        panel: u16,
        /// Table id.
        table: String,
        /// Column ids in the order of the answer.
        columns: Vec<String>,
        /// All filters must hold.
        filter: Vec<Filter>,
        /// Sort criteria in order of priority; ties keep the table order.
        sort: Vec<Sort>,
        /// Rows to return.
        window: Window,
    },
    /// Texts of the text columns of a table.
    Dictionary {
        /// Table id.
        table: String,
    },
    /// Details of one object.
    Detail {
        /// The object.
        about: Handle,
    },
}

/// Texts of a table's text columns with their version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Dictionary {
    /// Version, also in every table header.
    pub version: u32,
    /// Texts by index.
    pub entries: Vec<String>,
}

/// Details of an object.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Detail {
    /// A person of the newest frame.
    Person {
        /// Position east in cells.
        x: f32,
        /// Position south in cells.
        y: f32,
        /// Floor.
        floor: i8,
        /// State text from the dictionary of `debugPersons`.
        state: String,
    },
}

/// Answer of a query.
#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    /// Table in the layout above.
    Table(Vec<u8>),
    /// Dictionary.
    Dictionary(Dictionary),
    /// Detail; `None` would mean a stale handle, which is an error instead.
    Detail(Detail),
}

/// Error codes of S-7.
pub mod code {
    /// Table id unknown.
    pub const UNKNOWN_TABLE: &str = "unknownTable";
    /// Column id unknown.
    pub const UNKNOWN_COLUMN: &str = "unknownColumn";
    /// Filter value not finite.
    pub const INVALID_FILTER: &str = "invalidFilter";
    /// Limit, columns, filters or sort criteria above their maximum.
    pub const LIMIT_EXCEEDED: &str = "limitExceeded";
    /// Handle does not name an existing object.
    pub const STALE_HANDLE: &str = "staleHandle";

    /// Every code above; the base pack has a text for each
    /// (`core/tests/lang.rs`).
    pub const ALL: &[&str] = &[
        UNKNOWN_TABLE,
        UNKNOWN_COLUMN,
        INVALID_FILTER,
        LIMIT_EXCEEDED,
        STALE_HANDLE,
    ];
}

/// Prefix of the localisation keys of query errors.
pub const ERROR_TEXT_PREFIX: &str = "query.error";

fn table(id: &str) -> Result<TableId, &'static str> {
    match id {
        "debugPersons" => Ok(TableId::DebugPersons),
        _ => Err(code::UNKNOWN_TABLE),
    }
}

fn column(id: &str) -> Result<ColumnId, &'static str> {
    ColumnId::parse(id).ok_or(code::UNKNOWN_COLUMN)
}

fn state_of(heading: u8) -> u16 {
    use crate::wayfinding::flowfield::{ARRIVED, DOWN, UNREACHABLE, UP};
    match heading {
        UP | DOWN => 1,
        ARRIVED => 2,
        UNREACHABLE => 3,
        _ => 0,
    }
}

fn value(snapshot: &Snapshot, row: usize, column: ColumnId) -> f64 {
    let Some(p) = snapshot.person(row) else {
        return 0.0;
    };
    match column {
        ColumnId::Handle => row as f64,
        ColumnId::X => f64::from(p.x),
        ColumnId::Y => f64::from(p.y),
        ColumnId::Floor => f64::from(p.floor),
        ColumnId::Heading => f64::from(p.heading),
        ColumnId::State => f64::from(state_of(p.heading)),
    }
}

fn keep(op: FilterOp, have: f64, want: f64) -> bool {
    match op {
        FilterOp::Eq => have == want,
        FilterOp::Ne => have != want,
        FilterOp::Lt => have < want,
        FilterOp::Le => have <= want,
        FilterOp::Gt => have > want,
        FilterOp::Ge => have >= want,
    }
}

/// Answers `query` from `snapshot`.
///
/// # Errors
///
/// Returns the error code of S-7.
pub fn answer(query: &Query, snapshot: &Snapshot) -> Result<Answer, &'static str> {
    match query {
        Query::Table {
            table: id,
            columns,
            filter,
            sort,
            window,
            ..
        } => {
            table(id)?;
            if window.limit > MAX_LIMIT
                || columns.len() > MAX_COLUMNS
                || filter.len() > MAX_FILTERS
                || sort.len() > MAX_SORTS
            {
                return Err(code::LIMIT_EXCEEDED);
            }
            let columns: Vec<ColumnId> = columns
                .iter()
                .map(|c| column(c))
                .collect::<Result<_, _>>()?;
            let filter: Vec<(ColumnId, FilterOp, f64)> = filter
                .iter()
                .map(|f| {
                    if !f.value.is_finite() {
                        return Err(code::INVALID_FILTER);
                    }
                    Ok((column(&f.column)?, f.op, f.value))
                })
                .collect::<Result<_, _>>()?;
            let sort: Vec<(ColumnId, bool)> = sort
                .iter()
                .map(|s| Ok((column(&s.column)?, s.descending)))
                .collect::<Result<_, _>>()?;
            Ok(Answer::Table(encode_table(
                snapshot, &columns, &filter, &sort, *window,
            )))
        }
        Query::Dictionary { table: id } => {
            table(id)?;
            Ok(Answer::Dictionary(Dictionary {
                version: PERSONS_DICTIONARY_VERSION,
                entries: PERSON_STATES.iter().map(|&s| s.to_owned()).collect(),
            }))
        }
        Query::Detail { about } => {
            let index = usize::try_from(about.index).map_err(|_| code::STALE_HANDLE)?;
            let p = snapshot
                .person(index)
                .filter(|_| about.generation == snapshot.session())
                .ok_or(code::STALE_HANDLE)?;
            let state = PERSON_STATES[usize::from(state_of(p.heading))];
            Ok(Answer::Detail(Detail::Person {
                x: p.x,
                y: p.y,
                floor: p.floor,
                state: state.to_owned(),
            }))
        }
    }
}

fn encode_table(
    snapshot: &Snapshot,
    columns: &[ColumnId],
    filter: &[(ColumnId, FilterOp, f64)],
    sort: &[(ColumnId, bool)],
    window: Window,
) -> Vec<u8> {
    let mut rows: Vec<usize> = (0..snapshot.persons())
        .filter(|&row| {
            filter
                .iter()
                .all(|&(c, op, want)| keep(op, value(snapshot, row, c), want))
        })
        .collect();
    if !sort.is_empty() {
        rows.sort_by(|&a, &b| {
            for &(c, descending) in sort {
                let order = value(snapshot, a, c).total_cmp(&value(snapshot, b, c));
                let order = if descending { order.reverse() } else { order };
                if order != Ordering::Equal {
                    return order;
                }
            }
            Ordering::Equal
        });
    }
    let total = rows.len();
    let start = usize::try_from(window.offset)
        .unwrap_or(usize::MAX)
        .min(total);
    let end = start.saturating_add(usize::from(window.limit)).min(total);
    let shown = &rows[start..end];

    let mut out = vec![0u8; TABLE_HEADER_BYTES];
    let generation = snapshot.session();
    for &c in columns {
        while !out.len().is_multiple_of(4) {
            out.push(0);
        }
        out.reserve(shown.len() * c.width());
        for &row in shown {
            let p = snapshot.person(row).unwrap_or_default();
            match c {
                ColumnId::Handle => {
                    out.extend_from_slice(&u32::try_from(row).unwrap_or(u32::MAX).to_le_bytes());
                    out.extend_from_slice(&generation.to_le_bytes());
                }
                ColumnId::X => out.extend_from_slice(&p.x.to_le_bytes()),
                ColumnId::Y => out.extend_from_slice(&p.y.to_le_bytes()),
                ColumnId::Floor => out.extend_from_slice(&p.floor.to_le_bytes()),
                ColumnId::Heading => out.push(p.heading),
                ColumnId::State => out.extend_from_slice(&state_of(p.heading).to_le_bytes()),
            }
        }
    }
    let header = &mut out[..TABLE_HEADER_BYTES];
    header[0..2].copy_from_slice(&TABLE_LAYOUT_VERSION.to_le_bytes());
    header[2..4].copy_from_slice(&u16::try_from(TABLE_HEADER_BYTES).unwrap_or(0).to_le_bytes());
    header[4..8].copy_from_slice(&snapshot.tick().to_le_bytes());
    header[8..12].copy_from_slice(&u32::try_from(total).unwrap_or(u32::MAX).to_le_bytes());
    header[12..16].copy_from_slice(&u32::try_from(start).unwrap_or(u32::MAX).to_le_bytes());
    header[16..20].copy_from_slice(&u32::try_from(shown.len()).unwrap_or(0).to_le_bytes());
    header[20..22].copy_from_slice(&u16::try_from(columns.len()).unwrap_or(0).to_le_bytes());
    header[24..28].copy_from_slice(&PERSONS_DICTIONARY_VERSION.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::header_u32;
    use crate::geometry::{CellPos, MapSize};
    use crate::world::World;

    fn snapshot() -> Snapshot {
        let mut world = World::open_hall(MapSize::new(64, 64).unwrap(), 0, 1);
        world.spawn_persons(
            &[
                CellPos {
                    x: 5,
                    y: 9,
                    floor: 0,
                },
                CellPos {
                    x: 1,
                    y: 2,
                    floor: 0,
                },
                CellPos {
                    x: 7,
                    y: 2,
                    floor: 0,
                },
            ],
            3,
        );
        let mut snap = Snapshot::default();
        snap.capture(4, 50, &world);
        snap
    }

    fn table_query(
        columns: &[&str],
        filter: Vec<Filter>,
        sort: Vec<Sort>,
        window: Window,
    ) -> Query {
        Query::Table {
            panel: 1,
            table: "debugPersons".into(),
            columns: columns.iter().map(|&c| c.to_owned()).collect(),
            filter,
            sort,
            window,
        }
    }

    fn all() -> Window {
        Window {
            offset: 0,
            limit: 1000,
        }
    }

    fn table_of(query: &Query) -> Vec<u8> {
        match answer(query, &snapshot()) {
            Ok(Answer::Table(buf)) => buf,
            other => panic!("no table: {other:?}"),
        }
    }

    fn f32_column(buf: &[u8], offset: usize, rows: usize) -> Vec<f32> {
        (0..rows)
            .map(|i| {
                let at = offset + i * 4;
                f32::from_le_bytes(buf[at..at + 4].try_into().unwrap())
            })
            .collect()
    }

    #[test]
    fn table_sorts_filters_and_cuts_the_window() {
        let sort = vec![
            Sort {
                column: "y".into(),
                descending: false,
            },
            Sort {
                column: "x".into(),
                descending: true,
            },
        ];
        let buf = table_of(&table_query(
            &["handle", "x"],
            Vec::new(),
            sort.clone(),
            all(),
        ));
        assert_eq!(u16::from_le_bytes([buf[0], buf[1]]), TABLE_LAYOUT_VERSION);
        assert_eq!(header_u32(&buf, 4), Some(50));
        assert_eq!(header_u32(&buf, 8), Some(3));
        assert_eq!(header_u32(&buf, 16), Some(3));
        assert_eq!(u16::from_le_bytes([buf[20], buf[21]]), 2);
        assert_eq!(header_u32(&buf, 24), Some(PERSONS_DICTIONARY_VERSION));
        // Handles: rows 2 (7.5, 2.5), 1 (1.5, 2.5), 0 (5.5, 9.5); generation = session.
        let handles: Vec<(u32, u32)> = (0..3)
            .map(|i| {
                let at = TABLE_HEADER_BYTES + i * 8;
                (
                    header_u32(&buf, at).unwrap(),
                    header_u32(&buf, at + 4).unwrap(),
                )
            })
            .collect();
        assert_eq!(handles, vec![(2, 4), (1, 4), (0, 4)]);
        assert_eq!(
            f32_column(&buf, TABLE_HEADER_BYTES + 24, 3),
            vec![7.5, 1.5, 5.5]
        );

        let filter = vec![Filter {
            column: "x".into(),
            op: FilterOp::Gt,
            value: 2.0,
        }];
        let window = Window {
            offset: 1,
            limit: 5,
        };
        let buf = table_of(&table_query(&["x"], filter, sort, window));
        assert_eq!(header_u32(&buf, 8), Some(2), "two rows match");
        assert_eq!(header_u32(&buf, 12), Some(1));
        assert_eq!(header_u32(&buf, 16), Some(1));
        assert_eq!(f32_column(&buf, TABLE_HEADER_BYTES, 1), vec![5.5]);

        let beyond = Window {
            offset: 10,
            limit: 5,
        };
        let buf = table_of(&table_query(&["x"], Vec::new(), Vec::new(), beyond));
        assert_eq!(
            (header_u32(&buf, 12), header_u32(&buf, 16)),
            (Some(3), Some(0))
        );
    }

    #[test]
    fn narrow_columns_start_at_four_byte_boundaries() {
        let buf = table_of(&table_query(
            &["floor", "state", "heading", "y"],
            Vec::new(),
            Vec::new(),
            all(),
        ));
        // floor: 3 bytes + 1 padding, state: 6 + 2, heading: 3 + 1, y: 12.
        assert_eq!(buf.len(), TABLE_HEADER_BYTES + 4 + 8 + 4 + 12);
        let state = u16::from_le_bytes([buf[TABLE_HEADER_BYTES + 4], buf[TABLE_HEADER_BYTES + 5]]);
        assert_eq!(PERSON_STATES[usize::from(state)], "unreachable");
    }

    #[test]
    fn errors_name_the_problem() {
        let snap = snapshot();
        let check = |q: Query, code: &str| assert_eq!(answer(&q, &snap).err(), Some(code));
        let mut q = table_query(&["x"], Vec::new(), Vec::new(), all());
        if let Query::Table { table, .. } = &mut q {
            *table = "flights".into();
        }
        check(q, code::UNKNOWN_TABLE);
        check(
            table_query(&["altitude"], Vec::new(), Vec::new(), all()),
            code::UNKNOWN_COLUMN,
        );
        let nan = vec![Filter {
            column: "x".into(),
            op: FilterOp::Eq,
            value: f64::NAN,
        }];
        check(
            table_query(&["x"], nan, Vec::new(), all()),
            code::INVALID_FILTER,
        );
        let big = Window {
            offset: 0,
            limit: 1001,
        };
        check(
            table_query(&["x"], Vec::new(), Vec::new(), big),
            code::LIMIT_EXCEEDED,
        );
        let many: Vec<&str> = std::iter::repeat_n("x", 65).collect();
        check(
            table_query(&many, Vec::new(), Vec::new(), all()),
            code::LIMIT_EXCEEDED,
        );
        let bad_sort = vec![Sort {
            column: "z".into(),
            descending: false,
        }];
        check(
            table_query(&["x"], Vec::new(), bad_sort, all()),
            code::UNKNOWN_COLUMN,
        );
        check(
            Query::Dictionary {
                table: "flights".into(),
            },
            code::UNKNOWN_TABLE,
        );
        check(
            Query::Detail {
                about: Handle {
                    index: 3,
                    generation: 4,
                },
            },
            code::STALE_HANDLE,
        );
        check(
            Query::Detail {
                about: Handle {
                    index: 0,
                    generation: 3,
                },
            },
            code::STALE_HANDLE,
        );
    }

    #[test]
    fn dictionary_and_detail() {
        let snap = snapshot();
        let dict = answer(
            &Query::Dictionary {
                table: "debugPersons".into(),
            },
            &snap,
        );
        assert!(
            matches!(dict, Ok(Answer::Dictionary(d)) if d.entries.len() == 4 && d.version == 1)
        );
        let detail = answer(
            &Query::Detail {
                about: Handle {
                    index: 1,
                    generation: 4,
                },
            },
            &snap,
        );
        assert_eq!(
            detail,
            Ok(Answer::Detail(Detail::Person {
                x: 1.5,
                y: 2.5,
                floor: 0,
                state: "unreachable".into(),
            }))
        );
        assert_eq!(state_of(crate::wayfinding::flowfield::UP), 1);
        assert_eq!(state_of(crate::wayfinding::flowfield::ARRIVED), 2);
        assert_eq!(state_of(0), 0);
        let ops = [
            (FilterOp::Eq, true),
            (FilterOp::Ne, false),
            (FilterOp::Lt, false),
            (FilterOp::Le, true),
            (FilterOp::Gt, false),
            (FilterOp::Ge, true),
        ];
        for (op, expected) in ops {
            assert_eq!(keep(op, 1.0, 1.0), expected, "{op:?}");
        }
        assert_eq!(value(&snap, 99, ColumnId::X), 0.0);
    }
}
