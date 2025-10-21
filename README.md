# Luau Obfuscator

**Commercial-grade Luau/Roblox script obfuscation CLI tool with cryptographic protection and license management.**

## 🚀 Project Status

### ✅ Phase 1: Foundation & Core Infrastructure (Complete)
- ✅ Project setup with Cargo dependencies
- ✅ CLI framework with clap
- ✅ Error handling infrastructure
- ✅ Logging and progress reporting
- ✅ Configuration management

### ✅ Phase 2: Luau Parsing (Complete)
- ✅ Full_moon parser integration
- ✅ AST visitor pattern
- ✅ String literal extraction
- ✅ Numeric constant identification
- ✅ Roblox API preservation
- ✅ Parser tests

### ✅ Phase 3: Analysis Engine (Complete)
- ✅ Control flow mapping
- ✅ Scope analysis
- ✅ Roblox API detection
- ✅ Function boundary detection
- ✅ Dependency analysis

### ✅ Phase 4: Cryptography Module (Complete)
- ✅ Argon2id key derivation
- ✅ AES-256-GCM encryption
- ✅ Watermarking system
- ✅ Comprehensive tests

### ✅ Phase 5: Obfuscation Transformations (Complete)
- ✅ String encryption
- ✅ Constant obfuscation
- ✅ Name mangling
- ✅ Control flow flattening
- ✅ Dead code injection
- ✅ Tier system (Basic/Standard/Premium)

### ✅ Phase 6: Code Generation (Complete)
- ✅ Pure Luau ChaCha20 runtime template
- ✅ License validation template
- ✅ HWID binding template
- ✅ Template processing system
- ✅ Script assembly engine
- ✅ Watermark embedding

### ✅ Phase 7: API Client Integration (Complete)
- ✅ HTTP client with reqwest
- ✅ License validation endpoint integration
- ✅ License generation endpoint integration
- ✅ Analytics tracking endpoint
- ✅ Retry logic with exponential backoff
- ✅ CLI integration for protect/generate-license commands
- ✅ Offline mode fallback

### 📋 Remaining Phases
- **Phase 8**: Testing & Validation
- **Phase 9**: Documentation & Polish
- **Phase 10**: Beta Release

## 🎯 Features

### Security Model
- **Per-customer unique encryption** - Each buyer gets uniquely encrypted version
- **AES-256-GCM** encryption (service-side)
- **Argon2id** key derivation (m=262144 KiB, t=4, p=2)
- **Pure Luau ChaCha20** runtime (Roblox-compatible)
- **Cryptographic watermarking** - Traceable to individual purchases

### License System
- **Centralized validation API** - Scripts phone home to validate
- **HWID binding** - Validates Roblox UserId + PlaceId
- **Revocation support** - Server-side license management
- **Analytics tracking** - Usage monitoring

### Obfuscation Tiers

**Tier 1: Basic (Fast, Light)**
- String encryption (sensitive strings only)
- Simple name mangling
- ~10-20% overhead

**Tier 2: Standard (Balanced)**
- All strings encrypted
- Constant obfuscation
- Name mangling + light control flow flattening
- ~50-100% overhead

**Tier 3: Premium (Maximum Security)**
- Maximum encryption
- Heavy control flow flattening
- Dead code injection
- Anti-debugging
- ~2-5x overhead

## 📦 Installation

```bash
# From source
cargo install --path .

# From crates.io (when published)
cargo install luau-obfuscator
```

## 🔧 Usage

### Protect a Script
```bash
luau-obfuscator protect input.lua \
  --output protected.lua \
  --license-key XXXX-XXXX-XXXX-XXXX \
  --hwid 123456789 \
  --tier standard \
  --api-endpoint https://api.example.com
```

### Generate License
```bash
luau-obfuscator generate-license \
  --script-id my-admin-script \
  --buyer-userid 123456789 \
  --api-key YOUR_DEV_API_KEY
```

### Validate Protected Script
```bash
luau-obfuscator validate protected.lua
```

## 🏗️ Architecture

```
CLI Frontend (clap)
    ↓
API Client (reqwest) ← License Validation
    ↓
Luau Parser (full_moon)
    ↓
Analysis Engine
    ↓
Cryptography Module (ring + argon2)
    ↓
Obfuscation Transformations
    ↓
Code Generation (Pure Luau ChaCha20)
    ↓
Protected Script Output
```

## 🧪 Development

### Run Tests
```bash
cargo test
```

### Run Benchmarks
```bash
cargo bench
```

### Enable Logging
```bash
RUST_LOG=debug cargo run -- protect input.lua
```

## 📄 License

MIT License - See [LICENSE](LICENSE) file

## 🤝 Contributing

This is a commercial project. For business inquiries, please contact the maintainers.

## 🔗 Links

- [Documentation](https://docs.example.com) (Coming soon)
- [API Reference](https://api-docs.example.com) (Coming soon)
- [Discord Community](https://discord.gg/example) (Coming soon)

## 📊 Progress Summary

**Completed:** 7 of 10 phases (70%)
**Status:** API integration complete, ready for comprehensive testing
**Next:** Phase 8 - Testing & Validation

## 🎉 Phase 7 Completion

Phase 7: API Client Integration is now complete with:
- Full HTTP client implementation using reqwest
- Retry logic with exponential backoff (3 attempts: 1s, 2s, 4s delays)
- License validation API integration
- License generation API integration
- Analytics tracking for obfuscation events
- CLI command integration (protect, generate-license)
- Offline mode fallback when API is unreachable
- Comprehensive error handling
- Integration tests for API models and client

**API Endpoints:**
- `POST /api/v1/validate-license` - Validate license keys
- `POST /api/v1/generate-license` - Generate new licenses
- `POST /api/v1/track-obfuscation` - Track obfuscation events
- `GET /health` - Health check endpoint

**Features:**
- 30-second default timeout (configurable)
- Automatic retry on failures
- Graceful degradation for non-critical operations
- Detailed logging of API interactions
