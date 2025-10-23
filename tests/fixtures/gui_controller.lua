-- GUI Controller System
-- Manages UI elements, animations, and user interactions

local Players = game:GetService("Players")
local TweenService = game:GetService("TweenService")
local UserInputService = game:GetService("UserInputService")
local RunService = game:GetService("RunService")

local GUIController = {}
GUIController.__index = GUIController

function GUIController.new(player)
	local self = setmetatable({}, GUIController)
	self.player = player
	self.screenGui = nil
	self.elements = {}
	self.animations = {}
	self:initialize()
	return self
end

function GUIController:initialize()
	-- Create main ScreenGui
	self.screenGui = Instance.new("ScreenGui")
	self.screenGui.Name = "MainUI"
	self.screenGui.ResetOnSpawn = false
	self.screenGui.ZIndexBehavior = Enum.ZIndexBehavior.Sibling
	self.screenGui.Parent = self.player:WaitForChild("PlayerGui")
	
	-- Create main container
	local mainFrame = Instance.new("Frame")
	mainFrame.Name = "MainContainer"
	mainFrame.Size = UDim2.new(1, 0, 1, 0)
	mainFrame.Position = UDim2.new(0, 0, 0, 0)
	mainFrame.BackgroundTransparency = 1
	mainFrame.Parent = self.screenGui
	
	self.elements.mainContainer = mainFrame
	
	-- Setup default elements
	self:createHealthBar()
	self:createInventoryButton()
	self:createNotificationArea()
end

function GUIController:createHealthBar()
	local healthBarBg = Instance.new("Frame")
	healthBarBg.Name = "HealthBarBackground"
	healthBarBg.Size = UDim2.new(0, 300, 0, 30)
	healthBarBg.Position = UDim2.new(0, 20, 1, -50)
	healthBarBg.AnchorPoint = Vector2.new(0, 1)
	healthBarBg.BackgroundColor3 = Color3.fromRGB(40, 40, 40)
	healthBarBg.BorderSizePixel = 2
	healthBarBg.BorderColor3 = Color3.fromRGB(0, 0, 0)
	healthBarBg.Parent = self.elements.mainContainer
	
	local healthBar = Instance.new("Frame")
	healthBar.Name = "HealthBar"
	healthBar.Size = UDim2.new(1, -4, 1, -4)
	healthBar.Position = UDim2.new(0, 2, 0, 2)
	healthBar.BackgroundColor3 = Color3.fromRGB(255, 50, 50)
	healthBar.BorderSizePixel = 0
	healthBar.Parent = healthBarBg
	
	local healthText = Instance.new("TextLabel")
	healthText.Name = "HealthText"
	healthText.Size = UDim2.new(1, 0, 1, 0)
	healthText.BackgroundTransparency = 1
	healthText.Text = "100 / 100"
	healthText.TextColor3 = Color3.fromRGB(255, 255, 255)
	healthText.TextStrokeTransparency = 0.5
	healthText.TextSize = 18
	healthText.Font = Enum.Font.GothamBold
	healthText.Parent = healthBar
	
	self.elements.healthBar = healthBar
	self.elements.healthText = healthText
	
	-- Connect to character health
	self:connectHealthUpdates()
end

function GUIController:connectHealthUpdates()
	local character = self.player.Character or self.player.CharacterAdded:Wait()
	local humanoid = character:WaitForChild("Humanoid")
	
	local function updateHealth()
		local currentHealth = math.floor(humanoid.Health)
		local maxHealth = math.floor(humanoid.MaxHealth)
		local healthPercent = currentHealth / maxHealth
		
		-- Update health bar size with animation
		local tweenInfo = TweenInfo.new(
			0.3,
			Enum.EasingStyle.Quad,
			Enum.EasingDirection.Out
		)
		
		local tween = TweenService:Create(
			self.elements.healthBar,
			tweenInfo,
			{Size = UDim2.new(healthPercent, -4, 1, -4)}
		)
		tween:Play()
		
		-- Update text
		self.elements.healthText.Text = currentHealth .. " / " .. maxHealth
		
		-- Change color based on health percentage
		if healthPercent > 0.5 then
			self.elements.healthBar.BackgroundColor3 = Color3.fromRGB(50, 255, 50)
		elseif healthPercent > 0.25 then
			self.elements.healthBar.BackgroundColor3 = Color3.fromRGB(255, 200, 50)
		else
			self.elements.healthBar.BackgroundColor3 = Color3.fromRGB(255, 50, 50)
		end
	end
	
	humanoid.HealthChanged:Connect(updateHealth)
	updateHealth()
end

function GUIController:createInventoryButton()
	local button = Instance.new("TextButton")
	button.Name = "InventoryButton"
	button.Size = UDim2.new(0, 60, 0, 60)
	button.Position = UDim2.new(1, -80, 1, -80)
	button.AnchorPoint = Vector2.new(0, 1)
	button.BackgroundColor3 = Color3.fromRGB(60, 60, 60)
	button.BorderSizePixel = 2
	button.BorderColor3 = Color3.fromRGB(0, 0, 0)
	button.Text = "I"
	button.TextColor3 = Color3.fromRGB(255, 255, 255)
	button.TextSize = 32
	button.Font = Enum.Font.GothamBold
	button.Parent = self.elements.mainContainer
	
	-- Add hover effect
	button.MouseEnter:Connect(function()
		self:animateButtonHover(button, true)
	end)
	
	button.MouseLeave:Connect(function()
		self:animateButtonHover(button, false)
	end)
	
	button.MouseButton1Click:Connect(function()
		self:toggleInventory()
	end)
	
	self.elements.inventoryButton = button
