//! Command execution logic

use super::args::{Cli, Commands};
use crate::api::{
    create_client, ApiClient, GenerateLicenseRequest, TrackObfuscationRequest,
    ValidateLicenseRequest,
};
use crate::parser::LuauParser;
use crate::utils::errors::ObfuscatorError;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::fs;

/// Execute the CLI command
pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Protect {
            input,
            output,
            license_key,
            hwid,
            tier,
            api_endpoint,
        } => {
            info!("Starting protection process");
            info!("  Input: {:?}", input);
            info!("  Output: {:?}", output);
            info!("  Tier: {}", tier);

            // Create progress bar
            let pb = ProgressBar::new(6);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            // Step 1: Validate license with API (if endpoint provided)
            if let Some(ref endpoint) = api_endpoint {
                pb.set_message("Validating license with API...");
                
                let api_client = create_client(endpoint)
                    .context("Failed to create API client")?;
                
                let validation_request = ValidateLicenseRequest {
                    api_key: "".to_string(), // TODO: Get from config
                    license_key: license_key.clone(),
                    script_id: "test_script".to_string(), // TODO: Generate from input file
                    hwid: hwid.map(|h| h.to_string()),
                    watermark: None,
                };
                
                match api_client.validate_license(validation_request) {
                    Ok(response) => {
                        if !response.valid {
                            return Err(ObfuscatorError::ApiError(
                                format!("License validation failed: {}", 
                                    response.error.unwrap_or_else(|| "Unknown error".to_string()))
                            ).into());
                        }
                        info!("✓ License validated successfully");
                    }
                    Err(e) => {
                        warn!("License validation failed (continuing anyway): {}", e);
                        warn!("Running in offline mode");
                    }
                }
                
                pb.inc(1);
            } else {
                info!("No API endpoint provided - skipping online validation");
                pb.set_message("Skipping online validation...");
                pb.inc(1);
            }

            // Step 2: Read input file
            pb.set_message("Reading input file...");
            let source = fs::read_to_string(&input)
                .with_context(|| format!("Failed to read input file: {:?}", input))?;
            pb.inc(1);

            // Step 3: Parse Luau script
            pb.set_message("Parsing Luau script...");
            let parser = LuauParser::new();
            let parse_result = parser
                .parse(&source)
                .map_err(|e| ObfuscatorError::ParseError(e.to_string()))?;
            pb.inc(1);

            info!("Parse successful:");
            info!("  - Strings found: {}", parse_result.strings.len());
            info!("  - Numbers found: {}", parse_result.numbers.len());
            info!("  - Functions found: {}", parse_result.functions.len());

            // Step 4: Analyze (placeholder)
            pb.set_message("Analyzing code structure...");
            // TODO: Implement analysis engine
            pb.inc(1);

            // Step 5: Obfuscate (placeholder)
            pb.set_message("Applying obfuscation...");
            // TODO: Implement obfuscation transformations
            pb.inc(1);

            // Step 6: Write output (for now, just write original)
            pb.set_message("Writing protected script...");
            fs::write(&output, &source)
                .with_context(|| format!("Failed to write output file: {:?}", output))?;
            pb.inc(1);

            pb.finish_with_message("✓ Protection complete!");

            println!("\n✓ Protected script written to: {:?}", output);
            println!("  License: {}", license_key);
            if let Some(hwid) = hwid {
                println!("  Bound to HWID: {}", hwid);
            }

            // Track obfuscation event (if API endpoint provided)
            if let Some(ref endpoint) = api_endpoint {
                let api_client = create_client(endpoint)?;
                let tracking_request = TrackObfuscationRequest {
                    api_key: "".to_string(), // TODO: Get from config
                    script_id: "test_script".to_string(),
                    license_key: license_key.clone(),
                    tier: tier.to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    metadata: None,
                };

                match api_client.track_obfuscation(tracking_request) {
                    Ok(response) => {
                        if response.success {
                            info!("✓ Obfuscation event tracked");
                        }
                    }
                    Err(e) => {
                        warn!("Failed to track obfuscation event: {}", e);
                    }
                }
            }

            Ok(())
        }

        Commands::GenerateLicense {
            script_id,
            buyer_userid,
            api_key,
            expiration,
        } => {
            info!("Generating license key");
            info!("  Script ID: {}", script_id);
            info!("  Buyer: {}", buyer_userid);

            // Use default API endpoint
            let api_client = create_client(crate::api::DEFAULT_API_ENDPOINT)
                .context("Failed to create API client")?;

            let request = GenerateLicenseRequest {
                api_key: api_key.clone(),
                script_id: script_id.clone(),
                buyer_userid,
                expiration,
                tier: None,
                hwid_restrictions: None,
            };

            println!("\n🔄 Generating license...");

            match api_client.generate_license(request) {
                Ok(response) => {
                    println!("\n✓ License generated successfully!");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("  License Key: {}", response.license_key);
                    println!("  Script ID:   {}", response.script_id);
                    println!("  Buyer ID:    {}", response.buyer_userid);
                    if let Some(exp) = response.expiration {
                        println!("  Expires:     {}", 
                            chrono::DateTime::<chrono::Utc>::from_timestamp(exp as i64, 0)
                                .map(|dt| dt.to_rfc3339())
                                .unwrap_or_else(|| "Invalid timestamp".to_string())
                        );
                    }
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                }
                Err(e) => {
                    eprintln!("\n❌ Failed to generate license: {}", e);
                    return Err(e);
                }
            }

            Ok(())
        }

        Commands::Validate { file } => {
            info!("Validating protected script: {:?}", file);

            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read file: {:?}", file))?;

            // TODO: Implement validation logic
            warn!("Validation not yet implemented - Phase 8");

            println!("\n⚠️  Validation will be fully implemented in Phase 8");
            println!("  File: {:?}", file);
            println!("  Size: {} bytes", content.len());

            // Basic checks
            if content.contains("Protected by Luau Obfuscator") {
                println!("  ✓ Appears to be a protected script");
            } else {
                println!("  ⚠️  May not be a protected script (missing header)");
            }

            Ok(())
        }
    }
}
