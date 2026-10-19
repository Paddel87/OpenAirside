//! `cargo run -p openairside-assets --release [-- <repository root>]`: builds the
//! texture atlases and the program icon from the SVG sources (roadmap step
//! 2.10, ADR-033). Called by `npm run build:assets`; the root defaults to the
//! current folder.

use std::path::PathBuf;
use std::process::ExitCode;

use openairside_assets::{Layout, run};

fn main() -> ExitCode {
    let root = std::env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("."), PathBuf::from);
    match run(&Layout::repository(&root)) {
        Ok(report) => {
            println!(
                "assets: {} graphics, {} atlas pages, {} bytes",
                report.sources, report.pages, report.bytes
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("assets: {e}");
            ExitCode::FAILURE
        }
    }
}
