-- Complex Luau script with various features

local MODULE_VERSION = "1.0.0"
local API_KEY = "secret_key_12345"
local MAX_RETRIES = 3
local TIMEOUT = 5.5

local DataManager = {}
DataManager.__index = DataManager

function DataManager.new()
    local self = setmetatable({}, DataManager)
    self.cache = {}
    self.retryCount = 0
    return self
end

function DataManager:fetchData(userId)
    if self.cache[userId] then
        return self.cache[userId]
    end
    
    for i = 1, MAX_RETRIES do
        local success, result = pcall(function()
            -- Simulate API call
            return {userId = userId, data = "sample"}
        end)
        
        if success then
            self.cache[userId] = result
            return result
        else
            self.retryCount = self.retryCount + 1
            wait(TIMEOUT)
        end
    end
    
    error("Failed to fetch data after " .. MAX_RETRIES .. " retries")
end

function DataManager:clearCache()
    self.cache = {}
    print("Cache cleared")
end

return DataManager
