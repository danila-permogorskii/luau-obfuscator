-- Tycoon Manager System
-- Manages tycoon ownership, button purchases, and revenue generation

local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local DataStoreService = game:GetService("DataStoreService")
local RunService = game:GetService("RunService")

local TycoonDataStore = DataStoreService:GetDataStore("TycoonData")

local TycoonManager = {}
TycoonManager.__index = TycoonManager

-- Tycoon button configurations
local ButtonConfigs = {
	["conveyor_1"] = {
		cost = 100,
		revenuePerSecond = 5,
		unlocks = {"conveyor_2"},
		model = "ConveyorModel1"
	},
	["conveyor_2"] = {
		cost = 500,
		revenuePerSecond = 15,
		unlocks = {"upgrader_1", "conveyor_3"},
		model = "ConveyorModel2"
	},
	["conveyor_3"] = {
		cost = 2000,
		revenuePerSecond = 50,
		unlocks = {"upgrader_2"},
		model = "ConveyorModel3"
	},
	["upgrader_1"] = {
		cost = 1000,
		revenueMultiplier = 2.0,
		unlocks = {},
		model = "UpgraderModel1"
	},
	["upgrader_2"] = {
		cost = 5000,
		revenueMultiplier = 3.0,
		unlocks = {"super_upgrader"},
		model = "UpgraderModel2"
	},
	["super_upgrader"] = {
		cost = 25000,
		revenueMultiplier = 5.0,
		unlocks = {},
		model = "SuperUpgraderModel"
	}
}

function TycoonManager.new(tycoonModel, ownerPad)
	local self = setmetatable({}, TycoonManager)
	self.model = tycoonModel
	self.ownerPad = ownerPad
	self.owner = nil
	self.cash = 0
	self.purchasedButtons = {}
	self.revenuePerSecond = 0
	self.revenueMultiplier = 1.0
	self.buttons = {}
	self:initialize()
	return self
end

function TycoonManager:initialize()
	-- Setup ownership pad
	if self.ownerPad then
		self.ownerPad.Touched:Connect(function(hit)
			self:handleOwnerPadTouch(hit)
		end)
	end
	
	-- Setup all buttons
	local buttonsFolder = self.model:FindFirstChild("Buttons")
	if buttonsFolder then
		for _, button in ipairs(buttonsFolder:GetChildren()) do
			if button:IsA("Model") or button:IsA("Part") then
				self:setupButton(button)
			end
		end
	end
	
	-- Start revenue generation
	RunService.Heartbeat:Connect(function(deltaTime)
		self:generateRevenue(deltaTime)
	end)
end

function TycoonManager:handleOwnerPadTouch(hit)
	if self.owner then
		return -- Already owned
	end
	
	local character = hit.Parent
	local player = Players:GetPlayerFromCharacter(character)
	
	if player then
		self:setOwner(player)
	end
end

function TycoonManager:setOwner(player)
	if self.owner then
		return false
	end
	
	self.owner = player
	
	-- Load saved data
	self:loadData()
	
	-- Update owner pad
	if self.ownerPad then
		local billboard = self.ownerPad:FindFirstChild("BillboardGui")
		if billboard then
			local label = billboard:FindFirstChild("TextLabel")
			if label then
				label.Text = player.Name .. "'s Tycoon"
			end
		end
		self.ownerPad.BrickColor = BrickColor.new("Bright green")
	end
	
	-- Handle player leaving
	player.AncestryChanged:Connect(function()
		if not player:IsDescendantOf(game) then
			self:saveData()
			self:resetTycoon()
		end
	end)
	
	return true
end

function TycoonManager:setupButton(button)
	local buttonId = button.Name
	local config = ButtonConfigs[buttonId]
	
	if not config then
		return
	end
	
	self.buttons[buttonId] = {
		model = button,
		config = config,
		purchased = false
	}
	
	-- Find clickable part
	local clickPart = button:FindFirstChild("ClickPart") or button
	
	if clickPart and clickPart:IsA("BasePart") then
		local clickDetector = clickPart:FindFirstChild("ClickDetector")
		if not clickDetector then
			clickDetector = Instance.new("ClickDetector")
			clickDetector.Parent = clickPart
		end
		
		clickDetector.MouseClick:Connect(function(player)
			self:handleButtonClick(player, buttonId)
		end)
	end
	
	-- Update button display
	self:updateButtonDisplay(buttonId)
end

function TycoonManager:handleButtonClick(player, buttonId)
	if player ~= self.owner then
		return
	end
	
	local buttonData = self.buttons[buttonId]
	if not buttonData or buttonData.purchased then
		return
	end
	
	local config = buttonData.config
	
	-- Check if player has enough cash
	if self.cash < config.cost then
		-- Show insufficient funds message
		return
	end
	
	-- Check prerequisites (previous buttons must be purchased)
	if not self:checkPrerequisites(buttonId) then
		return
	end
	
	-- Purchase button
	self.cash = self.cash - config.cost
	self:purchaseButton(buttonId)
end

function TycoonManager:checkPrerequisites(buttonId)
	-- Check if all required buttons are purchased
	for existingButtonId, buttonData in pairs(self.buttons) do
		local existingConfig = buttonData.config
		if existingConfig.unlocks then
			for _, unlockId in ipairs(existingConfig.unlocks) do
				if unlockId == buttonId then
					-- This button is unlocked by existingButtonId
					if not buttonData.purchased then
						return false
					end
				end
			end
		end
	end
	
	return true
end

