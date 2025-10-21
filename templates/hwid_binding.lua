-- HWID Binding Template
-- Enforces hardware/user ID binding for license protection
-- Template variables: {{AUTHORIZED_USERID}}, {{AUTHORIZED_PLACEID}}, {{BINDING_MODE}}

local AUTHORIZED_USERID = {{AUTHORIZED_USERID}} -- nil or number
local AUTHORIZED_PLACEID = {{AUTHORIZED_PLACEID}} -- nil or number
local BINDING_MODE = "{{BINDING_MODE}}" -- "userid", "placeid", "both", "whitelist"

-- HWID validation state
local _hwid_valid = false
local _hwid_error = nil

-- Whitelist of authorized UserIds (for multi-user licenses)
local AUTHORIZED_USERS = {
    -- Populated by template processor if BINDING_MODE == "whitelist"
    {{AUTHORIZED_USERS_LIST}}
}

-- Get current UserId
local function get_user_id()
    local success, result = pcall(function()
        local Players = game:GetService("Players")
        local player = Players.LocalPlayer
        if player then
            return player.UserId
        end
        return nil
    end)
    
    if success then
        return result
    end
    return nil
end

-- Get current PlaceId
local function get_place_id()
    return game.PlaceId
end

-- Validate HWID binding
local function validate_hwid()
    if _hwid_valid then
        return true
    end
    
    if BINDING_MODE == "userid" then
        -- Check UserId binding
        if AUTHORIZED_USERID == nil then
            _hwid_error = "No authorized UserId configured"
            return false
        end
        
        local current_userid = get_user_id()
        if current_userid == nil then
            _hwid_error = "Failed to get current UserId (server-side script?)"
            return false
        end
        
        if current_userid ~= AUTHORIZED_USERID then
            _hwid_error = string.format(
                "UserId mismatch: expected %d, got %d",
                AUTHORIZED_USERID,
                current_userid
            )
            error("[HWID] " .. _hwid_error)
            return false
        end
        
    elseif BINDING_MODE == "placeid" then
        -- Check PlaceId binding
        if AUTHORIZED_PLACEID == nil then
            _hwid_error = "No authorized PlaceId configured"
            return false
        end
        
        local current_placeid = get_place_id()
        if current_placeid ~= AUTHORIZED_PLACEID then
            _hwid_error = string.format(
                "PlaceId mismatch: expected %d, got %d",
                AUTHORIZED_PLACEID,
                current_placeid
            )
            error("[HWID] " .. _hwid_error)
            return false
        end
        
    elseif BINDING_MODE == "both" then
        -- Check both UserId and PlaceId
        if AUTHORIZED_USERID == nil or AUTHORIZED_PLACEID == nil then
            _hwid_error = "Incomplete HWID configuration"
            return false
        end
        
        local current_userid = get_user_id()
        local current_placeid = get_place_id()
        
        if current_userid == nil then
            _hwid_error = "Failed to get current UserId"
            return false
        end
        
        if current_userid ~= AUTHORIZED_USERID then
            _hwid_error = string.format("UserId mismatch: expected %d, got %d", 
                AUTHORIZED_USERID, current_userid)
            error("[HWID] " .. _hwid_error)
            return false
        end
        
        if current_placeid ~= AUTHORIZED_PLACEID then
            _hwid_error = string.format("PlaceId mismatch: expected %d, got %d", 
                AUTHORIZED_PLACEID, current_placeid)
            error("[HWID] " .. _hwid_error)
            return false
        end
        
    elseif BINDING_MODE == "whitelist" then
        -- Check if current UserId is in whitelist
        local current_userid = get_user_id()
        if current_userid == nil then
            _hwid_error = "Failed to get current UserId"
            return false
        end
        
        local found = false
        for _, authorized_id in ipairs(AUTHORIZED_USERS) do
            if current_userid == authorized_id then
                found = true
                break
            end
        end
        
        if not found then
            _hwid_error = string.format("UserId %d not in authorized whitelist", current_userid)
            error("[HWID] " .. _hwid_error)
            return false
        end
        
    else
        -- Invalid binding mode
        _hwid_error = "Invalid HWID binding mode: " .. tostring(BINDING_MODE)
        return false
    end
    
    -- Validation successful
    _hwid_valid = true
    print("[HWID] ✓ Hardware ID validated successfully")
    return true
end

-- Check if HWID is valid (public API)
local function is_hwid_valid()
    return _hwid_valid
end

-- Get HWID validation error (if any)
local function get_hwid_error()
    return _hwid_error
end

-- Periodic HWID re-validation (in case of player changes)
local HWID_CHECK_INTERVAL = 60 -- 1 minute

spawn(function()
    while true do
        wait(HWID_CHECK_INTERVAL)
        
        -- Re-validate HWID silently
        local old_valid = _hwid_valid
        _hwid_valid = false
        
        if not validate_hwid() then
            error("[HWID] HWID validation failed during periodic check: " .. tostring(_hwid_error))
        end
    end
end)

-- Export HWID API
return {
    validate = validate_hwid,
    is_valid = is_hwid_valid,
    get_error = get_hwid_error
}
