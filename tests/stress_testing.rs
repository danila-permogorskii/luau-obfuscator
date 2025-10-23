//! Stress Testing
//!
//! Tests obfuscation performance and reliability with large scripts,
//! high volumes of operations, and resource-intensive scenarios.

use luau_obfuscator::{
    analysis::Analyzer,
    crypto::CryptoContext,
    obfuscation::{Obfuscator, ObfuscationTier},
    parser::LuauParser,
};
use std::time::Instant;

#[test]
fn test_large_script_1000_lines() {
    let mut script = String::new();
    
    // Generate 1000 lines of varied code
    for i in 0..1000 {
        script.push_str(&format!(
            r#"
local function func_{}()
    local x = {}
    local s = "string_{}"
    local t = {{a = {}, b = "{}"}}
    return x + t.a
end
"#,
            i, i, i, i * 2, i
        ));
    }
    
    let parser = LuauParser::new();
    let start = Instant::now();
    let parse_result = parser.parse(&script).unwrap();
    let parse_time = start.elapsed();
    println!("Parse 1000 lines: {:?}", parse_time);
    
    let analyzer = Analyzer::new();
    let start = Instant::now();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    let analysis_time = start.elapsed();
    println!("Analyze 1000 lines: {:?}", analysis_time);
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    
    let start = Instant::now();
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    let obfuscation_time = start.elapsed();
    println!("Obfuscate 1000 lines: {:?}", obfuscation_time);
    
    assert!(result.is_ok(), "Should handle 1000-line script");
    
    // Performance targets
    assert!(
        parse_time.as_millis() < 1000,
        "Parse should complete in <1s, took {:?}",
        parse_time
    );
    assert!(
        obfuscation_time.as_millis() < 2000,
        "Obfuscation should complete in <2s for 1000 lines, took {:?}",
        obfuscation_time
    );
}

#[test]
fn test_large_script_10000_lines() {
    let mut script = String::new();
    
    // Generate 10,000 lines of code
    for i in 0..10000 {
        script.push_str(&format!("local var_{} = {}\n", i, i));
    }
    
    let parser = LuauParser::new();
    let start = Instant::now();
    let parse_result = parser.parse(&script).unwrap();
    let parse_time = start.elapsed();
    println!("Parse 10,000 lines: {:?}", parse_time);
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Basic, crypto_ctx);
    
    let start = Instant::now();
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    let obfuscation_time = start.elapsed();
    println!("Obfuscate 10,000 lines (Basic tier): {:?}", obfuscation_time);
    
    assert!(result.is_ok(), "Should handle 10,000-line script");
    
    // Should complete in reasonable time even for large scripts
    assert!(
        obfuscation_time.as_secs() < 10,
        "Should complete in <10s, took {:?}",
        obfuscation_time
    );
}

#[test]
fn test_many_strings_1000() {
    let mut script = String::from("local strings = {\n");
    
    // 1000 unique strings
    for i in 0..1000 {
        script.push_str(&format!("    \"string_number_{}_{}\",\n", i, i * 7));
    }
    script.push_str("}\n");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    
    let start = Instant::now();
    let result = obfuscator.obfuscate(&parse_result, &analysis).unwrap();
    let time = start.elapsed();
    println!("Encrypt 1000 strings: {:?}", time);
    
    // Should encrypt all strings
    assert_eq!(
        result.encrypted_strings.len(),
        1000,
        "Should encrypt all 1000 strings"
    );
    
    // Should complete in reasonable time
    assert!(
        time.as_secs() < 5,
        "String encryption should complete in <5s, took {:?}",
        time
    );
}

#[test]
fn test_deeply_nested_tables() {
    let mut script = String::from("local t = ");
    
    // Create 50-level deep nested table
    for _ in 0..50 {
        script.push_str("{ nested = ");
    }
    script.push_str("\"deep_value\"");
    for _ in 0..50 {
        script.push_str(" }");
    }
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle deeply nested tables");
}

