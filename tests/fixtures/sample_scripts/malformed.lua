-- Intentionally malformed Lua script for parser error handling tests

-- Missing end keyword
function missingEnd()
    print("oops")
-- function never closed

-- Unclosed string
local badString = "this string never closes

-- Invalid syntax
local x = = 5

-- Unclosed parenthesis
local y = (10 + 20

-- Unexpected token
local z = 1 + + 2

-- Invalid table
local badTable = {1, 2,, 3}

-- Missing comma in table
local anotherBad = {a = 1 b = 2}

-- Incomplete if statement
if condition then
    doSomething()
-- missing end

-- Invalid function call
func(()

-- Unclosed comment
--[[ This comment
never closes
