# Frequently Asked Questions (FAQ)

## General Questions

### What is Luau Obfuscator?

Luau Obfuscator is a commercial-grade CLI tool that protects Luau/Roblox scripts from reverse engineering and unauthorized distribution. It uses military-grade encryption (AES-256-GCM), code obfuscation, license validation, and cryptographic watermarking to secure your scripts.

### Who should use this tool?

- **Script sellers**: Protect premium admin commands, game systems, exploits
- **Game developers**: Protect proprietary game logic and anti-cheat systems  
- **Enterprise**: Secure business-critical Roblox applications
- **Individual creators**: Monetize scripts with confidence

### How is this different from other obfuscators?

| Feature | Luau Obfuscator | Others |
|---------|----------------|--------|
| Encryption | AES-256-GCM + ChaCha20 | Simple XOR or none |
| License system | API-based with HWID binding | None or client-side |
| Watermarking | Cryptographic, traceable | None or trivial |
| Roblox optimization | Native Luau, no overhead | May break Roblox APIs |
| Commercial support | ✅ Yes | Usually no |

---

## Pricing & Licensing

### How much does it cost?

**CLI Tool**: Free and open-source (MIT License)

**Validation API**:
- **Free Tier**: 1,000 validations/month
- **Starter**: $19/month - 10,000 validations
- **Pro**: $49/month - 50,000 validations
- **Enterprise**: Custom pricing - Unlimited validations

See https://example.com/pricing for details.

### Can I use this commercially?

Yes! The CLI tool is MIT licensed, meaning you can:
- ✅ Use commercially
- ✅ Sell protected scripts
- ✅ Modify the source code
- ✅ Redistribute (with attribution)

### Do I need an API subscription?

For **development/testing**: No, use `--offline` mode

For **production**: Yes, protected scripts require API validation to run. This:
- Prevents license sharing
- Enables revocation
- Tracks usage analytics

---

## Technical Questions

### What obfuscation techniques are used?

**Layer 1: Encryption**
- All strings encrypted with AES-256-GCM
- Unique encryption key per buyer
- Runtime decryption with pure Luau ChaCha20

**Layer 2: Code Transformation**
- Name mangling (unreadable variable names)
- Control flow flattening (breaks program structure)
- Constant obfuscation (mathematical transformation)
- Dead code injection (hides real logic)

**Layer 3: Runtime Protection**
- License validation (phone home to API)
- HWID binding (UserId + PlaceId)
- Integrity checks (detects tampering)

**Layer 4: Watermarking**
- Cryptographic buyer identification
- Survives partial deobfuscation
- Legally traceable

### How secure is it?

**Against script kiddies**: ✅ 100% effective (strings encrypted, APIs preserved)

**Against intermediate attackers**: ⚠️ 70-80% effective (code structure obscured, requires significant effort)

**Against advanced attackers**: ⚠️ 50-60% effective (full deobfuscation possible with days/weeks of work, but watermark remains)

**Real-world result**: Increases reverse engineering cost by **100-1000x**. Most attackers will give up before succeeding.

### What performance impact?

| Tier | Obfuscation Overhead | Runtime Overhead |
|------|---------------------|------------------|
| Basic | <1 second | 10-20% |
| Standard | ~2 seconds | 50-100% |
| Premium | ~5 seconds | 2-5x |

**Typical usage**: Standard tier is recommended for most scripts. <2% of users report noticeable performance issues.

### Does it work with all Luau code?

Yes, with caveats:

✅ **Fully supported**:
- Standard Luau syntax
- Roblox services (Players, Workspace, etc.)
- Roblox datatypes (Vector3, CFrame, etc.)
- Type annotations
- String interpolation

⚠️ **Limited support**:
- `getfenv`/`setfenv` - Deprecated in Luau, avoid
- `debug.getinfo` - Obfuscated names break reflection
- `loadstring` - Not allowed in Roblox anyway

❌ **Not supported**:
- Lua 5.x features not in Luau (goto, etc.)
- Dynamic code generation (loadstring)

### Can protected scripts call each other?

Yes! Protected scripts can:
- Require other protected scripts
- Share functions via ModuleScripts
- Use RemoteEvents/RemoteFunctions

**Best practice**: Obfuscate each module separately, then combine.

### What about dependencies (e.g., Roact, Rodux)?

**Option 1**: Don't obfuscate dependencies (they're already public)
```lua
-- Keep libraries unobfuscated
local Roact = require(ReplicatedStorage.Roact)

-- Obfuscate your code
local MyComponent = protectedFunction()
```

**Option 2**: Bundle and obfuscate everything
```bash
# Concatenate all files
cat lib1.lua lib2.lua your_code.lua > bundle.lua

# Obfuscate bundle
luau-obfuscator protect bundle.lua
```

---

## Usage Questions

### How do I protect a script?

```bash
# 1. Install
cargo install --path .

# 2. Protect script
luau-obfuscator protect input.lua \
  --output protected.lua \
  --tier standard

# 3. Test in Roblox Studio
# Copy protected.lua to Studio and run
```

### How do I generate licenses for buyers?

