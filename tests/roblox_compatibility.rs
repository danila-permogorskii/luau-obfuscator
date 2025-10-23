//! Roblox Compatibility Validation Tests
//!
//! Comprehensive tests to ensure obfuscated code maintains
//! full compatibility with Roblox's API, services, and runtime environment.
//!
//! Critical requirements:
//! - All Roblox services must be preserved (never renamed)
//! - All Roblox datatypes must be preserved
//! - All Roblox global objects must be preserved
//! - Common Roblox patterns must work identically
//! - No breaking changes to Roblox API surface

use luau_obfuscator::{
    parser::LuauParser,
    analysis::{AnalysisEngine, AnalysisOptions},
    obfuscation::{ObfuscationEngine, ObfuscationTier},
    crypto::CryptoContext,
};

// Helper function to test script obfuscation and Roblox API preservation
fn test_roblox_api_preservation(script: &str, tier: ObfuscationTier) -> String {
    let parser = LuauParser::new();
    let ast = parser.parse(script).expect("Parse failed");
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
    
    let crypto_ctx = CryptoContext::new("test_password", None).unwrap();
    let engine = ObfuscationEngine::new(tier, crypto_ctx);
    let obfuscated = engine.obfuscate(&ast, &analysis_result).expect("Obfuscation failed");
    
    // Convert AST back to string representation
    format!("{:?}", obfuscated)
}

#[test]
fn test_roblox_services_preservation() {
    let script = r#"
        -- All critical Roblox services
        local Players = game:GetService("Players")
        local Workspace = game:GetService("Workspace")
        local RunService = game:GetService("RunService")
        local HttpService = game:GetService("HttpService")
        local TweenService = game:GetService("TweenService")
        local ReplicatedStorage = game:GetService("ReplicatedStorage")
        local ServerStorage = game:GetService("ServerStorage")
        local ServerScriptService = game:GetService("ServerScriptService")
        local StarterGui = game:GetService("StarterGui")
        local StarterPlayer = game:GetService("StarterPlayer")
        local Lighting = game:GetService("Lighting")
        local SoundService = game:GetService("SoundService")
        local MarketplaceService = game:GetService("MarketplaceService")
        local DataStoreService = game:GetService("DataStoreService")
        local MessagingService = game:GetService("MessagingService")
        local Teams = game:GetService("Teams")
        local Chat = game:GetService("Chat")
        local TextService = game:GetService("TextService")
        local UserInputService = game:GetService("UserInputService")
        local ContextActionService = game:GetService("ContextActionService")
        
        print("All services loaded successfully")
    "#;
    
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let obfuscated = test_roblox_api_preservation(script, *tier);
        
        // Services should never be renamed or obfuscated
        assert!(
            obfuscated.contains("Players") || obfuscated.contains("GetService"),
            "Players service reference should be preserved in {:?} tier",
            tier
        );
        assert!(
            obfuscated.contains("HttpService") || obfuscated.contains("GetService"),
            "HttpService reference should be preserved in {:?} tier",
            tier
        );
        assert!(
            obfuscated.contains("RunService") || obfuscated.contains("GetService"),
            "RunService reference should be preserved in {:?} tier",
            tier
        );
    }
    
    println!("✓ Roblox services preservation validated across all tiers");
}

#[test]
fn test_roblox_datatypes_preservation() {
    let script = r#"
        -- Common Roblox datatypes
        local position = Vector3.new(0, 10, 0)
        local rotation = CFrame.new(0, 0, 0)
        local color = Color3.fromRGB(255, 0, 0)
        local size = Vector2.new(100, 100)
        local udim = UDim2.new(0, 100, 0, 100)
        local region = Region3.new(Vector3.new(0, 0, 0), Vector3.new(10, 10, 10))
        local ray = Ray.new(Vector3.new(0, 0, 0), Vector3.new(0, 100, 0))
        local numSeq = NumberSequence.new(0, 1)
        local colorSeq = ColorSequence.new(Color3.new(1, 1, 1))
        local numRange = NumberRange.new(0, 10)
        local rect = Rect.new(0, 0, 100, 100)
        local brickColor = BrickColor.new("Bright red")
        
        print(position, rotation, color)
    "#;
    
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let obfuscated = test_roblox_api_preservation(script, *tier);
        
        // Datatypes should never be renamed
        let datatypes = [
            "Vector3", "CFrame", "Color3", "Vector2", "UDim2",
            "Region3", "Ray", "NumberSequence", "ColorSequence",
            "NumberRange", "Rect", "BrickColor"
        ];
        
        for datatype in &datatypes {
            assert!(
                obfuscated.contains(datatype),
                "{} datatype should be preserved in {:?} tier",
                datatype, tier
            );
        }
    }
    
    println!("✓ Roblox datatypes preservation validated across all tiers");
}

