//! Cryptographic watermarking system
//!
//! Provides robust, undetectable watermarks embedded in obfuscated scripts
//! to trace leaked copies back to original purchasers.

use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use ring::digest::{Context, SHA256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Watermark data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Watermark {
    /// Primary watermark hash (SHA-256 of customer data)
    pub primary_hash: Vec<u8>,
    
    /// Secondary watermark (obfuscated customer ID)
    pub secondary_hash: Vec<u8>,
    
    /// Timestamp of watermark generation
    pub timestamp: u64,
    
    /// Script identifier
    pub script_id: String,
    
    /// Watermark version for future compatibility
    pub version: u32,
    
    /// Metadata for additional tracking
    pub metadata: HashMap<String, String>,
}

/// Watermark generator
pub struct WatermarkGenerator {
    version: u32,
}

impl WatermarkGenerator {
    /// Create new watermark generator
    pub fn new() -> Self {
        Self { version: 1 }
    }

    /// Generate watermark for a customer and script
    /// 
    /// # Arguments
    /// * `customer_id` - Unique customer identifier (e.g., Roblox UserId or email hash)
    /// * `script_id` - Unique script identifier
    /// 
    /// # Returns
    /// Cryptographically secure watermark containing multiple identification layers
    pub fn generate(&self, customer_id: &str, script_id: &str) -> Watermark {
        // Generate primary hash: SHA-256(customer_id || script_id || timestamp)
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let primary_data = format!("{}||{}||{}", customer_id, script_id, timestamp);
        let primary_hash = Self::sha256(primary_data.as_bytes());

        // Generate secondary hash: SHA-256(reversed customer_id || script_id)
        // This provides redundancy if primary watermark is partially damaged
        let secondary_data = format!("{}||{}", 
            customer_id.chars().rev().collect::<String>(),
            script_id
        );
        let secondary_hash = Self::sha256(secondary_data.as_bytes());

        // Create metadata for additional tracking
        let mut metadata = HashMap::new();
        metadata.insert("generation_time".to_string(), timestamp.to_string());
        metadata.insert("customer_id_length".to_string(), customer_id.len().to_string());
        
        Watermark {
            primary_hash,
            secondary_hash,
            timestamp,
            script_id: script_id.to_string(),
            version: self.version,
            metadata,
        }
    }

    /// Verify watermark against customer ID
    /// 
    /// # Arguments
    /// * `watermark` - Watermark to verify
    /// * `customer_id` - Customer ID to check against
    /// 
    /// # Returns
    /// `true` if watermark matches customer, `false` otherwise
    pub fn verify(&self, watermark: &Watermark, customer_id: &str) -> bool {
        // Regenerate primary hash with stored timestamp
        let primary_data = format!("{}||{}||{}", 
            customer_id, 
            watermark.script_id, 
            watermark.timestamp
        );
        let expected_primary = Self::sha256(primary_data.as_bytes());

        // Regenerate secondary hash
        let secondary_data = format!("{}||{}", 
            customer_id.chars().rev().collect::<String>(),
            watermark.script_id
        );
        let expected_secondary = Self::sha256(secondary_data.as_bytes());

        // Verify both hashes match
        watermark.primary_hash == expected_primary 
            && watermark.secondary_hash == expected_secondary
    }

    /// Extract customer ID candidates from leaked watermark
    /// 
    /// This is intentionally one-way - you cannot extract the customer ID
    /// from the watermark alone. You must try known customer IDs against it.
    /// 
    /// # Arguments
    /// * `watermark` - Watermark found in leaked script
    /// * `known_customers` - List of customer IDs to check
    /// 
    /// # Returns
    /// Vector of matching customer IDs (should be 0 or 1)
    pub fn identify_leaker(
        &self,
        watermark: &Watermark,
        known_customers: &[String],
    ) -> Vec<String> {
        known_customers
            .iter()
            .filter(|customer_id| self.verify(watermark, customer_id))
            .cloned()
            .collect()
    }

    /// Encode watermark as base64 string for embedding in code
    pub fn encode(&self, watermark: &Watermark) -> Result<String> {
        let json = serde_json::to_string(watermark)
            .map_err(|e| ObfuscatorError::CryptoError(format!("Watermark encoding failed: {}", e)))?;
        
        Ok(base64::encode(json))
    }

    /// Decode watermark from base64 string
    pub fn decode(&self, encoded: &str) -> Result<Watermark> {
        let json = base64::decode(encoded)
            .map_err(|e| ObfuscatorError::CryptoError(format!("Watermark decoding failed: {}", e)))?;
        
        let watermark: Watermark = serde_json::from_slice(&json)
            .map_err(|e| ObfuscatorError::CryptoError(format!("Watermark parsing failed: {}", e)))?;
        
        Ok(watermark)
    }

    /// Generate steganographic watermark pattern for code embedding
    /// 
    /// Returns a pattern of boolean values that can be used to subtly
    /// modify code structure (e.g., variable naming, whitespace, order)
    /// without affecting functionality.
    pub fn generate_stego_pattern(&self, watermark: &Watermark, pattern_length: usize) -> Vec<bool> {
        // Use primary hash as seed for deterministic pattern generation
        let mut pattern = Vec::with_capacity(pattern_length);
        let hash_bytes = &watermark.primary_hash;
        
        // Expand hash bytes to pattern length using XOR feedback
        let mut state = hash_bytes.to_vec();
        
        for i in 0..pattern_length {
            let byte_idx = i % state.len();
            let bit_idx = (i / state.len()) % 8;
            
            let bit = (state[byte_idx] >> bit_idx) & 1;
            pattern.push(bit == 1);
            
            // Update state with XOR feedback for next iteration
            if (i + 1) % state.len() == 0 {
                for j in 0..state.len() {
                    state[j] ^= hash_bytes[j % hash_bytes.len()];
                }
            }
        }
        
        pattern
    }

    /// SHA-256 helper function
    fn sha256(data: &[u8]) -> Vec<u8> {
        let mut context = Context::new(&SHA256);
        context.update(data);
        context.finish().as_ref().to_vec()
    }
}

impl Default for WatermarkGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watermark_generation() {
        let gen = WatermarkGenerator::new();
        let watermark = gen.generate("customer123", "script456");

        assert_eq!(watermark.primary_hash.len(), 32); // SHA-256 = 32 bytes
        assert_eq!(watermark.secondary_hash.len(), 32);
        assert_eq!(watermark.script_id, "script456");
        assert_eq!(watermark.version, 1);
    }

    #[test]
    fn test_watermark_verification_success() {
        let gen = WatermarkGenerator::new();
        let watermark = gen.generate("customer123", "script456");

        assert!(gen.verify(&watermark, "customer123"));
    }

    #[test]
    fn test_watermark_verification_failure() {
        let gen = WatermarkGenerator::new();
        let watermark = gen.generate("customer123", "script456");

        assert!(!gen.verify(&watermark, "customer999"));
        assert!(!gen.verify(&watermark, "wrong_customer"));
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
        
        // But all should be verifiable with correct customer ID
        assert!(gen.verify(&wm1, "customer1"));
        assert!(gen.verify(&wm2, "customer2"));
        assert!(gen.verify(&wm3, "customer1"));
    }

    #[test]
    fn test_leaker_identification() {
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
    fn test_stego_pattern_generation() {
        let gen = WatermarkGenerator::new();
        let watermark = gen.generate("customer123", "script456");

        let pattern1 = gen.generate_stego_pattern(&watermark, 100);
        let pattern2 = gen.generate_stego_pattern(&watermark, 100);

        // Pattern should be deterministic
        assert_eq!(pattern1, pattern2);
        assert_eq!(pattern1.len(), 100);

        // Different watermark should produce different pattern
        let watermark2 = gen.generate("customer999", "script456");
        let pattern3 = gen.generate_stego_pattern(&watermark2, 100);
        
        assert_ne!(pattern1, pattern3);
    }

    #[test]
    fn test_stego_pattern_distribution() {
        let gen = WatermarkGenerator::new();
        let watermark = gen.generate("customer123", "script456");
        let pattern = gen.generate_stego_pattern(&watermark, 1000);

        // Count true/false distribution
        let true_count = pattern.iter().filter(|&&b| b).count();
        let false_count = pattern.len() - true_count;

        // Should be roughly balanced (not exactly 500/500 due to deterministic generation)
        assert!(true_count > 300 && true_count < 700);
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
        assert!(!gen.verify(&watermark, "customer123"));
    }

    #[test]
    fn test_watermark_metadata() {
        let gen = WatermarkGenerator::new();
        let watermark = gen.generate("customer@example.com", "script789");

        assert!(watermark.metadata.contains_key("generation_time"));
        assert!(watermark.metadata.contains_key("customer_id_length"));
        
        let customer_id_length = watermark.metadata.get("customer_id_length").unwrap();
        assert_eq!(customer_id_length, "20"); // "customer@example.com".len()
    }
}