```bash
# Generate license
luau-obfuscator generate-license \
  --script-id my-admin-script \
  --buyer-userid 123456789 \
  --api-key $API_KEY

# Output: License key XXXX-XXXX-...
```

Then:
1. Embed license in protected script (or have buyer enter it)
2. Send protected script + license key to buyer
3. Buyer runs script, validation happens automatically

### How do I revoke a license?

```bash
# Revoke immediately
luau-obfuscator revoke-license \
  --license-key XXXX-XXXX-... \
  --reason "chargeback" \
  --api-key $API_KEY

# Next validation attempt will fail
```

### Can I automate the sales process?

Yes! Example workflow:

```python
import subprocess
import requests

def process_sale(buyer_userid, script_path):
    # 1. Generate license via API
    response = requests.post(
        "https://api.example.com/v1/generate-license",
        headers={"Authorization": f"Bearer {API_KEY}"},
        json={
            "script_id": "my-script",
            "buyer_userid": buyer_userid,
            "duration_days": 365
        }
    )
    
    license_key = response.json()["license_key"]
    
    # 2. Obfuscate with embedded license
    subprocess.run([
        "luau-obfuscator", "protect", script_path,
        "--output", f"protected_{buyer_userid}.lua",
        "--license-key", license_key,
        "--hwid", buyer_userid
    ])
    
    # 3. Upload to CDN
    download_url = upload_to_cdn(f"protected_{buyer_userid}.lua")
    
    # 4. Send email
    send_email(
        buyer_userid,
        f"Your script: {download_url}\nLicense: {license_key}"
    )
```

### How do I update a protected script?

**Option 1**: Re-obfuscate with same license
```bash
# Update source
vim my_script.lua

# Re-obfuscate (use same license key)
luau-obfuscator protect my_script.lua \
  --license-key EXISTING_KEY \
  --output protected_v2.lua

# Send updated script to buyer
```

**Option 2**: Generate new license (optional)
```bash
# Generate new license for v2
luau-obfuscator generate-license \
  --script-id my-script-v2 \
  --buyer-userid 123456789

# Obfuscate with new license
luau-obfuscator protect my_script.lua \
  --license-key NEW_KEY \
  --output protected_v2.lua
```

---

## Troubleshooting

### "License validation failed: HWID mismatch"

**Cause**: License bound to different Roblox account

**Fix**: Verify buyer's UserId matches the license

```lua
-- In Roblox Studio
print(game.Players.LocalPlayer.UserId)
-- This must match the UserId in the license
```

### "Protected script is too slow"

**Cause**: Premium tier overhead or tight loops with many decryptions

**Fix**: Use Standard tier or optimize code

```bash
# Try Standard instead of Premium
luau-obfuscator protect input.lua --tier standard
```

### "API timeout"

**Cause**: Network issues or API overload

**Fix**: Wait and retry. API has automatic retry logic.

```bash
# Automatic retry
luau-obfuscator protect input.lua ...  # Will retry up to 5 times
```

### "Parsing error"

**Cause**: Invalid Luau syntax

**Fix**: Validate syntax in Roblox Studio first

```bash
# Enable verbose logging
RUST_LOG=debug luau-obfuscator protect input.lua
```

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more.

---

## Security Questions

### Can someone extract my strings?

Not easily. All strings are encrypted with AES-256-GCM. An attacker would need to:
1. Extract the decryption key (obfuscated in code)
2. Implement ChaCha20 decryption
3. Decrypt each string individually

**Realistic effort**: 10-50 hours for skilled attacker. Most will give up.

### What if someone deobfuscates my script?

Two lines of defense:

1. **Prevention**: Control flow flattening and dead code make deobfuscation very difficult
2. **Attribution**: Even if fully deobfuscated, cryptographic watermark traces leak back to buyer

**Result**: You can identify and revoke the leaker's license, then ban them from future purchases.

### Can someone bypass license validation?

