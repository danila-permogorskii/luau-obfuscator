# Luau Obfuscator - User Guide

Complete guide for using the Luau Obfuscator CLI tool to protect your Roblox scripts.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [CLI Commands](#cli-commands)
4. [Obfuscation Tiers](#obfuscation-tiers)
5. [License System](#license-system)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)
8. [FAQ](#faq)

---

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/danila-permogorskii/luau-obfuscator.git
cd luau-obfuscator

# Build and install
cargo install --path .
```

### From Crates.io (Coming Soon)

```bash
cargo install luau-obfuscator
```

### Verify Installation

```bash
luau-obfuscator --version
```

---

## Quick Start

### 1. Protect Your First Script

```bash
luau-obfuscator protect input.lua \
  --output protected.lua \
  --license-key XXXX-XXXX-XXXX-XXXX \
  --hwid 123456789 \
  --tier standard
```

### 2. Validate Protected Script

```bash
luau-obfuscator validate protected.lua
```

### 3. Generate License for Customer

```bash
luau-obfuscator generate-license \
  --script-id my-admin-script \
  --buyer-userid 123456789 \
  --api-key YOUR_DEV_API_KEY \
  --api-endpoint https://api.yourservice.com
```

---

## CLI Commands

### `protect` - Obfuscate and Protect a Script

**Purpose:** Transform your Luau script into a protected version with encryption, license validation, and HWID binding.

**Syntax:**
```bash
luau-obfuscator protect <INPUT> [OPTIONS]
```

**Required Arguments:**
- `<INPUT>` - Path to the input Luau script file

**Options:**

| Option | Short | Description | Required | Default |
|--------|-------|-------------|----------|--------|
| `--output <PATH>` | `-o` | Output file path | No | `<input>_protected.lua` |
| `--license-key <KEY>` | `-l` | License key for validation | **Yes** | - |
| `--hwid <ID>` | `-h` | Hardware ID (Roblox UserId) | **Yes** | - |
| `--tier <TIER>` | `-t` | Obfuscation tier | No | `standard` |
| `--api-endpoint <URL>` | `-a` | API endpoint for license validation | No | - |
| `--offline-mode` | | Skip license validation | No | `false` |
| `--password <PASS>` | `-p` | Encryption password | No | Auto-generated |
| `--watermark <DATA>` | `-w` | Custom watermark data | No | Auto-generated |

**Obfuscation Tiers:**
- `basic` - Fast, light protection (~10-20% overhead)
- `standard` - Balanced security and performance (~50-100% overhead)
- `premium` - Maximum security (~2-5x overhead)

**Examples:**

**Basic Protection:**
```bash
luau-obfuscator protect script.lua \
  --output protected.lua \
  --license-key ABC1-2345-6789-DEFG \
  --hwid 123456789
```

**Premium Protection with Custom Settings:**
```bash
luau-obfuscator protect admin_commands.lua \
  --output protected_admin.lua \
  --license-key ABC1-2345-6789-DEFG \
  --hwid 987654321 \
  --tier premium \
  --password "my_secure_password_2024" \
  --api-endpoint https://api.myservice.com
```

**Offline Mode (No License Validation):**
```bash
luau-obfuscator protect test_script.lua \
  --license-key TEST-KEY-FOR-DEMO \
  --hwid 999999 \
  --offline-mode
```

---

### `generate-license` - Create License for Customer

**Purpose:** Generate a new license key for a customer who purchased your script.

**Syntax:**
```bash
luau-obfuscator generate-license [OPTIONS]
```

**Required Options:**

| Option | Short | Description |
|--------|-------|-------------|
| `--script-id <ID>` | `-s` | Unique identifier for your script |
| `--buyer-userid <ID>` | `-b` | Buyer's Roblox UserId |
| `--api-key <KEY>` | `-k` | Your developer API key |
| `--api-endpoint <URL>` | `-a` | License API endpoint |

**Optional Settings:**

| Option | Description | Default |
|--------|-------------|--------|
| `--expiration <DATE>` | License expiration (ISO 8601) | Never expires |
| `--place-id <ID>` | Restrict to specific PlaceId | Any place |
| `--whitelist <IDS>` | Comma-separated UserIds allowed | Only buyer |

**Examples:**

**Basic License Generation:**
```bash
luau-obfuscator generate-license \
  --script-id admin-commands-v2 \
  --buyer-userid 123456789 \
  --api-key YOUR_DEV_API_KEY \
  --api-endpoint https://api.yourservice.com
```

**License with Expiration:**
```bash
luau-obfuscator generate-license \
  --script-id premium-loader \
  --buyer-userid 987654321 \
  --api-key YOUR_DEV_API_KEY \
  --api-endpoint https://api.yourservice.com \
  --expiration 2026-12-31T23:59:59Z
```

**Place-Specific License:**
```bash
luau-obfuscator generate-license \
  --script-id game-specific-script \
  --buyer-userid 555555555 \
  --api-key YOUR_DEV_API_KEY \
  --api-endpoint https://api.yourservice.com \
  --place-id 123456789
```

**Whitelist Multiple Users:**
```bash
luau-obfuscator generate-license \
  --script-id team-script \
  --buyer-userid 111111111 \
  --api-key YOUR_DEV_API_KEY \
  --api-endpoint https://api.yourservice.com \
  --whitelist 111111111,222222222,333333333
```

---

### `validate` - Validate Protected Script

**Purpose:** Check if a protected script is properly obfuscated and validate its structure.

**Syntax:**
```bash
luau-obfuscator validate <PROTECTED_SCRIPT>
```

**Output:**
- ✅ Watermark detection
- ✅ Encryption verification
- ✅ License validation code presence
- ✅ Structure integrity check

**Example:**
```bash
luau-obfuscator validate protected_admin.lua
```

**Sample Output:**
```
✅ Protected script validation successful

Script Information:
  - Obfuscation Tier: Premium
  - Watermark: Present (ID: abc123def456)
  - Encryption: ChaCha20 (verified)
  - License Validation: Active
  - Structure Integrity: Valid

Protection Summary:
  ✅ All security checks passed
  ✅ Ready for distribution
```

---

## Obfuscation Tiers

### Tier 1: Basic (Fast & Light)

**Best For:**
- Free scripts with basic protection needs
- Scripts that require minimal performance impact
- Development/testing environments

**Features:**
- ✅ Selective string encryption (sensitive strings only)
- ✅ Simple identifier name mangling
- ✅ Basic license validation
- ✅ Minimal runtime overhead (~10-20%)

**Trade-offs:**
- ⚠️ Control flow remains visible
- ⚠️ Constants are not obfuscated
- ⚠️ Less resistant to automated deobfuscation

**Use Case Example:**
```lua
-- Original
local apiKey = "secret_key_123"
print("Hello World")

-- After Basic Tier
local _a1b2 = DECRYPT("...encrypted...")
print("Hello World") -- Unencrypted string
```

---

### Tier 2: Standard (Balanced)

**Best For:**
- Commercial scripts ($5-$50 range)
- General protection needs
- Most Roblox scripts

**Features:**
- ✅ All strings encrypted
- ✅ Constant obfuscation (numbers, booleans)
- ✅ Advanced name mangling
- ✅ Light control flow flattening
- ✅ License validation with HWID binding
- ✅ Moderate runtime overhead (~50-100%)

**Trade-offs:**
- ⚠️ Noticeable performance impact for complex scripts
- ⚠️ May require optimization for high-frequency functions

**Use Case Example:**
```lua
-- Original
local function calculateDamage(base, multiplier)
    return base * multiplier
end

-- After Standard Tier
local _x9z = function(_a,_b)
    return ((__CONST[1]+__CONST[2])*_a)*_b
end
```

---

### Tier 3: Premium (Maximum Security)

**Best For:**
- Premium scripts ($50+ range)
- High-value intellectual property
- Anti-cheat systems
- Exploit protection

**Features:**
- ✅ Maximum encryption (all data)
- ✅ Heavy control flow flattening
- ✅ Dead code injection
- ✅ Anti-debugging measures
- ✅ Opaque predicates
- ✅ Runtime integrity checks
- ✅ Significant overhead (~2-5x)

**Trade-offs:**
- ⚠️ High performance impact
- ⚠️ Longer obfuscation time
- ⚠️ Not recommended for performance-critical code

**Use Case Example:**
```lua
-- Original
if player:IsAdmin() then
    grantAdminPowers(player)
end

-- After Premium Tier
-- [Heavily obfuscated with control flow flattening,
--  dead code, and anti-debugging - not readable]
```

---

## License System

### How License Validation Works

1. **At Obfuscation Time:**
   - Developer provides license key
   - Obfuscator embeds validation code in protected script
   - Unique watermark tied to license

2. **At Runtime (In Roblox):**
   - Protected script runs in player's client
   - Validation code calls your API endpoint
   - Server validates: `license_key + hwid + watermark`
   - Script executes only if validated

### API Integration

Your validation API should implement this endpoint:

**Endpoint:** `POST /api/v1/validate-license`

**Request:**
```json
{
  "license_key": "ABC1-2345-6789-DEFG",
  "hwid": "123456789",
  "watermark": "unique_watermark_data",
  "script_id": "admin-commands-v2"
}
```

**Response (Success):**
```json
{
  "status": "valid",
  "user_id": 123456789,
  "expires": "2026-12-31T23:59:59Z"
}
```

**Response (Failure):**
```json
{
  "status": "invalid",
  "reason": "license_expired"
}
```

### HWID Binding Options

**1. UserId Binding (Most Common):**
```bash
--hwid <BUYER_ROBLOX_USERID>
```
Script only works for specific Roblox user.

**2. PlaceId Binding:**
```bash
--place-id <ROBLOX_PLACEID>
```
Script only works in specific game.

**3. Combined Binding:**
Both UserId AND PlaceId must match.

**4. Whitelist:**
Multiple UserIds can use the same license.

---

## Best Practices

### 1. Choosing the Right Tier

✅ **DO:**
- Use `basic` for free/demo scripts
- Use `standard` for most commercial scripts
- Use `premium` only for high-value IP

❌ **DON'T:**
- Over-protect low-value scripts (wastes performance)
- Use `premium` for performance-critical game logic

### 2. Password Management

✅ **DO:**
- Use strong, unique passwords for each script
- Store passwords securely (password manager)
- Rotate passwords periodically

❌ **DON'T:**
- Use the same password for multiple scripts
- Include passwords in version control
- Share passwords with unauthorized users

### 3. License Generation

✅ **DO:**
- Validate buyer identity before generating license
- Set appropriate expiration dates
- Log all license generations
- Implement revocation capability

❌ **DON'T:**
- Generate licenses without buyer verification
- Use predictable license keys
- Share your developer API key

### 4. Testing Protected Scripts

✅ **DO:**
- Test in Roblox Studio before release
- Verify license validation works
- Check performance impact
- Test with different HWIDs

❌ **DON'T:**
- Release without testing
- Assume obfuscation doesn't affect behavior
- Skip validation testing

### 5. Distribution

✅ **DO:**
- Distribute only protected scripts to customers
- Keep original source code private
- Version your protected scripts
- Provide clear usage instructions

❌ **DON'T:**
- Distribute unprotected source code
- Forget to update licenses for new versions
- Mix protected and unprotected code

---

## Troubleshooting

### Error: "License validation failed"

**Cause:** API endpoint unreachable or license invalid

**Solutions:**
1. Check API endpoint is correct and accessible
2. Verify license key format: `XXXX-XXXX-XXXX-XXXX`
3. Ensure API server is running
4. Use `--offline-mode` for testing

### Error: "HWID mismatch"

**Cause:** Script executed by different user than licensed

**Solutions:**
1. Verify `--hwid` matches buyer's Roblox UserId
2. Check if whitelist includes the user
3. Regenerate license with correct HWID

### Error: "Obfuscation failed: Parse error"

**Cause:** Input script has syntax errors

**Solutions:**
1. Validate input script syntax in Roblox Studio
2. Check for Luau-specific syntax issues
3. Remove any invalid characters

### Error: "Protected script crashes in Roblox"

**Cause:** Roblox API incompatibility or performance issues

**Solutions:**
1. Test with lower obfuscation tier
2. Check if Roblox APIs are properly preserved
3. Verify no infinite loops in control flow
4. Report issue with script sample

### Performance Issues

**Symptoms:** Script runs slowly after obfuscation

**Solutions:**
1. Use lower tier for performance-critical code
2. Profile to identify bottlenecks
3. Consider selective obfuscation
4. Optimize original script first

---

## FAQ

### General Questions

**Q: Can protected scripts be deobfuscated?**

A: While no obfuscation is 100% secure, our multi-layered approach (encryption + control flow + name mangling) makes deobfuscation extremely difficult and time-consuming. The cryptographic watermarking also allows you to trace leaked scripts back to the buyer.

**Q: Does obfuscation affect script behavior?**

A: No. The obfuscator preserves all functionality while only changing how the code looks. However, performance may be impacted depending on the tier used.

**Q: Can I update a protected script?**

A: Yes. Re-obfuscate the updated source code with the same license key and password. Ensure version compatibility with existing deployments.

**Q: Is my source code sent to any server?**

A: No. All obfuscation happens locally on your machine. Only license validation calls your API at runtime.

### License System

**Q: What happens if my API server goes down?**

A: By default, scripts will fail validation. Consider implementing a grace period or offline validation for critical scenarios.

**Q: Can one license work for multiple users?**

A: Yes, use the `--whitelist` option when generating licenses to allow multiple UserIds.

**Q: How do I revoke a license?**

A: Update your API to return `invalid` status for the specific license key. The protected script will refuse to run on the next validation.

**Q: Can licenses expire?**

A: Yes, use `--expiration` when generating licenses. Your API should enforce expiration dates.

### Technical

**Q: What encryption algorithms are used?**

A: Service-side uses AES-256-GCM with Argon2id key derivation. Runtime uses pure Luau ChaCha20 (Roblox-compatible).

**Q: Does this work with ModuleScripts?**

A: Yes, all Luau script types are supported: LocalScript, Script, and ModuleScript.

**Q: Can I obfuscate already-obfuscated code?**

A: Not recommended. This can cause exponential performance degradation and may break functionality.

**Q: Does this protect against memory dumping?**

A: The runtime ChaCha20 decryption happens in memory, so determined attackers with memory access can extract decrypted code. However, the watermarking system helps trace leaks.

### Pricing & Licensing

**Q: Do I need to pay per obfuscation?**

A: The CLI tool is open-source and free. You only need to run your own license validation API.

**Q: Can I use this for commercial projects?**

A: Yes, the MIT license allows commercial use.

**Q: Is support available?**

A: Community support via GitHub Issues. Commercial support options may be available in the future.

---

## Getting Help

- **GitHub Issues:** https://github.com/danila-permogorskii/luau-obfuscator/issues
- **Documentation:** https://github.com/danila-permogorskii/luau-obfuscator/docs
- **API Reference:** See `DEVELOPER_GUIDE.md`

---

*Last Updated: October 2025*
*Version: 0.1.0*
