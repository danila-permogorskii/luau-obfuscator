//! Analysis engine module - Code analysis and metadata extraction

mod controlflow;
mod roblox;
mod scope;

pub use controlflow::{ControlFlowAnalyzer, ControlFlowGraph, BasicBlock};
pub use roblox::{RobloxApiDetector, RobloxApiType};
pub use scope::{ScopeAnalyzer, Scope, Variable, VariableType};

use crate::parser::ParseResult;
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;

/// Complete analysis result
#[derive(Debug)]
pub struct AnalysisResult {
    /// Control flow graph
    pub control_flow: ControlFlowGraph,
    /// Scope hierarchy
    pub scopes: Vec<Scope>,
    /// Detected Roblox API usage
    pub roblox_apis: Vec<RobloxApiType>,
    /// Variables that should be preserved (Roblox globals, etc.)
    pub preserved_identifiers: Vec<String>,
}

/// Main analyzer that coordinates all analysis passes
pub struct Analyzer {
    preserve_roblox_apis: bool,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            preserve_roblox_apis: true,
        }
    }

    /// Run all analysis passes on parsed code
    pub fn analyze(&self, parse_result: &ParseResult) -> Result<AnalysisResult> {
        // Detect Roblox API usage
        let mut roblox_detector = RobloxApiDetector::new();
        let roblox_apis = roblox_detector.detect(parse_result)?;

        // Build preserved identifiers list
        let mut preserved_identifiers = Vec::new();
        if self.preserve_roblox_apis {
            preserved_identifiers.extend(roblox_detector.get_preserved_names());
        }

        // Analyze scopes
        let scope_analyzer = ScopeAnalyzer::new();
        let scopes = scope_analyzer.analyze(parse_result)?;

        // Build control flow graph
        let cf_analyzer = ControlFlowAnalyzer::new();
        let control_flow = cf_analyzer.analyze(parse_result)?;

        Ok(AnalysisResult {
            control_flow,
            scopes,
            roblox_apis,
            preserved_identifiers,
        })
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
