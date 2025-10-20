//! CLI argument definitions using clap

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Protect a Luau script with obfuscation and encryption
    Protect {
        /// Input Luau script file
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output file path
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,

        /// License key for the protected script
        #[arg(short, long, value_name = "KEY")]
        license_key: String,

        /// Hardware ID (Roblox UserId) to bind to
        #[arg(long, value_name = "HWID")]
        hwid: Option<u64>,

        /// Obfuscation tier: basic, standard, or premium
        #[arg(short, long, value_name = "TIER", default_value = "standard")]
        tier: ObfuscationTier,

        /// API endpoint for license validation
        #[arg(long, value_name = "URL")]
        api_endpoint: Option<String>,
    },

    /// Generate a new license key
    GenerateLicense {
        /// Unique script identifier
        #[arg(long, value_name = "ID")]
        script_id: String,

        /// Buyer's Roblox UserId
        #[arg(long, value_name = "USERID")]
        buyer_userid: u64,

        /// Developer API key
        #[arg(long, value_name = "KEY")]
        api_key: String,

        /// License expiration date (optional)
        #[arg(long, value_name = "DATE")]
        expiration: Option<String>,
    },

    /// Validate a protected script locally
    Validate {
        /// Protected script file to validate
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ObfuscationTier {
    /// Basic obfuscation (fast, light overhead)
    Basic,
    /// Standard obfuscation (balanced security and performance)
    Standard,
    /// Premium obfuscation (maximum security, higher overhead)
    Premium,
}

impl std::fmt::Display for ObfuscationTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObfuscationTier::Basic => write!(f, "basic"),
            ObfuscationTier::Standard => write!(f, "standard"),
            ObfuscationTier::Premium => write!(f, "premium"),
        }
    }
}
