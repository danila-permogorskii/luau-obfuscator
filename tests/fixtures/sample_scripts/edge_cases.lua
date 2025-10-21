-- Edge case test script for obfuscator

-- Empty function
local function emptyFunction()
end

-- Single character identifiers
local a = 1
local b = 2
local c = a + b

-- Very long identifier name
local thisIsAVeryLongIdentifierNameThatExceedsMostReasonableLengthExpectationsAndShouldStillBeHandledCorrectlyByTheObfuscator = "long name"

-- Unicode in strings
local unicode = "Hello 世界 🌍"
local emoji = "🎮🎯🎲"

-- Empty strings
local empty = ""
local emptyTable = {}

-- Single line everything
local x = 1 local y = 2 local z = x + y

-- Nested functions
local function outer()
    local function middle()
        local function inner()
            return "deeply nested"
        end
        return inner()
    end
    return middle()
end

-- Large numbers
local maxInt = 9007199254740991  -- Max safe integer in Lua
local minInt = -9007199254740991
local largeFloat = 1.7976931348623157e+308

-- Special characters in strings
local special = "\n\t\r\"\'\0"
local backslash = "\\"

-- Multiple return values
local function multiReturn()
    return 1, 2, 3, 4, 5
end

local a, b, c, d, e = multiReturn()

-- Varargs
local function varargs(...)
    local args = {...}
    return #args
end

-- Trailing comma in table
local tableWithTrailingComma = {
    1,
    2,
    3,
}

-- Semicolons (valid but unusual in Lua)
local withSemicolon = 1;
local anotherOne = 2;

-- Comments everywhere
--[[Multiline
comment
block]]--
local commented = "value" -- inline comment

-- Single statement on multiple lines
local multiline =
    "This is " ..
    "a string " ..
    "concatenated " ..
    "across lines"

-- Return value
return {
    test = "edge cases handled",
    unicode = unicode,
    emoji = emoji,
    nested = outer(),
    count = varargs(1, 2, 3, 4, 5)
}
