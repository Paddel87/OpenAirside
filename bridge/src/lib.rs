//! napi-rs binding of the OpenAirside simulation core (ADR-009, rule 006).
//!
//! Only translates between JavaScript and the public Rust API of
//! `openairside-core`; no game logic lives here.

use napi_derive::napi;

/// Identification of the loaded core, as seen from JavaScript.
#[napi(object)]
pub struct CoreInfo {
    /// Crate name of the core.
    pub name: String,
    /// Semantic version of the core.
    pub version: String,
}

/// Returns name and version of the simulation core.
#[napi]
#[must_use]
pub fn core_info() -> CoreInfo {
    let info = openairside_core::core_info();
    CoreInfo {
        name: info.name.to_owned(),
        version: info.version.to_owned(),
    }
}
