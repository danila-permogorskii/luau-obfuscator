//! Edge Case Testing
//!
//! This module tests various edge cases and boundary conditions
//! to ensure robustness of the obfuscation system.

use luau_obfuscator::{
    analysis::Analyzer,
    crypto::CryptoContext,
    obfuscation::{Obfuscator, ObfuscationTier},
    parser::LuauParser,
};

/// Load test fixture
fn load_fixture(name: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/{}", name))
        .expect("Failed to load fixture")
}

#[test]
fn test_empty_script() {
    let source = "";
    let parser = LuauParser::new();
    
    // Should handle empty scripts gracefully
    let result = parser.parse(source);
    assert!(result.is_ok(), "Should parse empty script");
    
    let parse_result = result.unwrap();
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result);
    assert!(analysis.is_ok(), "Should analyze empty script");
}

#[test]
fn test_single_line_script() {
    let source = "print('Hello')";
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Basic, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should obfuscate single-line script");
}

#[test]
fn test_deeply_nested_functions() {
    let source = r#"
        local function level1()
            local function level2()
                local function level3()
                    local function level4()
                        local function level5()
                            return "deep"
                        end
                        return level5()
                    end
                    return level4()
                end
                return level3()
            end
            return level2()
        end
        return level1()
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Should handle deep nesting
    assert!(analysis.scope_tree.is_some(), "Should track nested scopes");
}

#[test]
fn test_maximum_string_length() {
    // Test with very long string (10KB)
    let long_string = "a".repeat(10_000);
    let source = format!(r#"local s = "{}""#, long_string);
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle very long strings");
}

#[test]
fn test_unicode_and_special_characters() {
    let source = r#"
        local emoji = "🚀 🎉 ❤️"
        local chinese = "你好世界"
        local russian = "Привет мир"
        local special = "\n\t\r\0"
        local escaped = "\\"
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle Unicode and special characters");
    let obf_result = result.unwrap();
    assert!(obf_result.encrypted_strings.len() >= 5, "Should encrypt all strings");
}

#[test]
fn test_extreme_number_values() {
    let source = r#"
        local maxint = 9007199254740991
        local minint = -9007199254740991
        local tiny = 0.000000000001
        local huge = 99999999999999.99999
        local inf = math.huge
        local neginf = -math.huge
        local nan = 0/0
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle extreme numeric values");
}

#[test]
fn test_reserved_roblox_identifiers() {
    let source = r#"
        local game = game
        local workspace = workspace
        local script = script
        local HttpService = game:GetService("HttpService")
        local Players = game:GetService("Players")
        local RunService = game:GetService("RunService")
        
        -- These should NEVER be obfuscated
        local v3 = Vector3.new(0, 0, 0)
        local cf = CFrame.new()
        local instance = Instance.new("Part")
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Verify Roblox API detection
    assert!(
        !analysis.should_obfuscate_name("game"),
        "Should preserve 'game'"
    );
    assert!(
        !analysis.should_obfuscate_name("workspace"),
        "Should preserve 'workspace'"
    );
    assert!(
        !analysis.should_obfuscate_name("script"),
        "Should preserve 'script'"
    );
    assert!(
        !analysis.should_obfuscate_name("HttpService"),
        "Should preserve service names"
    );
    assert!(
        !analysis.should_obfuscate_name("Vector3"),
        "Should preserve Roblox datatypes"
    );
}

#[test]
fn test_mixed_comment_styles() {
    let source = r#"
        -- Single line comment
        local x = 5 -- Inline comment
        
        --[[
            Multi-line
            comment block
        ]]
        
        local y = 10
        
        --[=[
            Nested multi-line
            --[[ Inner block ]]
            comment
        ]=]
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source);
    assert!(result.is_ok(), "Should handle all comment styles");
}

#[test]
fn test_table_with_mixed_keys() {
    let source = r#"
        local t = {
            -- Numeric keys
            [1] = "first",
            [2] = "second",
            
            -- String keys
            name = "test",
            ["with space"] = "value",
            
            -- Mixed
            ["123"] = "string key",
            [true] = "boolean key",
            
            -- Nested tables
            nested = {
                deep = {
                    value = 42
                }
            }
        }
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle complex table structures");
}

#[test]
fn test_coroutine_and_async_patterns() {
    let source = r#"
        local function asyncTask()
            coroutine.yield()
            return "done"
        end
        
        local co = coroutine.create(asyncTask)
        local success, result = coroutine.resume(co)
        
        -- Spawn pattern (common in Roblox)
        spawn(function()
            wait(1)
            print("async")
        end)
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle async patterns");
}

#[test]
fn test_metamethods_and_metatables() {
    let source = r#"
        local mt = {
            __index = function(t, k)
                return "default"
            end,
            __newindex = function(t, k, v)
                rawset(t, k, v)
            end,
            __add = function(a, b)
                return a.value + b.value
            end,
            __tostring = function(t)
                return "custom"
            end,
            __call = function(t, ...)
                return {...}
            end
        }
        
        local obj = setmetatable({}, mt)
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle metamethods correctly");
}

#[test]
fn test_error_handling_patterns() {
    let source = r#"
        local success, result = pcall(function()
            error("Test error")
        end)
        
        local function tryOperation()
            return pcall(function()
                return riskyOperation()
            end)
        end
        
        xpcall(function()
            -- Protected call
        end, function(err)
            warn("Error:", err)
        end)
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle error patterns");
}

#[test]
fn test_goto_and_labels() {
    let source = r#"
        local function useGoto()
            ::start::
            local x = 5
            if x > 3 then
                goto finish
            end
            x = x + 1
            goto start
            ::finish::
            return x
        end
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle goto statements");
}

#[test]
fn test_type_annotations_luau_specific() {
    let source = r#"
        local function typed(x: number, y: string): boolean
            return tonumber(y) == x
        end
        
        type Point = {x: number, y: number}
        type Optional<T> = T | nil
        
        local function process<T>(value: T): T
            return value
        end
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source);
    
    // Luau type annotations should be handled
    assert!(parse_result.is_ok(), "Should parse Luau type annotations");
}

#[test]
fn test_multiple_returns_and_varargs() {
    let source = r#"
        local function multiReturn()
            return 1, 2, 3, 4, 5
        end
        
        local function varargs(...)
            local args = {...}
            return #args, ...
        end
        
        local a, b, c = multiReturn()
        local count, rest = varargs(1, 2, 3)
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle varargs and multiple returns");
}

#[test]
fn test_global_vs_local_shadowing() {
    let source = r#"
        print = function(...) end  -- Shadow global
        
        local function test()
            local print = "local"  -- Shadow in scope
            
            do
                local print = "nested"  -- Shadow again
                return print
            end
        end
        
        _G.print = function() end  -- Explicit global
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Should track scope correctly
    assert!(analysis.scope_tree.is_some(), "Should track shadowing correctly");
}

#[test]
fn test_string_patterns_and_escapes() {
    let source = r#"
        local patterns = {
            "%w+",
            "%d%d%d",
            "[%a%-]+",
            "%(.*%)",
            "\n\t\r",
            "\\\"",
            "\z
            multiline",
        }
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle pattern strings and escapes");
}

#[test]
fn test_operator_precedence_edge_cases() {
    let source = r#"
        local a = 1 + 2 * 3 ^ 4 / 5 - 6 % 7
        local b = not false or true and false
        local c = #t .. "concat" .. #{}
        local d = (1 + 2) * (3 - 4)
        local e = 1 < 2 <= 3 > 4 >= 5 == 6 ~= 7
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(source).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should preserve operator precedence");
}
