//! Obfuscation transformations module
//!
//! Implements various obfuscation techniques:
//! - String encryption
//! - Constant obfuscation
//! - Name mangling
//! - Control flow flattening
//! - Dead code injection

mod constants;
mod controlflow;
mod deadcode;
mod names;
mod strings;

pub use constants::ConstantObfuscator;
pub use controlflow::ControlFlowFlattener;
pub use deadcode::DeadCodeInjector;
pub use names::NameMangler;
pub use strings::StringObfuscator;

use crate::analysis::AnalysisResult;
use crate::crypto::CryptoContext;
use crate::parser::ParseResult;
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;

/// Obfuscation tier levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObfuscationTier {
    /// Basic obfuscation (fast, light overhead)
    Basic,
    /// Standard obfuscation (balanced)
    Standard,
    /// Premium obfuscation (maximum security)
    Premium,
}

impl ObfuscationTier {
    /// Get recommended settings for this tier
    pub fn settings(&self) -> ObfuscationSettings {
        match self {
            ObfuscationTier::Basic => ObfuscationSettings {
                encrypt_strings: true,
                encrypt_all_strings: false,
                obfuscate_constants: false,
                mangle_names: true,
                mangle_functions: false,
                flatten_control_flow: false,
                inject_dead_code: false,
                dead_code_density: 0.0,
            },
            ObfuscationTier::Standard => ObfuscationSettings {
                encrypt_strings: true,
                encrypt_all_strings: true,
                obfuscate_constants: true,
                mangle_names: true,
                mangle_functions: true,
                flatten_control_flow: true,
                inject_dead_code: true,
                dead_code_density: 0.1,
            },
            ObfuscationTier::Premium => ObfuscationSettings {
                encrypt_strings: true,
                encrypt_all_strings: true,
                obfuscate_constants: true,
                mangle_names: true,
                mangle_functions: true,
                flatten_control_flow: true,
                inject_dead_code: true,
                dead_code_density: 0.3,
            },
        }
    }
}

/// Obfuscation settings
#[derive(Debug, Clone)]
pub struct ObfuscationSettings {
    pub encrypt_strings: bool,
    pub encrypt_all_strings: bool,
    pub obfuscate_constants: bool,
    pub mangle_names: bool,
    pub mangle_functions: bool,
    pub flatten_control_flow: bool,
    pub inject_dead_code: bool,
    pub dead_code_density: f32,
}

/// Main obfuscator coordinator
pub struct Obfuscator {
    tier: ObfuscationTier,
    settings: ObfuscationSettings,
    crypto_ctx: CryptoContext,
}

impl Obfuscator {
    /// Create new obfuscator with tier and crypto context
    pub fn new(tier: ObfuscationTier, crypto_ctx: CryptoContext) -> Self {
        let settings = tier.settings();
        Self {
            tier,
            settings,
            crypto_ctx,
        }
    }

    /// Apply all obfuscation transformations
    pub fn obfuscate(
        &self,
        parse_result: &ParseResult,
        analysis: &AnalysisResult,
    ) -> Result<ObfuscatedScript> {
        log::info!("Starting obfuscation with tier: {:?}", self.tier);

        // Step 1: String obfuscation
        let mut obfuscated = ObfuscatedScript::new();
        
        if self.settings.encrypt_strings {
            log::debug!("Encrypting strings...");
            let string_obfuscator = StringObfuscator::new(&self.crypto_ctx);
            obfuscated.encrypted_strings = string_obfuscator.obfuscate(
                &parse_result.strings,
                self.settings.encrypt_all_strings,
            )?;
        }

        // Step 2: Constant obfuscation
        if self.settings.obfuscate_constants {
            log::debug!("Obfuscating constants...");
            let const_obfuscator = ConstantObfuscator::new();
            obfuscated.obfuscated_constants = const_obfuscator.obfuscate(&parse_result.numbers)?;
        }

        // Step 3: Name mangling
        if self.settings.mangle_names {
            log::debug!("Mangling names...");
            let name_mangler = NameMangler::new(
                &analysis.preserved_identifiers,
                self.settings.mangle_functions,
            );
            obfuscated.name_mappings = name_mangler.generate_mappings(analysis)?;
        }

        // Step 4: Control flow flattening
        if self.settings.flatten_control_flow {
            log::debug!("Flattening control flow...");
            let cf_flattener = ControlFlowFlattener::new();
            obfuscated.flattened_blocks = cf_flattener.flatten(&analysis.control_flow)?;
        }

        // Step 5: Dead code injection
        if self.settings.inject_dead_code {
            log::debug!("Injecting dead code...");
            let dead_code_injector = DeadCodeInjector::new(self.settings.dead_code_density);
            obfuscated.dead_code_snippets = dead_code_injector.generate(parse_result)?;
        }

        log::info!("Obfuscation complete");
        Ok(obfuscated)
    }

    /// Get obfuscation statistics
    pub fn get_stats(&self, obfuscated: &ObfuscatedScript) -> ObfuscationStats {
        ObfuscationStats {
            tier: self.tier,
            encrypted_strings: obfuscated.encrypted_strings.len(),
            obfuscated_constants: obfuscated.obfuscated_constants.len(),
            renamed_identifiers: obfuscated.name_mappings.len(),
            flattened_blocks: obfuscated.flattened_blocks.len(),
            dead_code_snippets: obfuscated.dead_code_snippets.len(),
        }
    }
}

/// Obfuscated script data
#[derive(Debug, Clone)]
pub struct ObfuscatedScript {
    pub encrypted_strings: Vec<EncryptedString>,
    pub obfuscated_constants: Vec<ObfuscatedConstant>,
    pub name_mappings: std::collections::HashMap<String, String>,
    pub flattened_blocks: Vec<FlattenedBlock>,
    pub dead_code_snippets: Vec<String>,
}

impl ObfuscatedScript {
    fn new() -> Self {
        Self {
            encrypted_strings: Vec::new(),
            obfuscated_constants: Vec::new(),
            name_mappings: std::collections::HashMap::new(),
            flattened_blocks: Vec::new(),
            dead_code_snippets: Vec::new(),
        }
    }
}

/// Encrypted string with metadata
#[derive(Debug, Clone)]
pub struct EncryptedString {
    pub original: String,
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub line: usize,
    pub id: String,
}

/// Obfuscated constant
#[derive(Debug, Clone)]
pub struct ObfuscatedConstant {
    pub original: String,
    pub obfuscated_expr: String,
    pub line: usize,
}

/// Flattened control flow block
#[derive(Debug, Clone)]
pub struct FlattenedBlock {
    pub block_id: usize,
    pub state_machine_code: String,
}

/// Obfuscation statistics
#[derive(Debug, Clone)]
pub struct ObfuscationStats {
    pub tier: ObfuscationTier,
    pub encrypted_strings: usize,
    pub obfuscated_constants: usize,
    pub renamed_identifiers: usize,
    pub flattened_blocks: usize,
    pub dead_code_snippets: usize,
}

impl std::fmt::Display for ObfuscationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Obfuscation Statistics (Tier: {:?}):\n\
             - Encrypted strings: {}\n\
             - Obfuscated constants: {}\n\
             - Renamed identifiers: {}\n\
             - Flattened blocks: {}\n\
             - Dead code snippets: {}",
            self.tier,
            self.encrypted_strings,
            self.obfuscated_constants,
            self.renamed_identifiers,
            self.flattened_blocks,
            self.dead_code_snippets
        )
    }
}
