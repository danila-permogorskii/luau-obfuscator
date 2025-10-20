//! Error types for the obfuscator

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObfuscatorError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Cryptography error: {0}")]
    CryptoError(String),

    #[error("Obfuscation error: {0}")]
    ObfuscationError(String),

    #[error("Code generation error: {0}")]
    CodeGenError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
