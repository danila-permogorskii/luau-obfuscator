//! Cryptography security validation tests

use luau_obfuscator::crypto::*;

mod helpers;
use helpers::*;

use std::time::Duration;

#[test]
fn test_argon2id_key_derivation() {
    let kdf = KeyDerivation::new();
    let password = b"secure_password_123";
    let salt = b"0123456789abcdef"; // 16 bytes
    
    let key = kdf.derive_key(password, salt).unwrap();
    
    // Should produce 32-byte key
    assert_eq!(key.len(), 32, "Key should be 32 bytes");
}

#[test]
fn test_argon2id_deterministic() {
    let kdf = KeyDerivation::new();
    let password = b"test_password";
    let salt = b"fixed_salt_16byt";
    
    let key1 = kdf.derive_key(password, salt).unwrap();
    let key2 = kdf.derive_key(password, salt).unwrap();
    
    assert_eq!(key1, key2, "Same inputs should produce same key");
}

#[test]
fn test_argon2id_different_passwords() {
    let kdf = KeyDerivation::new();
    let salt = b"fixed_salt_16byt";
    
    let key1 = kdf.derive_key(b"password1", salt).unwrap();
    let key2 = kdf.derive_key(b"password2", salt).unwrap();
    
    assert_ne!(key1, key2, "Different passwords should produce different keys");
}

#[test]
fn test_argon2id_different_salts() {
    let kdf = KeyDerivation::new();
    let password = b"fixed_password";
    
    let key1 = kdf.derive_key(password, b"salt_version_01a").unwrap();
    let key2 = kdf.derive_key(password, b"salt_version_02b").unwrap();
    
    assert_ne!(key1, key2, "Different salts should produce different keys");
}

#[test]
fn test_argon2id_salt_too_short() {
    let kdf = KeyDerivation::new();
    let password = b"password";
    let short_salt = b"short"; // Only 5 bytes
    
    let result = kdf.derive_key(password, short_salt);
    assert!(result.is_err(), "Should reject salt shorter than 16 bytes");
}

#[test]
#[ignore] // Slow test (~2 seconds)
fn test_argon2id_timing() {
    let kdf = KeyDerivation::new();
    let password = b"test_password";
    let salt = b"0123456789abcdef";
    
    let (_key, duration) = kdf.derive_key_timed(password, salt).unwrap();
    
    // Argon2id with high-security params should take 1-3 seconds
    assert!(duration >= Duration::from_secs(1), "Should take at least 1 second");
    assert!(duration <= Duration::from_secs(5), "Should complete within 5 seconds");
    
    println!("Argon2id key derivation took: {:?}", duration);
}

#[test]
fn test_aes_256_gcm_encryption() {
    let key = vec![0u8; 32]; // 32-byte key
    let aes = AesEncryption::new(&key).unwrap();
    
    let plaintext = b"Hello, World!";
    let encrypted = aes.encrypt(plaintext).unwrap();
    
    // Ciphertext should be different from plaintext
    assert_ne!(&encrypted.ciphertext[..plaintext.len()], plaintext);
    
    // Nonce should be 12 bytes
    assert_eq!(encrypted.nonce.len(), 12);
    
    // Tag should be included (16 bytes)
    assert_eq!(encrypted.tag_len, 16);
}

