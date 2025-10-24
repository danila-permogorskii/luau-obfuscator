# Troubleshooting Guide

## Table of Contents

- [Common Issues](#common-issues)
- [Error Messages](#error-messages)
- [Performance Problems](#performance-problems)
- [API Issues](#api-issues)
- [Roblox Compatibility](#roblox-compatibility)
- [Build Issues](#build-issues)
- [Getting Help](#getting-help)

---

## Common Issues

### Issue: "Failed to parse Luau script"

**Symptoms**:
```
Error: Failed to parse Luau script
Caused by:
    Syntax error at line 42:15
```

**Causes**:
1. Invalid Luau syntax in input script
2. Using Lua 5.x features not supported in Luau
3. File encoding issues (non-UTF-8)

**Solutions**:

1. **Validate syntax in Roblox Studio**:
   ```bash
   # Open script in Studio first to check for syntax errors
   ```

2. **Check Luau compatibility**:
   ```lua
   -- ❌ Not supported in Luau
   setfenv(func, env)
   getfenv(func)
   
   -- ✅ Use Luau equivalents
   -- (no direct equivalent, refactor code)
   ```

3. **Fix encoding**:
   ```bash
   # Convert to UTF-8
   iconv -f ISO-8859-1 -t UTF-8 input.lua > input_utf8.lua
   luau-obfuscator protect input_utf8.lua ...
   ```

4. **Enable verbose logging**:
   ```bash
   RUST_LOG=debug luau-obfuscator protect input.lua --output protected.lua
   # Check logs for detailed syntax error location
   ```

---

### Issue: "License validation failed: HWID mismatch"

**Symptoms**:
```lua
Error: License validation failed: HWID mismatch
Expected HWID: 123456789
Actual HWID: 987654321
```

**Causes**:
1. License key bound to different Roblox UserId
2. Running on different account than intended
3. License key typo

**Solutions**:

1. **Verify UserId**:
   ```lua
   -- In Roblox Studio, run:
   print(game.Players.LocalPlayer.UserId)
   -- Compare with licensed UserId
   ```

2. **Contact seller for correct license**:
   ```
   "Hi, I purchased script XYZ but getting HWID mismatch.
   My UserId is: 123456789
   Can you verify the license?"
   ```

3. **Check for typos**:
   ```bash
   # Ensure license key is copied correctly (no extra spaces)
   # Format should be: XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX
   ```

---

### Issue: "API request timeout"

**Symptoms**:
```
Error: License validation timeout after 30s
```

**Causes**:
1. Network connectivity issues
2. Validation API temporarily unavailable
3. Firewall blocking HTTPS requests
4. Roblox HttpService throttling

**Solutions**:

1. **Check network connectivity**:
   ```bash
   # Test API reachability
   curl https://api.example.com/health
   # Should return: {"status": "ok"}
   ```

2. **Check Roblox HttpService**:
   ```lua
   -- In game settings, ensure HttpService is enabled:
   game:GetService("HttpService").HttpEnabled = true
   ```

3. **Wait and retry**:
   ```
   API may be temporarily overloaded.
   Wait 5-10 minutes and try again.
   ```

4. **Use offline mode (CLI only)**:
   ```bash
   # Skip API calls during obfuscation
   luau-obfuscator protect input.lua --offline
   # Note: Runtime validation will still require API
   ```

---

### Issue: "Protected script runs slowly"

**Symptoms**:
- Script takes longer to execute than original
- Lag or stuttering in Roblox game
- "Script timeout" errors

**Causes**:
1. Using Premium tier (2-5x overhead)
2. Many encrypted strings (decryption overhead)
3. Complex control flow flattening

**Solutions**:

1. **Use lower obfuscation tier**:
   ```bash
   # Switch from Premium to Standard
   luau-obfuscator protect input.lua \
     --tier standard \
     --output protected.lua
   ```

2. **Cache decrypted strings**:
   ```lua
   -- Automatically applied, but you can optimize:
   -- Move string decryption outside hot loops
   
   -- ❌ Bad (decrypts every iteration)
   for i = 1, 10000 do
       print(decrypt_string("..."))  -- Called 10k times!
   end
   
   -- ✅ Good (decrypt once)
   local decrypted = decrypt_string("...")
   for i = 1, 10000 do
       print(decrypted)  -- No overhead
   end
   ```

3. **Profile performance**:
   ```lua
   -- Add timing to identify bottlenecks
   local start = os.clock()
   -- Your code here
   local duration = os.clock() - start
   print("Execution time:", duration, "seconds")
   ```

4. **Contact seller**:
   ```
   If performance is unacceptable, request optimization
   or tier adjustment.
   ```

---

### Issue: "Obfuscation takes too long"

**Symptoms**:
```bash
$ luau-obfuscator protect large_script.lua
[Processing... 5 minutes elapsed]
```

**Causes**:
1. Very large script (>10,000 lines)
2. Premium tier with complex transformations
3. Slow Argon2id key derivation (intentional)

**Solutions**:

1. **Expected durations**:
   ```
   Basic tier:    <1s for 1000 lines
   Standard tier: ~2s for 1000 lines
   Premium tier:  ~5s for 1000 lines
   
   Argon2id: +1-2s (one-time per session)
   ```

2. **Optimize for large scripts**:
   ```bash
   # Use Basic tier for quick iteration
   luau-obfuscator protect large.lua --tier basic
   
   # Use Standard/Premium for final release
   luau-obfuscator protect large.lua --tier standard
   ```

3. **Split into modules**:
   ```lua
   -- Instead of one 10k line script:
   -- Split into 10 modules of 1k lines each
   -- Obfuscate each module separately
   
   local Module1 = require(script.Module1)
   local Module2 = require(script.Module2)
   -- ...
   ```

4. **Increase timeout**:
   ```bash
   # CLI timeout is 5 minutes by default
   # For very large scripts, this should be sufficient
   # If not, file a bug report
   ```

---

## Error Messages

### Parsing Errors

#### "Unexpected token"

```
Error: Unexpected token 'function' at line 42, column 15
```

**Meaning**: Syntax error in input script

**Fix**: Check line 42 for syntax issues

```lua
-- Line 42: ❌ Missing 'then'
if condition
    print("hello")
end

-- ✅ Fixed
if condition then
    print("hello")
end
```

#### "Unmatched parenthesis"

```
Error: Unmatched closing parenthesis at line 100
```

**Meaning**: Parentheses/brackets don't match

**Fix**: Balance all `(`, `{`, `[`

```lua
-- ❌ Unmatched
local result = calculate(a, b
print(result)  -- Missing closing )

-- ✅ Fixed
local result = calculate(a, b)
print(result)
```

### Cryptography Errors

#### "Failed to derive encryption key"

```
Error: Failed to derive encryption key
Caused by: Argon2 error: Invalid salt length
```

**Meaning**: Internal error in key derivation

**Fix**: This should not happen. File a bug report with:
- Full error message
- Command used
- Operating system

#### "Encryption failed: Invalid key size"

```
Error: Encryption failed: Invalid key size
Expected 32 bytes, got 16
```

**Meaning**: Internal error in crypto module

**Fix**: File a bug report (this is a code bug)

### API Errors

#### "401 Unauthorized"

```
Error: API request failed: 401 Unauthorized
Invalid API key
```

**Meaning**: API key is invalid or missing

**Fix**:
```bash
# Set API key
export LUAU_OBFUSCATOR_API_KEY="sk_live_..."

# Or pass via flag
luau-obfuscator generate-license --api-key sk_live_...
```

#### "429 Too Many Requests"

```
Error: API request failed: 429 Too Many Requests
Rate limit exceeded. Retry after 60 seconds.
```

**Meaning**: Hit rate limit (10 req/min for license generation)

**Fix**: Wait 60 seconds and retry

```bash
# Automatic retry with exponential backoff
luau-obfuscator generate-license ...  # Will retry automatically
```

#### "404 Not Found"

```
Error: License key not found
```

**Meaning**: License key doesn't exist in database

**Fix**: 
1. Check for typos in license key
2. Contact seller to verify license exists
3. License may have been revoked

### File I/O Errors

#### "Permission denied"

```
Error: Failed to write output file: Permission denied
```

**Fix**:
```bash
# Check permissions
ls -la protected.lua

# Make writable
chmod u+w protected.lua

# Or write to different location
luau-obfuscator protect input.lua --output ~/Documents/protected.lua
```

#### "File not found"

```
Error: Failed to read input file: No such file or directory
```

**Fix**:
```bash
# Check file exists
ls -la input.lua

# Use absolute path
luau-obfuscator protect /full/path/to/input.lua
```

---

## Performance Problems

### Slow Obfuscation

**Symptoms**: Obfuscation takes >10s for small scripts

**Diagnosis**:
```bash
# Enable profiling
RUST_LOG=trace luau-obfuscator protect input.lua 2>&1 | grep "duration"

# Look for bottlenecks:
# - "Argon2id key derivation: 2000ms" (expected)
# - "String encryption: 5000ms" (unexpected, report bug)
# - "AST transformation: 3000ms" (unexpected for small script)
```

**Solutions**:
1. Check system resources (CPU, RAM)
2. Close other applications
3. Try on different machine
4. File bug report if persistent

### Slow Runtime

**Symptoms**: Protected script runs 10x+ slower than original

**Diagnosis**:
```lua
-- Profile decryption overhead
local start = os.clock()
for i = 1, 1000 do
    decrypt_string("test_string", "test_nonce")
end
local duration = os.clock() - start
print("1000 decryptions:", duration, "seconds")
-- Expected: <0.1s
-- If >1s, report issue
```

**Solutions**:
1. Use Basic or Standard tier instead of Premium
2. Reduce number of encrypted strings (mark some as low sensitivity)
3. Optimize script logic (move decryption outside loops)

---

## API Issues

### Can't Connect to API

**Symptoms**:
```
Error: Connection refused (os error 111)
```

**Diagnosis**:
```bash
# 1. Check API status
curl https://api.example.com/health
# Should return: {"status":"ok"}

# 2. Check DNS resolution
nslookup api.example.com
# Should return IP address

# 3. Check firewall
# Ensure HTTPS (port 443) is not blocked
```

**Solutions**:
1. Check status page: https://status.example.com
2. Wait for service restoration
3. Use offline mode if possible

### Validation Keeps Failing

**Symptoms**: Script always fails license validation

**Diagnosis**:
```lua
-- Add debug logging
local success, err = pcall(function()
    return validateLicense()
end)

if not success then
    print("Validation error:", err)
    -- Check error message
end
```

**Common causes**:
```
"HWID mismatch" - Wrong Roblox account
"License expired" - Need to renew
"License revoked" - Contact seller
"Network error" - Check internet connection
"Rate limit" - Wait 1 hour
```

---

## Roblox Compatibility

### "HttpService is not enabled"

**Error**: Script fails with HttpService error

**Fix**:
```lua
-- In Studio: Game Settings > Security > Enable Studio Access to API Services
-- In published game: Game Settings > Security > Allow HTTP Requests

game:GetService("HttpService").HttpEnabled = true
```

### "Loadstring is not available"

**Error**: Protected script tries to use loadstring

**Fix**: This should never happen. Obfuscator never uses loadstring.
If you see this error, file a bug report.

### "Script timeout"

**Error**: Script takes too long to run

**Symptoms**:
```
Script timeout
Script exceeded execution time limit
```

**Causes**:
1. Premium tier obfuscation too slow
2. Script has infinite loop
3. Decrypting many strings in tight loop

**Fix**:
```lua
-- Option 1: Reduce obfuscation tier
-- Option 2: Optimize script

-- ❌ Bad
while true do
    for i = 1, 1000 do
        local str = decrypt_string(...)  -- Too many decryptions!
    end
end

-- ✅ Good
local cached_str = decrypt_string(...)
while true do
    for i = 1, 1000 do
        local str = cached_str  -- Use cached result
    end
    wait()  -- Yield to avoid timeout
end
```

---

## Build Issues

### "cargo build" fails

**Error**:
```
error[E0433]: failed to resolve: use of undeclared crate or module `ring`
```

**Fix**:
```bash
# Update dependencies
cargo update
cargo build --release

# If still failing, clean and rebuild
cargo clean
cargo build --release
```

### Missing dependencies

**Error**:
```
error: linking with `cc` failed: exit code: 1
ld: library not found for -lssl
```

**Fix (macOS)**:
```bash
# Install OpenSSL
brew install openssl

# Set environment variables
export OPENSSL_DIR=/usr/local/opt/openssl
cargo build --release
```

**Fix (Linux)**:
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev

# Fedora
sudo dnf install openssl-devel

cargo build --release
```

**Fix (Windows)**:
```powershell
# Use vcpkg
vcpkg install openssl:x64-windows
set OPENSSL_DIR=C:\vcpkg\installed\x64-windows
cargo build --release
```

---

## Getting Help

### Before Filing a Bug

1. **Search existing issues**: https://github.com/danila-permogorskii/luau-obfuscator/issues
2. **Check documentation**: Read USER_GUIDE.md and API_INTEGRATION.md
3. **Try latest version**: `cargo install --git https://github.com/danila-permogorskii/luau-obfuscator`

### Filing a Bug Report

Include:

```markdown
**Environment**:
- OS: macOS 14.0 / Windows 11 / Ubuntu 22.04
- Rust version: `rustc --version`
- Luau Obfuscator version: `luau-obfuscator --version`

**Description**:
Clear description of the issue

**Steps to Reproduce**:
1. Run command: `luau-obfuscator protect ...`
2. Observe error: "..."

**Expected Behavior**:
What should happen

**Actual Behavior**:
What actually happens

**Logs**:
```
RUST_LOG=debug luau-obfuscator ... 2>&1 | tee logs.txt
```
(Attach logs.txt)

**Additional Context**:
Any other relevant information
```

### Getting Support

- **GitHub Issues**: https://github.com/danila-permogorskii/luau-obfuscator/issues
- **Discord**: https://discord.gg/example
- **Email**: support@example.com
- **Documentation**: https://docs.example.com

**Response Times**:
- **Critical bugs**: 24-48 hours
- **General issues**: 2-7 days
- **Feature requests**: Triaged monthly

---

## Quick Reference

### Common Commands

```bash
# Check version
luau-obfuscator --version

# Enable debug logging
RUST_LOG=debug luau-obfuscator ...

# Test API connection
curl https://api.example.com/health

# Validate syntax before obfuscating
lua -l luac -p input.lua

# Check file encoding
file input.lua  # Should show "UTF-8 Unicode text"
```

### Error Code Quick Reference

| Code | Meaning | Action |
|------|---------|--------|
| 1 | CLI argument error | Check `--help` |
| 2 | File I/O error | Check permissions |
| 3 | Parse error | Fix Luau syntax |
| 4 | Crypto error | File bug report |
| 5 | API error | Check API key, network |
| 10 | Unknown error | File bug report |

---

*Last Updated: October 24, 2025*
