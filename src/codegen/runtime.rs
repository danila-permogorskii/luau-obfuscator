//! ChaCha20 runtime code generation

use crate::utils::errors::ObfuscatorError;
use anyhow::Result;

/// Generates Luau ChaCha20 runtime code
pub struct RuntimeGenerator;

impl RuntimeGenerator {
    pub fn new() -> Self {
        Self
    }
    
    /// Generate ChaCha20 runtime from template
    pub fn generate(&self) -> Result<String> {
        // Load template
        let template_path = std::env::current_dir()?
            .join("templates")
            .join("chacha20_runtime.lua");
        
        std::fs::read_to_string(&template_path)
            .map_err(|e| ObfuscatorError::CodeGenError(
                format!("Failed to load ChaCha20 runtime template: {}", e)
            ).into())
    }
    
    /// Generate optimized runtime (with minification)
    pub fn generate_optimized(&self) -> Result<String> {
        let runtime = self.generate()?;
        
        // Basic minification: remove comments and extra whitespace
        let lines: Vec<&str> = runtime
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("--") && !trimmed.is_empty()
            })
            .collect();
        
        Ok(lines.join("\n"))
    }
}

impl Default for RuntimeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires templates directory
    fn test_runtime_generation() {
        let generator = RuntimeGenerator::new();
        let runtime = generator.generate();
        assert!(runtime.is_ok());
    }
}