end

function GUIController:animateButtonHover(button, isHovering)
	local tweenInfo = TweenInfo.new(
		0.2,
		Enum.EasingStyle.Quad,
		Enum.EasingDirection.Out
	)
	
	local targetColor = isHovering 
		and Color3.fromRGB(80, 80, 80) 
		or Color3.fromRGB(60, 60, 60)
	
	local targetSize = isHovering
		and UDim2.new(0, 65, 0, 65)
		or UDim2.new(0, 60, 0, 60)
	
	local colorTween = TweenService:Create(button, tweenInfo, {BackgroundColor3 = targetColor})
	local sizeTween = TweenService:Create(button, tweenInfo, {Size = targetSize})
	
	colorTween:Play()
	sizeTween:Play()
end

function GUIController:createNotificationArea()
	local container = Instance.new("Frame")
	container.Name = "NotificationContainer"
	container.Size = UDim2.new(0, 300, 0, 400)
	container.Position = UDim2.new(1, -320, 0, 20)
	container.BackgroundTransparency = 1
	container.Parent = self.elements.mainContainer
	
	self.elements.notificationContainer = container
	self.activeNotifications = {}
end

function GUIController:showNotification(title, message, duration)
	duration = duration or 3
	
	local notification = Instance.new("Frame")
	notification.Size = UDim2.new(1, 0, 0, 80)
	notification.BackgroundColor3 = Color3.fromRGB(40, 40, 40)
	notification.BorderSizePixel = 2
	notification.BorderColor3 = Color3.fromRGB(100, 100, 100)
	
	local titleLabel = Instance.new("TextLabel")
	titleLabel.Name = "Title"
	titleLabel.Size = UDim2.new(1, -10, 0, 25)
	titleLabel.Position = UDim2.new(0, 5, 0, 5)
	titleLabel.BackgroundTransparency = 1
	titleLabel.Text = title
	titleLabel.TextColor3 = Color3.fromRGB(255, 255, 255)
	titleLabel.TextSize = 16
	titleLabel.Font = Enum.Font.GothamBold
	titleLabel.TextXAlignment = Enum.TextXAlignment.Left
	titleLabel.Parent = notification
	
	local messageLabel = Instance.new("TextLabel")
	messageLabel.Name = "Message"
	messageLabel.Size = UDim2.new(1, -10, 0, 45)
	messageLabel.Position = UDim2.new(0, 5, 0, 30)
	messageLabel.BackgroundTransparency = 1
	messageLabel.Text = message
	messageLabel.TextColor3 = Color3.fromRGB(200, 200, 200)
	messageLabel.TextSize = 14
	messageLabel.Font = Enum.Font.Gotham
	messageLabel.TextXAlignment = Enum.TextXAlignment.Left
	messageLabel.TextYAlignment = Enum.TextYAlignment.Top
	messageLabel.TextWrapped = true
	messageLabel.Parent = notification
	
	-- Position notification
	local yOffset = #self.activeNotifications * 90
	notification.Position = UDim2.new(0, 0, 0, yOffset)
	notification.Parent = self.elements.notificationContainer
	
	table.insert(self.activeNotifications, notification)
	
	-- Fade in animation
	notification.BackgroundTransparency = 1
	local fadeInTween = TweenService:Create(
		notification,
		TweenInfo.new(0.3, Enum.EasingStyle.Quad, Enum.EasingDirection.Out),
		{BackgroundTransparency = 0}
	)
	fadeInTween:Play()
	
	-- Auto-remove after duration
	delay(duration, function()
		self:removeNotification(notification)
	end)
end

function GUIController:removeNotification(notification)
	-- Fade out
	local fadeOutTween = TweenService:Create(
		notification,
		TweenInfo.new(0.3, Enum.EasingStyle.Quad, Enum.EasingDirection.Out),
		{BackgroundTransparency = 1}
	)
	fadeOutTween:Play()
	
	fadeOutTween.Completed:Connect(function()
		-- Remove from active notifications
		for i, notif in ipairs(self.activeNotifications) do
			if notif == notification then
				table.remove(self.activeNotifications, i)
				break
			end
		end
		
		notification:Destroy()
		
		-- Reposition remaining notifications
		self:repositionNotifications()
	end)
end

function GUIController:repositionNotifications()
	for i, notification in ipairs(self.activeNotifications) do
		local targetPosition = UDim2.new(0, 0, 0, (i - 1) * 90)
		
		local tween = TweenService:Create(
			notification,
			TweenInfo.new(0.3, Enum.EasingStyle.Quad, Enum.EasingDirection.Out),
			{Position = targetPosition}
		)
		tween:Play()
	end
end

function GUIController:toggleInventory()
	-- Placeholder for inventory toggle
	self:showNotification("Inventory", "Inventory system not yet implemented", 2)
end

function GUIController:cleanup()
	if self.screenGui then
		self.screenGui:Destroy()
	end
	
	self.elements = {}
	self.animations = {}
	self.activeNotifications = {}
end

return GUIController
