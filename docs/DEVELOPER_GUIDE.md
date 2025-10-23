# Luau Obfuscator - Developer Integration Guide

Comprehensive guide for integrating Luau Obfuscator into your development workflow and setting up the license validation API.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [API Integration](#api-integration)
3. [Rust Module Documentation](#rust-module-documentation)
4. [Custom Obfuscation Plugins](#custom-obfuscation-plugins)
5. [CI/CD Integration](#cicd-integration)
6. [Performance Optimization](#performance-optimization)
7. [Security Considerations](#security-considerations)

---

## Architecture Overview

### System Components

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Frontend (clap)                     │
│  • Argument parsing                                          │
│  • Command routing                                           │
│  • User interaction                                          │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  API Client (reqwest)                        │
│  • License validation                                        │
│  • License generation                                        │
│  • Analytics tracking                                        │
│  • Retry logic with exponential backoff                     │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│              Luau Parser (full_moon)                         │
│  • AST generation                                            │
│  • Syntax validation                                         │
│  • Token analysis                                            │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│                 Analysis Engine                              │
│  • String literal extraction                                 │
│  • Constant identification                                   │
│  • Control flow mapping                                      │
│  • Scope analysis                                            │
│  • Roblox API detection                                      │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│          Cryptography Module (ring + argon2)                 │
│  • AES-256-GCM encryption                                    │
│  • Argon2id key derivation                                   │
│  • Watermark generation                                      │
│  • Cryptographically secure RNG                              │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│           Obfuscation Transformations                        │
│  • String encryption                                         │
│  • Constant obfuscation                                      │
│  • Name mangling                                             │
│  • Control flow flattening                                   │
│  • Dead code injection                                       │
│  • Tier-based application                                    │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│      Code Generation (Pure Luau ChaCha20 Runtime)           │
│  • Template processing                                       │
│  • License validation injection                              │
│  • HWID binding code                                         │
│  • Watermark embedding                                       │
│  • Final script assembly                                     │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
                 Protected Script Output
```

### Data Flow

```
Input Script (.lua)
    ↓
[Parse] → AST
    ↓
[Analyze] → Extracted Data (strings, constants, functions)
    ↓
[Encrypt] → Encrypted Data
    ↓
[Transform] → Obfuscated AST
    ↓
[Generate] → Protected Script with Runtime
    ↓
Output Script (.lua)
```

---

## API Integration

### Setting Up Your License Validation API

#### Required Endpoints

##### 1. Validate License

**Endpoint:** `POST /api/v1/validate-license`

**Purpose:** Verify if a license is valid for the given HWID and script.

**Request Body:**
```json
{
  "license_key": "ABC1-2345-6789-DEFG",
  "hwid": "123456789",
  "watermark": "unique_watermark_identifier",
  "script_id": "admin-commands-v2",
  "timestamp": "2025-10-23T12:00:00Z"
}
```

**Success Response (200):**
```json
{
  "status": "valid",
  "user_id": 123456789,
  "expires": "2026-12-31T23:59:59Z",
  "features": ["admin", "moderation"],
  "max_uses": null
}
```

**Error Response (400/403):**
```json
{
  "status": "invalid",
  "reason": "license_expired",
  "message": "This license expired on 2025-06-01"
}
```

**Error Codes:**
- `license_not_found` - License key doesn't exist
- `license_expired` - License past expiration date
- `hwid_mismatch` - HWID doesn't match license binding
- `script_mismatch` - Wrong script_id for this license
- `license_revoked` - License has been revoked
- `rate_limit_exceeded` - Too many validation requests

---

##### 2. Generate License

**Endpoint:** `POST /api/v1/generate-license`

**Purpose:** Create a new license for a customer.

**Request Body:**
```json
{
  "api_key": "YOUR_DEVELOPER_API_KEY",
  "script_id": "admin-commands-v2",
  "buyer_userid": 123456789,
  "expiration": "2026-12-31T23:59:59Z",
  "place_id": null,
  "whitelist": [123456789],
  "max_uses": null,
  "features": ["admin", "moderation"],
  "metadata": {
    "purchase_id": "txn_abc123",
    "price": 50.00
  }
}
```

**Success Response (200):**
```json
{
  "license_key": "ABC1-2345-6789-DEFG",
  "script_id": "admin-commands-v2",
  "buyer_userid": 123456789,
  "created_at": "2025-10-23T12:00:00Z",
  "expires": "2026-12-31T23:59:59Z",
  "status": "active"
}
```

---

##### 3. Track Obfuscation (Analytics)

**Endpoint:** `POST /api/v1/track-obfuscation`

**Purpose:** Log obfuscation events for analytics.

**Request Body:**
```json
{
  "api_key": "YOUR_DEVELOPER_API_KEY",
  "script_id": "admin-commands-v2",
  "tier": "premium",
  "license_key": "ABC1-2345-6789-DEFG",
  "timestamp": "2025-10-23T12:00:00Z",
  "metadata": {
    "cli_version": "0.1.0",
    "script_size": 15000
  }
}
```

**Success Response (200):**
```json
{
  "tracked": true,
  "event_id": "evt_xyz789"
}
```

---

##### 4. Health Check

**Endpoint:** `GET /health`

**Purpose:** Check API availability.

**Success Response (200):**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime": 86400
}
```

---

### Sample API Implementation (Node.js/Express)

```javascript
const express = require('express');
const app = express();
app.use(express.json());

// In-memory store (use database in production)
const licenses = new Map();
const apiKeys = new Set(['YOUR_DEVELOPER_API_KEY']);

// Validate License
app.post('/api/v1/validate-license', (req, res) => {
  const { license_key, hwid, watermark, script_id } = req.body;
  
  const license = licenses.get(license_key);
  
  if (!license) {
    return res.status(403).json({
      status: 'invalid',
      reason: 'license_not_found'
    });
  }
  
  // Check expiration
  if (license.expires && new Date(license.expires) < new Date()) {
    return res.status(403).json({
      status: 'invalid',
      reason: 'license_expired'
    });
  }
  
  // Check HWID
  if (!license.whitelist.includes(parseInt(hwid))) {
    return res.status(403).json({
      status: 'invalid',
      reason: 'hwid_mismatch'
    });
  }
  
  // Check script ID
  if (license.script_id !== script_id) {
    return res.status(403).json({
      status: 'invalid',
      reason: 'script_mismatch'
    });
  }
  
  // Validation passed
  res.json({
    status: 'valid',
    user_id: license.buyer_userid,
    expires: license.expires,
    features: license.features
  });
});

// Generate License
app.post('/api/v1/generate-license', (req, res) => {
  const { api_key, script_id, buyer_userid, expiration, whitelist } = req.body;
  
  // Verify API key
  if (!apiKeys.has(api_key)) {
    return res.status(401).json({ error: 'Invalid API key' });
  }
  
  // Generate license key
  const license_key = generateLicenseKey();
  
  // Store license
  licenses.set(license_key, {
    script_id,
    buyer_userid,
    expires: expiration,
    whitelist: whitelist || [buyer_userid],
    created_at: new Date().toISOString(),
    status: 'active'
  });
  
  res.json({
    license_key,
    script_id,
    buyer_userid,
    created_at: new Date().toISOString(),
    expires: expiration,
    status: 'active'
  });
});

// Track Obfuscation
app.post('/api/v1/track-obfuscation', (req, res) => {
  const { api_key, script_id, tier, timestamp } = req.body;
  
  if (!apiKeys.has(api_key)) {
    return res.status(401).json({ error: 'Invalid API key' });
  }
  
  // Log event (implement your analytics here)
  console.log('Obfuscation tracked:', { script_id, tier, timestamp });
  
  res.json({
    tracked: true,
    event_id: `evt_${Date.now()}`
  });
});

// Health Check
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    version: '1.0.0',
    uptime: process.uptime()
  });
});

function generateLicenseKey() {
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
  const segments = [];
  
  for (let i = 0; i < 4; i++) {
    let segment = '';
    for (let j = 0; j < 4; j++) {
      segment += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    segments.push(segment);
  }
  
  return segments.join('-');
}

app.listen(3000, () => {
  console.log('License API running on port 3000');
});
```

---

## Rust Module Documentation

### Core Modules

#### 1. `parser` Module

**Purpose:** Parse Luau source code into an AST.

**Key Types:**
```rust
pub struct LuauParser {
    pub ast: full_moon::ast::Ast,
}

impl LuauParser {
    pub fn parse(source: &str) -> Result<Self>;
    pub fn visit_with<V: Visitor>(&mut self, visitor: &mut V);
}
```

**Usage:**
```rust
use luau_obfuscator::parser::LuauParser;

let source = std::fs::read_to_string("script.lua")?;
let parser = LuauParser::parse(&source)?;
```

---

#### 2. `analysis` Module

**Purpose:** Analyze AST to extract obfuscation targets.

**Key Types:**
```rust
pub struct AnalysisEngine {
    pub strings: Vec<StringLiteral>,
    pub constants: Vec<Constant>,
    pub functions: Vec<FunctionInfo>,
}

pub struct StringLiteral {
    pub value: String,
    pub location: SourceLocation,
    pub should_encrypt: bool,
}

pub struct Constant {
    pub value: ConstantValue,
    pub location: SourceLocation,
}

pub enum ConstantValue {
    Number(f64),
    Boolean(bool),
    Nil,
}
```

**Usage:**
```rust
use luau_obfuscator::analysis::AnalysisEngine;

let mut engine = AnalysisEngine::new();
engine.analyze(&parser.ast)?;

println!("Found {} strings to encrypt", engine.strings.len());
```

---

#### 3. `crypto` Module

**Purpose:** Cryptographic operations for obfuscation.

**Key Functions:**
```rust
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]>;
pub fn encrypt_aes_gcm(data: &[u8], key: &[u8]) -> Result<Vec<u8>>;
pub fn generate_watermark(license_key: &str) -> String;
```

**Usage:**
```rust
use luau_obfuscator::crypto;

let password = "my_secure_password";
let salt = crypto::generate_salt();
let key = crypto::derive_key(password, &salt)?;

let encrypted = crypto::encrypt_aes_gcm(data, &key)?;
```

---

#### 4. `obfuscation` Module

**Purpose:** Apply obfuscation transformations.

**Key Types:**
```rust
pub enum ObfuscationTier {
    Basic,
    Standard,
    Premium,
}

pub struct Obfuscator {
    tier: ObfuscationTier,
    config: ObfuscationConfig,
}

impl Obfuscator {
    pub fn new(tier: ObfuscationTier) -> Self;
    pub fn obfuscate(&mut self, ast: &mut Ast) -> Result<()>;
}
```

**Usage:**
```rust
use luau_obfuscator::obfuscation::{Obfuscator, ObfuscationTier};

let mut obfuscator = Obfuscator::new(ObfuscationTier::Standard);
obfuscator.obfuscate(&mut ast)?;
```

---

#### 5. `codegen` Module

**Purpose:** Generate final protected Luau script.

**Key Functions:**
```rust
pub fn generate_protected_script(
    ast: &Ast,
    config: &CodegenConfig,
) -> Result<String>;

pub struct CodegenConfig {
    pub license_key: String,
    pub hwid: String,
    pub api_endpoint: Option<String>,
    pub watermark: String,
}
```

**Usage:**
```rust
use luau_obfuscator::codegen;

let config = codegen::CodegenConfig {
    license_key: "ABC1-2345-6789-DEFG".into(),
    hwid: "123456789".into(),
    api_endpoint: Some("https://api.example.com".into()),
    watermark: watermark.clone(),
};

let protected_script = codegen::generate_protected_script(&ast, &config)?;
```

---

### End-to-End Example

```rust
use luau_obfuscator::{
    parser::LuauParser,
    analysis::AnalysisEngine,
    crypto,
    obfuscation::{Obfuscator, ObfuscationTier},
    codegen::{self, CodegenConfig},
};

fn main() -> anyhow::Result<()> {
    // 1. Parse input script
    let source = std::fs::read_to_string("input.lua")?;
    let mut parser = LuauParser::parse(&source)?;
    
    // 2. Analyze AST
    let mut engine = AnalysisEngine::new();
    engine.analyze(&parser.ast)?;
    
    // 3. Generate cryptographic materials
    let password = "my_secure_password";
    let salt = crypto::generate_salt();
    let key = crypto::derive_key(password, &salt)?;
    let watermark = crypto::generate_watermark("ABC1-2345-6789-DEFG");
    
    // 4. Obfuscate
    let mut obfuscator = Obfuscator::new(ObfuscationTier::Standard);
    obfuscator.obfuscate(&mut parser.ast)?;
    
    // 5. Generate protected script
    let config = CodegenConfig {
        license_key: "ABC1-2345-6789-DEFG".into(),
        hwid: "123456789".into(),
        api_endpoint: Some("https://api.example.com".into()),
        watermark,
    };
    
    let protected_script = codegen::generate_protected_script(&parser.ast, &config)?;
    
    // 6. Write output
    std::fs::write("output_protected.lua", protected_script)?;
    
    println!("✅ Obfuscation complete!");
    Ok(())
}
```

---

## Custom Obfuscation Plugins

### Creating a Custom Transformation

```rust
use luau_obfuscator::obfuscation::{Transformation, TransformContext};
use full_moon::ast::{Ast, Expression};

pub struct CustomStringEncoder;

impl Transformation for CustomStringEncoder {
    fn name(&self) -> &'static str {
        "custom_string_encoder"
    }
    
    fn transform(&mut self, ast: &mut Ast, ctx: &TransformContext) -> Result<()> {
        // Your custom transformation logic here
        for string in &ctx.strings {
            // Encode string using your custom algorithm
            let encoded = custom_encode(&string.value);
            // Replace in AST
            // ...
        }
        Ok(())
    }
}

fn custom_encode(s: &str) -> String {
    // Your custom encoding logic
    s.chars()
        .map(|c| (c as u8 ^ 0x42) as char)
        .collect()
}
```

### Registering Custom Transformations

```rust
let mut obfuscator = Obfuscator::new(ObfuscationTier::Premium);
obfuscator.register_transformation(Box::new(CustomStringEncoder));
obfuscator.obfuscate(&mut ast)?;
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Test

on:
  push:
    branches: [ master ]
  pull_request:
    branches: [ master ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Cache cargo dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/bin/
          ~/.cargo/registry/index/
          ~/.cargo/registry/cache/
          ~/.cargo/git/db/
          target/
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Run benchmarks
      run: cargo bench --no-run
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
```

---

## Performance Optimization

### Profiling

```bash
# Install profiling tools
cargo install cargo-flamegraph

# Run with profiling
cargo flamegraph --bin luau-obfuscator -- protect input.lua

# Open flamegraph.svg to analyze
```

### Optimization Tips

1. **Use Release Builds**
   ```bash
   cargo build --release
   ```

2. **Enable LTO**
   Already configured in `Cargo.toml`:
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   ```

3. **Parallel Processing**
   ```rust
   use rayon::prelude::*;
   
   strings.par_iter_mut().for_each(|string| {
       string.value = encrypt(&string.value);
   });
   ```

4. **Avoid Allocations**
   ```rust
   // Instead of
   let result = format!("{}_{}", prefix, suffix);
   
   // Use
   let mut result = String::with_capacity(prefix.len() + suffix.len() + 1);
   result.push_str(prefix);
   result.push('_');
   result.push_str(suffix);
   ```

---

## Security Considerations

### Password Security

❌ **DON'T:**
```rust
let password = "hardcoded_password"; // Never do this!
```

✅ **DO:**
```rust
use std::env;
let password = env::var("OBFUSCATOR_PASSWORD")?;
```

### API Key Storage

❌ **DON'T:**
```bash
# Never commit API keys
luau-obfuscator generate-license --api-key SECRET_KEY
```

✅ **DO:**
```bash
# Use environment variables
export LUAU_API_KEY="your_key_here"
luau-obfuscator generate-license --api-key "$LUAU_API_KEY"
```

### Watermark Security

Watermarks should be:
- Unique per license
- Cryptographically derived
- Not easily removable
- Traceable back to buyer

```rust
use sha2::{Sha256, Digest};

fn generate_secure_watermark(license_key: &str, buyer_id: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(license_key.as_bytes());
    hasher.update(buyer_id.to_le_bytes());
    hasher.update(b"secret_salt"); // Use a secret salt
    
    let result = hasher.finalize();
    hex::encode(&result[..16]) // First 128 bits
}
```

---

## Testing Integration

### Unit Testing Custom Transformations

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_transformation() {
        let source = r#"
            local x = "hello world"
            print(x)
        "#;
        
        let mut parser = LuauParser::parse(source).unwrap();
        let mut transform = CustomStringEncoder;
        transform.transform(&mut parser.ast, &Default::default()).unwrap();
        
        // Verify transformation
        // ...
    }
}
```

### Integration Testing with Mock API

```rust
use mockito::Server;

#[tokio::test]
async fn test_license_validation() {
    let mut server = Server::new_async().await;
    
    let mock = server.mock("POST", "/api/v1/validate-license")
        .with_status(200)
        .with_json_body(json!({
            "status": "valid",
            "user_id": 123456789
        }))
        .create();
    
    let client = ApiClient::new(&server.url());
    let result = client.validate_license("TEST-KEY", "123456789").await;
    
    assert!(result.is_ok());
    mock.assert();
}
```

---

## Additional Resources

- **Rust API Documentation:** Run `cargo doc --open`
- **Full_moon Parser Docs:** https://docs.rs/full_moon/
- **Ring Crypto Library:** https://docs.rs/ring/
- **Argon2 KDF:** https://docs.rs/argon2/

---

*Last Updated: October 2025*
*Version: 0.1.0*
