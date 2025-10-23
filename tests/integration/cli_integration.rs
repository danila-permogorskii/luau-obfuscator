//! CLI Integration Tests
//!
//! These tests validate the complete CLI interface including:
//! - Command execution and argument parsing
//! - Error handling and validation
//! - File I/O operations
//! - Progress reporting
//! - Configuration management

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a test script file
fn create_test_script(temp_dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let path = temp_dir.path().join(name);
    fs::write(&path, content).expect("Failed to write test script");
    path
}

/// Helper to get the binary command
fn luau_obfuscator() -> Command {
    Command::cargo_bin("luau-obfuscator").expect("Failed to find binary")
}

#[test]
fn test_cli_help_command() {
    luau_obfuscator()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Luau Obfuscator"))
        .stdout(predicate::str::contains("protect"))
        .stdout(predicate::str::contains("generate-license"))
        .stdout(predicate::str::contains("validate"));
}

#[test]
fn test_cli_version_command() {
    luau_obfuscator()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_protect_command_basic() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        r#"
local message = "Hello, World!"
print(message)
        "#,
    );
    let output_path = temp_dir.path().join("output.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--license-key")
        .arg("TEST-KEY-1234-5678")
        .arg("--hwid")
        .arg("123456789")
        .arg("--tier")
        .arg("basic")
        .arg("--offline") // Use offline mode to skip API calls
        .assert()
        .success()
        .stdout(predicate::str::contains("protected successfully"));

    // Verify output file was created
    assert!(output_path.exists(), "Output file should be created");
    
    // Verify output is not empty
    let output_content = fs::read_to_string(&output_path).expect("Failed to read output");
    assert!(!output_content.is_empty(), "Output should not be empty");
    
    // Verify output is different from input (obfuscated)
    let input_content = fs::read_to_string(&input_script).expect("Failed to read input");
    assert_ne!(
        input_content, output_content,
        "Output should be different from input"
    );
}

#[test]
fn test_protect_command_all_tiers() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        r#"
local secret = "sensitive data"
local function process()
    return secret
end
print(process())
        "#,
    );

    for tier in &["basic", "standard", "premium"] {
        let output_path = temp_dir.path().join(format!("output_{}.lua", tier));

        luau_obfuscator()
            .arg("protect")
            .arg(&input_script)
            .arg("--output")
            .arg(&output_path)
            .arg("--license-key")
            .arg("TEST-KEY-1234-5678")
            .arg("--hwid")
            .arg("123456789")
            .arg("--tier")
            .arg(tier)
            .arg("--offline")
            .assert()
            .success();

        assert!(output_path.exists(), "Output file for {} tier should exist", tier);
        
        let content = fs::read_to_string(&output_path).expect("Failed to read output");
        assert!(!content.is_empty(), "{} tier output should not be empty", tier);
    }
}

#[test]
fn test_protect_command_missing_input() {
    luau_obfuscator()
        .arg("protect")
        .arg("nonexistent.lua")
        .arg("--output")
        .arg("output.lua")
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file").or(predicate::str::contains("not found")));
}

#[test]
fn test_protect_command_missing_license_key() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('test')",
    );

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg("output.lua")
        .arg("--hwid")
        .arg("123456")
        .assert()
        .failure()
        .stderr(predicate::str::contains("license-key").or(predicate::str::contains("required")));
}

#[test]
fn test_protect_command_missing_hwid() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('test')",
    );

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg("output.lua")
        .arg("--license-key")
        .arg("TEST-KEY")
        .assert()
        .failure()
        .stderr(predicate::str::contains("hwid").or(predicate::str::contains("required")));
}

#[test]
fn test_protect_command_invalid_tier() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('test')",
    );

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg("output.lua")
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .arg("--tier")
        .arg("invalid_tier")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid").or(predicate::str::contains("tier")));
}

#[test]
fn test_protect_command_malformed_lua() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "malformed.lua",
        r#"
local x = {
    key = "value"
-- Missing closing brace
        "#,
    );

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg("output.lua")
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .arg("--offline")
        .assert()
        .failure()
        .stderr(predicate::str::contains("parse").or(predicate::str::contains("syntax")));
}

#[test]
fn test_protect_command_with_config_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('test with config')",
    );
    
    let config_content = r#"
[obfuscation]
tier = "standard"
offline = true

[license]
key = "CONFIG-KEY-1234-5678"
hwid = "987654321"
    "#;
    
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).expect("Failed to write config");

    let output_path = temp_dir.path().join("output.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--config")
        .arg(&config_path)
        .assert()
        .success();

    assert!(output_path.exists(), "Output with config should be created");
}

