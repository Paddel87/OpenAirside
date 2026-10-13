//! Spike 1.4: synthetic flight table for the panel prototype.
//!
//! The panel needs several thousand rows with values that change while the
//! game runs. The simulation from spikes 1.2/1.3 has no flights yet, so this
//! crate derives a flight schedule deterministically from the row index and
//! computes the live values (delay, status, passengers on board, bags,
//! remaining time) from the simulation tick. It stands in for what the flight
//! operations module will publish later; the numbers carry no game rules.
//!
//! The table is encoded column by column (one typed array per column, each
//! starting at a 4-byte boundary), so the renderer can read it through typed
//! array views without parsing. Texts (airlines, places, status names) are
//! sent once as a dictionary.
//!
//! Throwaway code; see README.md.

/// Game seconds per simulation tick (60 ticks/s = one game minute per second).
pub const SECONDS_PER_TICK: u64 = 1;
const DAY: i64 = 86_400;

/// Header size of an encoded table in bytes.
pub const HEADER_BYTES: usize = 32;
/// Format version written into the header.
pub const VERSION: u32 = 1;

/// Columns in the order they are encoded: name, bytes per value.
pub const COLUMNS: [(&str, usize); 12] = [
    ("flightNo", 2),  // u16
    ("airline", 1),   // u8, index into AIRLINES
    ("place", 2),     // u16, index into PLACES
    ("kind", 1),      // u8, 0 departure, 1 arrival
    ("sched", 2),     // u16, scheduled minute of the day
    ("delay", 2),     // i16, minutes (live)
    ("gate", 1),      // u8
    ("status", 1),    // u8, index into STATUS (live)
    ("booked", 2),    // u16
    ("onBoard", 2),   // u16 (live)
    ("bags", 2),      // u16 (live)
    ("remaining", 4), // i32, seconds until scheduled time plus delay (live)
];

/// Fictional airlines: two-letter code and name.
pub const AIRLINES: [(&str, &str); 16] = [
    ("NW", "Nordwind Air"),
    ("AF", "Alpenflug"),
    ("KS", "Küstensegler"),
    ("HB", "Hanse Blue"),
    ("SW", "Südwest Express"),
    ("OB", "Ostbrise"),
    ("LH", "Lichtung Airways"),
    ("ME", "Meridian Lines"),
    ("PO", "Polarstern"),
    ("SA", "Sahel Air"),
    ("TA", "Tasman Air"),
    ("AN", "Anden Connect"),
    ("BO", "Boreal Jet"),
    ("CO", "Cobalt Air"),
    ("DU", "Dünenflug"),
    ("GR", "Gezeiten Regional"),
];

/// Destinations and origins (real places, no material from any game).
pub const PLACES: [&str; 48] = [
    "Amsterdam",
    "Athen",
    "Bangkok",
    "Barcelona",
    "Berlin",
    "Bogotá",
    "Boston",
    "Brüssel",
    "Budapest",
    "Kairo",
    "Kapstadt",
    "Chicago",
    "Kopenhagen",
    "Dakar",
    "Delhi",
    "Dubai",
    "Dublin",
    "Edinburgh",
    "Frankfurt",
    "Genf",
    "Hamburg",
    "Helsinki",
    "Hongkong",
    "Istanbul",
    "Johannesburg",
    "Lissabon",
    "London",
    "Los Angeles",
    "Madrid",
    "Mailand",
    "Mexiko-Stadt",
    "Montreal",
    "Mumbai",
    "München",
    "Nairobi",
    "New York",
    "Oslo",
    "Paris",
    "Prag",
    "Reykjavík",
    "Rom",
    "São Paulo",
    "Seoul",
    "Singapur",
    "Stockholm",
    "Sydney",
    "Tokio",
    "Wien",
];

/// Status names; departures use 0..=4, arrivals 5..=9.
pub const STATUS: [&str; 10] = [
    "geplant",
    "Check-in",
    "Boarding",
    "geschlossen",
    "gestartet",
    "erwartet",
    "im Anflug",
    "gelandet",
    "Aussteigen",
    "beendet",
];

/// Bytes needed to encode a table with `rows` rows.
#[must_use]
pub fn max_bytes(rows: usize) -> usize {
    HEADER_BYTES
        + COLUMNS
            .iter()
            .map(|&(_, b)| (rows * b).next_multiple_of(4))
            .sum::<usize>()
}

