# Luau Obfuscator - Architecture Documentation

## Table of Contents

- [System Overview](#system-overview)
- [Core Components](#core-components)
- [Data Flow](#data-flow)
- [Security Architecture](#security-architecture)
- [Module Interactions](#module-interactions)
- [Performance Considerations](#performance-considerations)
- [Extension Points](#extension-points)

---

## System Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Layer (Clap)                       │
│  Command parsing │ Validation │ Progress reporting          │
└────────────┬────────────────────────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────────────────────────┐
│                   Application Layer                         │
│  - Orchestration      - Config management                   │
│  - Error handling     - Logging                             │
└────────────┬────────────────────────────────────────────────┘
             │
             ↓
┌────────────────────────────────────────────────────────────┐
│                    Core Pipeline                           │
│                                                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│  │  Parser  │→ │ Analysis │→ │  Crypto  │→ │ Obfusc   │ │
│  │(full_moon)│ │  Engine  │  │  Module  │  │ Transf.  │ │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘ │
│                                                ↓           │
│                                      ┌──────────────┐     │
│                                      │   Codegen    │     │
│                                      └──────────────┘     │
└────────────┬───────────────────────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────────────────────────┐
│               External Integration Layer                    │
│                                                             │
│  ┌──────────────────┐      ┌────────────────────┐         │
│  │   API Client     │      │   File System      │         │
│  │  (reqwest)       │      │   I/O Operations   │         │
│  └──────────────────┘      └────────────────────┘         │
└─────────────────────────────────────────────────────────────┘
             │                          │
             ↓                          ↓
    [Validation API]          [Protected Luau Script]
```

### Design Principles

1. **Separation of Concerns**: Each module has a single, well-defined responsibility
2. **Fail-Fast**: Input validation at boundaries, early error detection
3. **Type Safety**: Leverage Rust's type system for correctness guarantees
4. **Performance**: Zero-copy where possible, streaming for large inputs
5. **Security by Design**: Cryptographic operations isolated, secrets never logged
6. **Testability**: Pure functions, dependency injection, comprehensive test coverage

---

## Core Components

### 1. Parser Module (`src/parser/`)

**Purpose**: Parse Luau source code into an Abstract Syntax Tree (AST)

**Key Types**:
```rust
pub struct LuauParser {
    options: ParserOptions,
}

pub struct ParseResult {
    pub ast: Ast,
    pub tokens: Vec<Token>,
    pub errors: Vec<ParseError>,
}

pub enum AstNode {
    Block(Block),
    Statement(Statement),
    Expression(Expression),
    // ...
}
```

**Dependencies**:
- `full_moon` crate for Luau parsing
- Supports Luau-specific syntax (type annotations, string interpolation)

**Key Operations**:
1. `parse(source: &str) -> Result<ParseResult>` - Parse source into AST
2. `validate_syntax() -> Vec<SyntaxError>` - Validate Luau syntax
3. `extract_metadata() -> ScriptMetadata` - Extract script info (requires, exports)

**Visitor Pattern**:
```rust
pub trait AstVisitor {
    fn visit_block(&mut self, block: &Block);
    fn visit_statement(&mut self, stmt: &Statement);
    fn visit_expression(&mut self, expr: &Expression);
    // ...
}

impl LuauParser {
    pub fn walk<V: AstVisitor>(&self, ast: &Ast, visitor: &mut V) {
        // Traverse AST with visitor
    }
}
```

### 2. Analysis Engine (`src/analysis/`)

**Purpose**: Analyze AST to extract information for obfuscation

**Components**:

#### String Analysis (`strings.rs`)
```rust
pub struct StringAnalyzer {
    sensitivity_classifier: SensitivityClassifier,
}

pub enum StringSensitivity {
    High,    // Passwords, API keys
    Medium,  // User-visible strings
    Low,     // Debug strings, internal names
}

pub struct StringInfo {
    pub content: String,
    pub location: Location,
    pub sensitivity: StringSensitivity,
    pub context: StringContext,
}

impl StringAnalyzer {
    pub fn extract_strings(&self, ast: &Ast) -> Vec<StringInfo>;
    pub fn classify_sensitivity(&self, string: &str) -> StringSensitivity;
}
```

#### Control Flow Analysis (`controlflow.rs`)
```rust
pub struct ControlFlowGraph {
    pub blocks: Vec<BasicBlock>,
    pub edges: Vec<Edge>,
}

pub struct BasicBlock {
    pub id: BlockId,
    pub statements: Vec<Statement>,
    pub terminator: Terminator,
}

pub enum Terminator {
    Return,
    Branch { condition: Expr, true_block: BlockId, false_block: BlockId },
    Jump(BlockId),
}

impl ControlFlowAnalyzer {
    pub fn build_cfg(&self, ast: &Ast) -> ControlFlowGraph;
    pub fn find_dominators(&self, cfg: &ControlFlowGraph) -> DominatorTree;
}
```

#### Scope Analysis (`scope.rs`)
```rust
pub struct ScopeAnalyzer {
    scopes: Vec<Scope>,
}

pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub variables: HashMap<String, VariableInfo>,
}

pub struct VariableInfo {
    pub name: String,
    pub declaration_site: Location,
    pub usage_sites: Vec<Location>,
    pub is_captured: bool,
}

impl ScopeAnalyzer {
    pub fn analyze(&mut self, ast: &Ast) -> ScopeTree;
    pub fn resolve_variable(&self, name: &str, location: Location) -> Option<VariableInfo>;
}
```

#### Roblox API Detection (`roblox.rs`)
```rust
pub struct RobloxApiDetector {
    known_services: HashSet<String>,
    known_datatypes: HashSet<String>,
}

pub enum RobloxApiType {
    Service(String),      // game:GetService("Players")
    Datatype(String),     // Vector3.new()
    GlobalVariable(String), // workspace, game
}

impl RobloxApiDetector {
    pub fn detect(&self, expr: &Expression) -> Option<RobloxApiType>;
    pub fn is_safe_to_obfuscate(&self, name: &str) -> bool;
}
```

### 3. Cryptography Module (`src/crypto/`)

**Purpose**: Provide cryptographic primitives for obfuscation

**Architecture**:

```rust
// Key Derivation (kdf.rs)
pub struct Argon2idKdf {
    params: Argon2Params,
}

pub struct Argon2Params {
    pub memory_kib: u32,  // 262144 KiB (256 MB)
    pub iterations: u32,  // 4
    pub parallelism: u32, // 2
}

impl Argon2idKdf {
    pub fn derive_key(&self, password: &[u8], salt: &[u8]) -> Result<[u8; 32]>;
}

// AES Encryption (aes.rs)
pub struct Aes256Gcm {
    key: [u8; 32],
}

pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub tag: [u8; 16],
}

impl Aes256Gcm {
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData>;
    pub fn decrypt(&self, data: &EncryptedData) -> Result<Vec<u8>>;
}

// Watermarking (watermark.rs)
pub struct Watermark {
    pub buyer_id: String,
    pub script_id: String,
    pub timestamp: u64,
    pub nonce: [u8; 16],
}

pub struct WatermarkEmbedder {
    secret: [u8; 32],
}

impl WatermarkEmbedder {
    pub fn embed(&self, ast: &mut Ast, watermark: &Watermark) -> Result<()>;
    pub fn extract(&self, ast: &Ast) -> Option<Watermark>;
}
```

**Security Guarantees**:
- All RNG uses `ring::rand::SystemRandom` (CSPRNG)
- Keys are zeroized on drop (`zeroize` crate)
- Constant-time comparisons for MACs/signatures
- No key material in logs or error messages

### 4. Obfuscation Transformations (`src/obfuscation/`)

**Purpose**: Apply code transformations to obfuscate scripts

**Tier System**:
```rust
pub enum ObfuscationTier {
    Basic,    // Fast, 10-20% overhead
    Standard, // Balanced, 50-100% overhead
    Premium,  // Maximum security, 2-5x overhead
}

pub struct ObfuscationConfig {
    pub tier: ObfuscationTier,
    pub encrypt_strings: bool,
    pub obfuscate_constants: bool,
    pub mangle_names: bool,
    pub flatten_control_flow: bool,
    pub inject_dead_code: bool,
}
```

#### String Obfuscation (`strings.rs`)
```rust
pub struct StringObfuscator {
    encryptor: Aes256Gcm,
}

impl StringObfuscator {
    pub fn obfuscate(&self, string: &str) -> Expression {
        let encrypted = self.encryptor.encrypt(string.as_bytes());
        
        // Generate: decrypt_string("base64_ciphertext", "base64_nonce")
        Expression::FunctionCall {
            function: Box::new(Expression::Variable("decrypt_string".into())),
            args: vec![
                Expression::String(base64::encode(&encrypted.ciphertext)),
                Expression::String(base64::encode(&encrypted.nonce)),
            ],
        }
    }
}
```

#### Name Mangling (`names.rs`)
```rust
pub struct NameMangler {
    rename_map: HashMap<String, String>,
    rng: SystemRandom,
}

impl NameMangler {
    pub fn generate_mangled_name(&mut self, original: &str) -> String {
        // Generate short, obfuscated names: _0, _1, l1lI, O0oO
        let mut bytes = [0u8; 4];
        self.rng.fill(&mut bytes).unwrap();
        format!("_{}", hex::encode(&bytes[..2]))
    }
    
    pub fn should_mangle(&self, name: &str) -> bool {
        // Never mangle: Roblox APIs, keywords, already mangled
        !ROBLOX_GLOBALS.contains(name) && !LUA_KEYWORDS.contains(name)
    }
}
```

#### Control Flow Flattening (`controlflow.rs`)
```rust
pub struct ControlFlowFlattener {
    dispatcher_var: String,
}

impl ControlFlowFlattener {
    pub fn flatten(&self, cfg: &ControlFlowGraph) -> Block {
        // Transform:
        // if cond then ... else ... end
        // 
        // Into:
        // local _state = 0
        // while true do
        //     if _state == 0 then
        //         if cond then _state = 1 else _state = 2 end
        //     elseif _state == 1 then
        //         ... (true branch)
        //         _state = 3
        //     elseif _state == 2 then
        //         ... (false branch)
        //         _state = 3
        //     elseif _state == 3 then
        //         break
        //     end
        // end
    }
}
```

#### Dead Code Injection (`deadcode.rs`)
```rust
pub struct DeadCodeInjector {
    templates: Vec<Statement>,
}

impl DeadCodeInjector {
    pub fn inject(&self, block: &mut Block, density: f32) {
        // Inject unreachable code that looks legitimate:
        // - Never-executed if statements
        // - Unused variable declarations
        // - Dead loops
        // - Fake API calls
    }
}
```

### 5. Code Generation (`src/codegen/`)

**Purpose**: Generate final protected Luau script

**Components**:

#### Runtime Injection (`runtime.rs`)
```rust
pub struct RuntimeGenerator {
    templates: TemplateEngine,
}

impl RuntimeGenerator {
    pub fn generate_chacha20_runtime(&self) -> String {
        // Pure Luau ChaCha20 implementation
        include_str!("../../templates/chacha20_runtime.lua")
    }
    
    pub fn generate_license_validator(&self, config: &LicenseConfig) -> String {
        self.templates.render("license_validation.lua", &config)
    }
}
```

#### Script Assembly (`assembly.rs`)
```rust
pub struct ScriptAssembler {
    runtime_gen: RuntimeGenerator,
    watermark_embedder: WatermarkEmbedder,
}

pub struct AssemblyResult {
    pub script: String,
    pub metadata: ScriptMetadata,
}

impl ScriptAssembler {
    pub fn assemble(&self, 
        ast: &Ast,
        encrypted_data: &[EncryptedString],
        watermark: &Watermark,
        license_config: &LicenseConfig
    ) -> Result<AssemblyResult> {
        // 1. Generate runtime (ChaCha20, license validator)
        let runtime = self.runtime_gen.generate_all(license_config)?;
        
        // 2. Embed encrypted strings/constants
        let data_table = self.generate_data_table(encrypted_data)?;
        
        // 3. Embed watermark
        let mut final_ast = ast.clone();
        self.watermark_embedder.embed(&mut final_ast, watermark)?;
        
        // 4. Generate final script
        let script = format!(
            "-- Protected by Luau Obfuscator v{}\n{}\n{}\n{}",
            env!("CARGO_PKG_VERSION"),
            runtime,
            data_table,
            final_ast.to_string()
        );
        
        Ok(AssemblyResult {
            script,
            metadata: ScriptMetadata::extract(&final_ast),
        })
    }
}
```

### 6. API Client (`src/api/`)

**Purpose**: Communicate with validation API

**Architecture**:
```rust
pub struct ApiClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    api_key: Option<String>,
    retry_policy: RetryPolicy,
}

pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl ApiClient {
    pub fn validate_license(&self, req: &ValidateLicenseRequest) 
        -> Result<ValidateLicenseResponse> {
        self.request_with_retry(
            Method::POST,
            "/v1/validate-license",
            Some(req)
        )
    }
    
    fn request_with_retry<T, R>(&self, 
        method: Method,
        path: &str,
        body: Option<&T>
    ) -> Result<R> 
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let mut delay = self.retry_policy.initial_delay;
        
        for attempt in 1..=self.retry_policy.max_attempts {
            match self.send_request(method.clone(), path, body) {
                Ok(response) => return Ok(response),
                Err(e) if e.is_retryable() => {
                    if attempt < self.retry_policy.max_attempts {
                        thread::sleep(delay);
                        delay = (delay * self.retry_policy.multiplier as u32)
                            .min(self.retry_policy.max_delay);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(ApiError::MaxRetriesExceeded)
    }
}
```

---

## Data Flow

### Obfuscation Pipeline

```
┌──────────────────┐
│  Input Script    │
│  (input.lua)     │
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│  1. Parse        │ ← full_moon parser
│  Source → AST    │
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│  2. Analyze      │
│  - Strings       │ ← StringAnalyzer
│  - Constants     │ ← ConstantAnalyzer
│  - Control flow  │ ← ControlFlowAnalyzer
│  - Scope         │ ← ScopeAnalyzer
│  - Roblox APIs   │ ← RobloxApiDetector
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│  3. Encrypt      │
│  - Derive key    │ ← Argon2idKdf
│  - Encrypt data  │ ← Aes256Gcm
│  - Generate mark │ ← WatermarkEmbedder
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│  4. Transform    │
│  - String obf.   │ ← StringObfuscator
│  - Name mangle   │ ← NameMangler
│  - CF flatten    │ ← ControlFlowFlattener
│  - Dead code inj │ ← DeadCodeInjector
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│  5. Generate     │
│  - Runtime code  │ ← RuntimeGenerator
│  - License check │ ← LicenseGenerator
│  - Assemble      │ ← ScriptAssembler
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│  6. Validate     │
│  - API call      │ ← ApiClient
│  - Track usage   │
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│  Protected Script│
│  (output.lua)    │
└──────────────────┘
```

### License Validation Flow (Runtime)

```
┌──────────────────────┐
│  Protected Script    │
│  Starts Execution    │
└───────────┬──────────┘
            │
            ↓
┌──────────────────────┐
│  Extract Embedded    │
│  License Key + HWID  │
└───────────┬──────────┘
            │
            ↓
┌──────────────────────┐
│  HTTP Request to     │
│  Validation API      │
│  (POST /validate)    │
└───────────┬──────────┘
            │
         ┌──┴──┐
         │     │
    Valid│     │Invalid
         ↓     ↓
┌─────────┐ ┌──────────┐
│Execute  │ │Terminate │
│Script   │ │with Error│
└─────────┘ └──────────┘
```

---

## Security Architecture

### Threat Model

**Adversaries**:
1. **Script Kiddie**: Basic deobfuscation attempts, string extraction
2. **Advanced Reverse Engineer**: AST reconstruction, dynamic analysis
3. **Malicious Buyer**: Attempts to share/resell protected scripts

**Assets to Protect**:
1. Original source code structure and logic
2. Sensitive strings (API keys, passwords)
3. License keys and validation mechanism
4. Watermark (buyer traceability)

**Attack Vectors**:
1. Static analysis (AST inspection, pattern matching)
2. Dynamic analysis (runtime instrumentation, debugging)
3. Side-channel attacks (timing, memory access patterns)
4. Social engineering (fake buyers, chargebacks)

### Defense Layers

```
┌────────────────────────────────────────────────────┐
│  Layer 1: Cryptography                             │
│  - AES-256-GCM for string encryption               │
│  - Argon2id for key derivation (slow, memory-hard) │
│  - Unique encryption per buyer                     │
└────────────────────────────────────────────────────┘
                          ↓
┌────────────────────────────────────────────────────┐
│  Layer 2: Code Transformation                      │
│  - Name mangling (break semantic meaning)          │
│  - Control flow flattening (break structure)       │
│  - Dead code injection (hide real code)            │
└────────────────────────────────────────────────────┘
                          ↓
┌────────────────────────────────────────────────────┐
│  Layer 3: Runtime Protection                       │
│  - License validation (phone home)                 │
│  - HWID binding (prevent sharing)                  │
│  - Integrity checks (detect tampering)             │
└────────────────────────────────────────────────────┘
                          ↓
┌────────────────────────────────────────────────────┐
│  Layer 4: Watermarking                             │
│  - Cryptographic watermark (trace leaks)           │
│  - Multiple independent marks (redundancy)         │
│  - Survives partial deobfuscation                  │
└────────────────────────────────────────────────────┘
```

### Key Management

```
Developer API Key
    │
    ├─ Generate License
    │     │
    │     ↓
    │  Per-Buyer Password (random, 256-bit)
    │     │
    │     ↓ Argon2id
    │  Encryption Key (256-bit)
    │     │
    │     ↓ AES-256-GCM
    │  Encrypted Script Data
    │
    └─ Track Obfuscation
          Analytics

License Key (runtime)
    │
    ├─ Validate with API
    │     │
    │     ↓
    │  JWT Token (short-lived)
    │     │
    │     ↓
    │  Script Execution Allowed
    │
    └─ HWID Binding
          UserId + PlaceId
```

---

## Module Interactions

### Dependency Graph

```
                    main.rs
                       │
        ┌──────────────┼──────────────┐
        │              │              │
       CLI          Config         Logging
        │              │              │
        └──────────────┼──────────────┘
                       │
                  Orchestrator
                       │
        ┌──────────────┼──────────────┬──────────────┐
        │              │              │              │
     Parser        Analysis        Crypto       Obfuscation
        │              │              │              │
        └──────────────┴──────────────┴──────────────┘
                       │
                    Codegen
                       │
                   API Client
                       │
                  File Output
```

### Interface Contracts

```rust
// Parser → Analysis
trait Parseable {
    fn parse(&self, source: &str) -> Result<Ast>;
}

// Analysis → Obfuscation
trait Analyzable {
    fn extract_strings(&self, ast: &Ast) -> Vec<StringInfo>;
    fn extract_constants(&self, ast: &Ast) -> Vec<ConstantInfo>;
    fn build_cfg(&self, ast: &Ast) -> ControlFlowGraph;
}

// Crypto → Obfuscation
trait Encryptable {
    fn encrypt(&self, data: &[u8]) -> Result<EncryptedData>;
    fn decrypt(&self, data: &EncryptedData) -> Result<Vec<u8>>;
}

// Obfuscation → Codegen
trait Transformable {
    fn transform(&self, ast: &mut Ast, config: &ObfuscationConfig) -> Result<()>;
}

// Codegen → Output
trait Generatable {
    fn generate(&self, ast: &Ast, encrypted: &[EncryptedData]) -> Result<String>;
}
```

---

## Performance Considerations

### Optimization Strategies

1. **Parser**:
   - Zero-copy parsing where possible
   - Lazy AST node construction
   - Arena allocation for AST nodes

2. **Analysis**:
   - Single-pass analysis (combine multiple visitors)
   - Cached lookups (scope resolution, API detection)
   - Parallel analysis for independent modules

3. **Cryptography**:
   - Parallel encryption (Rayon for multiple strings)
   - Key derivation cached per session
   - Streaming encryption for large data

4. **Obfuscation**:
   - In-place AST transformation
   - Lazy code generation (dead code)
   - Configurable transformation density

### Memory Management

```rust
// Use arena for AST nodes (single allocation)
use bumpalo::Bump;

struct AstArena {
    bump: Bump,
}

impl AstArena {
    fn alloc_node<'a>(&'a self, node: AstNode) -> &'a AstNode {
        self.bump.alloc(node)
    }
}

// Pool buffers for encryption
struct BufferPool {
    pool: Vec<Vec<u8>>,
}

impl BufferPool {
    fn acquire(&mut self, size: usize) -> Vec<u8> {
        self.pool.pop()
            .map(|mut buf| { buf.resize(size, 0); buf })
            .unwrap_or_else(|| vec![0; size])
    }
    
    fn release(&mut self, buf: Vec<u8>) {
        self.pool.push(buf);
    }
}
```

### Benchmarking

**Target Metrics**:
- Parse: <100µs for 1000-line script
- Analyze: <500µs
- Encrypt: <50ms (Argon2id dominant)
- Transform: <1s (tier 2)
- Generate: <100ms
- **Total: <2s** for typical script

---

## Extension Points

### Adding New Obfuscation Transforms

```rust
// 1. Define transform trait
pub trait Transform {
    fn apply(&self, ast: &mut Ast) -> Result<()>;
    fn should_apply(&self, config: &ObfuscationConfig) -> bool;
}

// 2. Implement custom transform
pub struct CustomTransform {
    // ...
}

impl Transform for CustomTransform {
    fn apply(&self, ast: &mut Ast) -> Result<()> {
        // Custom transformation logic
        Ok(())
    }
    
    fn should_apply(&self, config: &ObfuscationConfig) -> bool {
        config.tier == ObfuscationTier::Premium
    }
}

// 3. Register with pipeline
impl ObfuscationPipeline {
    pub fn with_transform<T: Transform>(mut self, transform: T) -> Self {
        self.transforms.push(Box::new(transform));
        self
    }
}
```

### Custom Analysis Passes

```rust
pub trait AnalysisPass {
    type Output;
    
    fn run(&self, ast: &Ast) -> Result<Self::Output>;
}

// Example: Find all function calls
pub struct FunctionCallFinder;

impl AnalysisPass for FunctionCallFinder {
    type Output = Vec<FunctionCall>;
    
    fn run(&self, ast: &Ast) -> Result<Vec<FunctionCall>> {
        let mut visitor = FunctionCallVisitor::new();
        ast.walk(&mut visitor);
        Ok(visitor.calls)
    }
}
```

### Plugin System (Future)

```rust
// Dynamic plugin loading
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn init(&mut self, context: &PluginContext) -> Result<()>;
}

#[derive(Clone)]
pub struct PluginContext {
    pub config: Arc<Config>,
    pub logger: Logger,
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn load_plugin(&mut self, path: &Path) -> Result<()> {
        // Load dynamic library, instantiate plugin
        // (Requires unsafe, ABI stability)
    }
}
```

---

## Versioning & Compatibility

### Script Format Versioning

```lua
-- Protected script header
--[[LUAU_OBFUSCATOR_VERSION:1.0.0]]
--[[FORMAT_VERSION:1]]
--[[CREATED_AT:2025-10-24T12:00:00Z]]
```

**Compatibility Matrix**:

| CLI Version | Format Version | Compatibility |
|-------------|----------------|---------------|
| 0.1.x       | 1             | ✓             |
| 0.2.x       | 1, 2          | ✓             |
| 1.0.x       | 1, 2, 3       | ✓             |

### API Versioning

- **URL Versioning**: `/v1/`, `/v2/`
- **Backwards Compatible**: Old clients work with new API
- **Deprecation Period**: 6 months notice for breaking changes

---

## Monitoring & Observability

### Structured Logging

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(ast))]
pub fn obfuscate(ast: &Ast, config: &ObfuscationConfig) -> Result<String> {
    info!(tier = ?config.tier, "Starting obfuscation");
    
    let start = Instant::now();
    
    // ... obfuscation logic
    
    let duration = start.elapsed();
    info!(
        duration_ms = duration.as_millis(),
        "Obfuscation complete"
    );
    
    Ok(result)
}
```

### Metrics Collection

```rust
pub struct Metrics {
    obfuscations_total: Counter,
    obfuscation_duration: Histogram,
    api_requests: Counter,
    errors: Counter,
}

impl Metrics {
    pub fn record_obfuscation(&self, tier: ObfuscationTier, duration: Duration) {
        self.obfuscations_total.inc();
        self.obfuscation_duration.observe(duration.as_secs_f64());
    }
}
```

---

## Testing Architecture

### Test Pyramid

```
        ┌────┐
        │ E2E│  5%  - Integration tests
        ├────┤
       │ Int. │ 15% - Component integration
      ├────────┤
     │  Unit    │ 80% - Unit tests
    └────────────┘
```

### Test Organization

```
tests/
├── unit/
│   ├── parser_tests.rs
│   ├── analysis_tests.rs
│   ├── crypto_tests.rs
│   └── obfuscation_tests.rs
├── integration/
│   ├── end_to_end_workflow.rs
│   ├── roblox_compatibility.rs
│   └── api_integration.rs
└── fixtures/
    ├── sample_scripts/
    └── expected_outputs/
```

---

## Deployment Architecture

### Distribution Channels

1. **GitHub Releases**: Precompiled binaries
2. **Cargo**: `cargo install luau-obfuscator`
3. **Homebrew** (macOS): `brew install luau-obfuscator`
4. **Chocolatey** (Windows): `choco install luau-obfuscator`

### Auto-Update Mechanism

```rust
pub struct UpdateChecker {
    current_version: Version,
    github_api: GitHubApi,
}

impl UpdateChecker {
    pub async fn check_for_updates(&self) -> Result<Option<Release>> {
        let latest = self.github_api.get_latest_release().await?;
        
        if latest.version > self.current_version {
            Ok(Some(latest))
        } else {
            Ok(None)
        }
    }
}
```

---

## Conclusion

This architecture prioritizes:

✅ **Security**: Multi-layered protection, cryptographic guarantees
✅ **Performance**: <2s obfuscation, minimal runtime overhead
✅ **Reliability**: Comprehensive testing, error handling
✅ **Maintainability**: Modular design, clear interfaces
✅ **Extensibility**: Plugin system, custom transforms

For more details on specific modules, see:
- [User Guide](USER_GUIDE.md)
- [Developer Guide](DEVELOPER_GUIDE.md)
- [API Integration](API_INTEGRATION.md)
- [Security Documentation](SECURITY.md)
