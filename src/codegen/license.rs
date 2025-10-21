//! License validation code generation

use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// License configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseConfig {
    pub license_key: String,
    pub script_id: String,
    pub api_endpoint: String,
    pub watermark: String,
}

/// Generates license validation code
pub struct LicenseValidator;

impl LicenseValidator {
    pub fn new() -> Self {
        Self
    }
    
    /// Generate license validation code from template
    pub fn generate(&self, config: LicenseConfig) -> Result<String> {
        // Load template
        let template_path = std::env::current_dir()?
            .join("templates")
            .join("license_validation.lua");
        
        let template = std::fs::read_to_string(&template_path)
            .map_err(|e| ObfuscatorError::CodeGenError(
                format!("Failed to load license validation template: {}", e)
            ))?;
        
        // Process template variables
        let mut vars = HashMap::new();
        vars.insert("LICENSE_KEY".to_string(), config.license_key);
        vars.insert("SCRIPT_ID".to_string(), config.script_id);
        vars.insert("API_ENDPOINT".to_string(), config.api_endpoint);
        vars.insert("WATERMARK".to_string(), config.watermark);
        
        let mut result = template;
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, &value);
        }
        
        Ok(result)
    }
}

impl Default for LicenseValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires templates directory
    fn test_license_generation() {
        let validator = LicenseValidator::new();
        let config = LicenseConfig {
            license_key: "TEST-1234-5678-9012".to_string(),
            script_id: "test_script".to_string(),
            api_endpoint: "https://api.example.com".to_string(),
            watermark: "abc123".to_string(),
        };
        
        let license = validator.generate(config);
        assert!(license.is_ok());
    }
}