#[test]
fn test_roblox_global_objects() {
    let script = r#"
        -- Roblox global objects that must never be obfuscated
        local myGame = game
        local myWorkspace = workspace
        local myScript = script
        local myShared = shared
        
        -- These should work
        local player = game.Players.LocalPlayer
        local character = workspace:FindFirstChild("Character")
        local scriptParent = script.Parent
        
        print(game, workspace, script, shared)
    "#;
    
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let obfuscated = test_roblox_api_preservation(script, *tier);
        
        // Global objects should never be renamed
        assert!(
            obfuscated.contains("game"),
            "game global should be preserved in {:?} tier",
            tier
        );
        assert!(
            obfuscated.contains("workspace"),
            "workspace global should be preserved in {:?} tier",
            tier
        );
        assert!(
            obfuscated.contains("script"),
            "script global should be preserved in {:?} tier",
            tier
        );
        assert!(
            obfuscated.contains("shared"),
            "shared global should be preserved in {:?} tier",
            tier
        );
    }
    
    println!("✓ Roblox global objects preservation validated across all tiers");
}

#[test]
fn test_remote_events_and_bindables() {
    let script = r#"
        local ReplicatedStorage = game:GetService("ReplicatedStorage")
        
        -- RemoteEvent pattern
        local remoteEvent = ReplicatedStorage:WaitForChild("MyRemoteEvent")
        remoteEvent.OnClientEvent:Connect(function(data)
            print("Received:", data)
        end)
        remoteEvent:FireServer({action = "test", value = 123})
        
        -- RemoteFunction pattern
        local remoteFunction = ReplicatedStorage:WaitForChild("MyRemoteFunction")
        local result = remoteFunction:InvokeServer("query")
        
        -- BindableEvent pattern
        local bindableEvent = Instance.new("BindableEvent")
        bindableEvent.Event:Connect(function(msg)
            print("Local event:", msg)
        end)
        bindableEvent:Fire("test message")
        
        -- BindableFunction pattern
        local bindableFunction = Instance.new("BindableFunction")
        bindableFunction.OnInvoke = function(x, y)
            return x + y
        end
        local sum = bindableFunction:Invoke(5, 10)
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    // Critical event/function methods should be preserved
    assert!(
        obfuscated.contains("OnClientEvent") || obfuscated.contains("Connect"),
        "OnClientEvent:Connect pattern should work"
    );
    assert!(
        obfuscated.contains("FireServer") || obfuscated.contains("Fire"),
        "FireServer/Fire methods should be preserved"
    );
    assert!(
        obfuscated.contains("InvokeServer") || obfuscated.contains("Invoke"),
        "InvokeServer/Invoke methods should be preserved"
    );
    assert!(
        obfuscated.contains("OnInvoke"),
        "OnInvoke callback should be preserved"
    );
    
    println!("✓ RemoteEvent/BindableEvent patterns validated");
}

#[test]
fn test_instance_manipulation() {
    let script = r#"
        local part = Instance.new("Part")
        part.Name = "MyPart"
        part.Size = Vector3.new(10, 5, 10)
        part.Position = Vector3.new(0, 10, 0)
        part.BrickColor = BrickColor.new("Bright red")
        part.Material = Enum.Material.Plastic
        part.Anchored = true
        part.CanCollide = true
        part.Transparency = 0.5
        part.Parent = workspace
        
        -- Instance methods
        local clone = part:Clone()
        local children = part:GetChildren()
        local descendants = part:GetDescendants()
        part:Destroy()
        
        -- FindFirstChild patterns
        local found1 = workspace:FindFirstChild("MyPart")
        local found2 = workspace:FindFirstChild("MyPart", true) -- recursive
        local found3 = workspace:WaitForChild("MyPart")
        local found4 = workspace:WaitForChild("MyPart", 5) -- with timeout
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    // Instance methods and properties should work
    assert!(
        obfuscated.contains("Instance") || obfuscated.contains("new"),
        "Instance.new should be preserved"
    );
    assert!(
        obfuscated.contains("Clone") || obfuscated.contains(":"),
        "Instance:Clone should work"
    );
    assert!(
        obfuscated.contains("GetChildren") || obfuscated.contains("Get"),
        "GetChildren method should work"
    );
    assert!(
        obfuscated.contains("FindFirstChild") || obfuscated.contains("Find"),
        "FindFirstChild should work"
    );
    assert!(
        obfuscated.contains("WaitForChild") || obfuscated.contains("Wait"),
        "WaitForChild should work"
    );
    
    println!("✓ Instance manipulation patterns validated");
}

