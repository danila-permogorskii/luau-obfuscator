//! Luau-specific parser implementation using full_moon

use super::ast::{FunctionInfo, NumericLiteral, ParseResult, StringLiteral};
use super::visitor::AstVisitor;
use anyhow::{Context, Result};
use full_moon::ast::Ast;
use full_moon::parse;
use log::debug;

/// Luau parser using full_moon
pub struct LuauParser {
    // Parser configuration could go here
}

impl LuauParser {
    /// Create a new Luau parser
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a Luau source string into an AST and extract information
    pub fn parse(&self, source: &str) -> Result<ParseResult> {
        debug!("Parsing Luau source ({} bytes)", source.len());

        // Parse the source using full_moon
        let ast = parse(source).context("Failed to parse Luau source")?;

        debug!("Parse successful, visiting AST nodes");

        // Create visitor and traverse AST
        let mut visitor = AstVisitor::new();
        visitor.visit_ast(&ast);

        // Extract results
        let result = ParseResult {
            ast: Some(ast),
            strings: visitor.strings,
            numbers: visitor.numbers,
            functions: visitor.functions,
        };

        debug!(
            "Extraction complete: {} strings, {} numbers, {} functions",
            result.strings.len(),
            result.numbers.len(),
            result.functions.len()
        );

        Ok(result)
    }
}

impl Default for LuauParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_script() {
        let parser = LuauParser::new();
        let source = r#"
            local x = 42
            print("hello")
        "#;

        let result = parser.parse(source).unwrap();
        assert!(result.numbers.len() > 0);
        assert!(result.strings.len() > 0);
    }

    #[test]
    fn test_parse_function() {
        let parser = LuauParser::new();
        let source = r#"
            function greet(name)
                return "Hello, " .. name
            end
        "#;

        let result = parser.parse(source).unwrap();
        assert_eq!(result.functions.len(), 1);
        assert_eq!(result.functions[0].name, Some("greet".to_string()));
    }

    #[test]
    fn test_parse_roblox_api() {
        let parser = LuauParser::new();
        let source = r#"
            local player = game.Players.LocalPlayer
            workspace.CurrentCamera.CFrame = CFrame.new(0, 10, 0)
        "#;

        let result = parser.parse(source);
        assert!(result.is_ok(), "Should parse Roblox API calls");
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let parser = LuauParser::new();
        let source = "function foo( { return";

        let result = parser.parse(source);
        assert!(result.is_err(), "Should fail on invalid syntax");
    }
}
