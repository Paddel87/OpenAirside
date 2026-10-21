//! Save games (S-5, FR-004): after loading, the state hash equals the one
//! before saving, also while a recomputation of the flow fields is running;
//! damaged or manipulated files are rejected with a code instead of a
//! crash.

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use openairside_core::adoption::AdoptionMode;
use openairside_core::build::grid::{Cell, Zone};
use openairside_core::command::Command;
use openairside_core::datapack::{self, Catalog, game_version};
use openairside_core::geometry::{CellPos, MapSize};
use openairside_core::hash::StateHasher;
use openairside_core::savegame::{
    self, Limits, MAX_INFO_BYTES, SaveKind, SaveRequest, code, decode, read_info,
};
use openairside_core::scenario::reference_terminal;
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::Speed;
use openairside_core::world::World;

const EDGE: u16 = 256;
const TEST_LIMITS: Limits = Limits { map_edges: &[EDGE] };

fn config(adoption: AdoptionMode) -> SimulationConfig {
    SimulationConfig {
        map: MapSize::new(EDGE, EDGE).unwrap(),
        threads: 2,
        background_threads: 2,
        adoption,
        persons: 0,
        seed: 0,
        start_tick: 0,
    }
}

fn base_content() -> Arc<Catalog> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../content/base");
    let folders = datapack::discover(Some(&base), None);
    Arc::new(datapack::load(&folders, &[], &game_version()).catalog)
}

/// Reference terminal of 256 × 256 cells with 2,000 persons and the base
/// pack, running at 3×.
fn terminal(adoption: AdoptionMode) -> Simulation {
    let map = MapSize::new(EDGE, EDGE).unwrap();
    let world = reference_terminal(map, EDGE, 17, 2_000, 7).unwrap();
    let mut sim = Simulation::with_world(config(adoption), world)
        .unwrap()
        .with_content(base_content());
    sim.submit(1, 1, Command::SetSpeed { speed: Speed::X3 });
    sim
}

/// Another simulation with an empty world, as the target of loading.
fn other(adoption: AdoptionMode, content: Arc<Catalog>) -> Simulation {
    let map = MapSize::new(EDGE, EDGE).unwrap();
    Simulation::with_world(config(adoption), World::new(map))
        .unwrap()
        .with_content(content)
}

fn steps(sim: &mut Simulation, n: u32) {
    for _ in 0..n {
        sim.step();
        for record in sim.take_events(usize::MAX) {
            sim.ack(sim.session(), record.seq);
        }
    }
}

/// Closes a door of the terminal and requests a recomputation.
fn build_and_recompute(sim: &mut Simulation) -> u32 {
    for x in 30..34 {
        let pos = CellPos {
            x,
            y: EDGE / 2 + 30,
            floor: 0,
        };
        sim.world_mut()
            .build_mut()
            .grid_mut()
            .set(pos, Cell::VOID)
            .unwrap();
    }
    sim.request_recompute()
}

fn request(kind: SaveKind) -> SaveRequest {
    SaveRequest {
        name: "Testflughafen".into(),
        kind,
        saved_at: "2026-10-19T02:30:00Z".into(),
        preview: png(320, 180),
    }
}

/// A minimal PNG header of the given size (the core reads only the header).
fn png(width: u32, height: u32) -> Vec<u8> {
    let mut out = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR".to_vec();
    out.extend_from_slice(&width.to_be_bytes());
    out.extend_from_slice(&height.to_be_bytes());
    out.extend_from_slice(&[8, 6, 0, 0, 0, 0, 0, 0, 0]);
    out
}

fn save(sim: &Simulation) -> Vec<u8> {
    sim.save_snapshot()
        .encode(&request(SaveKind::Manual), "", "0.0.0")
        .unwrap()
}

/// Rewrites the checksum after a manipulation, so that the checks behind it
/// are reached.
fn fix_checksum(bytes: &mut [u8]) {
    let end = bytes.len() - 8;
    let mut hasher = StateHasher::new();
    hasher.bytes(&bytes[..end]);
    bytes[end..].copy_from_slice(&hasher.finish().to_le_bytes());
}

fn code_of(bytes: &[u8]) -> String {
    match decode(bytes, &TEST_LIMITS) {
        Ok(_) => "ok".into(),
        Err(reason) => reason.code,
    }
}