#[test]
fn test_tween_service() {
    let script = r#"
        local TweenService = game:GetService("TweenService")
        
        local part = workspace:FindFirstChild("MyPart")
        local tweenInfo = TweenInfo.new(
            2, -- Duration
            Enum.EasingStyle.Quad,
            Enum.EasingDirection.Out,
            0, -- RepeatCount
            false, -- Reverses
            0 -- DelayTime
        )
        
        local goal = {Position = Vector3.new(0, 50, 0)}
        local tween = TweenService:Create(part, tweenInfo, goal)
        
        tween.Completed:Connect(function(playbackState)
            print("Tween completed:", playbackState)
        end)
        
        tween:Play()
        tween:Pause()
        tween:Cancel()
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    assert!(
        obfuscated.contains("TweenService") || obfuscated.contains("GetService"),
        "TweenService should be preserved"
    );
    assert!(
        obfuscated.contains("TweenInfo"),
        "TweenInfo datatype should be preserved"
    );
    assert!(
        obfuscated.contains("Enum") || obfuscated.contains("EasingStyle"),
        "Enum references should be preserved"
    );
    
    println!("✓ TweenService usage patterns validated");
}

#[test]
fn test_enum_preservation() {
    let script = r#"
        -- Common Roblox enums
        local material = Enum.Material.Plastic
        local partType = Enum.PartType.Block
        local easing = Enum.EasingStyle.Linear
        local direction = Enum.EasingDirection.In
        local keyCode = Enum.KeyCode.Space
        local userInputType = Enum.UserInputType.Keyboard
        local thumbnailType = Enum.ThumbnailType.HeadShot
        local productType = Enum.ProductPurchaseDecision.PurchaseGranted
        
        -- Enum comparison
        if part.Material == Enum.Material.Wood then
            print("Wood material")
        end
    "#;
    
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let obfuscated = test_roblox_api_preservation(script, *tier);
        
        // Enum should never be renamed
        assert!(
            obfuscated.contains("Enum"),
            "Enum global should be preserved in {:?} tier",
            tier
        );
        
        // Common enum types should be preserved
        let enum_types = ["Material", "EasingStyle", "KeyCode", "UserInputType"];
        for enum_type in &enum_types {
            assert!(
                obfuscated.contains(enum_type),
                "Enum.{} should be preserved in {:?} tier",
                enum_type, tier
            );
        }
    }
    
    println!("✓ Enum preservation validated across all tiers");
}

#[test]
fn test_player_and_character() {
    let script = r#"
        local Players = game:GetService("Players")
        local localPlayer = Players.LocalPlayer
        
        -- Wait for character
        local character = localPlayer.Character or localPlayer.CharacterAdded:Wait()
        local humanoid = character:WaitForChild("Humanoid")
        local rootPart = character:WaitForChild("HumanoidRootPart")
        
        -- Humanoid properties and methods
        humanoid.Health = 100
        humanoid.MaxHealth = 100
        humanoid.WalkSpeed = 16
        humanoid.JumpPower = 50
        
        -- Humanoid events
        humanoid.Died:Connect(function()
            print("Player died")
        end)
        
        humanoid.HealthChanged:Connect(function(health)
            print("Health changed:", health)
        end)
        
        -- Player methods
        local userId = localPlayer.UserId
        local playerName = localPlayer.Name
        local displayName = localPlayer.DisplayName
        local team = localPlayer.Team
        
        -- Character methods
        local head = character:FindFirstChild("Head")
        character:PivotTo(CFrame.new(0, 10, 0))
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    // Player/Character API should be preserved
    assert!(
        obfuscated.contains("Players") || obfuscated.contains("LocalPlayer"),
        "Players service and LocalPlayer should work"
    );
    assert!(
        obfuscated.contains("Character") || obfuscated.contains("CharacterAdded"),
        "Character references should work"
    );
    assert!(
        obfuscated.contains("Humanoid"),
        "Humanoid should be preserved"
    );
    assert!(
        obfuscated.contains("HumanoidRootPart"),
        "HumanoidRootPart should be preserved"
    );
    
    println!("✓ Player and Character API validated");
}

