//! HTTP client for API communication

use super::models::*;
use crate::utils::errors::ObfuscatorError;
use anyhow::{Context, Result};
use log::{debug, warn};
use reqwest::blocking::Client;
use std::time::Duration;

/// API client for license validation and management
pub struct ApiClient {
    client: Client,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.into(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        })
    }

    /// Create a client with custom timeout
    pub fn with_timeout(base_url: impl Into<String>, timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.into(),
            timeout,
            max_retries: 3,
        })
    }

    /// Validate a license key
    pub fn validate_license(&self, request: ValidateLicenseRequest) -> Result<ValidateLicenseResponse> {
        let url = format!("{}/api/v1/validate-license", self.base_url);
        
        debug!("Validating license with API: {}", url);
        
        self.retry_request(|| {
            let response = self.client
                .post(&url)
                .json(&request)
                .send()
                .context("Failed to send validate license request")?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ObfuscatorError::ApiError(
                    format!("API returned error {}: {}", status, error_text)
                ).into());
            }

            let result: ValidateLicenseResponse = response
                .json()
                .context("Failed to parse validate license response")?;

            Ok(result)
        })
    }

    /// Generate a new license key
    pub fn generate_license(&self, request: GenerateLicenseRequest) -> Result<GenerateLicenseResponse> {
        let url = format!("{}/api/v1/generate-license", self.base_url);
        
        debug!("Generating license with API: {}", url);
        
        self.retry_request(|| {
            let response = self.client
                .post(&url)
                .json(&request)
                .send()
                .context("Failed to send generate license request")?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ObfuscatorError::ApiError(
                    format!("API returned error {}: {}", status, error_text)
                ).into());
            }

            let result: GenerateLicenseResponse = response
                .json()
                .context("Failed to parse generate license response")?;

            Ok(result)
        })
    }

    /// Track an obfuscation event
    pub fn track_obfuscation(&self, request: TrackObfuscationRequest) -> Result<TrackObfuscationResponse> {
        let url = format!("{}/api/v1/track-obfuscation", self.base_url);
        
        debug!("Tracking obfuscation event: {}", url);
        
        // Don't retry tracking requests - fail fast
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .context("Failed to send track obfuscation request")?;

        if !response.status().is_success() {
            warn!("Failed to track obfuscation event: {}", response.status());
            return Ok(TrackObfuscationResponse {
                success: false,
                event_id: None,
            });
        }

        let result: TrackObfuscationResponse = response
            .json()
            .context("Failed to parse track obfuscation response")?;

        Ok(result)
    }

    /// Retry a request with exponential backoff
    fn retry_request<T, F>(&self, mut f: F) -> Result<T>
    where
        F: FnMut() -> Result<T>,
    {
        let mut last_error = None;
        
        for attempt in 1..=self.max_retries {
            match f() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    warn!("API request attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.max_retries {
                        // Exponential backoff: 1s, 2s, 4s
                        let delay = Duration::from_secs(2u64.pow(attempt - 1));
                        debug!("Retrying in {:?}...", delay);
                        std::thread::sleep(delay);
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| {
            ObfuscatorError::ApiError("All retry attempts failed".to_string()).into()
        }))
    }

    /// Check if API is reachable (health check)
    pub fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);
        
        match self.client.get(&url).send() {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_client_creation() {
        let client = ApiClient::new("https://api.example.com");
        assert!(client.is_ok());
    }

    #[test]
    fn test_api_client_with_timeout() {
        let client = ApiClient::with_timeout(
            "https://api.example.com",
            Duration::from_secs(10)
        );
        assert!(client.is_ok());
        assert_eq!(client.unwrap().timeout, Duration::from_secs(10));
    }

    // Note: Integration tests with mocked server would go in tests/integration/
}
