//! API request and response models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to validate a license key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateLicenseRequest {
    /// Developer API key
    pub api_key: String,
    /// License key to validate
    pub license_key: String,
    /// Script identifier
    pub script_id: String,
    /// Hardware ID (Roblox UserId or PlaceId)
    pub hwid: Option<String>,
    /// Watermark for tracking
    pub watermark: Option<String>,
}

/// Response from license validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateLicenseResponse {
    /// Whether the license is valid
    pub valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
    /// License metadata
    pub metadata: Option<LicenseMetadata>,
}

/// License metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseMetadata {
    /// Buyer's Roblox UserId
    pub buyer_userid: u64,
    /// Script identifier
    pub script_id: String,
    /// Expiration timestamp (Unix epoch)
    pub expiration: Option<u64>,
    /// License creation timestamp
    pub created_at: u64,
    /// License tier
    pub tier: String,
}

/// Request to generate a new license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateLicenseRequest {
    /// Developer API key
    pub api_key: String,
    /// Unique script identifier
    pub script_id: String,
    /// Buyer's Roblox UserId
    pub buyer_userid: u64,
    /// Optional expiration date (ISO 8601 format)
    pub expiration: Option<String>,
    /// Optional license tier
    pub tier: Option<String>,
    /// Optional HWID restrictions
    pub hwid_restrictions: Option<Vec<String>>,
}

/// Response from license generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateLicenseResponse {
    /// Generated license key
    pub license_key: String,
    /// Script identifier
    pub script_id: String,
    /// Buyer UserId
    pub buyer_userid: u64,
    /// License creation timestamp
    pub created_at: u64,
    /// Expiration timestamp (if applicable)
    pub expiration: Option<u64>,
}

/// Request to track obfuscation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackObfuscationRequest {
    /// Developer API key
    pub api_key: String,
    /// Script identifier
    pub script_id: String,
    /// License key used
    pub license_key: String,
    /// Obfuscation tier
    pub tier: String,
    /// Timestamp (ISO 8601 format)
    pub timestamp: String,
    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Response from tracking obfuscation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackObfuscationResponse {
    /// Whether tracking was successful
    pub success: bool,
    /// Event ID
    pub event_id: Option<String>,
}

/// Generic error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Additional details
    pub details: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_license_request_serialization() {
        let request = ValidateLicenseRequest {
            api_key: "test_key".to_string(),
            license_key: "XXXX-XXXX-XXXX-XXXX".to_string(),
            script_id: "test_script".to_string(),
            hwid: Some("123456".to_string()),
            watermark: Some("abc123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test_key"));
        assert!(json.contains("XXXX-XXXX-XXXX-XXXX"));
    }

    #[test]
    fn test_validate_license_response_deserialization() {
        let json = r#"{
            "valid": true,
            "error": null,
            "metadata": {
                "buyer_userid": 123456789,
                "script_id": "test_script",
                "expiration": null,
                "created_at": 1700000000,
                "tier": "standard"
            }
        }"#;

        let response: ValidateLicenseResponse = serde_json::from_str(json).unwrap();
        assert!(response.valid);
        assert!(response.metadata.is_some());
    }

    #[test]
    fn test_generate_license_request() {
        let request = GenerateLicenseRequest {
            api_key: "dev_key".to_string(),
            script_id: "my_script".to_string(),
            buyer_userid: 987654321,
            expiration: Some("2026-01-01T00:00:00Z".to_string()),
            tier: Some("premium".to_string()),
            hwid_restrictions: Some(vec!["userid:123".to_string()]),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("my_script"));
        assert!(json.contains("987654321"));
    }
}
