//! Identifier name mangling

use crate::analysis::AnalysisResult;
use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

/// Name mangler for identifier obfuscation
pub struct NameMangler {
    preserved_names: Vec<String>,
    mangle_functions: bool,
    counter: usize,
}

impl NameMangler {
    pub fn new(preserved_names: &[String], mangle_functions: bool) -> Self {
        Self {
            preserved_names: preserved_names.to_vec(),
            mangle_functions,
            counter: 0,
        }
    }

    /// Generate name mappings for all identifiers
    pub fn generate_mappings(&mut self, analysis: &AnalysisResult) -> Result<HashMap<String, String>> {
        let mut mappings = HashMap::new();

        // Collect all identifiers from scopes
        for scope in &analysis.scopes {
            for (var_name, var) in &scope.variables {
                // Skip preserved identifiers (Roblox APIs, etc.)
                if self.should_preserve(var_name) {
                    continue;
                }

                // Skip function names if not mangling functions
                if !self.mangle_functions
                    && matches!(
                        var.var_type,
                        crate::analysis::VariableType::Function
                    )
                {
                    continue;
                }

                // Generate mangled name if not already mapped
                if !mappings.contains_key(var_name) {
                    let mangled = self.generate_mangled_name();
                    mappings.insert(var_name.clone(), mangled);
                }
            }
        }

        log::debug!("Generated {} name mappings", mappings.len());
        Ok(mappings)
    }

    /// Check if a name should be preserved
    fn should_preserve(&self, name: &str) -> bool {
        self.preserved_names.contains(&name.to_string())
    }

    /// Generate a mangled name
    fn generate_mangled_name(&mut self) -> String {
        // Use short hex-style identifiers for compactness
        let mangled = format!("_0x{:x}", self.counter);
        self.counter += 1;
        mangled
    }

    /// Generate random-style mangled name (alternative strategy)
    #[allow(dead_code)]
    fn generate_random_name() -> String {
        let random_str: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        
        format!("_{}", random_str)
    }

    /// Generate dictionary-style mangled name (uses common short words)
    #[allow(dead_code)]
    fn generate_dict_name(&mut self) -> String {
        const DICT: &[&str] = &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
            "aa", "ab", "ac", "ad", "ae", "af", "ag", "ah",
        ];
        
        let idx = self.counter % DICT.len();
        self.counter += 1;
        
        format!("_{}", DICT[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::{Scope, Variable, VariableType};

    fn create_test_analysis() -> AnalysisResult {
        let mut scope = Scope {
            id: 0,
            parent: None,
            variables: HashMap::new(),
            children: Vec::new(),
        };

        scope.variables.insert(
            "myVar".to_string(),
            Variable {
                name: "myVar".to_string(),
                var_type: VariableType::Local,
                line: 1,
                can_rename: true,
            },
        );

        scope.variables.insert(
            "game".to_string(), // Should be preserved
            Variable {
                name: "game".to_string(),
                var_type: VariableType::Global,
                line: 1,
                can_rename: false,
            },
        );

        AnalysisResult {
            control_flow: crate::analysis::ControlFlowGraph {
                blocks: HashMap::new(),
                entry_block: 0,
                exit_blocks: vec![],
            },
            scopes: vec![scope],
            roblox_apis: vec![],
            preserved_identifiers: vec!["game".to_string()],
        }
    }

    #[test]
    fn test_name_mangling() {
        let preserved = vec!["game".to_string(), "workspace".to_string()];
        let mut mangler = NameMangler::new(&preserved, true);
        
        let analysis = create_test_analysis();
        let mappings = mangler.generate_mappings(&analysis).unwrap();
        
        // Should mangle myVar
        assert!(mappings.contains_key("myVar"));
        assert!(mappings["myVar"].starts_with("_0x"));
        
        // Should NOT mangle game (preserved)
        assert!(!mappings.contains_key("game"));
    }

    #[test]
    fn test_sequential_naming() {
        let mut mangler = NameMangler::new(&[], true);
        
        let name1 = mangler.generate_mangled_name();
        let name2 = mangler.generate_mangled_name();
        let name3 = mangler.generate_mangled_name();
        
        assert_eq!(name1, "_0x0");
        assert_eq!(name2, "_0x1");
        assert_eq!(name3, "_0x2");
    }

    #[test]
    fn test_function_mangling_control() {
        // Test with function mangling disabled
        let mangler = NameMangler::new(&[], false);
        assert!(!mangler.mangle_functions);
        
        // Test with function mangling enabled
        let mangler = NameMangler::new(&[], true);
        assert!(mangler.mangle_functions);
    }

    #[test]
    fn test_random_name_generation() {
        let name1 = NameMangler::generate_random_name();
        let name2 = NameMangler::generate_random_name();
        
        assert!(name1.starts_with('_'));
        assert!(name2.starts_with('_'));
        assert_ne!(name1, name2); // Should be different (very high probability)
    }
}