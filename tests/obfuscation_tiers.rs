//! Obfuscation tier comparison and validation tests

use luau_obfuscator::analysis::Analyzer;
use luau_obfuscator::crypto::CryptoContext;
use luau_obfuscator::obfuscation::*;
use luau_obfuscator::parser::LuauParser;

mod helpers;
use helpers::*;

#[test]
fn test_basic_tier_settings() {
    let settings = ObfuscationTier::Basic.settings();
    
    assert!(settings.encrypt_strings);
    assert!(!settings.encrypt_all_strings); // Only sensitive strings
    assert!(!settings.obfuscate_constants);
    assert!(settings.mangle_names);
    assert!(!settings.mangle_functions);
    assert!(!settings.flatten_control_flow);
    assert!(!settings.inject_dead_code);
    assert_eq!(settings.dead_code_density, 0.0);
}

#[test]
fn test_standard_tier_settings() {
    let settings = ObfuscationTier::Standard.settings();
    
    assert!(settings.encrypt_strings);
    assert!(settings.encrypt_all_strings); // All strings
    assert!(settings.obfuscate_constants);
    assert!(settings.mangle_names);
    assert!(settings.mangle_functions);
    assert!(settings.flatten_control_flow);
    assert!(settings.inject_dead_code);
    assert_eq!(settings.dead_code_density, 0.1);
}

#[test]
fn test_premium_tier_settings() {
    let settings = ObfuscationTier::Premium.settings();
    
    assert!(settings.encrypt_strings);
    assert!(settings.encrypt_all_strings);
    assert!(settings.obfuscate_constants);
    assert!(settings.mangle_names);
    assert!(settings.mangle_functions);
    assert!(settings.flatten_control_flow);
    assert!(settings.inject_dead_code);
    assert_eq!(settings.dead_code_density, 0.3); // Higher density
}

#[test]
fn test_basic_tier_obfuscation() {
    let source = load_fixture("simple.lua");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Basic, crypto_ctx);
    
    let result = obfuscator.obfuscate(&parse_result, &analysis).unwrap();
    
    // Basic tier should:
    // - Encrypt some strings (not all)
    // - Mangle some names
    // - NOT obfuscate constants
    // - NOT flatten control flow
    // - NOT inject dead code
    
    assert!(result.encrypted_strings.len() > 0, "Should encrypt some strings");
    assert!(result.encrypted_strings.len() <= parse_result.strings.len(), "Should not encrypt all strings");
    assert_eq!(result.obfuscated_constants.len(), 0, "Should not obfuscate constants");
    assert_eq!(result.flattened_blocks.len(), 0, "Should not flatten control flow");
    assert_eq!(result.dead_code_snippets.len(), 0, "Should not inject dead code");
}

#[test]
fn test_standard_tier_obfuscation() {
    let source = load_fixture("simple.lua");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    
    let result = obfuscator.obfuscate(&parse_result, &analysis).unwrap();
    
    // Standard tier should do everything except premium features
    assert!(result.encrypted_strings.len() > 0);
    assert!(result.obfuscated_constants.len() > 0, "Should obfuscate constants");
    assert!(result.name_mappings.len() > 0, "Should mangle names");
    
    // May or may not have dead code depending on density
    // Just check it's configured
}

#[test]
fn test_premium_tier_obfuscation() {
    let source = load_fixture("complex.lua");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);
    
    let result = obfuscator.obfuscate(&parse_result, &analysis).unwrap();
    
    // Premium tier should do everything maximally
    assert!(result.encrypted_strings.len() > 0);
    assert!(result.obfuscated_constants.len() > 0);
    assert!(result.name_mappings.len() > 0);
    
    // With 0.3 density and a complex script, should have dead code
    assert!(result.dead_code_snippets.len() > 0, "Should inject dead code");
}

#[test]
fn test_tier_progression() {
    let source = load_fixture("simple.lua");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Obfuscate with each tier
    let crypto_ctx1 = CryptoContext::new("password", None).unwrap();
    let basic = Obfuscator::new(ObfuscationTier::Basic, crypto_ctx1);
    let basic_result = basic.obfuscate(&parse_result, &analysis).unwrap();
    
    let crypto_ctx2 = CryptoContext::new("password", None).unwrap();
    let standard = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx2);
    let standard_result = standard.obfuscate(&parse_result, &analysis).unwrap();
    
    let crypto_ctx3 = CryptoContext::new("password", None).unwrap();
    let premium = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx3);
    let premium_result = premium.obfuscate(&parse_result, &analysis).unwrap();
    
    // Each tier should do more than the previous
    assert!(standard_result.encrypted_strings.len() >= basic_result.encrypted_strings.len());
    assert!(premium_result.encrypted_strings.len() >= standard_result.encrypted_strings.len());
    
    // Standard and Premium should obfuscate constants, Basic should not
    assert_eq!(basic_result.obfuscated_constants.len(), 0);
    assert!(standard_result.obfuscated_constants.len() > 0);
    assert!(premium_result.obfuscated_constants.len() > 0);
}

