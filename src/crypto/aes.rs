//! AES-256-GCM encryption

use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use ring::{
    aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM},
    error::Unspecified,
    rand::{SecureRandom, SystemRandom},
};
use serde::{Deserialize, Serialize};

/// Encrypted data with nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Ciphertext
    pub ciphertext: Vec<u8>,
    /// Nonce used for encryption (12 bytes)
    pub nonce: Vec<u8>,
    /// Authentication tag (16 bytes, included in ciphertext by ring)
    pub tag_len: usize,
}

/// Custom nonce sequence for ring
struct CounterNonceSequence {
    nonce: [u8; 12],
}

impl CounterNonceSequence {
    fn new(nonce: &[u8]) -> Result<Self> {
        if nonce.len() != 12 {
            return Err(ObfuscatorError::CryptoError(
                "Nonce must be exactly 12 bytes".to_string(),
            )
            .into());
        }

        let mut nonce_arr = [0u8; 12];
        nonce_arr.copy_from_slice(nonce);

        Ok(Self { nonce: nonce_arr })
    }
}

impl NonceSequence for CounterNonceSequence {
    fn advance(&mut self) -> std::result::Result<Nonce, Unspecified> {
        Nonce::try_assume_unique_for_key(&self.nonce)
    }
}

/// AES-256-GCM encryption
pub struct AesEncryption {
    key: Vec<u8>,
    rng: SystemRandom,
}

impl AesEncryption {
    /// Create new AES-256-GCM encryptor with 32-byte key
    pub fn new(key: &[u8]) -> Result<Self> {
        if key.len() != 32 {
            return Err(ObfuscatorError::CryptoError(
                "AES-256 key must be exactly 32 bytes".to_string(),
            )
            .into());
        }

        Ok(Self {
            key: key.to_vec(),
            rng: SystemRandom::new(),
        })
    }

    /// Encrypt plaintext
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData> {
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|_| ObfuscatorError::CryptoError("Failed to generate nonce".to_string()))?;

        // Create unbound key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(|_| ObfuscatorError::CryptoError("Invalid key".to_string()))?;

        // Create nonce sequence
        let nonce_sequence = CounterNonceSequence::new(&nonce_bytes)?;

        // Create sealing key
        let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);

        // Prepare data for encryption (plaintext + space for tag)
        let mut in_out = plaintext.to_vec();

        // Seal (encrypt + authenticate)
        sealing_key
            .seal_in_place_append_tag(Aad::empty(), &mut in_out)
            .map_err(|_| ObfuscatorError::CryptoError("Encryption failed".to_string()))?;

        Ok(EncryptedData {
            ciphertext: in_out,
            nonce: nonce_bytes.to_vec(),
            tag_len: AES_256_GCM.tag_len(),
        })
    }

    /// Decrypt ciphertext
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        // Create unbound key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(|_| ObfuscatorError::CryptoError("Invalid key".to_string()))?;

        // Create nonce sequence
        let nonce_sequence = CounterNonceSequence::new(&encrypted.nonce)?;

        // Create opening key
        let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

        // Prepare data for decryption
        let mut in_out = encrypted.ciphertext.clone();

        // Open (decrypt + verify)
        let plaintext = opening_key
            .open_in_place(Aad::empty(), &mut in_out)
            .map_err(|_| {
                ObfuscatorError::CryptoError("Decryption failed (wrong key or corrupted data)".to_string())
            })?;

        Ok(plaintext.to_vec())
    }

    /// Encrypt multiple items (batch operation)
    pub fn encrypt_batch(&self, plaintexts: &[&[u8]]) -> Result<Vec<EncryptedData>> {
        plaintexts.iter().map(|p| self.encrypt(p)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_key() -> Vec<u8> {
        vec![0u8; 32] // 32 zero bytes for testing
    }

    #[test]
    fn test_aes_creation() {
        let key = get_test_key();
        let aes = AesEncryption::new(&key);
        assert!(aes.is_ok());
    }

    #[test]
    fn test_aes_wrong_key_size() {
        let short_key = vec![0u8; 16]; // Only 16 bytes
        let result = AesEncryption::new(&short_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = get_test_key();
        let aes = AesEncryption::new(&key).unwrap();
        let plaintext = b"Hello, World!";

        let encrypted = aes.encrypt(plaintext).unwrap();
        let decrypted = aes.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_encrypt_produces_different_nonces() {
        let key = get_test_key();
        let aes = AesEncryption::new(&key).unwrap();
        let plaintext = b"Same plaintext";

        let encrypted1 = aes.encrypt(plaintext).unwrap();
        let encrypted2 = aes.encrypt(plaintext).unwrap();

        // Different nonces for same plaintext
        assert_ne!(encrypted1.nonce, encrypted2.nonce);
        // Different ciphertexts due to different nonces
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    }

    #[test]
    fn test_wrong_key_decrypt_fails() {
        let key1 = get_test_key();
        let key2 = vec![1u8; 32]; // Different key

        let aes1 = AesEncryption::new(&key1).unwrap();
        let aes2 = AesEncryption::new(&key2).unwrap();

        let plaintext = b"Secret message";
        let encrypted = aes1.encrypt(plaintext).unwrap();

        // Decryption with wrong key should fail
        let result = aes2.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_corrupted_ciphertext_fails() {
        let key = get_test_key();
        let aes = AesEncryption::new(&key).unwrap();
        let plaintext = b"Original message";

        let mut encrypted = aes.encrypt(plaintext).unwrap();

        // Corrupt the ciphertext
        encrypted.ciphertext[0] ^= 0xFF;

        // Decryption should fail due to authentication
        let result = aes.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_plaintext() {
        let key = get_test_key();
        let aes = AesEncryption::new(&key).unwrap();
        let plaintext = b"";

        let encrypted = aes.encrypt(plaintext).unwrap();
        let decrypted = aes.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_large_plaintext() {
        let key = get_test_key();
        let aes = AesEncryption::new(&key).unwrap();
        let plaintext = vec![42u8; 10000]; // 10KB of data

        let encrypted = aes.encrypt(&plaintext).unwrap();
        let decrypted = aes.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_batch_encryption() {
        let key = get_test_key();
        let aes = AesEncryption::new(&key).unwrap();

        let plaintexts: Vec<&[u8]> = vec![b"first", b"second", b"third"];
        let encrypted_batch = aes.encrypt_batch(&plaintexts).unwrap();

        assert_eq!(encrypted_batch.len(), 3);

        // Verify each can be decrypted
        for (i, encrypted) in encrypted_batch.iter().enumerate() {
            let decrypted = aes.decrypt(encrypted).unwrap();
            assert_eq!(plaintexts[i], decrypted.as_slice());
        }
    }
}
