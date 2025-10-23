//! Performance benchmarks for obfuscation operations
//!
//! These benchmarks measure:
//! - End-to-end obfuscation time across all tiers
//! - Individual transformation performance
//! - Parsing performance for various script sizes
//! - Cryptographic operations performance
//! - Memory usage and allocation patterns

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use luau_obfuscator::{
    analysis::{AnalysisEngine, AnalysisOptions},
    crypto::{CryptoEngine, KdfParams},
    obfuscation::{ObfuscationEngine, ObfuscationTier},
    parser::LuauParser,
};
use std::time::Duration;

// Sample scripts of varying complexity
const SIMPLE_SCRIPT: &str = r#"
local message = "Hello, World!"
print(message)
"#;

const MEDIUM_SCRIPT: &str = r#"
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local function greetPlayer(player)
    local message = "Welcome, " .. player.Name .. "!"
    print(message)
    
    local data = {
        userId = player.UserId,
        name = player.Name,
        joinTime = tick()
    }
    
    return data
end

local function processData(data)
    for i = 1, 10 do
        local result = data.userId * i
        if result > 100 then
            print("High value detected: " .. result)
        end
    end
end

Players.PlayerAdded:Connect(function(player)
    local playerData = greetPlayer(player)
    processData(playerData)
end)
"#;

const LARGE_SCRIPT: &str = r#"
-- Complex admin command system
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local HttpService = game:GetService("HttpService")

local AdminModule = {}
AdminModule.Version = "2.0.1"
AdminModule.Commands = {}

-- Command registration
function AdminModule:RegisterCommand(name, aliases, callback, permission)
    self.Commands[name] = {
        name = name,
        aliases = aliases or {},
        callback = callback,
        permission = permission or 0
    }
end

-- User permission checking
function AdminModule:HasPermission(player, level)
    local userId = player.UserId
    local permissions = {
        [123456] = 3,  -- Owner
        [789012] = 2,  -- Admin
        [345678] = 1   -- Moderator
    }
    return (permissions[userId] or 0) >= level
end

-- Kick command
AdminModule:RegisterCommand("kick", {"boot"}, function(caller, target)
    if AdminModule:HasPermission(caller, 2) then
        if target then
            target:Kick("You have been kicked by " .. caller.Name)
            return true, "Player kicked successfully"
        end
    end
    return false, "Insufficient permissions or invalid target"
end, 2)

-- Ban command
AdminModule:RegisterCommand("ban", {"permaban"}, function(caller, target)
    if AdminModule:HasPermission(caller, 3) then
        if target then
            local banData = {
                userId = target.UserId,
                bannedBy = caller.UserId,
                timestamp = tick(),
                reason = "Banned by admin"
            }
            -- Store ban data
            target:Kick("You have been banned")
            return true, "Player banned successfully"
        end
    end
    return false, "Insufficient permissions or invalid target"
end, 3)

-- Teleport command
AdminModule:RegisterCommand("teleport", {"tp"}, function(caller, target, destination)
    if AdminModule:HasPermission(caller, 1) then
        if target and destination then
            if target.Character and destination.Character then
                target.Character:MoveTo(destination.Character.Position)
                return true, "Teleported successfully"
            end
        end
    end
    return false, "Invalid teleport parameters"
end, 1)

-- Execute command parser
function AdminModule:ParseCommand(player, message)
    local parts = message:split(" ")
    local commandName = parts[1]:lower():sub(2) -- Remove prefix
    
    for name, data in pairs(self.Commands) do
        if name == commandName or table.find(data.aliases, commandName) then
            local args = {table.unpack(parts, 2)}
            local success, result = data.callback(player, table.unpack(args))
            return success, result
        end
    end
    
    return false, "Unknown command"
end

-- Event handlers
Players.PlayerAdded:Connect(function(player)
    player.Chatted:Connect(function(message)
        if message:sub(1, 1) == ":" then
            local success, result = AdminModule:ParseCommand(player, message)
            if success then
                print("[Admin] " .. result)
            else
                warn("[Admin] Error: " .. result)
            end
        end
    end)
end)

return AdminModule
"#;

