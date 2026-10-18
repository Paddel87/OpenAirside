//! Simulation core of OpenAirside.
//!
//! Holds the complete game logic and knows nothing about rendering or about
//! JavaScript (rule 006 in `docs/decisions.md`). Step 2.2 provides the tick
//! with time control, the command channel S-2, the event stream S-3 and the
//! verification mode; the world grid and flow fields follow in step 2.3,
//! the data pack loader (S-4) in step 2.4.

pub mod adoption;
pub mod command;
pub mod datapack;
pub mod event;
pub mod flowfield;
pub mod frame;
pub mod geometry;
pub mod grid;
pub mod hash;
pub mod host;
pub mod network;
pub mod pacer;
pub mod port;
pub mod query;
pub mod runner;
pub mod scenario;
pub mod simulation;
pub mod target;
pub mod text;
pub mod time;
pub mod world;

/// Version of the simulation core, taken from the workspace manifest.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Identification of the core, reported to the presentation on start-up.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CoreInfo {
    /// Crate name of the core.
    pub name: &'static str,
    /// Semantic version of the core.
    pub version: &'static str,
}

/// Returns the identification of this core build.
#[must_use]
pub fn core_info() -> CoreInfo {
    CoreInfo {
        name: env!("CARGO_PKG_NAME"),
        version: VERSION,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_info_reports_crate_name_and_version() {
        let info = core_info();
        assert_eq!(info.name, "openairside-core");
        assert_eq!(info.version, VERSION);
        assert!(!info.version.is_empty());
    }
}
