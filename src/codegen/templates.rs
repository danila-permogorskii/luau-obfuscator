//! Template processing system

use crate::utils::errors::ObfuscatorError;
use anyhow::{Context, Result};
use std::collections::HashMap;

/// Template processor for Luau code generation
pub struct TemplateProcessor {
    templates_dir: std::path::PathBuf,
}

impl TemplateProcessor {
    pub fn new() -> Result<Self> {
        // Templates are in the templates/ directory at project root
        let templates_dir = std::env::current_dir()?
            .join("templates");
        
        if !templates_dir.exists() {
            return Err(ObfuscatorError::CodeGenError(
                format!("Templates directory not found: {:?}", templates_dir)
            ).into());
        }
        
        Ok(Self { templates_dir })
    }
    
    /// Load a template file
    pub fn load_template(&self, name: &str) -> Result<String> {
        let path = self.templates_dir.join(name);
        std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to load template: {:?}", path))
    }
    
    /// Process template with variable substitution
    pub fn process(&self, template: &str, vars: HashMap<String, String>) -> Result<String> {
        let mut result = template.to_string();
        
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, &value);
        }
        
        // Check for unprocessed placeholders
        if result.contains("{{") && result.contains("}}") {
            log::warn!("Template contains unprocessed placeholders");
        }
        
        Ok(result)
    }
}

impl Default for TemplateProcessor {
    fn default() -> Self {
        Self::new().expect("Failed to create TemplateProcessor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_substitution() {
        let processor = TemplateProcessor::new().unwrap();
        let template = "Hello {{NAME}}, you are {{AGE}} years old";
        
        let mut vars = HashMap::new();
        vars.insert("NAME".to_string(), "Alice".to_string());
        vars.insert("AGE".to_string(), "30".to_string());
        
        let result = processor.process(template, vars).unwrap();
        assert_eq!(result, "Hello Alice, you are 30 years old");
    }
}