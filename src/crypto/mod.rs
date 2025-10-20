//! Cryptography module - Key derivation, encryption, and watermarking

mod aes;
mod kdf;
mod watermark;

pub use aes::{AesEncryption, EncryptedData};
pub use kdf::KeyDerivation;
pub use watermark::{Watermark, WatermarkGenerator};

use anyhow::Result;
use rand::Rng;

/// Master cryptographic context
pub struct CryptoContext {
    kdf: KeyDerivation,
    aes: AesEncryption,
    watermark_gen: WatermarkGenerator,
}

impl CryptoContext {
    /// Create new crypto context with password
    pub fn new(password: &str, salt: Option<&[u8]>) -> Result<Self> {
        // Generate salt if not provided
        let salt = match salt {
            Some(s) => s.to_vec(),
            None => {
                let mut rng = rand::thread_rng();
                (0..32).map(|_| rng.gen::<u8>()).collect()
            }
        };

        // Derive master key using Argon2id
        let kdf = KeyDerivation::new();
        let master_key = kdf.derive_key(password.as_bytes(), &salt)?;

        // Initialize AES-256-GCM
        let aes = AesEncryption::new(&master_key)?;

        // Initialize watermark generator
        let watermark_gen = WatermarkGenerator::new();

        Ok(Self {
            kdf,
            aes,
            watermark_gen,
        })
    }

    /// Encrypt data
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData> {
        self.aes.encrypt(plaintext)
    }

    /// Decrypt data
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        self.aes.decrypt(encrypted)
    }

    /// Generate watermark for customer
    pub fn generate_watermark(&self, customer_id: &str, script_id: &str) -> Watermark {
        self.watermark_gen.generate(customer_id, script_id)
    }

    /// Verify watermark
    pub fn verify_watermark(&self, watermark: &Watermark, customer_id: &str) -> bool {
        self.watermark_gen.verify(watermark, customer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_context_creation() {
        let ctx = CryptoContext::new("test_password", None);
        assert!(ctx.is_ok());
    }

    #[test]
    fn test_encryption_decryption() {
        let ctx = CryptoContext::new("test_password", None).unwrap();
        let plaintext = b"Hello, World!";

        let encrypted = ctx.encrypt(plaintext).unwrap();
        let decrypted = ctx.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_watermark_generation() {
        let ctx = CryptoContext::new("test_password", None).unwrap();
        let watermark = ctx.generate_watermark("customer123", "script456");

        assert!(ctx.verify_watermark(&watermark, "customer123"));
        assert!(!ctx.verify_watermark(&watermark, "customer999"));
    }
}
