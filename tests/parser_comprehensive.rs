//! Comprehensive parser tests for all Luau features

use luau_obfuscator::parser::LuauParser;

mod helpers;
use helpers::*;

#[test]
fn test_parse_simple_script() {
    let parser = LuauParser::new();
    let source = load_fixture("simple.lua");
    
    let result = parser.parse(&source);
    assert!(result.is_ok(), "Failed to parse simple script");
    
    let parse_result = result.unwrap();
    assert!(parse_result.strings.len() > 0, "Should extract strings");
    assert!(parse_result.numbers.len() > 0, "Should extract numbers");
    assert!(parse_result.functions.len() > 0, "Should extract functions");
}

#[test]
fn test_parse_complex_script() {
    let parser = LuauParser::new();
    let source = load_fixture("complex.lua");
    
    let result = parser.parse(&source);
    assert!(result.is_ok(), "Failed to parse complex script");
    
    let parse_result = result.unwrap();
    
    // Complex script should have:
    // - Module constants (MODULE_VERSION, API_KEY, etc.)
    // - String literals
    // - Numeric literals
    // - Multiple functions
    assert!(parse_result.strings.len() >= 3, "Should extract multiple strings");
    assert!(parse_result.numbers.len() >= 2, "Should extract multiple numbers");
    assert!(parse_result.functions.len() >= 2, "Should extract multiple functions");
}

#[test]
fn test_parse_roblox_api_script() {
    let parser = LuauParser::new();
    let source = load_fixture("roblox_api.lua");
    
    let result = parser.parse(&source);
    assert!(result.is_ok(), "Failed to parse Roblox API script");
    
    let parse_result = result.unwrap();
    
    // Should detect Roblox service calls
    let has_service_strings = parse_result.strings.iter()
        .any(|s| s.value.contains("Service"));
    assert!(has_service_strings, "Should detect Roblox service strings");
}

#[test]
fn test_parse_type_annotations() {
    let parser = LuauParser::new();
    let source = load_fixture("type_annotations.lua");
    
    let result = parser.parse(&source);
    assert!(result.is_ok(), "Failed to parse type annotations script");
    
    let parse_result = result.unwrap();
    
    // Type annotations should not break parsing
    assert!(parse_result.functions.len() > 0, "Should extract functions with type annotations");
    assert!(parse_result.strings.len() > 0, "Should extract strings from typed script");
}

#[test]
fn test_parse_edge_cases() {
    let parser = LuauParser::new();
    let source = load_fixture("edge_cases.lua");
    
    let result = parser.parse(&source);
    assert!(result.is_ok(), "Failed to parse edge cases script");
    
    let parse_result = result.unwrap();
    
    // Edge cases include:
    // - Empty functions
    // - Unicode strings
    // - Very long identifiers
    // - Nested functions
    // - Special characters
    
    // Check unicode handling
    let has_unicode = parse_result.strings.iter()
        .any(|s| s.value.contains("\u{4e16}") || s.value.contains("世"));
    assert!(has_unicode, "Should handle unicode strings");
    
    // Check nested function detection
    assert!(parse_result.functions.len() >= 3, "Should detect nested functions");
}

#[test]
fn test_parse_malformed_script() {
    let parser = LuauParser::new();
    let source = load_fixture("malformed.lua");
    
    let result = parser.parse(&source);
    assert!(result.is_err(), "Should fail on malformed script");
}

#[test]
fn test_parse_empty_script() {
    let parser = LuauParser::new();
    let source = "";
    
    let result = parser.parse(source);
    assert!(result.is_ok(), "Should handle empty script");
    
    let parse_result = result.unwrap();
    assert_eq!(parse_result.strings.len(), 0);
    assert_eq!(parse_result.numbers.len(), 0);
    assert_eq!(parse_result.functions.len(), 0);
}

#[test]
fn test_parse_comments_only() {
    let parser = LuauParser::new();
    let source = r#"
        -- This is a comment
        --[[ Multiline
             comment block ]]
    "#;
    
    let result = parser.parse(source);
    assert!(result.is_ok(), "Should handle comments-only script");
}

#[test]
fn test_parse_numeric_literals() {
    let parser = LuauParser::new();
    let source = r#"
        local int = 42
        local float = 3.14
        local hex = 0xFF
        local scientific = 1.23e10
        local negative = -999
    "#;
    
    let result = parser.parse(source).unwrap();
    
    // Should extract all numeric types
    assert!(result.numbers.len() >= 5, "Should extract various numeric literals");
    
    // Check for float detection
    let has_float = result.numbers.iter().any(|n| n.is_float);
    assert!(has_float, "Should detect float literals");
}

#[test]
fn test_parse_string_literals() {
    let parser = LuauParser::new();
    let source = r#"
        local double = "double quoted"
        local single = 'single quoted'
        local empty = ""
        local multiline = [[ 
            multiline
            string
        ]]
    "#;
    
    let result = parser.parse(source).unwrap();
    
    // Should extract all string types
    assert!(result.strings.len() >= 3, "Should extract various string literals");
}

#[test]
fn test_parse_function_parameters() {
    let parser = LuauParser::new();
    let source = r#"
        function noParams()
        end
        
        function oneParam(x)
        end
        
        function multipleParams(a, b, c)
        end
        
        function varargs(...)
        end
    "#;
    
    let result = parser.parse(source).unwrap();
    
    assert_eq!(result.functions.len(), 4, "Should extract all function definitions");
    
    // Check parameter counts
    let no_params = result.functions.iter().find(|f| f.name.as_ref().map_or(false, |n| n == "noParams")).unwrap();
    assert_eq!(no_params.parameters.len(), 0);
    
    let one_param = result.functions.iter().find(|f| f.name.as_ref().map_or(false, |n| n == "oneParam")).unwrap();
    assert_eq!(one_param.parameters.len(), 1);
    
    let multi_params = result.functions.iter().find(|f| f.name.as_ref().map_or(false, |n| n == "multipleParams")).unwrap();
    assert_eq!(multi_params.parameters.len(), 3);
}

#[test]
fn test_parse_local_vs_global_functions() {
    let parser = LuauParser::new();
    let source = r#"
        function globalFunc()
        end
        
        local function localFunc()
        end
    "#;
    
    let result = parser.parse(source).unwrap();
    
    assert_eq!(result.functions.len(), 2);
    
    let global = result.functions.iter().find(|f| !f.is_local).unwrap();
    assert_eq!(global.name.as_ref().unwrap(), "globalFunc");
    
    let local = result.functions.iter().find(|f| f.is_local).unwrap();
    assert_eq!(local.name.as_ref().unwrap(), "localFunc");
}

#[test]
fn test_parse_performance() {
    let parser = LuauParser::new();
    let source = load_fixture("complex.lua");
    
    let perf = PerformanceMeasurement::start("parse_complex_script");
    let result = parser.parse(&source);
    let duration = perf.finish();
    
    assert!(result.is_ok());
    assert!(duration.as_millis() < 100, "Parsing should complete in <100ms");
}
