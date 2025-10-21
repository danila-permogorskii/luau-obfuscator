//! End-to-end obfuscation workflow integration tests
//!
//! These tests validate the complete obfuscation pipeline from
//! input script parsing through final protected output generation.

use luau_obfuscator::{
    analysis::Analyzer,
    cli::args::ObfuscationTier,
    codegen::{CodeGenConfig, CodeGenerator},
    crypto::CryptoContext,
    obfuscation::Obfuscator,
    parser::LuauParser,
};
use std::fs;
use tempfile::TempDir;

/// Test complete obfuscation workflow with Basic tier
#[test]
fn test_end_to_end_basic_tier() {
    // Setup
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.lua");
    let output_path = temp_dir.path().join("output.lua");

    // Create test script
    let test_script = r#"
        local message = "Hello, Roblox!"
        local count = 42

        function greet(name)
            return "Hello, " .. name .. "!"
        end

        print(message)
        print(greet("World"))
    "#;

    fs::write(&input_path, test_script).unwrap();

    // Phase 1: Parse
    let parser = LuauParser::new();
    let parse_result = parser.parse(test_script).unwrap();

    assert!(parse_result.strings.len() >= 2, "Should extract strings");
    assert!(parse_result.numbers.len() >= 1, "Should extract numbers");
    assert!(parse_result.functions.len() >= 1, "Should extract functions");

    // Phase 2: Analyze
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();

    assert!(
        !analysis.preserved_identifiers.is_empty(),
        "Should have preserved Roblox APIs"
    );

    // Phase 3: Obfuscate (Basic tier)
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Basic, crypto_ctx.clone());
    let obfuscated = obfuscator.obfuscate(&parse_result, &analysis).unwrap();

    assert!(
        !obfuscated.encrypted_strings.is_empty(),
        "Should encrypt strings in basic tier"
    );

    // Phase 4: Generate output
    let codegen_config = CodeGenConfig {
        license_key: "TEST-1234-5678-9012".to_string(),
        script_id: "test_script".to_string(),
        api_endpoint: "https://api.test.com".to_string(),
        hwid: Some(123456),
        ..Default::default()
    };

    let code_generator = CodeGenerator::new(codegen_config, crypto_ctx).unwrap();

    // Create encrypted string pairs for codegen
    let encrypted_strings: Vec<(String, _)> = obfuscated
        .encrypted_strings
        .iter()
        .map(|es| {
            (
                es.original.clone(),
                luau_obfuscator::crypto::EncryptedData {
                    ciphertext: es.encrypted_data.clone(),
                    nonce: es.nonce.clone(),
                    tag_len: 16,
                },
            )
        })
        .collect();

    // Generate fake obfuscation result for testing
    let obfuscation_result = luau_obfuscator::obfuscation::ObfuscationResult {
        code: "-- Obfuscated code placeholder\nprint('test')".to_string(),
    };

    let protected_script = code_generator
        .generate(&obfuscation_result, &encrypted_strings)
        .unwrap();

    // Validate output
    assert!(
        protected_script.contains("Protected by Luau Obfuscator"),
        "Should have header"
    );
    assert!(
        protected_script.contains("Watermark:"),
        "Should have watermark"
    );
    assert!(
        protected_script.contains("ChaCha20") || protected_script.contains("[RUNTIME]"),
        "Should have runtime or runtime placeholder"
    );

    // Write output
    fs::write(&output_path, protected_script).unwrap();
    assert!(output_path.exists(), "Output file should exist");

    // Verify output size is reasonable
    let output_size = fs::metadata(&output_path).unwrap().len();
    assert!(
        output_size > test_script.len() as u64,
        "Protected script should be larger than original"
    );

    println!("✓ End-to-end basic tier workflow completed successfully");
}

/// Test complete obfuscation workflow with Standard tier
#[test]
fn test_end_to_end_standard_tier() {
    let test_script = r#"
        local Players = game:GetService("Players")
        local player = Players.LocalPlayer
        
        local API_KEY = "secret_12345"
        local MAX_RETRIES = 3
        
        function processRequest(data)
            for i = 1, MAX_RETRIES do
                local success = pcall(function()
                    print("Processing:", data)
                end)
                if success then
                    return true
                end
            end
            return false
        end
        
        processRequest("test")
    "#;

    // Parse
    let parser = LuauParser::new();
    let parse_result = parser.parse(test_script).unwrap();

    // Analyze
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();

    // Verify Roblox API preservation
    assert!(
        analysis
            .preserved_identifiers
            .iter()
            .any(|id| id == "game" || id == "Players"),
        "Should preserve Roblox APIs"
    );

    // Obfuscate (Standard tier)
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let obfuscated = obfuscator.obfuscate(&parse_result, &analysis).unwrap();

    let stats = obfuscator.get_stats(&obfuscated);

    // Standard tier should have more aggressive obfuscation
    assert!(
        stats.encrypted_strings > 0,
        "Should encrypt strings"
    );
    assert!(
        stats.obfuscated_constants > 0,
        "Should obfuscate constants"
    );
    assert!(
        stats.renamed_identifiers > 0,
        "Should rename identifiers"
    );

    println!("✓ End-to-end standard tier workflow completed");
    println!("  Stats: {}", stats);
}