#[test]
fn loading_restores_the_state_hash() {
    let mut sim = terminal(AdoptionMode::WhenReady);
    steps(&mut sim, 300);
    let before = sim.state_hash();
    let bytes = save(&sim);
    let game = decode(&bytes, &TEST_LIMITS).unwrap();
    assert_eq!(game.meta().tick, 300);
    assert_eq!(game.meta().name, "Testflughafen");
    assert_eq!(game.meta().packs.len(), 1);
    assert_eq!(game.meta().packs[0].id, "base");

    let mut loaded = other(AdoptionMode::WhenReady, base_content());
    let session = loaded.session();
    loaded.load_game(game).unwrap();
    assert_eq!(loaded.state_hash(), before);
    assert_eq!(loaded.tick(), 300);
    assert_eq!(loaded.speed(), Speed::X3);
    assert_eq!(
        loaded.session(),
        session + 1,
        "loading starts a new session"
    );
    assert_eq!(loaded.world().persons().len(), 2_000);
}

#[test]
fn loading_during_a_recomputation_restores_the_hash_and_continues_bit_for_bit() {
    let mode = AdoptionMode::FixedTick { delay: 20 };
    let mut sim = terminal(mode);
    steps(&mut sim, 60);
    let revision = build_and_recompute(&mut sim);
    steps(&mut sim, 5);
    assert!(sim.is_recomputing(), "the recomputation is still open");
    let before = sim.state_hash();
    let bytes = save(&sim);

    let mut loaded = other(mode, base_content());
    loaded
        .load_game(decode(&bytes, &TEST_LIMITS).unwrap())
        .unwrap();
    assert_eq!(loaded.state_hash(), before);
    assert!(loaded.is_recomputing(), "the recomputation starts again");

    // Both continue identically: the result is adopted at the saved tick.
    steps(&mut sim, 100);
    steps(&mut loaded, 100);
    assert_eq!(sim.world().wayfinding().fields_revision(), revision);
    assert_eq!(loaded.world().wayfinding().fields_revision(), revision);
    assert_eq!(loaded.state_hash(), sim.state_hash());
}

#[test]
fn loading_while_recomputing_in_normal_play_restores_the_hash() {
    let mut sim = terminal(AdoptionMode::WhenReady);
    steps(&mut sim, 30);
    build_and_recompute(&mut sim);
    let before = sim.state_hash();
    let bytes = save(&sim);
    let mut loaded = other(AdoptionMode::WhenReady, base_content());
    loaded
        .load_game(decode(&bytes, &TEST_LIMITS).unwrap())
        .unwrap();
    assert_eq!(loaded.state_hash(), before);
}

#[test]
fn a_failed_load_keeps_the_running_session() {
    let mut sim = terminal(AdoptionMode::WhenReady);
    steps(&mut sim, 10);
    let bytes = save(&sim);
    let mut target = other(AdoptionMode::WhenReady, Arc::new(Catalog::empty()));
    let (session, hash) = (target.session(), target.state_hash());
    let reason = target
        .load_game(decode(&bytes, &TEST_LIMITS).unwrap())
        .unwrap_err();
    assert_eq!(reason.code, code::PACK_MISSING);
    assert_eq!(reason.text_key, "save.error.packMissing");
    assert_eq!(
        reason.params.get("packs"),
        Some(&openairside_core::text::ParamValue::Text("base".into()))
    );
    assert_eq!((target.session(), target.state_hash()), (session, hash));
}

#[test]
fn another_major_version_of_a_pack_is_rejected() {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!(
        "openairside-savegame-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("manifest.json"),
        r#"{"id":"base","name":"Base","version":"1.0.0","format":1,"game":">=0.0.0"}"#,
    )
    .unwrap();
    let catalog =
        datapack::load(&datapack::discover(Some(&dir), None), &[], &game_version()).catalog;
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(catalog.packs()[0].version, "1.0.0");

    let bytes = save(&terminal(AdoptionMode::WhenReady));
    let mut target = other(AdoptionMode::WhenReady, Arc::new(catalog));
    let reason = target
        .load_game(decode(&bytes, &TEST_LIMITS).unwrap())
        .unwrap_err();
    assert_eq!(reason.code, code::PACK_VERSION_MISMATCH);
}