#[test]
fn test_many_functions_500() {
    let mut script = String::new();
    
    // 500 functions calling each other
    for i in 0..500 {
        script.push_str(&format!(
            r#"
local function func_{}()
    if {} > 250 then
        return {}
    else
        return func_{}()
    end
end
"#,
            i,
            i,
            i,
            (i + 1) % 500
        ));
    }
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&script).unwrap();
    
    let analyzer = Analyzer::new();
    let start = Instant::now();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    let analysis_time = start.elapsed();
    println!("Analyze 500 functions: {:?}", analysis_time);
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    
    let start = Instant::now();
    let result = obfuscator.obfuscate(&parse_result, &analysis).unwrap();
    let obfuscation_time = start.elapsed();
    println!("Obfuscate 500 functions: {:?}", obfuscation_time);
    
    // Should mangle all function names
    assert!(
        result.name_mappings.len() >= 500,
        "Should mangle function names"
    );
}

#[test]
fn test_complex_control_flow() {
    let mut script = String::from(
        r#"
local function complex()
    local result = 0
"#,
    );
    
    // Generate complex nested control flow (50 levels)
    for i in 0..50 {
        script.push_str(&format!(
            r#"
    if {} % 2 == 0 then
        for j = 1, {} do
            while j > 0 do
                repeat
                    result = result + 1
                until result > {}
                j = j - 1
            end
        end
    else
"#,
            i, i + 1, i
        ));
    }
    
    // Close all if-else blocks
    for _ in 0..50 {
        script.push_str("    end\n");
    }
    
    script.push_str("    return result\nend\n");
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);
    
    let start = Instant::now();
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    let time = start.elapsed();
    println!("Obfuscate complex control flow: {:?}", time);
    
    assert!(result.is_ok(), "Should handle complex control flow");
}

#[test]
fn test_massive_table_literal() {
    let mut script = String::from("local data = {\n");
    
    // 5000-element table
    for i in 0..5000 {
        script.push_str(&format!("    item_{} = {},\n", i, i));
    }
    script.push_str("}\n");
    
    let parser = LuauParser::new();
    let start = Instant::now();
    let parse_result = parser.parse(&script).unwrap();
    let parse_time = start.elapsed();
    println!("Parse 5000-element table: {:?}", parse_time);
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle massive table literals");
}

#[test]
fn test_string_concatenation_chain() {
    let mut script = String::from("local s = \"start\"");
    
    // Chain 200 concatenations
    for i in 0..200 {
        script.push_str(&format!(" .. \"part_{}\"", i));
    }
    script.push('\n');
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    
    assert!(result.is_ok(), "Should handle long concatenation chains");
}

#[test]
fn test_memory_efficiency_multiple_obfuscations() {
    // Test memory doesn't leak by running multiple obfuscations
    let script = r#"
        local function test()
            local s = "test string"
            local n = 42
            return s, n
        end
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Run 100 obfuscations in sequence
    for i in 0..100 {
        let crypto_ctx = CryptoContext::new(&format!("password_{}", i), None).unwrap();
        let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
        let result = obfuscator.obfuscate(&parse_result, &analysis);
        assert!(result.is_ok(), "Iteration {} should succeed", i);
    }
    
    println!("Completed 100 sequential obfuscations successfully");
}

#[test]
fn test_all_tiers_performance_comparison() {
    let script = r#"
        local HttpService = game:GetService("HttpService")
        local Players = game:GetService("Players")
        
        local function processPlayer(player)
            local data = {
                name = player.Name,
                id = player.UserId,
                timestamp = os.time()
            }
            
            local json = HttpService:JSONEncode(data)
            return json
        end
        
        for i = 1, 100 do
            local result = processPlayer(Players.LocalPlayer)
            print("Result:", result)
        end
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Test all three tiers
    for tier in [
        ObfuscationTier::Basic,
        ObfuscationTier::Standard,
        ObfuscationTier::Premium,
    ] {
        let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
        let obfuscator = Obfuscator::new(tier.clone(), crypto_ctx);
        
        let start = Instant::now();
        let result = obfuscator.obfuscate(&parse_result, &analysis);
        let time = start.elapsed();
        
        assert!(result.is_ok(), "Tier {:?} should succeed", tier);
        println!("Tier {:?} completed in {:?}", tier, time);
    }
}