/// Test complete obfuscation workflow with Premium tier
#[test]
fn test_end_to_end_premium_tier() {
    let test_script = r#"
        -- Complex script with multiple features
        local MODULE_VERSION = "1.0.0"
        local CONFIG = {
            timeout = 5,
            retries = 3,
            endpoints = {"https://api1.com", "https://api2.com"}
        }
        
        local DataManager = {}
        DataManager.__index = DataManager
        
        function DataManager.new()
            local self = setmetatable({}, DataManager)
            self.cache = {}
            return self
        end
        
        function DataManager:fetchData(id)
            if self.cache[id] then
                return self.cache[id]
            end
            
            for i = 1, CONFIG.retries do
                local success, data = pcall(function()
                    -- Simulate fetch
                    return {id = id, data = "sample"}
                end)
                
                if success then
                    self.cache[id] = data
                    return data
                end
            end
            
            error("Failed to fetch data")
        end
        
        return DataManager
    "#;

    // Parse
    let parser = LuauParser::new();
    let parse_result = parser.parse(test_script).unwrap();

    assert!(
        parse_result.strings.len() >= 5,
        "Should extract multiple strings"
    );
    assert!(
        parse_result.numbers.len() >= 3,
        "Should extract multiple numbers"
    );
    assert!(
        parse_result.functions.len() >= 2,
        "Should extract multiple functions"
    );

    // Analyze
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();

    // Obfuscate (Premium tier)
    let crypto_ctx = CryptoContext::new("premium_password", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);
    let obfuscated = obfuscator.obfuscate(&parse_result, &analysis).unwrap();

    let stats = obfuscator.get_stats(&obfuscated);

    // Premium tier should have maximum obfuscation
    assert!(
        stats.encrypted_strings >= parse_result.strings.len(),
        "Should encrypt all strings"
    );
    assert!(
        stats.obfuscated_constants >= parse_result.numbers.len(),
        "Should obfuscate all constants"
    );
    assert!(stats.renamed_identifiers > 5, "Should rename many identifiers");
    assert!(
        stats.flattened_blocks > 0,
        "Should have control flow flattening"
    );
    assert!(
        stats.dead_code_snippets > 0,
        "Should inject dead code"
    );

    println!("✓ End-to-end premium tier workflow completed");
    println!("  Stats: {}", stats);
}

/// Test obfuscation preserves Roblox API calls
#[test]
fn test_roblox_api_preservation() {
    let test_script = r#"
        local Players = game:GetService("Players")
        local ReplicatedStorage = game:GetService("ReplicatedStorage")
        local RunService = game:GetService("RunService")
        
        local player = Players.LocalPlayer
        local character = player.Character or player.CharacterAdded:Wait()
        
        -- Use Vector3 and CFrame
        local spawnPos = Vector3.new(0, 10, 0)
        workspace.CurrentCamera.CFrame = CFrame.new(spawnPos)
        
        -- RemoteEvent
        local remoteEvent = ReplicatedStorage:WaitForChild("GameEvent")
        remoteEvent:FireServer("test", 123)
    "#;

    let parser = LuauParser::new();
    let parse_result = parser.parse(test_script).unwrap();

    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();

    // Verify all Roblox APIs are preserved
    let preserved_apis = [
        "game",
        "Players",
        "ReplicatedStorage",
        "RunService",
        "workspace",
        "Vector3",
        "CFrame",
    ];

    for api in preserved_apis.iter() {
        assert!(
            analysis.preserved_identifiers.contains(&api.to_string()),
            "Should preserve Roblox API: {}",
            api
        );
    }

    // Obfuscate and verify preservation
    let crypto_ctx = CryptoContext::new("test", None).unwrap();
    let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);
    let obfuscated = obfuscator.obfuscate(&parse_result, &analysis).unwrap();

    // Verify that Roblox APIs are NOT in the renamed identifiers
    for api in preserved_apis.iter() {
        assert!(
            !obfuscated.name_mappings.contains_key(*api),
            "Should NOT rename Roblox API: {}",
            api
        );
    }

    println!("✓ Roblox API preservation test passed");
}

/// Test watermark embedding and verification
#[test]
fn test_watermark_integration() {
    let customer_id = "customer_12345";
    let script_id = "premium_script_v1";

    let crypto_ctx = CryptoContext::new("watermark_test", None).unwrap();
    let watermark = crypto_ctx.generate_watermark(customer_id, script_id);

    // Verify watermark
    assert!(
        crypto_ctx.verify_watermark(&watermark, customer_id),
        "Should verify correct customer"
    );
    assert!(
        !crypto_ctx.verify_watermark(&watermark, "wrong_customer"),
        "Should reject wrong customer"
    );

    // Test watermark encoding/decoding
    let encoded = serde_json::to_string(&watermark).unwrap();
    let decoded: luau_obfuscator::crypto::Watermark =
        serde_json::from_str(&encoded).unwrap();

    assert_eq!(watermark.primary_hash, decoded.primary_hash);
    assert_eq!(watermark.secondary_hash, decoded.secondary_hash);
    assert_eq!(watermark.script_id, decoded.script_id);

    println!("✓ Watermark integration test passed");
}

/// Test error handling in obfuscation pipeline
#[test]
fn test_error_handling() {
    // Test 1: Invalid Lua syntax
    let invalid_script = "function foo( { return";
    let parser = LuauParser::new();
    let result = parser.parse(invalid_script);
    assert!(result.is_err(), "Should fail on invalid syntax");

    // Test 2: Empty script
    let empty_script = "";
    let result = parser.parse(empty_script);
    assert!(result.is_ok(), "Should handle empty script gracefully");

    // Test 3: Very large script (stress test)
    let large_script = "local x = 1\n".repeat(10000);
    let result = parser.parse(&large_script);
    assert!(result.is_ok(), "Should handle large scripts");

    println!("✓ Error handling tests passed");
}