#[test]
fn test_generate_license_command() {
    luau_obfuscator()
        .arg("generate-license")
        .arg("--script-id")
        .arg("test-script-001")
        .arg("--buyer-userid")
        .arg("123456789")
        .arg("--api-key")
        .arg("DEV_API_KEY")
        .arg("--offline") // Offline mode for testing without API
        .assert()
        .success()
        .stdout(predicate::str::contains("License").or(predicate::str::contains("generated")));
}

#[test]
fn test_generate_license_missing_script_id() {
    luau_obfuscator()
        .arg("generate-license")
        .arg("--buyer-userid")
        .arg("123456789")
        .arg("--api-key")
        .arg("DEV_API_KEY")
        .assert()
        .failure()
        .stderr(predicate::str::contains("script-id").or(predicate::str::contains("required")));
}

#[test]
fn test_validate_command_protected_script() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // First, create a protected script
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('validation test')",
    );
    let output_path = temp_dir.path().join("protected.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--license-key")
        .arg("VALID-KEY-1234-5678")
        .arg("--hwid")
        .arg("123456789")
        .arg("--tier")
        .arg("basic")
        .arg("--offline")
        .assert()
        .success();

    // Now validate it
    luau_obfuscator()
        .arg("validate")
        .arg(&output_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("valid").or(predicate::str::contains("watermark")));
}

#[test]
fn test_validate_command_unprotected_script() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let unprotected_script = create_test_script(
        &temp_dir,
        "unprotected.lua",
        "print('not protected')",
    );

    luau_obfuscator()
        .arg("validate")
        .arg(&unprotected_script)
        .assert()
        .failure()
        .stdout(predicate::str::contains("not protected").or(predicate::str::contains("no watermark")));
}

#[test]
fn test_protect_with_verbose_logging() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('verbose test')",
    );
    let output_path = temp_dir.path().join("output.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .arg("--tier")
        .arg("basic")
        .arg("--offline")
        .arg("--verbose")
        .env("RUST_LOG", "debug")
        .assert()
        .success()
        .stdout(predicate::str::contains("Parsing").or(predicate::str::contains("DEBUG")));
}

#[test]
fn test_protect_with_custom_api_endpoint() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('custom api test')",
    );
    let output_path = temp_dir.path().join("output.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .arg("--tier")
        .arg("basic")
        .arg("--api-endpoint")
        .arg("https://custom-api.example.com")
        .arg("--offline") // Still offline to avoid actual network calls
        .assert()
        .success();
}

#[test]
fn test_protect_preserves_roblox_apis() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "roblox.lua",
        r#"
local players = game:GetService("Players")
local workspace = game.Workspace
local part = Instance.new("Part")
part.Position = Vector3.new(0, 10, 0)
part.Parent = workspace
        "#,
    );
    let output_path = temp_dir.path().join("output.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .arg("--tier")
        .arg("standard")
        .arg("--offline")
        .assert()
        .success();

    let output_content = fs::read_to_string(&output_path).expect("Failed to read output");
    
    // Verify critical Roblox APIs are preserved (not obfuscated)
    assert!(
        output_content.contains("game") || output_content.contains("GetService"),
        "Roblox game API should be preserved"
    );
    assert!(
        output_content.contains("workspace") || output_content.contains("Workspace"),
        "Roblox workspace should be preserved"
    );
}

#[test]
fn test_protect_handles_output_directory_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('directory test')",
    );
    
    // Create output path in a non-existent nested directory
    let output_path = temp_dir.path().join("nested/subdirectory/output.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .arg("--tier")
        .arg("basic")
        .arg("--offline")
        .assert()
        .success();

    assert!(output_path.exists(), "Output file in nested directory should be created");
}

#[test]
fn test_protect_dry_run_mode() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_script = create_test_script(
        &temp_dir,
        "input.lua",
        "print('dry run test')",
    );
    let output_path = temp_dir.path().join("output.lua");

    luau_obfuscator()
        .arg("protect")
        .arg(&input_script)
        .arg("--output")
        .arg(&output_path)
        .arg("--license-key")
        .arg("TEST-KEY")
        .arg("--hwid")
        .arg("123456")
        .arg("--tier")
        .arg("basic")
        .arg("--offline")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("dry run").or(predicate::str::contains("would")));

    // In dry-run mode, output file should NOT be created
    assert!(!output_path.exists(), "Output file should not be created in dry-run mode");
}
