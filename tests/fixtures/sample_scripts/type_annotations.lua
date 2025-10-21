-- Luau type annotation test script

-- Type annotations on variables
local name: string = "TestPlayer"
local age: number = 25
local isActive: boolean = true

-- Function with type annotations
local function greet(playerName: string): string
    return "Hello, " .. playerName .. "!"
end

-- Function with multiple typed parameters
local function calculate(x: number, y: number, operation: string): number
    if operation == "add" then
        return x + y
    elseif operation == "multiply" then
        return x * y
    end
    return 0
end

-- Table type annotation
type PlayerData = {
    userId: number,
    username: string,
    level: number,
    inventory: {string}
}

local player: PlayerData = {
    userId = 123456,
    username = "Player123",
    level = 50,
    inventory = {"sword", "shield", "potion"}
}

-- Optional types
local optional: string? = nil
optional = "now it has a value"

-- Union types
type StringOrNumber = string | number
local flexible: StringOrNumber = "string"
flexible = 42

-- Generic type
type Array<T> = {T}
local numbers: Array<number> = {1, 2, 3, 4, 5}
local strings: Array<string> = {"a", "b", "c"}

-- Interface-style type
type GameObject = {
    position: Vector3,
    rotation: CFrame,
    scale: Vector3,
    update: (self: GameObject, deltaTime: number) -> ()
}

-- Type aliases
type UserId = number
type Username = string

local function getUserInfo(id: UserId): Username
    return "User" .. tostring(id)
end

-- Intersection types (using &)
type Named = { name: string }
type Aged = { age: number }
type Person = Named & Aged

local person: Person = {
    name = "Alice",
    age = 30
}

-- Function type annotations
type MathOperation = (number, number) -> number

local add: MathOperation = function(a, b)
    return a + b
end

local multiply: MathOperation = function(a, b)
    return a * b
end

-- Complex nested types
type GameState = {
    players: {[UserId]: PlayerData},
    settings: {
        maxPlayers: number,
        difficulty: "easy" | "medium" | "hard",
        options: {[string]: boolean}
    },
    timestamp: number
}

local gameState: GameState = {
    players = {},
    settings = {
        maxPlayers = 10,
        difficulty = "medium",
        options = {
            pvpEnabled = true,
            friendlyFire = false
        }
    },
    timestamp = os.time()
}

-- Literal types
type Direction = "north" | "south" | "east" | "west"
local heading: Direction = "north"

-- Type casting
local anyValue: any = "123"
local numberValue: number = anyValue :: number

-- Return multiple values with types
local function getPosition(): (number, number, number)
    return 10, 20, 30
end

local x: number, y: number, z: number = getPosition()

-- Export types (for modules)
export type PublicPlayerData = {
    username: string,
    level: number
}

return {
    greet = greet,
    calculate = calculate,
    player = player,
    gameState = gameState
}
