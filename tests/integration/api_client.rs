//! Integration tests for API client functionality
//!
//! Note: These tests use mocked responses. For real API testing,
//! you would need to set up a test server or use a mocking library like mockito.

use luau_obfuscator::api::{
    ApiClient, GenerateLicenseRequest, TrackObfuscationRequest, ValidateLicenseRequest,
};
use std::time::Duration;

#[test]
fn test_api_client_creation() {
    let client = ApiClient::new("https://api.example.com");
    assert!(client.is_ok());
}

#[test]
fn test_api_client_with_custom_timeout() {
    let client = ApiClient::with_timeout("https://api.example.com", Duration::from_secs(10));
    assert!(client.is_ok());
}

#[test]
fn test_validate_license_request_structure() {
    let request = ValidateLicenseRequest {
        api_key: "test_key\".to_string(),
        license_key: \"XXXX-XXXX-XXXX-XXXX\".to_string(),
        script_id: \"test_script\".to_string(),
        hwid: Some(\"123456\".to_string()),
        watermark: Some(\"abc123\".to_string()),
    };

    // Verify structure is serializable
    let json = serde_json::to_string(&request);
    assert!(json.is_ok());
    
    let json_str = json.unwrap();
    assert!(json_str.contains(\"test_key\"));
    assert!(json_str.contains(\"XXXX-XXXX-XXXX-XXXX\"));
}

#[test]
fn test_generate_license_request_structure() {
    let request = GenerateLicenseRequest {
        api_key: \"dev_key\".to_string(),
        script_id: \"my_script\".to_string(),
        buyer_userid: 987654321,
        expiration: Some(\"2026-01-01T00:00:00Z\".to_string()),
        tier: Some(\"premium\".to_string()),
        hwid_restrictions: Some(vec![\"userid:123\".to_string()]),
    };

    // Verify structure is serializable
    let json = serde_json::to_string(&request);
    assert!(json.is_ok());
    
    let json_str = json.unwrap();
    assert!(json_str.contains(\"my_script\"));
    assert!(json_str.contains(\"987654321\"));
}

#[test]
fn test_track_obfuscation_request_structure() {
    let request = TrackObfuscationRequest {
        api_key: \"test_key\".to_string(),
        script_id: \"test_script\".to_string(),
        license_key: \"XXXX-XXXX\".to_string(),
        tier: \"standard\".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        metadata: None,
    };

    // Verify structure is serializable
    let json = serde_json::to_string(&request);
    assert!(json.is_ok());
}

// Note: Actual integration tests with a mocked server would require
// a library like mockito or wiremock. These tests just verify the
// basic structure and serialization of requests.
//
// Example with mockito (not implemented):
//
// #[test]
// fn test_validate_license_with_mock_server() {
//     let _m = mock(\"POST\", \"/api/v1/validate-license\")
//         .with_status(200)
//         .with_header(\"content-type\", \"application/json\")
//         .with_body(r#\"{\"valid\":true,\"error\":null}\"#)
//         .create();
//
//     let client = ApiClient::new(mockito::server_url()).unwrap();
//     let request = ValidateLicenseRequest { /* ... */ };
//     let response = client.validate_license(request).unwrap();
//     
//     assert!(response.valid);
// }