/// Dictionary as JSON (sent once): airlines, places, status names, columns.
#[must_use]
pub fn dictionary_json() -> String {
    let quote = |s: &str| format!("\"{}\"", s.replace('"', "\\\""));
    let airlines: Vec<String> = AIRLINES
        .iter()
        .map(|(c, n)| format!("[{},{}]", quote(c), quote(n)))
        .collect();
    let places: Vec<String> = PLACES.iter().map(|p| quote(p)).collect();
    let status: Vec<String> = STATUS.iter().map(|s| quote(s)).collect();
    let columns: Vec<String> = COLUMNS
        .iter()
        .map(|(n, b)| format!("[{},{b}]", quote(n)))
        .collect();
    format!(
        "{{\"airlines\":[{}],\"places\":[{}],\"status\":[{}],\"columns\":[{}],\"headerBytes\":{HEADER_BYTES},\"version\":{VERSION}}}",
        airlines.join(","),
        places.join(","),
        status.join(","),
        columns.join(",")
    )
}

/// splitmix64: cheap, deterministic hash for synthetic values.
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// One row with all values at a given game time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flight {
    pub flight_no: u16,
    pub airline: u8,
    pub place: u16,
    pub kind: u8,
    pub sched: u16,
    pub delay: i16,
    pub gate: u8,
    pub status: u8,
    pub booked: u16,
    pub on_board: u16,
    pub bags: u16,
    pub remaining: i32,
}

/// Wraps a difference of seconds of the day into [-12 h, 12 h).
fn wrap(dt: i64) -> i64 {
    (dt + DAY / 2).rem_euclid(DAY) - DAY / 2
}

/// Share in 0..=1 of a linear ramp that starts `from` seconds before the event
/// and ends `to` seconds before it (`dt` = seconds until the event).
fn ramp(dt: i64, from: i64, to: i64) -> f64 {
    if dt >= from {
        0.0
    } else if dt <= to {
        1.0
    } else {
        (from - dt) as f64 / (from - to) as f64
    }
}

/// Flight `i` of `rows` at simulation tick `tick`.
#[must_use]
pub fn flight(i: usize, rows: usize, tick: u64) -> Flight {
    let h = mix(i as u64);
    let rows = rows.max(1) as i64;
    let sched_s = (i as i64 * DAY) / rows;
    let now = ((tick * SECONDS_PER_TICK) % DAY as u64) as i64;
    // Delay changes every 10 game minutes, staggered per flight.
    let offset = (h >> 40) as i64 % 600;
    let bucket = (now + offset) / 600;
    let hd = mix(h ^ (bucket as u64).wrapping_mul(0x51_7CC1_B727_220A));
    let delay_min: i64 = if hd % 10 < 7 {
        0
    } else {
        5 + (hd >> 8) as i64 % 116
    };
    let dt = wrap(sched_s + delay_min * 60 - now);
    let kind = u8::from(h & 1 == 1);
    let booked = 80 + (h >> 16) as u16 % 300;
    let (status, on_board, bags) = if kind == 0 {
        let status = match dt {
            d if d > 7_200 => 0,
            d if d > 2_400 => 1,
            d if d > 600 => 2,
            d if d > 0 => 3,
            _ => 4,
        };
        let share = ramp(dt, 2_400, 600);
        let bags_share = ramp(dt, 7_200, 900);
        (status, share, bags_share)
    } else {
        let status = match dt {
            d if d > 1_800 => 5,
            d if d > 0 => 6,
            d if d > -300 => 7,
            d if d > -1_500 => 8,
            _ => 9,
        };
        // Passengers leave the aircraft after landing; bags follow later.
        let share = 1.0 - ramp(dt, -300, -1_500);
        let bags_share = 1.0 - ramp(dt, -600, -2_400);
        (status, share, bags_share)
    };
    Flight {
        flight_no: 100 + (h >> 24) as u16 % 9_000,
        airline: ((h >> 8) % AIRLINES.len() as u64) as u8,
        place: ((h >> 32) % PLACES.len() as u64) as u16,
        kind,
        sched: (sched_s / 60) as u16,
        delay: delay_min as i16,
        gate: 1 + ((h >> 48) % 17) as u8,
        status,
        booked,
        on_board: (f64::from(booked) * on_board).round() as u16,
        bags: (f64::from(booked) * 1.2 * bags).round() as u16,
        remaining: dt as i32,
    }
}

