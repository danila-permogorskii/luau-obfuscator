-- Combat System
-- Handles damage calculation, hit detection, and combat mechanics

local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local RunService = game:GetService("RunService")

local CombatSystem = {}
CombatSystem.__index = CombatSystem

-- Damage types
local DamageTypes = {
	Physical = "Physical",
	Fire = "Fire",
	Ice = "Ice",
	Poison = "Poison",
	Magic = "Magic"
}

-- Weapon configurations
local WeaponConfigs = {
	["sword"] = {
		damage = 20,
		damageType = DamageTypes.Physical,
		cooldown = 1.0,
		range = 10,
		critChance = 0.15,
		critMultiplier = 2.0
	},
	["hammer"] = {
		damage = 35,
		damageType = DamageTypes.Physical,
		cooldown = 2.0,
		range = 8,
		critChance = 0.10,
		critMultiplier = 2.5
	},
	["staff"] = {
		damage = 25,
		damageType = DamageTypes.Magic,
		cooldown = 1.5,
		range = 20,
		critChance = 0.20,
		critMultiplier = 1.8
	}
}

function CombatSystem.new()
	local self = setmetatable({}, CombatSystem)
	self.activeCombatants = {} -- {player = {lastAttack, combo, statusEffects}}
	self.damageHistory = {} -- Recent damage events for anti-cheat
	return self
end

function CombatSystem:registerCombatant(player)
	self.activeCombatants[player] = {
		lastAttack = 0,
		combo = 0,
		statusEffects = {},
		blocking = false
	}
end

function CombatSystem:unregisterCombatant(player)
	self.activeCombatants[player] = nil
end

function CombatSystem:calculateDamage(attacker, target, weaponId)
	local weapon = WeaponConfigs[weaponId]
	if not weapon then
		return 0
	end
	
	local baseDamage = weapon.damage
	
	-- Apply combo multiplier
	local combatData = self.activeCombatants[attacker]
	if combatData then
		local comboBonus = math.min(combatData.combo * 0.05, 0.5) -- Max 50% bonus
		baseDamage = baseDamage * (1 + comboBonus)
	end
	
	-- Critical hit calculation
	local isCrit = math.random() < weapon.critChance
	if isCrit then
		baseDamage = baseDamage * weapon.critMultiplier
	end
	
	-- Apply target's resistances
	local targetCombatData = self.activeCombatants[target]
	if targetCombatData then
		-- Blocking reduces damage
		if targetCombatData.blocking then
			baseDamage = baseDamage * 0.3 -- 70% damage reduction
		end
		
		-- Status effect modifiers
		for effect, data in pairs(targetCombatData.statusEffects) do
			if effect == "Armor" then
				baseDamage = baseDamage * (1 - data.reduction)
			elseif effect == "Vulnerable" then
				baseDamage = baseDamage * 1.5
			end
		end
	end
	
	return math.floor(baseDamage), isCrit
end

function CombatSystem:performAttack(attacker, target, weaponId)
	local weapon = WeaponConfigs[weaponId]
	if not weapon then
		return false, "Invalid weapon"
	end
	
	local combatData = self.activeCombatants[attacker]
	if not combatData then
		return false, "Attacker not registered"
	end
	
	-- Cooldown check
	local currentTime = tick()
	if currentTime - combatData.lastAttack < weapon.cooldown then
		return false, "Weapon on cooldown"
	end
	
	-- Range check
	local attackerChar = attacker.Character
	local targetChar = target.Character
	
	if not attackerChar or not targetChar then
		return false, "Missing character"
	end
	
	local attackerHRP = attackerChar:FindFirstChild("HumanoidRootPart")
	local targetHRP = targetChar:FindFirstChild("HumanoidRootPart")
	
	if not attackerHRP or not targetHRP then
		return false, "Missing HumanoidRootPart"
	end
	
	local distance = (attackerHRP.Position - targetHRP.Position).Magnitude
	if distance > weapon.range then
		return false, "Target out of range"
	end
	
	-- Calculate and apply damage
	local damage, isCrit = self:calculateDamage(attacker, target, weaponId)
	
	local targetHumanoid = targetChar:FindFirstChild("Humanoid")
	if targetHumanoid then
		targetHumanoid:TakeDamage(damage)
		
		-- Update combat data
		combatData.lastAttack = currentTime
		combatData.combo = combatData.combo + 1
		
		-- Reset combo after delay
		delay(3, function()
			if self.activeCombatants[attacker] then
				self.activeCombatants[attacker].combo = 0
			end
		end)
		
		-- Log damage event (for anti-cheat)
		table.insert(self.damageHistory, {
			attacker = attacker.UserId,
			target = target.UserId,
			damage = damage,
			isCrit = isCrit,
			weapon = weaponId,
			timestamp = currentTime
		})
		
		return true, damage, isCrit
	end
	
	return false, "Failed to apply damage"
end

function CombatSystem:applyStatusEffect(target, effectName, duration, data)
	local combatData = self.activeCombatants[target]
	if not combatData then
		return false
	end
	
	combatData.statusEffects[effectName] = {
		data = data,
		expireTime = tick() + duration
	}
	
	-- Auto-remove after duration
	delay(duration, function()
		if self.activeCombatants[target] then
			self.activeCombatants[target].statusEffects[effectName] = nil
		end
	end)
	
	return true
end

function CombatSystem:setBlocking(player, isBlocking)
	local combatData = self.activeCombatants[player]
	if combatData then
		combatData.blocking = isBlocking
		return true
	end
	return false
end

function CombatSystem:healPlayer(player, amount)
	local character = player.Character
	if not character then
		return false
	end
	
	local humanoid = character:FindFirstChild("Humanoid")
	if humanoid then
		humanoid.Health = math.min(humanoid.Health + amount, humanoid.MaxHealth)
		return true
	end
	
	return false
end

function CombatSystem:getCombatStats(player)
	local combatData = self.activeCombatants[player]
	if not combatData then
		return nil
	end
	
	return {
		combo = combatData.combo,
		blocking = combatData.blocking,
		statusEffects = combatData.statusEffects,
		lastAttack = combatData.lastAttack
	}
end

function CombatSystem:resetCombat(player)
	local combatData = self.activeCombatants[player]
	if combatData then
		combatData.combo = 0
		combatData.statusEffects = {}
		combatData.blocking = false
		return true
	end
	return false
end

return CombatSystem