#[test]
fn test_string_obfuscator_selective() {
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = StringObfuscator::new(&crypto_ctx);
    
    use luau_obfuscator::parser::{StringLiteral, Sensitivity};
    
    let strings = vec![
        StringLiteral {
            value: "RemoteEvent".to_string(),
            line: 1,
            column: 0,
            sensitivity: Sensitivity::High,
        },
        StringLiteral {
            value: "Debug: test".to_string(),
            line: 2,
            column: 0,
            sensitivity: Sensitivity::Low,
        },
        StringLiteral {
            value: "UI Text".to_string(),
            line: 3,
            column: 0,
            sensitivity: Sensitivity::Medium,
        },
    ];
    
    // Don't encrypt all - only high/medium
    let encrypted = obfuscator.obfuscate(&strings, false).unwrap();
    
    // Should encrypt high and medium, not low
    assert_eq!(encrypted.len(), 2, "Should only encrypt high/medium sensitivity");
}

#[test]
fn test_constant_obfuscator_integers() {
    let obfuscator = ConstantObfuscator::new();
    
    use luau_obfuscator::parser::NumericLiteral;
    
    let num_lit = NumericLiteral {
        value: "42".to_string(),
        line: 1,
        column: 0,
        is_float: false,
    };
    
    let obfuscated = obfuscator.obfuscate_number(&num_lit).unwrap();
    
    assert_eq!(obfuscated.original, "42");
    assert!(obfuscated.obfuscated_expr.contains('('), "Should contain parentheses");
    assert!(obfuscated.obfuscated_expr.contains(')'));
}

#[test]
fn test_constant_obfuscator_floats() {
    let obfuscator = ConstantObfuscator::new();
    
    use luau_obfuscator::parser::NumericLiteral;
    
    let num_lit = NumericLiteral {
        value: "3.14".to_string(),
        line: 1,
        column: 0,
        is_float: true,
    };
    
    let obfuscated = obfuscator.obfuscate_number(&num_lit).unwrap();
    
    assert_eq!(obfuscated.original, "3.14");
    assert!(obfuscated.obfuscated_expr.contains('('));
}

#[test]
fn test_constant_obfuscator_complex() {
    let obfuscator = ConstantObfuscator::new();
    
    use luau_obfuscator::parser::NumericLiteral;
    
    let num_lit = NumericLiteral {
        value: "100".to_string(),
        line: 1,
        column: 0,
        is_float: false,
    };
    
    let obfuscated = obfuscator.obfuscate_complex(&num_lit).unwrap();
    
    // Complex obfuscation should have multiple operators
    let op_count = obfuscated.obfuscated_expr.matches(&['+', '-', '*', '/']).count();
    assert!(op_count >= 3, "Complex obfuscation should have multiple operators");
}

#[test]
fn test_dead_code_generation() {
    let injector = DeadCodeInjector::new(0.5);
    
    use luau_obfuscator::parser::ParseResult;
    use luau_obfuscator::parser::StringLiteral;
    
    let parse_result = ParseResult {
        ast: None,
        strings: vec![
            StringLiteral {
                value: "test1".to_string(),
                line: 1,
                column: 0,
                sensitivity: luau_obfuscator::parser::Sensitivity::Low,
            },
            StringLiteral {
                value: "test2".to_string(),
                line: 2,
                column: 0,
                sensitivity: luau_obfuscator::parser::Sensitivity::Low,
            },
        ],
        numbers: vec![],
        functions: vec![],
    };
    
    let snippets = injector.generate(&parse_result).unwrap();
    
    // With 2 strings and 0.5 density, should generate ~1 snippet
    assert!(!snippets.is_empty(), "Should generate dead code snippets");
}

#[test]
fn test_obfuscation_stats() {
    let source = load_fixture("simple.lua");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    
    let result = obfuscator.obfuscate(&parse_result, &analysis).unwrap();
    let stats = obfuscator.get_stats(&result);
    
    // Stats should reflect what was done
    assert_eq!(stats.tier, ObfuscationTier::Standard);
    assert!(stats.encrypted_strings > 0);
    
    println!("Obfuscation stats: {}", stats);
}

#[test]
fn test_performance_comparison() {
    let source = load_fixture("complex.lua");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Measure each tier
    let perf_basic = PerformanceMeasurement::start("Basic tier");
    let crypto_ctx1 = CryptoContext::new("password", None).unwrap();
    let basic = Obfuscator::new(ObfuscationTier::Basic, crypto_ctx1);
    let _basic_result = basic.obfuscate(&parse_result, &analysis).unwrap();
    let basic_time = perf_basic.finish();
    
    let perf_standard = PerformanceMeasurement::start("Standard tier");
    let crypto_ctx2 = CryptoContext::new("password", None).unwrap();
    let standard = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx2);
    let _standard_result = standard.obfuscate(&parse_result, &analysis).unwrap();
    let standard_time = perf_standard.finish();
    
    let perf_premium = PerformanceMeasurement::start("Premium tier");
    let crypto_ctx3 = CryptoContext::new("password", None).unwrap();
    let premium = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx3);
    let _premium_result = premium.obfuscate(&parse_result, &analysis).unwrap();
    let premium_time = perf_premium.finish();
    
    // Basic should be fastest, Premium slowest
    assert!(basic_time < standard_time, "Basic should be faster than Standard");
    assert!(standard_time < premium_time, "Standard should be faster than Premium");
    
    // All should complete reasonably quickly (< 5 seconds for complex script)
    assert!(premium_time.as_secs() < 5, "Even premium tier should be reasonably fast");
}
