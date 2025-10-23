-- Admin Command System
-- A comprehensive admin system with command processing and permission levels

local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local MessagingService = game:GetService("MessagingService")

local AdminSystem = {}
AdminSystem.__index = AdminSystem

-- Permission levels
local PermissionLevels = {
	Moderator = 1,
	Admin = 2,
	Owner = 3
}

-- Admin list (UserId -> Permission Level)
local AdminList = {
	[123456] = PermissionLevels.Owner,
	[789012] = PermissionLevels.Admin,
	[345678] = PermissionLevels.Moderator
}

function AdminSystem.new()
	local self = setmetatable({}, AdminSystem)
	self.commands = {}
	self.commandHistory = {}
	self:registerDefaultCommands()
	return self
end

function AdminSystem:isAdmin(player)
	return AdminList[player.UserId] ~= nil
end

function AdminSystem:getPermissionLevel(player)
	return AdminList[player.UserId] or 0
end

function AdminSystem:registerCommand(name, permission, callback)
	self.commands[name:lower()] = {
		permission = permission,
		callback = callback
	}
end

function AdminSystem:executeCommand(player, commandString)
	if not self:isAdmin(player) then
		return false, "You do not have permission to use admin commands"
	end
	
	local parts = commandString:split(" ")
	local commandName = parts[1]:lower()
	local args = {}
	for i = 2, #parts do
		table.insert(args, parts[i])
	end
	
	local command = self.commands[commandName]
	if not command then
		return false, "Command not found: " .. commandName
	end
	
	local playerPermission = self:getPermissionLevel(player)
	if playerPermission < command.permission then
		return false, "Insufficient permission level for this command"
	end
	
	-- Log command execution
	table.insert(self.commandHistory, {
		player = player.Name,
		userId = player.UserId,
		command = commandString,
		timestamp = os.time()
	})
	
	local success, result = pcall(function()
		return command.callback(player, args)
	end)
	
	if success then
		return true, result or "Command executed successfully"
	else
		return false, "Command execution failed: " .. tostring(result)
	end
end

function AdminSystem:registerDefaultCommands()
	-- Kick command
	self:registerCommand("kick", PermissionLevels.Moderator, function(executor, args)
		if #args < 1 then
			return "Usage: kick <player>"
		end
		
		local targetName = args[1]
		local target = Players:FindFirstChild(targetName)
		
		if not target then
			return "Player not found: " .. targetName
		end
		
		target:Kick("You have been kicked by an administrator")
		return "Kicked " .. target.Name
	end)
	
	-- Ban command
	self:registerCommand("ban", PermissionLevels.Admin, function(executor, args)
		if #args < 1 then
			return "Usage: ban <player>"
		end
		
		local targetName = args[1]
		local target = Players:FindFirstChild(targetName)
		
		if not target then
			return "Player not found: " .. targetName
		end
		
		-- Store ban in DataStore (simplified here)
		target:Kick("You have been banned")
		return "Banned " .. target.Name
	end)
	
	-- Teleport command
	self:registerCommand("tp", PermissionLevels.Moderator, function(executor, args)
		if #args < 2 then
			return "Usage: tp <player1> <player2>"
		end
		
		local player1 = Players:FindFirstChild(args[1])
		local player2 = Players:FindFirstChild(args[2])
		
		if not player1 or not player2 then
			return "One or both players not found"
		end
		
		local char1 = player1.Character
		local char2 = player2.Character
		
		if not char1 or not char2 then
			return "One or both players do not have a character"
		end
		
		local hrp1 = char1:FindFirstChild("HumanoidRootPart")
		local hrp2 = char2:FindFirstChild("HumanoidRootPart")
		
		if hrp1 and hrp2 then
			hrp1.CFrame = hrp2.CFrame
			return "Teleported " .. player1.Name .. " to " .. player2.Name
		end
		
		return "Failed to teleport"
	end)
	
	-- Speed command
	self:registerCommand("speed", PermissionLevels.Moderator, function(executor, args)
		if #args < 2 then
			return "Usage: speed <player> <speed>"
		end
		
		local targetName = args[1]
		local speed = tonumber(args[2])
		
		if not speed then
			return "Invalid speed value"
		end
		
		local target = Players:FindFirstChild(targetName)
		if not target then
			return "Player not found: " .. targetName
		end
		
		local character = target.Character
		if not character then
			return "Player does not have a character"
		end
		
		local humanoid = character:FindFirstChild("Humanoid")
		if humanoid then
			humanoid.WalkSpeed = speed
			return "Set " .. target.Name .. "'s speed to " .. speed
		end
		
		return "Failed to set speed"
	end)
	
	-- Shutdown command
	self:registerCommand("shutdown", PermissionLevels.Owner, function(executor, args)
		local reason = table.concat(args, " ") or "Server shutdown by administrator"
		
		for _, player in ipairs(Players:GetPlayers()) do
			player:Kick(reason)
		end
		
		return "Server shutdown initiated"
	end)
end

return AdminSystem
