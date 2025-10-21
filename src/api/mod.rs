//! API client module for license validation and management
//!
//! This module provides HTTP client functionality to interact with the
//! license validation and management API.

mod client;
mod models;

pub use client::ApiClient;
pub use models::{
    ErrorResponse, GenerateLicenseRequest, GenerateLicenseResponse, LicenseMetadata,
    TrackObfuscationRequest, TrackObfuscationResponse, ValidateLicenseRequest,
    ValidateLicenseResponse,
};

use anyhow::Result;

/// Default API endpoint (can be overridden via config or CLI)
pub const DEFAULT_API_ENDPOINT: &str = "https://api.luau-obfuscator.com";

/// Create a default API client
pub fn create_default_client() -> Result<ApiClient> {
    ApiClient::new(DEFAULT_API_ENDPOINT)
}

/// Create an API client with custom endpoint
pub fn create_client(endpoint: impl Into<String>) -> Result<ApiClient> {
    ApiClient::new(endpoint)
}