/// Generate a large script with many repetitive patterns
fn generate_stress_test_script(lines: usize) -> String {
    let mut script = String::new();
    script.push_str("-- Stress test script\n");
    script.push_str("local data = {}\n\n");
    
    for i in 0..lines {
        script.push_str(&format!(
            "local var{} = \"string value {}\"\n",
            i, i
        ));
        script.push_str(&format!(
            "local num{} = {}\n",
            i, i * 42
        ));
        script.push_str(&format!(
            "local func{} = function(x) return x * {} end\n",
            i, i
        ));
        
        if i % 10 == 0 {
            script.push_str(&format!("table.insert(data, {{var = var{}, num = num{}, func = func{}}})\n", i, i, i));
        }
    }
    
    script.push_str("\nreturn data\n");
    script
}

// Parsing benchmarks
fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    
    for (name, script) in &[
        ("simple", SIMPLE_SCRIPT),
        ("medium", MEDIUM_SCRIPT),
        ("large", LARGE_SCRIPT),
    ] {
        group.throughput(Throughput::Bytes(script.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), script, |b, script| {
            b.iter(|| {
                let parser = LuauParser::new();
                black_box(parser.parse(script).expect("Parse failed"))
            });
        });
    }
    
    group.finish();
}

// Analysis benchmarks
fn bench_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("analysis");
    
    for (name, script) in &[
        ("simple", SIMPLE_SCRIPT),
        ("medium", MEDIUM_SCRIPT),
        ("large", LARGE_SCRIPT),
    ] {
        group.throughput(Throughput::Bytes(script.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), script, |b, script| {
            let parser = LuauParser::new();
            let ast = parser.parse(script).expect("Parse failed");
            
            b.iter(|| {
                let options = AnalysisOptions::default();
                let engine = AnalysisEngine::new(options);
                black_box(engine.analyze(&ast).expect("Analysis failed"))
            });
        });
    }
    
    group.finish();
}

// Cryptographic operation benchmarks
fn bench_crypto(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto");
    group.measurement_time(Duration::from_secs(10)); // Longer measurement for crypto
    
    // Argon2id KDF benchmark
    group.bench_function("argon2id_kdf", |b| {
        let params = KdfParams {
            memory_cost: 262144, // 256 MiB
            time_cost: 4,
            parallelism: 2,
        };
        
        b.iter(|| {
            let engine = CryptoEngine::new(params);
            black_box(engine.derive_key("test_password", b"salt_bytes_12345"))
        });
    });
    
    // AES-256-GCM encryption benchmark
    group.bench_function("aes_gcm_encrypt_small", |b| {
        let params = KdfParams::default();
        let engine = CryptoEngine::new(params);
        let key = engine.derive_key("password", b"salt_bytes_12345").expect("KDF failed");
        let plaintext = b"Small test data for encryption";
        
        b.iter(|| {
            black_box(engine.encrypt(&key, plaintext).expect("Encrypt failed"))
        });
    });
    
    group.bench_function("aes_gcm_encrypt_large", |b| {
        let params = KdfParams::default();
        let engine = CryptoEngine::new(params);
        let key = engine.derive_key("password", b"salt_bytes_12345").expect("KDF failed");
        let plaintext = vec![42u8; 10_000]; // 10 KB
        
        b.iter(|| {
            black_box(engine.encrypt(&key, &plaintext).expect("Encrypt failed"))
        });
    });
    
    // AES-256-GCM decryption benchmark
    group.bench_function("aes_gcm_decrypt", |b| {
        let params = KdfParams::default();
        let engine = CryptoEngine::new(params);
        let key = engine.derive_key("password", b"salt_bytes_12345").expect("KDF failed");
        let plaintext = b"Test data for decryption benchmark";
        let ciphertext = engine.encrypt(&key, plaintext).expect("Encrypt failed");
        
        b.iter(|| {
            black_box(engine.decrypt(&key, &ciphertext).expect("Decrypt failed"))
        });
    });
    
    group.finish();
}

// Obfuscation tier benchmarks
fn bench_obfuscation_tiers(c: &mut Criterion) {
    let mut group = c.benchmark_group("obfuscation_tiers");
    group.measurement_time(Duration::from_secs(15));
    
    for (name, script) in &[
        ("simple", SIMPLE_SCRIPT),
        ("medium", MEDIUM_SCRIPT),
        ("large", LARGE_SCRIPT),
    ] {
        for tier in &[
            ObfuscationTier::Basic,
            ObfuscationTier::Standard,
            ObfuscationTier::Premium,
        ] {
            let bench_name = format!("{}_{:?}", name, tier);
            group.bench_with_input(
                BenchmarkId::from_parameter(bench_name),
                &(script, tier),
                |b, (script, tier)| {
                    let parser = LuauParser::new();
                    let ast = parser.parse(script).expect("Parse failed");
                    
                    let analysis_options = AnalysisOptions::default();
                    let analysis_engine = AnalysisEngine::new(analysis_options);
                    let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
                    
                    b.iter(|| {
                        let engine = ObfuscationEngine::new(**tier);
                        black_box(
                            engine
                                .obfuscate(&ast, &analysis_result)
                                .expect("Obfuscation failed"),
                        )
                    });
                },
            );
        }
    }
    
    group.finish();
}

