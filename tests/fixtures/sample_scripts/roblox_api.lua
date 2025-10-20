-- Roblox API usage test script

local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local player = Players.LocalPlayer
local character = player.Character or player.CharacterAdded:Wait()

local remoteEvent = ReplicatedStorage:WaitForChild("GameEvent")

function onPlayerJoined(newPlayer)
    print("Player joined: " .. newPlayer.Name)
    remoteEvent:FireServer("PlayerJoined", newPlayer.UserId)
end

Players.PlayerAdded:Connect(onPlayerJoined)

-- Vector3 and CFrame usage
local spawnPosition = Vector3.new(0, 10, 0)
workspace.CurrentCamera.CFrame = CFrame.new(spawnPosition)
