//! Scope analysis and variable tracking

use crate::parser::{ParseResult, FunctionInfo};
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use std::collections::HashMap;

/// Type of variable
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableType {
    Local,
    Global,
    Parameter,
    Function,
}

/// Variable information
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: VariableType,
    pub line: usize,
    pub can_rename: bool,
}

/// Scope information
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: usize,
    pub parent: Option<usize>,
    pub variables: HashMap<String, Variable>,
    pub children: Vec<usize>,
}

/// Analyzes variable scopes
pub struct ScopeAnalyzer {
    scopes: Vec<Scope>,
    current_scope: usize,
    next_scope_id: usize,
}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        // Create global scope
        let global_scope = Scope {
            id: 0,
            parent: None,
            variables: HashMap::new(),
            children: Vec::new(),
        };

        Self {
            scopes: vec![global_scope],
            current_scope: 0,
            next_scope_id: 1,
        }
    }

    /// Analyze scopes in parsed code
    pub fn analyze(&self, parse_result: &ParseResult) -> Result<Vec<Scope>> {
        let mut analyzer = Self::new();

        // Process all functions to create scope hierarchy
        for func in &parse_result.functions {
            analyzer.process_function(func)?;
        }

        Ok(analyzer.scopes)
    }

    fn process_function(&mut self, func: &FunctionInfo) -> Result<()> {
        // Create new scope for function
        let scope_id = self.enter_scope();

        // Add parameters as local variables
        for param in &func.parameters {
            self.add_variable(Variable {
                name: param.clone(),
                var_type: VariableType::Parameter,
                line: func.line,
                can_rename: true,
            })?;
        }

        // Add function name if it exists
        if let Some(name) = &func.name {
            let parent_scope = self.scopes[self.current_scope].parent.unwrap();
            self.scopes[parent_scope].variables.insert(
                name.clone(),
                Variable {
                    name: name.clone(),
                    var_type: VariableType::Function,
                    line: func.line,
                    can_rename: !func.is_local, // Local functions can be renamed
                },
            );
        }

        self.exit_scope();
        Ok(())
    }

    fn enter_scope(&mut self) -> usize {
        let new_scope = Scope {
            id: self.next_scope_id,
            parent: Some(self.current_scope),
            variables: HashMap::new(),
            children: Vec::new(),
        };

        let new_id = self.next_scope_id;
        self.next_scope_id += 1;

        self.scopes[self.current_scope].children.push(new_id);
        self.scopes.push(new_scope);
        self.current_scope = new_id;

        new_id
    }

    fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    fn add_variable(&mut self, var: Variable) -> Result<()> {
        self.scopes[self.current_scope]
            .variables
            .insert(var.name.clone(), var);
        Ok(())
    }

    /// Find a variable in the scope hierarchy
    pub fn find_variable(&self, name: &str, scope_id: usize) -> Option<&Variable> {
        let scope = &self.scopes[scope_id];

        // Check current scope
        if let Some(var) = scope.variables.get(name) {
            return Some(var);
        }

        // Check parent scope recursively
        if let Some(parent_id) = scope.parent {
            return self.find_variable(name, parent_id);
        }

        None
    }
}

impl Default for ScopeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_scope_created() {
        let analyzer = ScopeAnalyzer::new();
        assert_eq!(analyzer.scopes.len(), 1);
        assert_eq!(analyzer.scopes[0].id, 0);
        assert!(analyzer.scopes[0].parent.is_none());
    }

    #[test]
    fn test_enter_exit_scope() {
        let mut analyzer = ScopeAnalyzer::new();
        
        let scope1 = analyzer.enter_scope();
        assert_eq!(scope1, 1);
        assert_eq!(analyzer.current_scope, 1);
        
        let scope2 = analyzer.enter_scope();
        assert_eq!(scope2, 2);
        assert_eq!(analyzer.current_scope, 2);
        
        analyzer.exit_scope();
        assert_eq!(analyzer.current_scope, 1);
        
        analyzer.exit_scope();
        assert_eq!(analyzer.current_scope, 0);
    }

    #[test]
    fn test_add_variable() {
        let mut analyzer = ScopeAnalyzer::new();
        
        let var = Variable {
            name: "test".to_string(),
            var_type: VariableType::Local,
            line: 1,
            can_rename: true,
        };
        
        analyzer.add_variable(var).unwrap();
        assert!(analyzer.scopes[0].variables.contains_key("test"));
    }
}