#[test]
fn the_list_reads_header_and_metadata_only() {
    let sim = terminal(AdoptionMode::WhenReady);
    let bytes = save(&sim);
    let prefix = &bytes[..MAX_INFO_BYTES.min(bytes.len())];
    let info = read_info(prefix, bytes.len() as u64).unwrap();
    assert_eq!(info.game_version, "0.0.0");
    assert_eq!(info.meta.kind, SaveKind::Manual);
    assert_eq!(info.meta.map, MapSize::new(EDGE, EDGE).unwrap());
    assert_eq!((info.meta.day, info.meta.year), (1, 1));
    let start = usize::try_from(info.preview_offset).unwrap();
    let end = start + info.preview_len as usize;
    assert_eq!(&bytes[start..end], png(320, 180).as_slice());

    // Damage behind the metadata is not seen by the list, but by loading.
    let mut damaged = bytes.clone();
    let last = damaged.len() - 20;
    damaged[last] ^= 0xff;
    assert!(read_info(&damaged[..prefix.len()], damaged.len() as u64).is_ok());
    assert_eq!(code_of(&damaged), code::CHECKSUM_MISMATCH);
    // A list entry needs header and metadata.
    assert_eq!(
        read_info(&bytes[..20], bytes.len() as u64)
            .unwrap_err()
            .code,
        code::TRUNCATED
    );
}

#[test]
fn damaged_files_are_rejected_in_the_order_of_the_contract() {
    let bytes = save(&terminal(AdoptionMode::WhenReady));
    assert_eq!(code_of(&bytes), "ok");
    assert_eq!(code_of(b""), code::TRUNCATED);
    assert_eq!(code_of(b"OASA"), code::TRUNCATED);
    assert_eq!(code_of(b"PK\x03\x04 a zip file"), code::BAD_MAGIC);
    assert_eq!(code_of(&bytes[..bytes.len() - 1]), code::TRUNCATED);
    assert_eq!(code_of(&bytes[..10]), code::TRUNCATED);
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(code_of(&longer), code::CHECKSUM_MISMATCH);

    assert_eq!(bytes[8], 2, "world state format 2 (ADR-039)");
    let mut newer = bytes.clone();
    newer[8] = 3; // world state format 3
    assert_eq!(code_of(&newer), code::VERSION_TOO_NEW);
    let mut container = bytes.clone();
    container[6] = 9;
    assert_eq!(code_of(&container), code::VERSION_TOO_NEW);
    for format in [0, 1] {
        // Format 1 (before ADR-039) is not migrated before version 1.0 (S-5).
        let mut older = bytes.clone();
        older[8] = format;
        assert_eq!(code_of(&older), code::MIGRATION_FAILED);
    }

    // Length fields: header is 12 + 1 + 5 ("0.0.0") bytes, then four u32.
    let lengths = 18;
    let mut meta_len = bytes.clone();
    meta_len[lengths..lengths + 4].copy_from_slice(&(64 * 1024 + 1u32).to_le_bytes());
    assert_eq!(code_of(&meta_len), code::LIMIT_EXCEEDED);
    let mut unpacked = bytes.clone();
    unpacked[lengths + 12..lengths + 16].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(code_of(&unpacked), code::LIMIT_EXCEEDED);
    // Just above the limit of ADR-030 (512 MB): refused before unpacking.
    let mut above = bytes.clone();
    above[lengths + 12..lengths + 16]
        .copy_from_slice(&(openairside_core::savegame::MAX_UNPACKED_BYTES + 1).to_le_bytes());
    assert_eq!(code_of(&above), code::LIMIT_EXCEEDED);
    let mut world_len = bytes.clone();
    world_len[lengths + 8..lengths + 12].copy_from_slice(&(1u32 << 29).to_le_bytes());
    assert_eq!(code_of(&world_len), code::TRUNCATED);

    let mut flipped = bytes.clone();
    let middle = flipped.len() / 2;
    flipped[middle] ^= 1;
    assert_eq!(code_of(&flipped), code::CHECKSUM_MISMATCH);
}