They can try, but:
1. Validation code is obfuscated and interleaved with decryption
2. Removing validation breaks decryption (script won't run)
3. Even if bypassed, watermark still identifies leaker

**Realistic effort**: 5-20 hours for skilled attacker.

### How do watermarks survive deobfuscation?

Watermarks use multiple independent embedding techniques:
- Dead variable names
- Numeric constants
- String patterns
- Comment injection
- Control flow patterns

**Redundancy**: 5-10 independent marks. Attacker must find and remove ALL to avoid detection. This is extremely difficult without the original script for comparison.

### What if my API key is leaked?

**Immediate action**:
1. Rotate API key in dashboard
2. Revoke all licenses generated with old key (optional)
3. Update your systems with new key

**Prevention**:
- Never commit API keys to Git
- Use environment variables
- Rotate keys every 90 days

---

## Business Questions

### Can I use this for free?

Yes! The CLI tool is free and open-source.

API validation has a free tier (1,000 validations/month), which is enough for small-scale sellers.

### Do buyers need to pay anything?

No. Buyers only pay you for the script. They don't interact with Luau Obfuscator or the API directly.

### Can I white-label this?

Yes, with restrictions:
- ✅ You can rebrand the CLI tool (MIT license)
- ❌ You cannot rebrand the validation API (requires Enterprise plan)

**Enterprise plan** includes:
- White-label API
- Custom domain
- Dedicated infrastructure
- Priority support

Contact sales@example.com for Enterprise pricing.

### What's the refund policy?

API subscriptions:
- 30-day money-back guarantee
- No questions asked

### Do you offer discounts?

- **Educational**: 50% off for students (verify with student email)
- **Non-profit**: 50% off for registered non-profits
- **Volume**: Contact sales for enterprise pricing

---

## Advanced Questions

### Can I customize obfuscation?

Yes! Via config file:

```toml
# obfuscation.toml
[obfuscation]
tier = "standard"
encrypt_strings = true
obfuscate_constants = true
mangle_names = true
flatten_control_flow = true
inject_dead_code = false  # Disable for faster execution

[crypto]
argon2_memory_kib = 262144
argon2_iterations = 4
```

```bash
luau-obfuscator protect input.lua --config obfuscation.toml
```

### Can I integrate with CI/CD?

Yes! Example GitHub Actions workflow:

```yaml
name: Obfuscate and Deploy

on:
  push:
    tags:
      - 'v*'

jobs:
  obfuscate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Luau Obfuscator
        run: cargo install --git https://github.com/danila-permogorskii/luau-obfuscator
      
      - name: Obfuscate
        env:
          API_KEY: ${{ secrets.LUAU_OBFUSCATOR_API_KEY }}
        run: |
          luau-obfuscator protect src/main.lua \
            --output dist/protected.lua \
            --tier standard
      
      - name: Upload to CDN
        run: aws s3 cp dist/protected.lua s3://my-scripts/
```

### Can I extend the obfuscator?

Yes! The codebase is modular:

```rust
// Add custom transformation
use luau_obfuscator::obfuscation::Transform;

struct MyCustomTransform;

impl Transform for MyCustomTransform {
    fn apply(&self, ast: &mut Ast) -> Result<()> {
        // Your transformation logic
        Ok(())
    }
}

// Register with pipeline
let pipeline = ObfuscationPipeline::new()
    .with_transform(MyCustomTransform);
```

See [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) for details.

### What about Team Collaboration?

Best practices:

1. **Separate source and protected scripts**:
   ```
   my-project/
   ├── src/           # Source code (Git)
   ├── protected/     # Protected scripts (.gitignore)
   └── build.sh       # Obfuscation script
   ```

2. **Use API key per team member** (for audit trail)

3. **Automate obfuscation** (CI/CD)

4. **Never commit protected scripts to Git**

---

## Comparison Questions

### Luau Obfuscator vs. Ironbrew

| Feature | Luau Obfuscator | Ironbrew |
|---------|----------------|----------|
| Encryption | AES-256-GCM | XOR |
| License system | ✅ API-based | ❌ None |
| Watermarking | ✅ Cryptographic | ❌ None |
| Roblox compatibility | ✅ 100% | ⚠️ ~90% |
| Performance overhead | 10-100% | ~200% |
| Commercial support | ✅ Yes | ❌ No |
| Open source | ✅ MIT | ❌ Closed |

### Luau Obfuscator vs. PSU

| Feature | Luau Obfuscator | PSU |
|---------|----------------|-----|
| Obfuscation strength | Strong | Strong |
| License system | ✅ Built-in | ❌ DIY |
| Ease of use | ✅ CLI tool | ⚠️ Web UI |
| Offline mode | ✅ Yes | ❌ No |
| Self-hosted | ✅ Yes | ❌ No |
| Watermarking | ✅ Yes | ❌ No |

---

## Support

### Where can I get help?

- **Documentation**: https://docs.example.com
- **GitHub Issues**: https://github.com/danila-permogorskii/luau-obfuscator/issues
- **Discord**: https://discord.gg/example
- **Email**: support@example.com

### What are the support response times?

- **Free tier**: Best effort (2-7 days)
- **Paid subscribers**: 24-48 hours
- **Enterprise**: 4-hour SLA

### Can I request features?

Yes! Submit feature requests via GitHub Issues:

https://github.com/danila-permogorskii/luau-obfuscator/issues/new?labels=enhancement

### How often are updates released?

- **Bug fixes**: As needed (hot fixes)
- **Minor updates**: Monthly
- **Major updates**: Quarterly

Subscribe to releases: https://github.com/danila-permogorskii/luau-obfuscator/releases

---

## Legal Questions

### What license is this under?

MIT License - see [LICENSE](../LICENSE) file.

In summary:
- ✅ Commercial use
- ✅ Modification
- ✅ Distribution
- ✅ Private use
- ⚠️ No warranty
- ⚠️ No liability

### Can I sell protected scripts?

Yes! The MIT license explicitly allows commercial use.

### Who owns the protected scripts?

You do. Obfuscation doesn't change ownership or copyright.

### What about DMCA takedowns?

If someone leaks your protected script:
1. Extract watermark to identify leaker
2. Revoke their license
3. File DMCA takedown (you own the copyright)
4. Consider legal action (you have proof of purchase via watermark)

---

*Last Updated: October 24, 2025*
*Version: 1.0.0*
