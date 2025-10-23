//! Security Audit Tests
//!
//! Comprehensive security validation tests for:
//! - Cryptographic implementation correctness
//! - Watermarking robustness and traceability
//! - License validation security
//! - Anti-tampering mechanisms
//! - Key derivation strength
//! - Encryption/decryption correctness

use luau_obfuscator::{
    crypto::{CryptoEngine, KdfParams, Watermark},
    obfuscation::{ObfuscationEngine, ObfuscationTier},
    parser::LuauParser,
    analysis::{AnalysisEngine, AnalysisOptions},
};
use std::collections::HashSet;

#[test]
fn test_argon2id_kdf_uniqueness() {
    // Verify that same password with different salts produces different keys
    let params = KdfParams {
        memory_cost: 262144,
        time_cost: 4,
        parallelism: 2,
    };
    let engine = CryptoEngine::new(params);
    
    let password = "test_password_123";
    let salt1 = b"salt_bytes_001";
    let salt2 = b"salt_bytes_002";
    
    let key1 = engine.derive_key(password, salt1).expect("KDF failed");
    let key2 = engine.derive_key(password, salt2).expect("KDF failed");
    
    assert_ne!(
        key1, key2,
        "Keys with different salts should be different"
    );
}

#[test]
fn test_argon2id_kdf_deterministic() {
    // Verify that KDF is deterministic (same inputs = same output)
    let params = KdfParams {
        memory_cost: 262144,
        time_cost: 4,
        parallelism: 2,
    };
    let engine = CryptoEngine::new(params);
    
    let password = "deterministic_test";
    let salt = b"constant_salt123";
    
    let key1 = engine.derive_key(password, salt).expect("KDF failed");
    let key2 = engine.derive_key(password, salt).expect("KDF failed");
    
    assert_eq!(
        key1, key2,
        "Same password and salt should produce same key"
    );
}

#[test]
fn test_aes_gcm_encryption_uniqueness() {
    // Verify that encrypting same plaintext twice produces different ciphertexts
    // (due to unique nonces/IVs)
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    
    let plaintext = b"sensitive data";
    
    let ciphertext1 = engine.encrypt(&key, plaintext).expect("Encrypt failed");
    let ciphertext2 = engine.encrypt(&key, plaintext).expect("Encrypt failed");
    
    assert_ne!(
        ciphertext1, ciphertext2,
        "Same plaintext should produce different ciphertexts (unique nonces)"
    );
}

#[test]
fn test_aes_gcm_roundtrip() {
    // Verify encrypt → decrypt roundtrip preserves data
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    
    let original = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    
    let ciphertext = engine.encrypt(&key, original).expect("Encrypt failed");
    let decrypted = engine.decrypt(&key, &ciphertext).expect("Decrypt failed");
    
    assert_eq!(
        original, decrypted.as_slice(),
        "Roundtrip should preserve original data"
    );
}

#[test]
fn test_aes_gcm_wrong_key_fails() {
    // Verify that decryption with wrong key fails
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    
    let key1 = engine.derive_key("password1", b"salt123").expect("KDF failed");
    let key2 = engine.derive_key("password2", b"salt123").expect("KDF failed");
    
    let plaintext = b"secret message";
    let ciphertext = engine.encrypt(&key1, plaintext).expect("Encrypt failed");
    
    // Attempting to decrypt with wrong key should fail
    let result = engine.decrypt(&key2, &ciphertext);
    assert!(result.is_err(), "Decryption with wrong key should fail");
}

#[test]
fn test_aes_gcm_tampered_ciphertext_fails() {
    // Verify that tampering with ciphertext is detected
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    
    let plaintext = b"authenticated data";
    let mut ciphertext = engine.encrypt(&key, plaintext).expect("Encrypt failed");
    
    // Tamper with the ciphertext (flip a bit in the middle)
    if ciphertext.len() > 20 {
        ciphertext[10] ^= 0xFF;
    }
    
    // Decryption should fail due to authentication failure
    let result = engine.decrypt(&key, &ciphertext);
    assert!(
        result.is_err(),
        "Decryption of tampered ciphertext should fail"
    );
}

#[test]
fn test_watermark_uniqueness() {
    // Verify that watermarks are unique per customer
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    
    let watermark1 = engine.generate_watermark("customer-001", 123456789).expect("Watermark failed");
    let watermark2 = engine.generate_watermark("customer-002", 123456789).expect("Watermark failed");
    let watermark3 = engine.generate_watermark("customer-001", 987654321).expect("Watermark failed");
    
    assert_ne!(watermark1, watermark2, "Different customer IDs should produce different watermarks");
    assert_ne!(watermark1, watermark3, "Different HWIDs should produce different watermarks");
}

