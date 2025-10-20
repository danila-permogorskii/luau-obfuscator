//! Configuration management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the obfuscator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default API endpoint
    pub api_endpoint: Option<String>,

    /// Default obfuscation tier
    pub default_tier: String,

    /// Developer API key (for license generation)
    pub api_key: Option<String>,

    /// Cache directory
    pub cache_dir: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_endpoint: None,
            default_tier: "standard".to_string(),
            api_key: None,
            cache_dir: None,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> anyhow::Result<Self> {
        // TODO: Implement config loading from ~/.luau-obfuscator/config.toml
        Ok(Self::default())
    }

    /// Save configuration to file
    pub fn save(&self) -> anyhow::Result<()> {
        // TODO: Implement config saving
        Ok(())
    }
}
