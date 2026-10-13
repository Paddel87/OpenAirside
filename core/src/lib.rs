//! Simulation core of OpenAirside.
//!
//! Holds the complete game logic and knows nothing about rendering or about
//! JavaScript (rule 006 in `docs/decisions.md`). Roadmap step 2.1 only
//! provides the crate skeleton; the tick, world state and commands follow in
//! steps 2.2 and 2.3.

/// Version of the simulation core, taken from the workspace manifest.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Identification of the core, reported to the presentation on start-up.
#[derive(Debug, Clone, PartialEq, Eq)]
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
