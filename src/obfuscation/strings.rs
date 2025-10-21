//! String encryption obfuscation

use super::{EncryptedString, ObfuscatedConstant};
use crate::crypto::{CryptoContext, EncryptedData};
use crate::parser::{Sensitivity, StringLiteral};
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};

/// String obfuscator using encryption
pub struct StringObfuscator<'a> {
    crypto_ctx: &'a CryptoContext,
}

impl<'a> StringObfuscator<'a> {
    pub fn new(crypto_ctx: &'a CryptoContext) -> Self {
        Self { crypto_ctx }
    }

    /// Obfuscate string literals
    pub fn obfuscate(
        &self,
        strings: &[StringLiteral],
        encrypt_all: bool,
    ) -> Result<Vec<EncryptedString>> {
        let mut encrypted_strings = Vec::new();

        for string_lit in strings {
            // Determine if this string should be encrypted
            let should_encrypt = encrypt_all
                || matches!(
                    string_lit.sensitivity,
                    Sensitivity::High | Sensitivity::Medium
                );

            if should_encrypt {
                let encrypted = self.encrypt_string(string_lit)?;
                encrypted_strings.push(encrypted);
            }
        }

        log::debug!("Encrypted {} strings", encrypted_strings.len());
        Ok(encrypted_strings)
    }

    /// Encrypt a single string
    fn encrypt_string(&self, string_lit: &StringLiteral) -> Result<EncryptedString> {
        let plaintext = string_lit.value.as_bytes();
        let encrypted_data = self.crypto_ctx.encrypt(plaintext)?;

        // Generate unique ID for this encrypted string
        let id = Self::generate_string_id();

        Ok(EncryptedString {
            original: string_lit.value.clone(),
            encrypted_data: encrypted_data.ciphertext,
            nonce: encrypted_data.nonce,
            line: string_lit.line,
            id,
        })
    }

    /// Generate unique identifier for encrypted string
    fn generate_string_id() -> String {
        let random_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        
        format!("_S{}", random_suffix)
    }

    /// Generate Luau code to decrypt string at runtime
    pub fn generate_decrypt_call(encrypted: &EncryptedString) -> String {
        // This will be replaced with actual ChaCha20 decrypt call in codegen
        format!(
            "_decrypt(\"{}\")",
            base64::encode(&encrypted.encrypted_data)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::CryptoContext;
    use crate::parser::{Sensitivity, StringLiteral};

    #[test]
    fn test_string_encryption() {
        let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
        let obfuscator = StringObfuscator::new(&crypto_ctx);

        let string_lit = StringLiteral {
            value: "Hello, World!".to_string(),
            line: 1,
            column: 0,
            sensitivity: Sensitivity::High,
        };

        let encrypted = obfuscator.encrypt_string(&string_lit).unwrap();
        
        assert_eq!(encrypted.original, "Hello, World!");
        assert!(!encrypted.encrypted_data.is_empty());
        assert_eq!(encrypted.nonce.len(), 12); // AES-GCM nonce is 12 bytes
        assert!(encrypted.id.starts_with("_S"));
    }

    #[test]
    fn test_selective_encryption() {
        let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
        let obfuscator = StringObfuscator::new(&crypto_ctx);

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
        ];

        // Don't encrypt all - only high/medium sensitivity
        let encrypted = obfuscator.obfuscate(&strings, false).unwrap();
        
        // Should only encrypt the high-sensitivity string
        assert_eq!(encrypted.len(), 1);
        assert_eq!(encrypted[0].original, "RemoteEvent");
    }

    #[test]
    fn test_encrypt_all_strings() {
        let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
        let obfuscator = StringObfuscator::new(&crypto_ctx);

        let strings = vec![
            StringLiteral {
                value: "String 1".to_string(),
                line: 1,
                column: 0,
                sensitivity: Sensitivity::Low,
            },
            StringLiteral {
                value: "String 2".to_string(),
                line: 2,
                column: 0,
                sensitivity: Sensitivity::Medium,
            },
        ];

        // Encrypt all strings regardless of sensitivity
        let encrypted = obfuscator.obfuscate(&strings, true).unwrap();
        
        assert_eq!(encrypted.len(), 2);
    }

    #[test]
    fn test_decrypt_call_generation() {
        let encrypted = EncryptedString {
            original: "test".to_string(),
            encrypted_data: vec![1, 2, 3, 4],
            nonce: vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            line: 1,
            id: "_S12345678".to_string(),
        };

        let decrypt_call = StringObfuscator::generate_decrypt_call(&encrypted);
        
        assert!(decrypt_call.starts_with("_decrypt("));
        assert!(decrypt_call.contains("\"AQIDBA==")); // base64 of [1,2,3,4]
    }
}