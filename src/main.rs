//! Luau Obfuscator CLI Tool
//!
//! A commercial-grade CLI tool for obfuscating Luau/Roblox scripts with
//! cryptographic protection and license management.

mod analysis;
mod cli;
mod parser;
mod utils;

use anyhow::Result;
use env_logger::Env;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Parse CLI arguments and execute command
    cli::run()
}