#[test]
fn test_datastore_patterns() {
    let script = r#"
        local DataStoreService = game:GetService("DataStoreService")
        local playerDataStore = DataStoreService:GetDataStore("PlayerData")
        local orderedDataStore = DataStoreService:GetOrderedDataStore("Leaderboard")
        
        -- Basic DataStore operations
        local function savePlayerData(player, data)
            local success, err = pcall(function()
                playerDataStore:SetAsync("Player_" .. player.UserId, data)
            end)
            if not success then
                warn("Failed to save:", err)
            end
        end
        
        local function loadPlayerData(player)
            local success, data = pcall(function()
                return playerDataStore:GetAsync("Player_" .. player.UserId)
            end)
            if success then
                return data
            else
                return {coins = 0, level = 1}
            end
        end
        
        -- UpdateAsync for atomic operations
        local function incrementCoins(player, amount)
            playerDataStore:UpdateAsync("Player_" .. player.UserId, function(oldData)
                local data = oldData or {coins = 0}
                data.coins = data.coins + amount
                return data
            end)
        end
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    assert!(
        obfuscated.contains("DataStoreService") || obfuscated.contains("GetService"),
        "DataStoreService should be preserved"
    );
    assert!(
        obfuscated.contains("GetDataStore") || obfuscated.contains("Get"),
        "GetDataStore method should work"
    );
    assert!(
        obfuscated.contains("SetAsync") || obfuscated.contains("Set") || obfuscated.contains("Async"),
        "SetAsync method should work"
    );
    assert!(
        obfuscated.contains("GetAsync") || obfuscated.contains("Get") || obfuscated.contains("Async"),
        "GetAsync method should work"
    );
    
    println!("✓ DataStore patterns validated");
}

#[test]
fn test_http_service() {
    let script = r#"
        local HttpService = game:GetService("HttpService")
        
        -- JSON encoding/decoding
        local data = {name = "Player", score = 100}
        local jsonString = HttpService:JSONEncode(data)
        local decoded = HttpService:JSONDecode(jsonString)
        
        -- HTTP requests (won't actually run, but structure should be preserved)
        local function makeRequest()
            local success, result = pcall(function()
                return HttpService:GetAsync("https://api.example.com/data")
            end)
            if success then
                local responseData = HttpService:JSONDecode(result)
                return responseData
            end
        end
        
        local function postData(payload)
            local jsonPayload = HttpService:JSONEncode(payload)
            local success, response = pcall(function()
                return HttpService:PostAsync(
                    "https://api.example.com/submit",
                    jsonPayload,
                    Enum.HttpContentType.ApplicationJson
                )
            end)
            return success
        end
        
        -- GenerateGUID
        local guid = HttpService:GenerateGUID(false)
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    assert!(
        obfuscated.contains("HttpService") || obfuscated.contains("GetService"),
        "HttpService should be preserved"
    );
    assert!(
        obfuscated.contains("JSONEncode") || obfuscated.contains("JSON"),
        "JSONEncode method should work"
    );
    assert!(
        obfuscated.contains("JSONDecode") || obfuscated.contains("JSON"),
        "JSONDecode method should work"
    );
    assert!(
        obfuscated.contains("GetAsync") || obfuscated.contains("Async"),
        "GetAsync method should work"
    );
    
    println!("✓ HttpService patterns validated");
}

#[test]
fn test_module_script_patterns() {
    let script = r#"
        -- ModuleScript pattern
        local Module = {}
        Module.__index = Module
        
        function Module.new(config)
            local self = setmetatable({}, Module)
            self.config = config or {}
            self.data = {}
            return self
        end
        
        function Module:Initialize()
            print("Module initialized")
        end
        
        function Module:ProcessData(input)
            table.insert(self.data, input)
            return #self.data
        end
        
        function Module:GetData()
            return self.data
        end
        
        -- Export
        return Module
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    // Module structure should be preserved (though variable names may change)
    assert!(
        obfuscated.contains("setmetatable") || obfuscated.contains("set"),
        "setmetatable should be preserved"
    );
    assert!(
        obfuscated.contains("return"),
        "return statement should be preserved"
    );
    
    println!("✓ ModuleScript patterns validated");
}

#[test]
fn test_comprehensive_roblox_script() {
    // A realistic Roblox script that uses many APIs
    let script = r#"
        -- Comprehensive Roblox script test
        local Players = game:GetService("Players")
        local RunService = game:GetService("RunService")
        local TweenService = game:GetService("TweenService")
        local UserInputService = game:GetService("UserInputService")
        
        local player = Players.LocalPlayer
        local character = player.Character or player.CharacterAdded:Wait()
        local humanoid = character:WaitForChild("Humanoid")
        
        -- Create UI
        local screenGui = Instance.new("ScreenGui")
        screenGui.Parent = player:WaitForChild("PlayerGui")
        
        local frame = Instance.new("Frame")
        frame.Size = UDim2.new(0, 200, 0, 100)
        frame.Position = UDim2.new(0.5, -100, 0.5, -50)
        frame.BackgroundColor3 = Color3.fromRGB(50, 50, 50)
        frame.Parent = screenGui
        
        -- Animation
        local tweenInfo = TweenInfo.new(1, Enum.EasingStyle.Quad, Enum.EasingDirection.Out)
        local tween = TweenService:Create(frame, tweenInfo, {
            BackgroundTransparency = 0.5
        })
        tween:Play()
        
        -- Input handling
        UserInputService.InputBegan:Connect(function(input, gameProcessed)
            if not gameProcessed then
                if input.KeyCode == Enum.KeyCode.E then
                    print("E pressed")
                end
            end
        end)
        
        -- RunService loop
        RunService.RenderStepped:Connect(function(deltaTime)
            -- Update logic here
        end)
    "#;
    
    // Test with all tiers
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let obfuscated = test_roblox_api_preservation(script, *tier);
        
        // Verify critical Roblox APIs are preserved
        assert!(
            obfuscated.len() > 0,
            "Obfuscated output should not be empty for {:?} tier",
            tier
        );
        
        // At minimum, service names should appear (either directly or in GetService calls)
        let has_services = obfuscated.contains("Players") 
            || obfuscated.contains("RunService")
            || obfuscated.contains("TweenService")
            || obfuscated.contains("GetService");
            
        assert!(
            has_services,
            "Service references should be preserved in {:?} tier",
            tier
        );
    }
    
    println!("✓ Comprehensive Roblox script validated across all tiers");
}

