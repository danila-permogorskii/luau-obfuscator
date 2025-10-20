//! Argon2id key derivation

use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, ParamsBuilder, Version,
};

/// Key derivation using Argon2id
pub struct KeyDerivation {
    argon2: Argon2<'static>,
}

impl KeyDerivation {
    /// Create new KDF with high-security parameters
    pub fn new() -> Self {
        // High-security parameters for service-side use
        let params = ParamsBuilder::new()
            .m_cost(262144) // 256 MB memory
            .t_cost(4) // 4 iterations
            .p_cost(2) // 2 parallel threads
            .build()
            .expect("Failed to build Argon2 params");

        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            params,
        );

        Self { argon2 }
    }

    /// Derive 32-byte key from password and salt
    pub fn derive_key(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
        // Ensure salt is at least 16 bytes
        if salt.len() < 16 {
            return Err(ObfuscatorError::CryptoError(
                "Salt must be at least 16 bytes".to_string(),
            )
            .into());
        }

        // Create salt string (argon2 expects base64)
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| ObfuscatorError::CryptoError(format!("Salt encoding failed: {}", e)))?;

        // Hash password with salt
        let hash = self
            .argon2
            .hash_password(password, &salt_string)
            .map_err(|e| ObfuscatorError::CryptoError(format!("Key derivation failed: {}", e)))?;

        // Extract the hash bytes (first 32 bytes)
        let hash_bytes = hash
            .hash
            .ok_or_else(|| ObfuscatorError::CryptoError("No hash produced".to_string()))?;

        Ok(hash_bytes.as_bytes().to_vec())
    }

    /// Derive key with timing information (for benchmarking)
    pub fn derive_key_timed(&self, password: &[u8], salt: &[u8]) -> Result<(Vec<u8>, std::time::Duration)> {
        let start = std::time::Instant::now();
        let key = self.derive_key(password, salt)?;
        let duration = start.elapsed();
        Ok((key, duration))
    }
}

impl Default for KeyDerivation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let kdf = KeyDerivation::new();
        let password = b"super_secure_password";
        let salt = b"0123456789abcdef"; // 16 bytes

        let key = kdf.derive_key(password, salt).unwrap();
        assert_eq!(key.len(), 32); // Should be 32 bytes
    }

    #[test]
    fn test_key_derivation_deterministic() {
        let kdf = KeyDerivation::new();
        let password = b"test_password";
        let salt = b"fixed_salt_16byt";

        let key1 = kdf.derive_key(password, salt).unwrap();
        let key2 = kdf.derive_key(password, salt).unwrap();

        assert_eq!(key1, key2); // Same inputs = same output
    }

    #[test]
    fn test_different_passwords_different_keys() {
        let kdf = KeyDerivation::new();
        let salt = b"fixed_salt_16byt";

        let key1 = kdf.derive_key(b"password1", salt).unwrap();
        let key2 = kdf.derive_key(b"password2", salt).unwrap();

        assert_ne!(key1, key2); // Different passwords = different keys
    }

    #[test]
    fn test_different_salts_different_keys() {
        let kdf = KeyDerivation::new();
        let password = b"fixed_password";

        let key1 = kdf.derive_key(password, b"salt_version_01a").unwrap();
        let key2 = kdf.derive_key(password, b"salt_version_02b").unwrap();

        assert_ne!(key1, key2); // Different salts = different keys
    }

    #[test]
    fn test_salt_too_short() {
        let kdf = KeyDerivation::new();
        let password = b"password";
        let short_salt = b"short"; // Only 5 bytes

        let result = kdf.derive_key(password, short_salt);
        assert!(result.is_err());
    }

    #[test]
    #[ignore] // This test is slow (~2 seconds)
    fn test_key_derivation_timing() {
        let kdf = KeyDerivation::new();
        let password = b"test_password";
        let salt = b"0123456789abcdef";

        let (_key, duration) = kdf.derive_key_timed(password, salt).unwrap();
        
        // Argon2id with these params should take 1-3 seconds
        assert!(duration.as_secs() >= 1);
        assert!(duration.as_secs() <= 5);
        
        println!("Key derivation took: {:?}", duration);
    }
}
