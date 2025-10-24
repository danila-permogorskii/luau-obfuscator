# API Integration Guide

## Overview

The Luau Obfuscator integrates with a centralized validation API to provide secure license management, HWID binding, and analytics tracking. This guide explains how to set up and integrate with the API.

## Table of Contents

- [API Architecture](#api-architecture)
- [Authentication](#authentication)
- [Endpoints](#endpoints)
- [Integration Examples](#integration-examples)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

---

## API Architecture

### System Overview

```
┌─────────────────────┐
│  CLI Tool (Client)  │
│  - Obfuscate script │
│  - Generate license │
│  - Validate         │
└──────────┬──────────┘
           │ HTTPS/TLS
           ↓
┌─────────────────────┐
│  Validation API     │
│  - License mgmt     │
│  - HWID validation  │
│  - Analytics        │
└──────────┬──────────┘
           │
           ↓
┌─────────────────────┐
│  Database           │
│  - Licenses         │
│  - HWIDs            │
│  - Usage stats      │
└─────────────────────┘
```

### Security Model

- **Transport Security**: All communication over HTTPS/TLS 1.3+
- **Authentication**: API keys for developer access, license keys for runtime validation
- **Rate Limiting**: 100 requests/minute per API key
- **Encryption**: Request/response payloads encrypted with API key derived secrets

---

## Authentication

### Developer API Keys

Developers use API keys to generate licenses and manage their scripts.

**Obtaining an API Key:**

1. Register at `https://api.example.com/register`
2. Verify email
3. Generate API key from dashboard

**Using API Keys:**

```bash
# Set environment variable
export LUAU_OBFUSCATOR_API_KEY="your-api-key-here"

# Or pass via CLI flag
luau-obfuscator generate-license \
  --api-key your-api-key-here \
  --script-id my-script \
  --buyer-userid 123456789
```

### License Keys

End users receive license keys to run protected scripts.

**License Key Format:**
```
XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX
```

**Embedding in Protected Scripts:**
```lua
local LICENSE_KEY = "XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX"
local HWID = game.Players.LocalPlayer.UserId
```

---

## Endpoints

### Base URL

```
https://api.example.com/v1
```

### 1. Validate License

Validates a license key and HWID combination.

**Endpoint:** `POST /validate-license`

**Request:**
```json
{
  "license_key": "XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX",
  "hwid": "123456789",
  "script_id": "my-admin-script",
  "client_version": "0.1.0"
}
```

**Response (Success):**
```json
{
  "valid": true,
  "expires_at": "2025-12-31T23:59:59Z",
  "features": ["premium", "auto-update"],
  "hwid_bound": true
}
```

**Response (Invalid):**
```json
{
  "valid": false,
  "reason": "license_expired",
  "message": "License expired on 2025-01-15"
}
```

**Error Codes:**
- `license_not_found` - License key doesn't exist
- `license_expired` - License has expired
- `license_revoked` - License was revoked by admin
- `hwid_mismatch` - HWID doesn't match bound value
- `rate_limit_exceeded` - Too many validation requests

### 2. Generate License

Creates a new license for a buyer.

**Endpoint:** `POST /generate-license`

**Headers:**
```
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json
```

**Request:**
```json
{
  "script_id": "my-admin-script",
  "buyer_userid": "123456789",
  "duration_days": 365,
  "features": ["premium"],
  "max_hwids": 1
}
```

**Response:**
```json
{
  "license_key": "XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX",
  "created_at": "2025-10-24T12:00:00Z",
  "expires_at": "2026-10-24T12:00:00Z",
  "buyer_userid": "123456789",
  "download_url": "https://cdn.example.com/protected/..."
}
```

### 3. Track Obfuscation

Records obfuscation events for analytics.

**Endpoint:** `POST /track-obfuscation`

**Headers:**
```
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json
```

**Request:**
```json
{
  "script_id": "my-admin-script",
  "tier": "standard",
  "lines_of_code": 1250,
  "obfuscation_time_ms": 1543,
  "client_version": "0.1.0"
}
```

**Response:**
```json
{
  "tracked": true,
  "event_id": "evt_abc123xyz"
}
```

### 4. List Licenses

Retrieves all licenses for a script.

**Endpoint:** `GET /licenses?script_id=my-admin-script`

**Headers:**
```
Authorization: Bearer YOUR_API_KEY
```

**Response:**
```json
{
  "licenses": [
    {
      "license_key": "XXXX-...",
      "buyer_userid": "123456789",
      "created_at": "2025-10-24T12:00:00Z",
      "expires_at": "2026-10-24T12:00:00Z",
      "status": "active",
      "last_validated": "2025-10-24T15:30:00Z"
    }
  ],
  "total": 1,
  "page": 1
}
```

### 5. Revoke License

Revokes a license key.

**Endpoint:** `POST /revoke-license`

**Headers:**
```
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json
```

**Request:**
```json
{
  "license_key": "XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX",
  "reason": "chargeback"
}
```

**Response:**
```json
{
  "revoked": true,
  "revoked_at": "2025-10-24T16:00:00Z"
}
```

---

## Integration Examples

### Example 1: Protect and Generate License

```bash
#!/bin/bash

# Step 1: Obfuscate script
luau-obfuscator protect input.lua \
  --output protected.lua \
  --tier standard \
  --script-id my-admin-script

# Step 2: Generate license for buyer
luau-obfuscator generate-license \
  --script-id my-admin-script \
  --buyer-userid 123456789 \
  --api-key $LUAU_OBFUSCATOR_API_KEY \
  --duration-days 365

# Output: License key XXXX-XXXX-...
```

### Example 2: Automated Sales Flow

```python
import requests
import subprocess

API_KEY = "your-api-key"
API_BASE = "https://api.example.com/v1"

def create_protected_script_for_buyer(script_path, buyer_userid):
    # 1. Generate unique license
    response = requests.post(
        f"{API_BASE}/generate-license",
        headers={"Authorization": f"Bearer {API_KEY}"},
        json={
            "script_id": "my-script",
            "buyer_userid": buyer_userid,
            "duration_days": 365,
            "max_hwids": 1
        }
    )
    
    license_data = response.json()
    license_key = license_data["license_key"]
    
    # 2. Obfuscate script with license embedded
    subprocess.run([
        "luau-obfuscator", "protect", script_path,
        "--output", f"protected_{buyer_userid}.lua",
        "--license-key", license_key,
        "--hwid", buyer_userid,
        "--tier", "standard"
    ])
    
    # 3. Return download link
    return {
        "license_key": license_key,
        "download_url": f"/downloads/protected_{buyer_userid}.lua",
        "expires_at": license_data["expires_at"]
    }

# Usage
result = create_protected_script_for_buyer("admin_commands.lua", "123456789")
print(f"License: {result['license_key']}")
print(f"Download: {result['download_url']}")
```

### Example 3: Runtime Validation in Protected Script

The obfuscator automatically injects validation code, but here's what it looks like:

```lua
-- Auto-generated validation code (embedded in protected script)
local HttpService = game:GetService("HttpService")
local Players = game:GetService("Players")

local LICENSE_KEY = "[EMBEDDED_LICENSE_KEY]"
local API_URL = "https://api.example.com/v1/validate-license"

local function validateLicense()
    local hwid = tostring(Players.LocalPlayer.UserId)
    
    local success, response = pcall(function()
        return HttpService:RequestAsync({
            Url = API_URL,
            Method = "POST",
            Headers = {
                ["Content-Type"] = "application/json"
            },
            Body = HttpService:JSONEncode({
                license_key = LICENSE_KEY,
                hwid = hwid,
                script_id = "[SCRIPT_ID]",
                client_version = "0.1.0"
            })
        })
    end)
    
    if not success then
        error("License validation failed: Network error")
    end
    
    local data = HttpService:JSONDecode(response.Body)
    
    if not data.valid then
        error("Invalid license: " .. (data.message or "Unknown error"))
    end
    
    return true
end

-- Validate before executing protected code
if not validateLicense() then
    return
end

-- Your protected code runs here...
```

---

## Error Handling

### HTTP Status Codes

- **200 OK** - Request successful
- **400 Bad Request** - Invalid request format
- **401 Unauthorized** - Invalid or missing API key
- **403 Forbidden** - API key doesn't have permission
- **404 Not Found** - Resource doesn't exist
- **429 Too Many Requests** - Rate limit exceeded
- **500 Internal Server Error** - Server error

### Retry Logic

The CLI tool implements exponential backoff:

```
Attempt 1: Immediate
Attempt 2: Wait 1 second
Attempt 3: Wait 2 seconds
Attempt 4: Wait 4 seconds
Max attempts: 5
```

### Offline Mode

If the API is unreachable during obfuscation:

```bash
# Enable offline mode (skips API calls)
luau-obfuscator protect input.lua \
  --output protected.lua \
  --offline
```

**Limitations:**
- No license generation
- No analytics tracking
- Runtime validation will fail if offline

---

## Best Practices

### Security

1. **Never commit API keys to version control**
   ```bash
   # Use environment variables
   export LUAU_OBFUSCATOR_API_KEY="..."
   
   # Or use .env files (add to .gitignore)
   echo "LUAU_OBFUSCATOR_API_KEY=..." > .env
   ```

2. **Rotate API keys periodically**
   - Rotate every 90 days
   - Rotate immediately if compromised

3. **Use HTTPS only**
   - Never send requests over HTTP
   - Verify SSL certificates

### Performance

1. **Cache validation results**
   ```lua
   -- Cache for 1 hour to reduce API calls
   local validationCache = {}
   local CACHE_TTL = 3600
   
   function cachedValidation(license_key, hwid)
       local cacheKey = license_key .. ":" .. hwid
       local cached = validationCache[cacheKey]
       
       if cached and (os.time() - cached.timestamp) < CACHE_TTL then
           return cached.result
       end
       
       local result = validateLicense(license_key, hwid)
       validationCache[cacheKey] = {
           result = result,
           timestamp = os.time()
       }
       
       return result
   end
   ```

2. **Batch license generation**
   ```bash
   # Generate multiple licenses at once
   for userid in $(cat buyer_list.txt); do
       luau-obfuscator generate-license \
         --script-id my-script \
         --buyer-userid $userid \
         --api-key $API_KEY &
   done
   wait
   ```

### Monitoring

1. **Track validation success rates**
2. **Monitor API latency**
3. **Alert on elevated error rates**
4. **Log all license generations**

### Testing

1. **Use test API keys for development**
   ```bash
   export LUAU_OBFUSCATOR_API_KEY="test_key_..."
   ```

2. **Mock API responses in tests**
   ```rust
   #[cfg(test)]
   mod tests {
       #[test]
       fn test_license_validation() {
           let mock_server = MockServer::start();
           mock_server.expect(
               Expectation::matching(request::method_path("POST", "/v1/validate-license"))
                   .respond_with(status_code(200).body(r#"{"valid": true}"#))
           );
           
           // Test validation logic...
       }
   }
   ```

---

## Rate Limits

### Developer API

- **License generation**: 10 requests/minute
- **License listing**: 60 requests/minute
- **Revocation**: 10 requests/minute

### Runtime Validation

- **Per license key**: 100 validations/hour
- **Per HWID**: 100 validations/hour

### Handling Rate Limits

```python
import time
from requests.exceptions import HTTPError

def generate_license_with_retry(data, max_retries=3):
    for attempt in range(max_retries):
        try:
            response = requests.post(f"{API_BASE}/generate-license", json=data)
            response.raise_for_status()
            return response.json()
        except HTTPError as e:
            if e.response.status_code == 429:
                retry_after = int(e.response.headers.get("Retry-After", 60))
                time.sleep(retry_after)
            else:
                raise
    
    raise Exception("Max retries exceeded")
```

---

## Support

For API issues or questions:

- **Documentation**: https://docs.example.com/api
- **Support Email**: api-support@example.com
- **Status Page**: https://status.example.com
- **Discord**: https://discord.gg/example

---

## Changelog

### v1.0.0 (2025-10-24)
- Initial API release
- License validation
- License generation
- Analytics tracking
- HWID binding