#[test]
fn test_premium_tier_with_large_script() {
    // Premium tier with all features enabled on a large script
    let mut script = String::new();
    
    for i in 0..200 {
        script.push_str(&format!(
            r#"
local function operation_{}(x, y, z)
    local temp1 = x + y
    local temp2 = y * z
    local temp3 = z - x
    
    if temp1 > temp2 then
        return temp3
    elseif temp2 > temp3 then
        return temp1
    else
        return temp2
    end
end
"#,
            i
        ));
    }
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(&script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);
    
    let start = Instant::now();
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    let time = start.elapsed();
    
    println!("Premium tier on 200 functions: {:?}", time);
    
    assert!(result.is_ok(), "Premium tier should succeed on large scripts");
    let obf_result = result.unwrap();
    
    // Premium tier should apply all transformations
    assert!(obf_result.encrypted_strings.len() > 0, "Should encrypt strings");
    assert!(obf_result.obfuscated_constants.len() > 0, "Should obfuscate constants");
    assert!(obf_result.name_mappings.len() > 0, "Should mangle names");
    assert!(obf_result.dead_code_snippets.len() > 0, "Should inject dead code");
    
    // Should complete in reasonable time
    assert!(
        time.as_secs() < 15,
        "Premium tier should complete in <15s, took {:?}",
        time
    );
}

#[test]
fn test_roblox_script_realistic_scenario() {
    // Realistic Roblox script scenario
    let script = r#"
        local ReplicatedStorage = game:GetService("ReplicatedStorage")
        local Players = game:GetService("Players")
        local RunService = game:GetService("RunService")
        local HttpService = game:GetService("HttpService")
        
        local RemoteEvents = ReplicatedStorage:WaitForChild("RemoteEvents")
        local DataStore = game:GetService("DataStoreService"):GetDataStore("PlayerData")
        
        local PlayerData = {}
        
        local function loadPlayerData(player)
            local userId = player.UserId
            local success, data = pcall(function()
                return DataStore:GetAsync("Player_" .. userId)
            end)
            
            if success and data then
                PlayerData[userId] = data
            else
                PlayerData[userId] = {
                    coins = 0,
                    level = 1,
                    inventory = {}
                }
            end
        end
        
        local function savePlayerData(player)
            local userId = player.UserId
            local data = PlayerData[userId]
            
            pcall(function()
                DataStore:SetAsync("Player_" .. userId, data)
            end)
        end
        
        Players.PlayerAdded:Connect(loadPlayerData)
        Players.PlayerRemoving:Connect(savePlayerData)
        
        RemoteEvents.PurchaseItem.OnServerEvent:Connect(function(player, itemId, cost)
            local userId = player.UserId
            local data = PlayerData[userId]
            
            if data.coins >= cost then
                data.coins = data.coins - cost
                table.insert(data.inventory, itemId)
                return true
            end
            return false
        end)
        
        -- Heartbeat for periodic saves
        RunService.Heartbeat:Connect(function()
            for _, player in ipairs(Players:GetPlayers()) do
                savePlayerData(player)
            end
        end)
    "#;
    
    let parser = LuauParser::new();
    let parse_result = parser.parse(script).unwrap();
    
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();
    
    // Verify Roblox API preservation
    assert!(!analysis.should_obfuscate_name("game"));
    assert!(!analysis.should_obfuscate_name("Players"));
    assert!(!analysis.should_obfuscate_name("RunService"));
    assert!(!analysis.should_obfuscate_name("HttpService"));
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    
    let start = Instant::now();
    let result = obfuscator.obfuscate(&parse_result, &analysis);
    let time = start.elapsed();
    
    println!("Realistic Roblox script obfuscation: {:?}", time);
    
    assert!(result.is_ok(), "Should handle realistic Roblox scripts");
}