#[test]
fn manipulated_content_with_a_valid_checksum_is_rejected_without_a_crash() {
    let bytes = save(&terminal(AdoptionMode::WhenReady));
    let info = read_info(
        &bytes[..MAX_INFO_BYTES.min(bytes.len())],
        bytes.len() as u64,
    )
    .unwrap();
    let meta_start = 18 + 16;
    let world_start = usize::try_from(info.preview_offset).unwrap() + info.preview_len as usize;

    // Metadata that is not JSON.
    let mut meta = bytes.clone();
    meta[meta_start] = b'#';
    fix_checksum(&mut meta);
    assert_eq!(code_of(&meta), code::INVALID_STATE);

    // World state that does not unpack.
    let mut world = bytes.clone();
    for b in &mut world[world_start..world_start + 16] {
        *b = 0xff;
    }
    fix_checksum(&mut world);
    assert_eq!(code_of(&world), code::INVALID_STATE);

    // Random single-byte changes anywhere after the identification never
    // crash; each is rejected or, rarely, still a valid game.
    let mut seed = 0x1234_5678_u64;
    for _ in 0..300 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let mut copy = bytes.clone();
        let at = 6 + usize::try_from(seed % (copy.len() as u64 - 14)).unwrap();
        copy[at] = copy[at].wrapping_add(u8::try_from(seed >> 56).unwrap() | 1);
        fix_checksum(&mut copy);
        let _ = decode(&copy, &TEST_LIMITS);
    }
}

#[test]
fn the_game_limits_allow_only_the_two_map_sizes() {
    let bytes = save(&terminal(AdoptionMode::WhenReady));
    let reason = decode(&bytes, &Limits::default()).err().unwrap();
    assert_eq!(reason.code, code::LIMIT_EXCEEDED);

    let map = MapSize::new(1536, 1536).unwrap();
    let mut cfg = config(AdoptionMode::WhenReady);
    cfg.map = map;
    let mut sim = Simulation::with_world(cfg, World::new(map)).unwrap();
    sim.world_mut()
        .build_mut()
        .grid_mut()
        .set(
            CellPos {
                x: 1,
                y: 1,
                floor: 1,
            },
            Cell::floor(Zone::Airside),
        )
        .unwrap();
    let before = sim.state_hash();
    let bytes = save(&sim);
    let mut loaded = Simulation::with_world(cfg, World::new(map)).unwrap();
    loaded
        .load_game(decode(&bytes, &Limits::default()).unwrap())
        .unwrap();
    assert_eq!(loaded.state_hash(), before);
}

#[test]
fn save_requests_are_checked() {
    let snapshot = terminal(AdoptionMode::WhenReady).save_snapshot();
    let check = |request: SaveRequest| {
        snapshot
            .encode(&request, "", "0.0.0")
            .err()
            .map(|r| (r.code, r.params.get("field").cloned()))
    };
    let text = |s: &str| Some(openairside_core::text::ParamValue::Text(s.into()));
    let mut long = request(SaveKind::Quick);
    long.name = "x".repeat(65);
    assert_eq!(
        check(long),
        Some((code::INVALID_ARGUMENT.into(), text("name")))
    );
    let mut control = request(SaveKind::Quick);
    control.name = "a\nb".into();
    assert_eq!(
        check(control),
        Some((code::INVALID_ARGUMENT.into(), text("name")))
    );
    let mut time = request(SaveKind::Auto);
    time.saved_at = "<script>".into();
    assert_eq!(
        check(time),
        Some((code::INVALID_ARGUMENT.into(), text("savedAt")))
    );
    let mut not_png = request(SaveKind::Auto);
    not_png.preview = b"GIF89a................".to_vec();
    assert_eq!(
        check(not_png).map(|c| c.0),
        Some(code::INVALID_STATE.into())
    );
    let mut large = request(SaveKind::Auto);
    large.preview = png(3840, 2160);
    assert_eq!(check(large).map(|c| c.0), Some(code::LIMIT_EXCEEDED.into()));
    let mut none = request(SaveKind::Auto);
    none.preview = Vec::new();
    assert!(check(none).is_none(), "a save game may have no preview");
    assert!(
        snapshot.encode(&request(SaveKind::Manual), "", "").is_err(),
        "the game version is required"
    );
    assert_eq!(savegame::EXTENSION, "oasave");
}
