//! CLI module - Command-line interface implementation

mod args;
mod commands;

pub use args::Cli;

use anyhow::Result;
use clap::Parser;

/// Run the CLI application
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    commands::execute(cli)
}
