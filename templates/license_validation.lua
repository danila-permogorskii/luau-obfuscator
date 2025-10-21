-- License Validation Template
-- Validates license key with API server and checks HWID binding
-- Template variables: {{LICENSE_KEY}}, {{SCRIPT_ID}}, {{API_ENDPOINT}}, {{WATERMARK}}

local LICENSE_KEY = "{{LICENSE_KEY}}"
local SCRIPT_ID = "{{SCRIPT_ID}}"
local API_ENDPOINT = "{{API_ENDPOINT}}"
local WATERMARK = "{{WATERMARK}}"

-- License validation state
local _license_validated = false
local _validation_error = nil

-- Validate license (called at script startup)
local function validate_license()
    if _license_validated then
        return true
    end
    
    -- Get HWID (Roblox UserId)
    local success, hwid = pcall(function()
        local Players = game:GetService("Players")
        local player = Players.LocalPlayer
        if not player then
            -- Server-side script - use PlaceId instead
            return tostring(game.PlaceId)
        end
        return tostring(player.UserId)
    end)
    
    if not success then
        _validation_error = "Failed to get HWID: " .. tostring(hwid)
        return false
    end
    
    -- Attempt online validation (phone home)
    local online_valid = false
    local online_error = nil
    
    pcall(function()
        local HttpService = game:GetService("HttpService")
        
        -- Build validation URL
        local url = string.format(
            "%s/validate?key=%s&script=%s&hwid=%s&watermark=%s",
            API_ENDPOINT,
            HttpService:UrlEncode(LICENSE_KEY),
            HttpService:UrlEncode(SCRIPT_ID),
            HttpService:UrlEncode(hwid),
            HttpService:UrlEncode(WATERMARK)
        )
        
        -- Make HTTP request (GET)
        local response = HttpService:GetAsync(url, true)
        
        -- Parse response
        local data = HttpService:JSONDecode(response)
        
        if data.valid == true then
            online_valid = true
        else
            online_error = data.error or "License validation failed"
        end
    end)
    
    -- For now, allow offline mode (graceful degradation)
    -- In production, you might want to enforce online validation
    if not online_valid then
        warn("[License] Online validation failed: " .. tostring(online_error))
        warn("[License] Running in offline mode")
        -- Still allow execution (graceful degradation)
        -- Change this to 'return false' for strict online-only mode
    else
        print("[License] ✓ License validated successfully")
    end
    
    _license_validated = true
    return true
end

-- Check if license is valid (public API)
local function is_license_valid()
    return _license_validated
end

-- Get validation error (if any)
local function get_validation_error()
    return _validation_error
end

-- Periodic validation (re-validate every N seconds)
local VALIDATION_INTERVAL = 300 -- 5 minutes

spawn(function()
    while true do
        wait(VALIDATION_INTERVAL)
        if _license_validated then
            -- Re-validate silently
            local old_validated = _license_validated
            _license_validated = false
            
            if not validate_license() then
                warn("[License] Re-validation failed - script may stop working")
                -- Optionally: Implement grace period or immediate shutdown
            end
        end
    end
end)

-- Export validation API
return {
    validate = validate_license,
    is_valid = is_license_valid,
    get_error = get_validation_error
}
