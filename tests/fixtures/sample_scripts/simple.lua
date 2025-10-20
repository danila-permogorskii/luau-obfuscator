-- Simple Luau script for testing

local message = "Hello, Roblox!"
local count = 42

function greet(name)
    return "Hello, " .. name .. "!"
end

local function calculate(x, y)
    return x + y * 2
end

print(message)
print(greet("World"))
print(calculate(10, 5))
