//! AST definitions and data structures

use full_moon::ast::Ast;
use serde::{Deserialize, Serialize};

/// Result of parsing a Luau script
#[derive(Debug)]
pub struct ParseResult {
    /// The full AST (optional, for further processing)
    pub ast: Option<Ast>,
    /// Extracted string literals
    pub strings: Vec<StringLiteral>,
    /// Extracted numeric literals
    pub numbers: Vec<NumericLiteral>,
    /// Extracted function information
    pub functions: Vec<FunctionInfo>,
}

/// String literal found in the source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringLiteral {
    /// The string value (without quotes)
    pub value: String,
    /// Source location (line number)
    pub line: usize,
    /// Source location (column number)
    pub column: usize,
    /// Sensitivity classification
    pub sensitivity: Sensitivity,
}

/// Numeric literal found in the source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericLiteral {
    /// The numeric value
    pub value: String,
    /// Source location (line number)
    pub line: usize,
    /// Source location (column number)
    pub column: usize,
    /// Whether this is a float
    pub is_float: bool,
}

/// Function information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Function name (if named)
    pub name: Option<String>,
    /// Parameter names
    pub parameters: Vec<String>,
    /// Source location (line number)
    pub line: usize,
    /// Whether this is a local function
    pub is_local: bool,
}

/// Sensitivity classification for strings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sensitivity {
    /// High sensitivity (API keys, RemoteEvent names, etc.)
    High,
    /// Medium sensitivity (UI text, config values)
    Medium,
    /// Low sensitivity (debug messages, comments)
    Low,
}

impl Sensitivity {
    /// Classify a string's sensitivity based on heuristics
    pub fn classify(value: &str) -> Self {
        // High sensitivity patterns
        if value.contains("Remote")
            || value.contains("Event")
            || value.contains("Function")
            || value.contains("API")
            || value.contains("Key")
            || value.contains("Secret")
            || value.contains("Token")
        {
            return Sensitivity::High;
        }

        // Low sensitivity patterns
        if value.starts_with("[")
            || value.starts_with("Debug:")
            || value.starts_with("Warning:")
            || value.starts_with("Error:")
        {
            return Sensitivity::Low;
        }

        // Default to medium
        Sensitivity::Medium
    }
}
