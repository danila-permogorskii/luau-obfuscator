-- Inventory Management System
-- Handles player inventory, item stacking, and persistence

local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local DataStoreService = game:GetService("DataStoreService")
local HttpService = game:GetService("HttpService")

local InventoryDataStore = DataStoreService:GetDataStore("PlayerInventory")

local InventorySystem = {}
InventorySystem.__index = InventorySystem

-- Item definitions
local ItemDatabase = {
	["sword"] = {
		name = "Sword",
		type = "weapon",
		stackable = false,
		maxStack = 1,
		value = 100
	},
	["potion"] = {
		name = "Health Potion",
		type = "consumable",
		stackable = true,
		maxStack = 99,
		value = 25
	},
	["coin"] = {
		name = "Gold Coin",
		type = "currency",
		stackable = true,
		maxStack = 999999,
		value = 1
	},
	["shield"] = {
		name = "Shield",
		type = "armor",
		stackable = false,
		maxStack = 1,
		value = 150
	}
}

function InventorySystem.new(player)
	local self = setmetatable({}, InventorySystem)
	self.player = player
	self.slots = {} -- {itemId, quantity}
	self.maxSlots = 20
	self:loadInventory()
	return self
end

function InventorySystem:loadInventory()
	local success, data = pcall(function()
		return InventoryDataStore:GetAsync("Player_" .. self.player.UserId)
	end)
	
	if success and data then
		self.slots = data.slots or {}
	else
		-- Initialize empty inventory
		for i = 1, self.maxSlots do
			self.slots[i] = {itemId = nil, quantity = 0}
		end
	end
end

function InventorySystem:saveInventory()
	local data = {
		slots = self.slots,
		lastSaved = os.time()
	}
	
	local success, err = pcall(function()
		InventoryDataStore:SetAsync("Player_" .. self.player.UserId, data)
	end)
	
	return success
end

function InventorySystem:getItem(itemId)
	return ItemDatabase[itemId]
end

function InventorySystem:findEmptySlot()
	for i = 1, self.maxSlots do
		if not self.slots[i].itemId then
			return i
		end
	end
	return nil
end

function InventorySystem:findItemSlot(itemId)
	-- Find first slot with this item
	for i = 1, self.maxSlots do
		if self.slots[i].itemId == itemId then
			return i
		end
	end
	return nil
end

function InventorySystem:addItem(itemId, quantity)
	local item = self:getItem(itemId)
	if not item then
		return false, "Invalid item ID"
	end
	
	if item.stackable then
		-- Try to add to existing stacks
		for i = 1, self.maxSlots do
			if self.slots[i].itemId == itemId then
				local currentStack = self.slots[i].quantity
				local spaceAvailable = item.maxStack - currentStack
				
				if spaceAvailable > 0 then
					local amountToAdd = math.min(quantity, spaceAvailable)
					self.slots[i].quantity = currentStack + amountToAdd
					quantity = quantity - amountToAdd
					
					if quantity == 0 then
						return true, "Item added to inventory"
					end
				end
			end
		end
		
		-- Create new stacks for remaining quantity
		while quantity > 0 do
			local emptySlot = self:findEmptySlot()
			if not emptySlot then
				return false, "Inventory full"
			end
			
			local amountToAdd = math.min(quantity, item.maxStack)
			self.slots[emptySlot] = {
				itemId = itemId,
				quantity = amountToAdd
			}
			quantity = quantity - amountToAdd
		end
		
		return true, "Item added to inventory"
	else
		-- Non-stackable item
		for i = 1, quantity do
			local emptySlot = self:findEmptySlot()
			if not emptySlot then
				return false, "Inventory full (" .. (i-1) .. " items added)"
			end
			
			self.slots[emptySlot] = {
				itemId = itemId,
				quantity = 1
			}
		end
		
		return true, "Item(s) added to inventory"
	end
end

function InventorySystem:removeItem(itemId, quantity)
	local totalRemoved = 0
	
	for i = 1, self.maxSlots do
		if self.slots[i].itemId == itemId then
			local slotQuantity = self.slots[i].quantity
			local amountToRemove = math.min(quantity - totalRemoved, slotQuantity)
			
			self.slots[i].quantity = slotQuantity - amountToRemove
			totalRemoved = totalRemoved + amountToRemove
			
			if self.slots[i].quantity == 0 then
				self.slots[i].itemId = nil
			end
			
			if totalRemoved >= quantity then
				break
			end
		end
	end
	
	if totalRemoved >= quantity then
		return true, "Item removed from inventory"
	else
		return false, "Not enough items to remove"
	end
end

function InventorySystem:hasItem(itemId, quantity)
	local totalCount = 0
	
	for i = 1, self.maxSlots do
		if self.slots[i].itemId == itemId then
			totalCount = totalCount + self.slots[i].quantity
		end
	end
	
	return totalCount >= quantity
end

function InventorySystem:getItemCount(itemId)
	local count = 0
	
	for i = 1, self.maxSlots do
		if self.slots[i].itemId == itemId then
			count = count + self.slots[i].quantity
		end
	end
	
	return count
end

function InventorySystem:swapSlots(slot1, slot2)
	if slot1 < 1 or slot1 > self.maxSlots or slot2 < 1 or slot2 > self.maxSlots then
		return false, "Invalid slot number"
	end
	
	local temp = self.slots[slot1]
	self.slots[slot1] = self.slots[slot2]
	self.slots[slot2] = temp
	
	return true, "Slots swapped"
end

function InventorySystem:clear()
	for i = 1, self.maxSlots do
		self.slots[i] = {itemId = nil, quantity = 0}
	end
end

function InventorySystem:getTotalValue()
	local totalValue = 0
	
	for i = 1, self.maxSlots do
		if self.slots[i].itemId then
			local item = self:getItem(self.slots[i].itemId)
			if item then
				totalValue = totalValue + (item.value * self.slots[i].quantity)
			end
		end
	end
	
	return totalValue
end

return InventorySystem