function TycoonManager:purchaseButton(buttonId)
	local buttonData = self.buttons[buttonId]
	if not buttonData then
		return false
	end
	
	buttonData.purchased = true
	table.insert(self.purchasedButtons, buttonId)
	
	local config = buttonData.config
	
	-- Update revenue
	if config.revenuePerSecond then
		self.revenuePerSecond = self.revenuePerSecond + config.revenuePerSecond
	end
	
	if config.revenueMultiplier then
		self.revenueMultiplier = self.revenueMultiplier * config.revenueMultiplier
	end
	
	-- Spawn model
	if config.model then
		self:spawnButtonModel(buttonId, config.model)
	end
	
	-- Update button display
	self:updateButtonDisplay(buttonId)
	
	-- Unlock dependent buttons
	if config.unlocks then
		for _, unlockId in ipairs(config.unlocks) do
			self:updateButtonDisplay(unlockId)
		end
	end
	
	return true
end

function TycoonManager:spawnButtonModel(buttonId, modelName)
	local modelsFolder = ReplicatedStorage:FindFirstChild("TycoonModels")
	if not modelsFolder then
		return
	end
	
	local model = modelsFolder:FindFirstChild(modelName)
	if not model then
		return
	end
	
	local clone = model:Clone()
	
	-- Position at spawn location
	local spawnLocation = self.model:FindFirstChild("Spawns"):FindFirstChild(buttonId)
	if spawnLocation then
		clone:SetPrimaryPartCFrame(spawnLocation.CFrame)
	end
	
	clone.Parent = self.model:FindFirstChild("Purchases") or self.model
end

function TycoonManager:updateButtonDisplay(buttonId)
	local buttonData = self.buttons[buttonId]
	if not buttonData then
		return
	end
	
	local button = buttonData.model
	local config = buttonData.config
	
	if buttonData.purchased then
		-- Hide purchased button
		button.Transparency = 1
		for _, child in ipairs(button:GetDescendants()) do
			if child:IsA("BasePart") then
				child.Transparency = 1
			end
		end
	else
		-- Check if button can be purchased
		local canPurchase = self:checkPrerequisites(buttonId)
		
		if canPurchase then
			-- Make visible
			button.Transparency = 0
			button.BrickColor = BrickColor.new("Bright green")
		else
			-- Locked
			button.Transparency = 0.5
			button.BrickColor = BrickColor.new("Really red")
		end
	end
end

function TycoonManager:generateRevenue(deltaTime)
	if not self.owner then
		return
	end
	
	local revenue = (self.revenuePerSecond * self.revenueMultiplier) * deltaTime
	self.cash = self.cash + revenue
	
	-- Update cash display
	self:updateCashDisplay()
end

function TycoonManager:updateCashDisplay()
	local cashDisplay = self.model:FindFirstChild("CashDisplay")
	if cashDisplay then
		local billboard = cashDisplay:FindFirstChild("BillboardGui")
		if billboard then
			local label = billboard:FindFirstChild("TextLabel")
			if label then
				label.Text = "$" .. math.floor(self.cash)
			end
		end
	end
end

function TycoonManager:addCash(amount)
	self.cash = self.cash + amount
	self:updateCashDisplay()
end

function TycoonManager:loadData()
	if not self.owner then
		return
	end
	
	local success, data = pcall(function()
		return TycoonDataStore:GetAsync("Tycoon_" .. self.owner.UserId)
	end)
	
	if success and data then
		self.cash = data.cash or 0
		self.purchasedButtons = data.purchasedButtons or {}
		
		-- Re-purchase buttons from saved data
		for _, buttonId in ipairs(self.purchasedButtons) do
			local buttonData = self.buttons[buttonId]
			if buttonData then
				buttonData.purchased = true
				self:spawnButtonModel(buttonId, buttonData.config.model)
				
				-- Update revenue calculations
				if buttonData.config.revenuePerSecond then
					self.revenuePerSecond = self.revenuePerSecond + buttonData.config.revenuePerSecond
				end
				if buttonData.config.revenueMultiplier then
					self.revenueMultiplier = self.revenueMultiplier * buttonData.config.revenueMultiplier
				end
			end
		end
		
		-- Update all button displays
		for buttonId, _ in pairs(self.buttons) do
			self:updateButtonDisplay(buttonId)
		end
	end
end

function TycoonManager:saveData()
	if not self.owner then
		return false
	end
	
	local data = {
		cash = self.cash,
		purchasedButtons = self.purchasedButtons,
		lastSaved = os.time()
	}
	
	local success, err = pcall(function()
		TycoonDataStore:SetAsync("Tycoon_" .. self.owner.UserId, data)
	end)
	
	return success
end

function TycoonManager:resetTycoon()
	self.owner = nil
	self.cash = 0
	self.purchasedButtons = {}
	self.revenuePerSecond = 0
	self.revenueMultiplier = 1.0
	
	-- Reset all buttons
	for buttonId, buttonData in pairs(self.buttons) do
		buttonData.purchased = false
		self:updateButtonDisplay(buttonId)
	end
	
	-- Clear spawned models
	local purchases = self.model:FindFirstChild("Purchases")
	if purchases then
		purchases:ClearAllChildren()
	end
	
	-- Reset owner pad
	if self.ownerPad then
		local billboard = self.ownerPad:FindFirstChild("BillboardGui")
		if billboard then
			local label = billboard:FindFirstChild("TextLabel")
			if label then
				label.Text = "Touch to claim!"
			end
		end
		self.ownerPad.BrickColor = BrickColor.new("White")
	end
end

return TycoonManager
