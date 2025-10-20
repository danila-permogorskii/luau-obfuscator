//! Integration tests for basic obfuscation functionality

use std::fs;
use std::path::PathBuf;

#[test]
fn test_cli_help() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Luau Obfuscator"));
}

#[test]
fn test_parse_simple_script() {
    // This test will be expanded once the protect command is fully functional
    let fixture_path = PathBuf::from("tests/fixtures/sample_scripts/simple.lua");
    assert!(fixture_path.exists(), "Fixture file should exist");

    let content = fs::read_to_string(fixture_path).unwrap();
    assert!(content.contains("Hello, Roblox!"));
}