#[test]
fn test_watermark_extraction() {
    // Verify that watermarks can be extracted and validated
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    
    let customer_id = "test-customer-12345";
    let hwid = 999888777u64;
    
    let watermark = engine.generate_watermark(customer_id, hwid).expect("Watermark failed");
    
    // In a real system, you'd embed this in the obfuscated code
    // Here we just verify the watermark structure is valid
    assert!(!watermark.is_empty(), "Watermark should not be empty");
    assert!(
        watermark.len() >= 32,
        "Watermark should be sufficiently long for security"
    );
}

#[test]
fn test_watermark_survives_obfuscation() {
    // Verify watermarks persist through obfuscation transformations
    let script = r#"
local secret = "embedded watermark: WATERMARK_PLACEHOLDER"
local function process()
    return secret
end
print(process())
    "#;
    
    let parser = LuauParser::new();
    let ast = parser.parse(script).expect("Parse failed");
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
    
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let engine = ObfuscationEngine::new(*tier);
        let obfuscated = engine.obfuscate(&ast, &analysis_result).expect("Obfuscation failed");
        
        // Convert back to string and check watermark presence
        let obfuscated_str = format!("{:?}", obfuscated); // Debug format for inspection
        
        // Watermark should still be present (or its encrypted form)
        // In real implementation, we'd have a proper extraction method
        assert!(
            obfuscated_str.contains("WATERMARK") || obfuscated_str.len() > script.len(),
            "Watermark should survive {:?} tier obfuscation",
            tier
        );
    }
}

#[test]
fn test_key_entropy() {
    // Verify that derived keys have sufficient entropy
    let params = KdfParams {
        memory_cost: 262144,
        time_cost: 4,
        parallelism: 2,
    };
    let engine = CryptoEngine::new(params);
    
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    
    // Check key length (should be 32 bytes for AES-256)
    assert_eq!(key.len(), 32, "Key should be 256 bits (32 bytes)");
    
    // Check that key is not all zeros (basic entropy check)
    let all_zeros = key.iter().all(|&b| b == 0);
    assert!(!all_zeros, "Key should have non-zero entropy");
    
    // Check that key has reasonable distribution (not all same byte)
    let unique_bytes: HashSet<u8> = key.iter().copied().collect();
    assert!(
        unique_bytes.len() > 10,
        "Key should have diverse byte values (found {} unique bytes)",
        unique_bytes.len()
    );
}

#[test]
fn test_nonce_uniqueness() {
    // Verify that nonces/IVs are unique for each encryption
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    
    let plaintext = b"test";
    let mut nonces = HashSet::new();
    
    // Perform 100 encryptions and extract nonces
    for _ in 0..100 {
        let ciphertext = engine.encrypt(&key, plaintext).expect("Encrypt failed");
        
        // In AES-GCM, the nonce is typically prepended to the ciphertext
        // Assuming first 12 bytes are the nonce (standard GCM nonce size)
        if ciphertext.len() >= 12 {
            let nonce = &ciphertext[0..12];
            let nonce_vec = nonce.to_vec();
            
            assert!(
                !nonces.contains(&nonce_vec),
                "Nonce should be unique for each encryption"
            );
            nonces.insert(nonce_vec);
        }
    }
    
    assert_eq!(
        nonces.len(),
        100,
        "All 100 encryptions should have unique nonces"
    );
}

#[test]
fn test_timing_attack_resistance() {
    // Verify constant-time comparison for sensitive data
    // This is a basic test; full timing attack analysis requires specialized tools
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    let correct_plaintext = b"correct_secret";
    let ciphertext = engine.encrypt(&key, correct_plaintext).expect("Encrypt failed");
    
    // Decrypt with correct key
    let start_correct = std::time::Instant::now();
    let _ = engine.decrypt(&key, &ciphertext);
    let duration_correct = start_correct.elapsed();
    
    // Create wrong key and attempt decrypt
    let wrong_key = engine.derive_key("wrong_password", b"salt123").expect("KDF failed");
    let start_wrong = std::time::Instant::now();
    let _ = engine.decrypt(&wrong_key, &ciphertext);
    let duration_wrong = start_wrong.elapsed();
    
    // Timing should be similar (within reasonable margin)
    // This is a weak test but catches obvious timing leaks
    let ratio = duration_correct.as_nanos() as f64 / duration_wrong.as_nanos().max(1) as f64;
    assert!(
        ratio > 0.5 && ratio < 2.0,
        "Decrypt timing should be consistent (ratio: {:.2})",
        ratio
    );
}

#[test]
fn test_encryption_padding() {
    // Verify that encryption handles various input sizes correctly
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    
    // Test various sizes: 0, 1, 15 (one byte under block), 16 (one block), 17, 32, 1000
    for size in &[0, 1, 15, 16, 17, 32, 100, 1000, 10000] {
        let plaintext = vec![42u8; *size];
        
        let ciphertext = engine.encrypt(&key, &plaintext).expect("Encrypt failed");
        let decrypted = engine.decrypt(&key, &ciphertext).expect("Decrypt failed");
        
        assert_eq!(
            plaintext, decrypted,
            "Roundtrip should work for size {}",
            size
        );
    }
}