// End-to-end obfuscation pipeline benchmark
fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10); // Fewer samples for long-running benchmarks
    
    for (name, script) in &[
        ("simple", SIMPLE_SCRIPT),
        ("medium", MEDIUM_SCRIPT),
        ("large", LARGE_SCRIPT),
    ] {
        for tier in &[
            ObfuscationTier::Basic,
            ObfuscationTier::Standard,
            ObfuscationTier::Premium,
        ] {
            let bench_name = format!("{}_{:?}", name, tier);
            group.throughput(Throughput::Bytes(script.len() as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(bench_name),
                &(script, tier),
                |b, (script, tier)| {
                    b.iter(|| {
                        // Complete pipeline: Parse → Analyze → Obfuscate → Generate
                        let parser = LuauParser::new();
                        let ast = parser.parse(script).expect("Parse failed");
                        
                        let analysis_options = AnalysisOptions::default();
                        let analysis_engine = AnalysisEngine::new(analysis_options);
                        let analysis_result =
                            analysis_engine.analyze(&ast).expect("Analysis failed");
                        
                        let obfuscation_engine = ObfuscationEngine::new(**tier);
                        let obfuscated_ast = obfuscation_engine
                            .obfuscate(&ast, &analysis_result)
                            .expect("Obfuscation failed");
                        
                        // Code generation would go here in the real pipeline
                        black_box(obfuscated_ast)
                    });
                },
            );
        }
    }
    
    group.finish();
}

// Large script stress test benchmarks
fn bench_stress_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_test");
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);
    
    for lines in &[1000, 5000, 10000] {
        let script = generate_stress_test_script(*lines);
        let bench_name = format!("{}_lines", lines);
        
        group.throughput(Throughput::Bytes(script.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(bench_name),
            &script,
            |b, script| {
                b.iter(|| {
                    let parser = LuauParser::new();
                    let ast = parser.parse(script).expect("Parse failed");
                    
                    let analysis_options = AnalysisOptions::default();
                    let analysis_engine = AnalysisEngine::new(analysis_options);
                    let analysis_result =
                        analysis_engine.analyze(&ast).expect("Analysis failed");
                    
                    let obfuscation_engine = ObfuscationEngine::new(ObfuscationTier::Standard);
                    black_box(
                        obfuscation_engine
                            .obfuscate(&ast, &analysis_result)
                            .expect("Obfuscation failed"),
                    )
                });
            },
        );
    }
    
    group.finish();
}

// Memory allocation benchmarks
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    // Benchmark memory efficiency of parsing
    group.bench_function("parse_large_no_alloc", |b| {
        let parser = LuauParser::new();
        b.iter(|| {
            black_box(parser.parse(LARGE_SCRIPT).expect("Parse failed"));
        });
    });
    
    // Benchmark memory reuse in iterative processing
    group.bench_function("repeated_obfuscation_reuse", |b| {
        let parser = LuauParser::new();
        let ast = parser.parse(MEDIUM_SCRIPT).expect("Parse failed");
        
        let analysis_options = AnalysisOptions::default();
        let analysis_engine = AnalysisEngine::new(analysis_options);
        let analysis_result = analysis_engine.analyze(&ast).expect("Analysis failed");
        
        let obfuscation_engine = ObfuscationEngine::new(ObfuscationTier::Basic);
        
        b.iter(|| {
            // Simulate processing multiple scripts with same engine (memory reuse)
            for _ in 0..10 {
                black_box(
                    obfuscation_engine
                        .obfuscate(&ast, &analysis_result)
                        .expect("Obfuscation failed"),
                );
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parsing,
    bench_analysis,
    bench_crypto,
    bench_obfuscation_tiers,
    bench_end_to_end,
    bench_stress_test,
    bench_memory_usage
);

criterion_main!(benches);