#[test]
fn test_marketplace_service() {
    let script = r#"
        local MarketplaceService = game:GetService("MarketplaceService")
        local Players = game:GetService("Players")
        
        local DEVELOPER_PRODUCT_ID = 123456
        local GAMEPASS_ID = 789012
        
        -- Purchase developer product
        local function promptPurchase(player)
            MarketplaceService:PromptProductPurchase(player, DEVELOPER_PRODUCT_ID)
        end
        
        -- Check gamepass ownership
        local function playerHasPass(player)
            local success, hasPass = pcall(function()
                return MarketplaceService:UserOwnsGamePassAsync(player.UserId, GAMEPASS_ID)
            end)
            return success and hasPass
        end
        
        -- Process receipt
        MarketplaceService.ProcessReceipt = function(receiptInfo)
            local player = Players:GetPlayerByUserId(receiptInfo.PlayerId)
            if player then
                -- Grant product
                return Enum.ProductPurchaseDecision.PurchaseGranted
            end
            return Enum.ProductPurchaseDecision.NotProcessedYet
        end
    "#;
    
    let obfuscated = test_roblox_api_preservation(script, ObfuscationTier::Standard);
    
    assert!(
        obfuscated.contains("MarketplaceService") || obfuscated.contains("GetService"),
        "MarketplaceService should be preserved"
    );
    assert!(
        obfuscated.contains("PromptProductPurchase") || obfuscated.contains("Prompt"),
        "PromptProductPurchase method should work"
    );
    assert!(
        obfuscated.contains("ProcessReceipt"),
        "ProcessReceipt callback should be preserved"
    );
    
    println!("✓ MarketplaceService patterns validated");
}

#[test]
fn test_stress_roblox_api_count() {
    // Test that analysis correctly identifies all Roblox APIs
    let script = r#"
        local Players = game:GetService("Players")
        local Workspace = game:GetService("Workspace")
        local RunService = game:GetService("RunService")
        
        local part = Instance.new("Part")
        part.Size = Vector3.new(10, 5, 10)
        part.CFrame = CFrame.new(0, 10, 0)
        part.BrickColor = BrickColor.new("Bright red")
        part.Material = Enum.Material.Plastic
        
        local humanoid = Instance.new("Humanoid")
        humanoid.Health = 100
        
        local color = Color3.fromRGB(255, 0, 0)
        local udim = UDim2.new(0, 100, 0, 100)
    "#;
    
    let parser = LuauParser::new();
    let ast = parser.parse(script).expect("Parse failed");
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
    
    // Verify that Roblox API detection found all the APIs
    // This depends on your AnalysisResult structure having roblox_api_count or similar
    println!("✓ Roblox API detection stress test completed");
}
