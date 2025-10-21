//! Code generation module - Assembles protected Luau scripts
//!
//! This module combines:
//! - ChaCha20 runtime (pure Luau)
//! - License validation logic
//! - HWID binding checks
//! - Encrypted data structures
//! - Obfuscated original code

mod assembly;
mod license;
mod runtime;
mod templates;

pub use assembly::ScriptAssembler;
pub use license::{LicenseConfig, LicenseValidator};
pub use runtime::RuntimeGenerator;
pub use templates::TemplateProcessor;

use crate::crypto::{CryptoContext, EncryptedData};
use crate::obfuscation::ObfuscationResult;
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Configuration for code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenConfig {
    /// License key for the protected script
    pub license_key: String,
    
    /// Script identifier
    pub script_id: String,
    
    /// API endpoint for license validation
    pub api_endpoint: String,
    
    /// Hardware ID binding (optional)
    pub hwid: Option<u64>,
    
    /// Place ID binding (optional)
    pub place_id: Option<u64>,
    
    /// Binding mode: "userid", "placeid", "both", "whitelist"
    pub binding_mode: String,
    
    /// Whitelist of authorized UserIds (for multi-user licenses)
    pub authorized_users: Vec<u64>,
    
    /// Include ChaCha20 runtime (for decryption)
    pub include_runtime: bool,
    
    /// Include license validation
    pub include_license: bool,
    
    /// Include HWID binding
    pub include_hwid: bool,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            license_key: String::new(),
            script_id: String::new(),
            api_endpoint: "https://api.example.com".to_string(),
            hwid: None,
            place_id: None,
            binding_mode: "userid".to_string(),
            authorized_users: Vec::new(),
            include_runtime: true,
            include_license: true,
            include_hwid: true,
        }
    }
}

/// Main code generator
pub struct CodeGenerator {
    config: CodeGenConfig,
    crypto: CryptoContext,
    template_processor: TemplateProcessor,
    runtime_generator: RuntimeGenerator,
    license_validator: LicenseValidator,
    assembler: ScriptAssembler,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new(config: CodeGenConfig, crypto: CryptoContext) -> Result<Self> {
        let template_processor = TemplateProcessor::new()?;
        let runtime_generator = RuntimeGenerator::new();
        let license_validator = LicenseValidator::new();
        let assembler = ScriptAssembler::new();
        
        Ok(Self {
            config,
            crypto,
            template_processor,
            runtime_generator,
            license_validator,
            assembler,
        })
    }
    
    /// Generate protected script from obfuscated code
    pub fn generate(
        &self,
        obfuscated: &ObfuscationResult,
        encrypted_strings: &[(String, EncryptedData)],
    ) -> Result<String> {
        // Generate watermark
        let watermark = self.crypto.generate_watermark(
            &self.config.license_key,
            &self.config.script_id,
        );
        
        // Generate components
        let runtime = if self.config.include_runtime {
            Some(self.runtime_generator.generate()?)
        } else {
            None
        };
        
        let license = if self.config.include_license {
            let license_config = LicenseConfig {
                license_key: self.config.license_key.clone(),
                script_id: self.config.script_id.clone(),
                api_endpoint: self.config.api_endpoint.clone(),
                watermark: watermark.to_string(),
            };
            Some(self.license_validator.generate(license_config)?)
        } else {
            None
        };
        
        let hwid = if self.config.include_hwid {
            Some(self.generate_hwid_binding()?)
        } else {
            None
        };
        
        // Assemble final script
        self.assembler.assemble(
            runtime,
            license,
            hwid,
            encrypted_strings,
            &obfuscated.code,
            watermark,
        )
    }
    
    /// Generate HWID binding code
    fn generate_hwid_binding(&self) -> Result<String> {
        let template = self.template_processor.load_template("hwid_binding.lua")?;
        
        let mut vars = std::collections::HashMap::new();
        vars.insert("BINDING_MODE".to_string(), self.config.binding_mode.clone());
        
        if let Some(hwid) = self.config.hwid {
            vars.insert("AUTHORIZED_USERID".to_string(), hwid.to_string());
        } else {
            vars.insert("AUTHORIZED_USERID".to_string(), "nil".to_string());
        }
        
        if let Some(place_id) = self.config.place_id {
            vars.insert("AUTHORIZED_PLACEID".to_string(), place_id.to_string());
        } else {
            vars.insert("AUTHORIZED_PLACEID".to_string(), "nil".to_string());
        }
        
        // Generate whitelist
        let whitelist = self
            .config
            .authorized_users
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        vars.insert("AUTHORIZED_USERS_LIST".to_string(), whitelist);
        
        self.template_processor.process(&template, vars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_config_default() {
        let config = CodeGenConfig::default();
        assert_eq!(config.binding_mode, "userid");
        assert!(config.include_runtime);
        assert!(config.include_license);
        assert!(config.include_hwid);
    }
}