#[test]
fn test_aes_256_gcm_decryption() {
    let key = vec![0u8; 32];
    let aes = AesEncryption::new(&key).unwrap();
    
    let plaintext = b"Secret message for decryption";
    let encrypted = aes.encrypt(plaintext).unwrap();
    let decrypted = aes.decrypt(&encrypted).unwrap();
    
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_aes_unique_nonces() {
    let key = vec![0u8; 32];
    let aes = AesEncryption::new(&key).unwrap();
    
    let plaintext = b"Same plaintext";
    
    let encrypted1 = aes.encrypt(plaintext).unwrap();
    let encrypted2 = aes.encrypt(plaintext).unwrap();
    
    // Different nonces for same plaintext
    assert_ne!(encrypted1.nonce, encrypted2.nonce, "Nonces should be unique");
    
    // Different ciphertexts due to different nonces
    assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
}

#[test]
fn test_aes_wrong_key_decryption() {
    let key1 = vec![0u8; 32];
    let key2 = vec![1u8; 32];
    
    let aes1 = AesEncryption::new(&key1).unwrap();
    let aes2 = AesEncryption::new(&key2).unwrap();
    
    let plaintext = b"Encrypted with key1";
    let encrypted = aes1.encrypt(plaintext).unwrap();
    
    // Decryption with wrong key should fail
    let result = aes2.decrypt(&encrypted);
    assert!(result.is_err(), "Should fail with wrong key");
}

#[test]
fn test_aes_tampered_ciphertext() {
    let key = vec![0u8; 32];
    let aes = AesEncryption::new(&key).unwrap();
    
    let plaintext = b"Original message";
    let mut encrypted = aes.encrypt(plaintext).unwrap();
    
    // Tamper with ciphertext
    encrypted.ciphertext[0] ^= 0xFF;
    
    // Decryption should fail due to authentication
    let result = aes.decrypt(&encrypted);
    assert!(result.is_err(), "Should detect tampering");
}

#[test]
fn test_aes_empty_plaintext() {
    let key = vec![0u8; 32];
    let aes = AesEncryption::new(&key).unwrap();
    
    let plaintext = b"";
    let encrypted = aes.encrypt(plaintext).unwrap();
    let decrypted = aes.decrypt(&encrypted).unwrap();
    
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_aes_large_plaintext() {
    let key = vec![0u8; 32];
    let aes = AesEncryption::new(&key).unwrap();
    
    let plaintext = vec![42u8; 10000]; // 10KB
    let encrypted = aes.encrypt(&plaintext).unwrap();
    let decrypted = aes.decrypt(&encrypted).unwrap();
    
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_watermark_generation() {
    let gen = WatermarkGenerator::new();
    let watermark = gen.generate("customer123", "script456");
    
    // Should have proper structure
    assert_eq!(watermark.primary_hash.len(), 32); // SHA-256
    assert_eq!(watermark.secondary_hash.len(), 32);
    assert_eq!(watermark.script_id, "script456");
    assert_eq!(watermark.version, 1);
}

#[test]
fn test_watermark_verification() {
    let gen = WatermarkGenerator::new();
    let watermark = gen.generate("customer123", "script456");
    
    // Correct customer should verify
    assert!(gen.verify(&watermark, "customer123"));
    
    // Wrong customer should not verify
    assert!(!gen.verify(&watermark, "customer999"));
}

#[test]
fn test_watermark_uniqueness() {
    let gen = WatermarkGenerator::new();
    
    let wm1 = gen.generate("customer1", "script1");
    let wm2 = gen.generate("customer2", "script1");
    let wm3 = gen.generate("customer1", "script2");
    
    // Different customers should produce different watermarks
    assert_ne!(wm1.primary_hash, wm2.primary_hash);
    
    // Same customer, different scripts should differ
    assert_ne!(wm1.primary_hash, wm3.primary_hash);
}

#[test]
fn test_watermark_leaker_identification() {
    let gen = WatermarkGenerator::new();
    let watermark = gen.generate("leaker@example.com", "premium_script");
    
    let known_customers = vec![
        "user1@example.com".to_string(),
        "user2@example.com".to_string(),
        "leaker@example.com".to_string(),
        "user3@example.com".to_string(),
    ];
    
    let leakers = gen.identify_leaker(&watermark, &known_customers);
    
    assert_eq!(leakers.len(), 1);
    assert_eq!(leakers[0], "leaker@example.com");
}

#[test]
fn test_watermark_encoding_decoding() {
    let gen = WatermarkGenerator::new();
    let original = gen.generate("customer123", "script456");
    
    let encoded = gen.encode(&original).unwrap();
    let decoded = gen.decode(&encoded).unwrap();
    
    assert_eq!(original, decoded);
}

#[test]
fn test_watermark_stego_pattern() {
    let gen = WatermarkGenerator::new();
    let watermark = gen.generate("customer123", "script456");
    
    let pattern1 = gen.generate_stego_pattern(&watermark, 100);
    let pattern2 = gen.generate_stego_pattern(&watermark, 100);
    
    // Pattern should be deterministic
    assert_eq!(pattern1, pattern2);
    assert_eq!(pattern1.len(), 100);
}

#[test]
fn test_watermark_stego_pattern_distribution() {
    let gen = WatermarkGenerator::new();
    let watermark = gen.generate("customer123", "script456");
    let pattern = gen.generate_stego_pattern(&watermark, 1000);
    
    // Count true/false distribution
    let true_count = pattern.iter().filter(|&&b| b).count();
    let false_count = pattern.len() - true_count;
    
    // Should be roughly balanced (not exactly 500/500)
    assert!(true_count > 300 && true_count < 700, "Pattern should be balanced");
    assert!(false_count > 300 && false_count < 700);
}

#[test]
fn test_watermark_tamper_detection() {
    let gen = WatermarkGenerator::new();
    let mut watermark = gen.generate("customer123", "script456");
    
    // Original should verify
    assert!(gen.verify(&watermark, "customer123"));
    
    // Tamper with primary hash
    watermark.primary_hash[0] ^= 0xFF;
    assert!(!gen.verify(&watermark, "customer123"), "Should detect tampering");
}

#[test]
fn test_crypto_context_integration() {
    let ctx = CryptoContext::new("test_password", None).unwrap();
    
    // Test encryption/decryption
    let plaintext = b"Integration test message";
    let encrypted = ctx.encrypt(plaintext).unwrap();
    let decrypted = ctx.decrypt(&encrypted).unwrap();
    assert_eq!(plaintext, decrypted.as_slice());
    
    // Test watermarking
    let watermark = ctx.generate_watermark("customer_id", "script_id");
    assert!(ctx.verify_watermark(&watermark, "customer_id"));
}

#[test]
fn test_batch_encryption_performance() {
    let ctx = CryptoContext::new("test_password", None).unwrap();
    
    let plaintexts: Vec<&[u8]> = vec![
        b"string 1",
        b"string 2",
        b"string 3",
        b"string 4",
        b"string 5",
    ];
    
    let perf = PerformanceMeasurement::start("batch_encryption");
    
    let mut encrypted_batch = Vec::new();
    for plaintext in &plaintexts {
        encrypted_batch.push(ctx.encrypt(plaintext).unwrap());
    }
    
    let duration = perf.finish();
    
    assert_eq!(encrypted_batch.len(), 5);
    assert!(duration.as_millis() < 100, "Batch encryption should be fast");
}