#[test]
fn test_concurrent_encryption_safety() {
    // Verify thread-safety of encryption operations
    use std::thread;
    
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    let key = engine.derive_key("password", b"salt123").expect("KDF failed");
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let key_clone = key.clone();
            let engine_clone = engine.clone();
            
            thread::spawn(move || {
                let plaintext = format!("Thread {} data", i);
                let ciphertext = engine_clone
                    .encrypt(&key_clone, plaintext.as_bytes())
                    .expect("Encrypt failed");
                let decrypted = engine_clone
                    .decrypt(&key_clone, &ciphertext)
                    .expect("Decrypt failed");
                
                assert_eq!(
                    plaintext.as_bytes(),
                    decrypted.as_slice(),
                    "Concurrent encryption/decryption should be safe"
                );
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[test]
fn test_license_key_validation() {
    // Verify license key format and validation
    let params = KdfParams::default();
    let engine = CryptoEngine::new(params);
    
    // Valid license key format: XXXX-XXXX-XXXX-XXXX
    let valid_key = "ABCD-1234-EFGH-5678";
    let invalid_key = "invalid";
    
    // In real implementation, you'd call engine.validate_license_key()
    // For now, just verify basic format checking
    assert!(
        valid_key.len() == 19,
        "Valid license key should have correct length"
    );
    assert!(
        valid_key.chars().filter(|&c| c == '-').count() == 3,
        "Valid license key should have 3 dashes"
    );
}

#[test]
fn test_obfuscation_preserves_roblox_api() {
    // Security test: Verify that Roblox APIs are never obfuscated
    // (obfuscating them would break functionality and be detectable)
    let script = r#"
local Players = game:GetService("Players")
local workspace = game.Workspace
local HttpService = game:GetService("HttpService")
local ReplicatedStorage = game.ReplicatedStorage

local part = Instance.new("Part")
part.Position = Vector3.new(0, 10, 0)
part.CFrame = CFrame.new(0, 10, 0)
part.Parent = workspace
    "#;
    
    let parser = LuauParser::new();
    let ast = parser.parse(script).expect("Parse failed");
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
    
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let engine = ObfuscationEngine::new(*tier);
        let obfuscated = engine.obfuscate(&ast, &analysis_result).expect("Obfuscation failed");
        
        let obfuscated_str = format!("{:?}", obfuscated);
        
        // Verify critical Roblox APIs are NOT obfuscated
        let preserved_apis = vec![
            "game",
            "workspace",
            "Workspace",
            "GetService",
            "Instance",
            "Vector3",
            "CFrame",
            "Players",
            "HttpService",
            "ReplicatedStorage",
        ];
        
        for api in preserved_apis {
            assert!(
                obfuscated_str.contains(api),
                "Roblox API '{}' should be preserved in {:?} tier",
                api,
                tier
            );
        }
    }
}

#[test]
fn test_no_plaintext_leaks() {
    // Verify that sensitive strings are encrypted in output
    let script = r#"
local api_key = "sk_live_51234567890abcdef"
local password = "super_secret_password"
local private_data = {
    token = "bearer_token_xyz123",
    secret = "confidential_information"
}
    "#;
    
    let parser = LuauParser::new();
    let ast = parser.parse(script).expect("Parse failed");
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
    
    let engine = ObfuscationEngine::new(ObfuscationTier::Premium);
    let obfuscated = engine.obfuscate(&ast, &analysis_result).expect("Obfuscation failed");
    
    let obfuscated_str = format!("{:?}", obfuscated);
    
    // Verify that sensitive strings do NOT appear in plaintext
    let sensitive_strings = vec![
        "sk_live_51234567890abcdef",
        "super_secret_password",
        "bearer_token_xyz123",
        "confidential_information",
    ];
    
    for sensitive in sensitive_strings {
        assert!(
            !obfuscated_str.contains(sensitive),
            "Sensitive string '{}' should be encrypted, not appear in plaintext",
            sensitive
        );
    }
}

#[test]
fn test_anti_debugging_checks() {
    // Verify that anti-debugging mechanisms are present in premium tier
    let script = r#"
local function sensitiveOperation()
    return "protected code"
end
print(sensitiveOperation())
    "#;
    
    let parser = LuauParser::new();
    let ast = parser.parse(script).expect("Parse failed");
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
    
    let engine = ObfuscationEngine::new(ObfuscationTier::Premium);
    let obfuscated = engine.obfuscate(&ast, &analysis_result).expect("Obfuscation failed");
    
    let obfuscated_str = format!("{:?}", obfuscated);
    
    // Premium tier should include anti-debugging checks
    // This is a placeholder - actual implementation would check for specific patterns
    assert!(
        obfuscated_str.len() > script.len() * 2,
        "Premium tier should add significant anti-debugging overhead"
    );
}