/// Encodes the table at `tick` into `out`; returns the number of bytes.
/// `out` must hold at least `max_bytes(rows)` bytes.
pub fn encode(rows: usize, tick: u64, out: &mut [u8]) -> usize {
    let mut offsets = [0usize; COLUMNS.len()];
    let mut pos = HEADER_BYTES;
    for (k, &(_, b)) in COLUMNS.iter().enumerate() {
        offsets[k] = pos;
        pos += (rows * b).next_multiple_of(4);
    }
    for i in 0..rows {
        let f = flight(i, rows, tick);
        let mut put = |k: usize, bytes: &[u8]| {
            let at = offsets[k] + i * bytes.len();
            out[at..at + bytes.len()].copy_from_slice(bytes);
        };
        put(0, &f.flight_no.to_le_bytes());
        put(1, &[f.airline]);
        put(2, &f.place.to_le_bytes());
        put(3, &[f.kind]);
        put(4, &f.sched.to_le_bytes());
        put(5, &f.delay.to_le_bytes());
        put(6, &[f.gate]);
        put(7, &[f.status]);
        put(8, &f.booked.to_le_bytes());
        put(9, &f.on_board.to_le_bytes());
        put(10, &f.bags.to_le_bytes());
        put(11, &f.remaining.to_le_bytes());
    }
    out[0..4].copy_from_slice(&VERSION.to_le_bytes());
    out[4..8].copy_from_slice(&(rows as u32).to_le_bytes());
    out[8..16].copy_from_slice(&(tick as f64).to_le_bytes());
    out[16..20].copy_from_slice(&(COLUMNS.len() as u32).to_le_bytes());
    out[20..32].fill(0);
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u32_at(b: &[u8], off: usize) -> u32 {
        u32::from_le_bytes(b[off..off + 4].try_into().expect("4 bytes"))
    }

    #[test]
    fn layout_matches_max_bytes_and_header() {
        let rows = 1_001;
        let mut out = vec![0u8; max_bytes(rows)];
        let n = encode(rows, 12_345, &mut out);
        assert_eq!(n, max_bytes(rows));
        assert_eq!(u32_at(&out, 0), VERSION);
        assert_eq!(u32_at(&out, 4), rows as u32);
        assert_eq!(u32_at(&out, 16), COLUMNS.len() as u32);
        let tick = f64::from_le_bytes(out[8..16].try_into().expect("8 bytes"));
        assert!((tick - 12_345.0).abs() < f64::EPSILON);
    }

    #[test]
    fn columns_hold_the_row_values() {
        let rows = 37;
        let tick = 50_000;
        let mut out = vec![0u8; max_bytes(rows)];
        encode(rows, tick, &mut out);
        // Offsets: each column starts at a multiple of 4 after the previous one.
        let mut off = HEADER_BYTES;
        let mut offsets = Vec::new();
        for &(_, b) in &COLUMNS {
            offsets.push(off);
            off += (rows * b).next_multiple_of(4);
        }
        for i in [0, 5, 36] {
            let f = flight(i, rows, tick);
            let at = |k: usize, size: usize| offsets[k] + i * size;
            assert_eq!(&out[at(0, 2)..at(0, 2) + 2], &f.flight_no.to_le_bytes());
            assert_eq!(out[at(7, 1)], f.status);
            assert_eq!(&out[at(5, 2)..at(5, 2) + 2], &f.delay.to_le_bytes());
            assert_eq!(&out[at(11, 4)..at(11, 4) + 4], &f.remaining.to_le_bytes());
        }
    }

    #[test]
    fn deterministic_and_live() {
        assert_eq!(flight(42, 5_000, 1_000), flight(42, 5_000, 1_000));
        // Remaining time counts down with the tick (no delay change in between).
        let a = flight(42, 5_000, 1_000);
        let b = flight(42, 5_000, 1_001);
        if a.delay == b.delay {
            assert_eq!(a.remaining - 1, b.remaining);
        }
        // Over a day every departure passes through boarding and departs.
        let mut seen = [false; 10];
        for t in (0..86_400).step_by(60) {
            seen[flight(7, 100, t).status as usize] = true;
        }
        let dep = flight(7, 100, 0).kind == 0;
        let expected: &[usize] = if dep {
            &[0, 1, 2, 3, 4]
        } else {
            &[5, 6, 7, 8, 9]
        };
        for &s in expected {
            assert!(seen[s], "status {s} never reached");
        }
    }

    #[test]
    fn passengers_board_before_departure() {
        // Find a departure without delay and check boarding progress.
        let rows = 500;
        let tick_for = |i: usize, before_s: i64| {
            ((i as i64 * DAY / rows as i64) - before_s).rem_euclid(DAY) as u64
        };
        let i = (0..rows)
            .find(|&i| {
                let f = flight(i, rows, tick_for(i, 1_500));
                f.kind == 0 && f.delay == 0
            })
            .expect("a departure without delay");
        let f = flight(i, rows, tick_for(i, 1_500));
        assert_eq!(f.status, 2, "boarding");
        assert!(f.on_board > 0 && f.on_board < f.booked);
    }

    #[test]
    fn dictionary_is_json_like() {
        let d = dictionary_json();
        assert!(d.starts_with('{') && d.ends_with('}'));
        assert!(d.contains("\"Nordwind Air\"") && d.contains("\"remaining\""));
    }
}
