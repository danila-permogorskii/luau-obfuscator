//! Final script assembly

use crate::crypto::{EncryptedData, Watermark};
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use base64::Engine;

/// Assembles the final protected Luau script
pub struct ScriptAssembler;

impl ScriptAssembler {
    pub fn new() -> Self {
        Self
    }
    
    /// Assemble final script from all components
    pub fn assemble(
        &self,
        runtime: Option<String>,
        license: Option<String>,
        hwid: Option<String>,
        encrypted_strings: &[(String, EncryptedData)],
        obfuscated_code: &str,
        watermark: Watermark,
    ) -> Result<String> {
        let mut output = String::new();
        
        // Header
        output.push_str("-- Protected by Luau Obfuscator\n");
        output.push_str("-- https://github.com/danila-permogorskii/luau-obfuscator\n");
        output.push_str("\n");
        
        // Watermark (hidden in comment)
        output.push_str(&format!("-- Watermark: {}\n", self.encode_watermark(&watermark)));
        output.push_str("\n");
        
        // ChaCha20 Runtime
        if let Some(runtime_code) = runtime {
            output.push_str("-- [RUNTIME] ChaCha20 Decryption\n");
            output.push_str(&runtime_code);
            output.push_str("\n\n");
        }
        
        // License Validation
        if let Some(license_code) = license {
            output.push_str("-- [LICENSE] License Validation\n");
            output.push_str(&license_code);
            output.push_str("\n\n");
        }
        
        // HWID Binding
        if let Some(hwid_code) = hwid {
            output.push_str("-- [HWID] Hardware ID Binding\n");
            output.push_str(&hwid_code);
            output.push_str("\n\n");
        }
        
        // Encrypted Data Structures
        if !encrypted_strings.is_empty() {
            output.push_str("-- [DATA] Encrypted Strings\n");
            output.push_str("local _encrypted_data = {\n");
            
            for (i, (original, encrypted)) in encrypted_strings.iter().enumerate() {
                let ciphertext_b64 = base64::engine::general_purpose::STANDARD
                    .encode(&encrypted.ciphertext);
                let nonce_b64 = base64::engine::general_purpose::STANDARD
                    .encode(&encrypted.nonce);
                
                output.push_str(&format!(
                    "    [{}] = {{ct = \"{}\", nonce = \"{}\"}},\n",
                    i + 1,
                    ciphertext_b64,
                    nonce_b64
                ));
            }
            
            output.push_str("}\n\n");
            
            // Decryption helper
            output.push_str("-- Decrypt string by index\n");
            output.push_str("local function _decrypt(index)\n");
            output.push_str("    local data = _encrypted_data[index]\n");
            output.push_str("    if not data then return nil end\n");
            output.push_str("    return ChaCha20.decrypt_string(data.ct, _key, data.nonce)\n");
            output.push_str("end\n\n");
        }
        
        // Validation Startup
        output.push_str("-- [INIT] Startup Validation\n");
        output.push_str("do\n");
        if license.is_some() {
            output.push_str("    local license_module = require(script.License)\n");
            output.push_str("    assert(license_module.validate(), \"License validation failed\")\n");
        }
        if hwid.is_some() {
            output.push_str("    local hwid_module = require(script.HWID)\n");
            output.push_str("    assert(hwid_module.validate(), \"HWID validation failed\")\n");
        }
        output.push_str("end\n\n");
        
        // Obfuscated Original Code
        output.push_str("-- [CODE] Protected Script\n");
        output.push_str(obfuscated_code);
        output.push_str("\n");
        
        Ok(output)
    }
    
    /// Encode watermark for embedding
    fn encode_watermark(&self, watermark: &Watermark) -> String {
        // Convert watermark to base64
        let watermark_str = format!("{:?}", watermark);
        base64::engine::general_purpose::STANDARD.encode(watermark_str.as_bytes())
    }
}

impl Default for ScriptAssembler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly_basic() {
        let assembler = ScriptAssembler::new();
        let watermark = Watermark {
            customer_id: "test123".to_string(),
            script_id: "script456".to_string(),
            hash: vec![1, 2, 3, 4],
        };
        
        let result = assembler.assemble(
            None,
            None,
            None,
            &[],
            "print('Hello, World!')",
            watermark,
        );
        
        assert!(result.is_ok());
        let script = result.unwrap();
        assert!(script.contains("Protected by Luau Obfuscator"));
        assert!(script.contains("print('Hello, World!')"));
    }
}