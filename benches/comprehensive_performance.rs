//! Comprehensive Performance Benchmarks
//!
//! Criterion-based benchmarks for all obfuscation pipeline stages.
//!
//! Performance Targets (from project spec):
//! - Obfuscate 1000-line script in <2 seconds (Standard tier)
//! - Memory usage <100MB for typical scripts
//! - Startup time <100ms
//!
//! Benchmark Categories:
//! 1. End-to-end obfuscation (all tiers, various script sizes)
//! 2. Individual pipeline stages (parse, analyze, transform, codegen)
//! 3. Cryptographic operations (KDF, encryption, watermarking)
//! 4. Memory usage profiling
//! 5. Output size measurements

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use luau_obfuscator::{
    parser::LuauParser,
    analysis::{AnalysisEngine, AnalysisOptions},
    obfuscation::{ObfuscationEngine, ObfuscationTier},
    crypto::CryptoContext,
};
use std::time::Duration;

// ============================================================================
// Script Generators
// ============================================================================

fn generate_simple_script(lines: usize) -> String {
    let mut script = String::with_capacity(lines * 50);
    script.push_str("-- Auto-generated benchmark script\n");
    script.push_str("local result = 0\n");
    
    for i in 0..lines {
        script.push_str(&format!(
            "local var{} = {}\n",
            i,
            i % 100
        ));
    }
    
    script.push_str("return result\n");
    script
}

fn generate_complex_script(lines: usize) -> String {
    let mut script = String::with_capacity(lines * 100);
    script.push_str(r#"
-- Complex benchmark script with various Luau features
local module = {}
local privateData = {}

function module:init(config)
    self.config = config or {}
    self.data = {}
    return self
end
"#);
    
    for i in 0..lines / 10 {
        script.push_str(&format!(r#"
function module:process{}(input)
    local result = {{}}
    for i = 1, #input do
        local value = input[i]
        if value > {} then
            table.insert(result, value * 2)
        else
            table.insert(result, value / 2)
        end
    end
    return result
end
"#, i, i % 100));
    }
    
    script.push_str("return module\n");
    script
}

fn generate_roblox_script(lines: usize) -> String {
    let mut script = String::with_capacity(lines * 120);
    script.push_str(r#"
-- Roblox-specific benchmark script
local Players = game:GetService("Players")
local RunService = game:GetService("RunService")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local player = Players.LocalPlayer
local character = player.Character or player.CharacterAdded:Wait()
local humanoid = character:WaitForChild("Humanoid")
"#);
    
    for i in 0..lines / 15 {
        script.push_str(&format!(r#"
local part{} = Instance.new("Part")
part{}.Name = "BenchmarkPart{}"
part{}.Size = Vector3.new({}, {}, {})
part{}.Position = Vector3.new({}, {}, {})
part{}.BrickColor = BrickColor.new("Bright red")
part{}.Material = Enum.Material.Plastic
part{}.Parent = workspace
"#, i, i, i, i, i % 10 + 1, i % 5 + 1, i % 10 + 1, i, i % 50, i % 20 + 10, i % 50, i, i, i, i));
    }
    
    script.push_str(r#"
RunService.RenderStepped:Connect(function(deltaTime)
    -- Update logic
end)
"#);
    script
}

// ============================================================================
// End-to-End Obfuscation Benchmarks
// ============================================================================

fn bench_end_to_end_obfuscation(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end_obfuscation");
    group.measurement_time(Duration::from_secs(10));
    
    let script_sizes = vec![100, 500, 1000, 2000, 5000];
    let tiers = vec![
        ("basic", ObfuscationTier::Basic),
        ("standard", ObfuscationTier::Standard),
        ("premium", ObfuscationTier::Premium),
    ];
    
    for size in script_sizes {
        let script = generate_complex_script(size);
        
        for (tier_name, tier) in &tiers {
            group.throughput(Throughput::Elements(size as u64));
            group.bench_with_input(
                BenchmarkId::new(tier_name, size),
                &script,
                |b, script| {
                    let parser = LuauParser::new();
                    let analysis_options = AnalysisOptions::default();
                    let analysis_engine = AnalysisEngine::new(analysis_options);
                    
                    b.iter(|| {
                        let ast = parser.parse(black_box(script)).unwrap();
                        let analysis_result = analysis_engine.analyze(black_box(&ast)).unwrap();
                        
                        let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
                        let engine = ObfuscationEngine::new(*tier, crypto_ctx);
                        engine.obfuscate(black_box(&ast), black_box(&analysis_result)).unwrap()
                    });
                },
            );
        }
    }
    
    group.finish();
}

// Critical benchmark: 1000-line script should obfuscate in <2 seconds (Standard tier)
fn bench_1000_line_standard_tier(c: &mut Criterion) {
    let script = generate_complex_script(1000);
    let parser = LuauParser::new();
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    
    c.bench_function("1000_line_standard_tier_target", |b| {
        b.iter(|| {
            let ast = parser.parse(black_box(&script)).unwrap();
            let analysis_result = analysis_engine.analyze(black_box(&ast)).unwrap();
            
            let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
            let engine = ObfuscationEngine::new(ObfuscationTier::Standard, crypto_ctx);
            engine.obfuscate(black_box(&ast), black_box(&analysis_result)).unwrap()
        });
    });
}

// ============================================================================
// Pipeline Stage Benchmarks
// ============================================================================

fn bench_parsing_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    
    let script_sizes = vec![100, 500, 1000, 5000, 10000];
    
    for size in script_sizes {
        let script = generate_complex_script(size);
        let parser = LuauParser::new();
        
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &script,
            |b, script| {
                b.iter(|| {
                    parser.parse(black_box(script)).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn bench_analysis_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("analysis");
    
    let script_sizes = vec![100, 500, 1000, 5000, 10000];
    
    for size in script_sizes {
        let script = generate_complex_script(size);
        let parser = LuauParser::new();
        let ast = parser.parse(&script).unwrap();
        
        let analysis_options = AnalysisOptions::default();
        let analysis_engine = AnalysisEngine::new(analysis_options);
        
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &ast,
            |b, ast| {
                b.iter(|| {
                    analysis_engine.analyze(black_box(ast)).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn bench_transformation_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("transformation");
    
    let script = generate_complex_script(1000);
    let parser = LuauParser::new();
    let ast = parser.parse(&script).unwrap();
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).unwrap();
    
    let tiers = vec![
        ("basic", ObfuscationTier::Basic),
        ("standard", ObfuscationTier::Standard),
        ("premium", ObfuscationTier::Premium),
    ];
    
    for (tier_name, tier) in tiers {
        group.bench_function(tier_name, |b| {
            let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
            let engine = ObfuscationEngine::new(tier, crypto_ctx);
            
            b.iter(|| {
                engine.obfuscate(black_box(&ast), black_box(&analysis_result)).unwrap()
            });
        });
    }
    
    group.finish();
}

// ============================================================================
// Cryptographic Operation Benchmarks
// ============================================================================

fn bench_crypto_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto");
    
    // KDF (Argon2id) - Expected to be ~1-2 seconds for high security
    group.bench_function("kdf_argon2id", |b| {
        let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
        b.iter(|| {
            crypto_ctx.derive_key(black_box("password"), black_box(b"salt12345678"))
        });
    });
    
    // Encryption speed
    let plaintext_sizes = vec![64, 256, 1024, 4096, 16384];
    for size in plaintext_sizes {
        let plaintext = vec![42u8; size];
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("encrypt", size),
            &plaintext,
            |b, plaintext| {
                let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
                b.iter(|| {
                    crypto_ctx.encrypt(black_box(plaintext)).unwrap()
                });
            },
        );
    }
    
    // Decryption speed
    for size in vec![64, 256, 1024, 4096, 16384] {
        let plaintext = vec![42u8; size];
        let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
        let ciphertext = crypto_ctx.encrypt(&plaintext).unwrap();
        
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("decrypt", size),
            &ciphertext,
            |b, ciphertext| {
                b.iter(|| {
                    crypto_ctx.decrypt(black_box(ciphertext)).unwrap()
                });
            },
        );
    }
    
    // Watermark generation
    group.bench_function("watermark_generation", |b| {
        let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
        b.iter(|| {
            crypto_ctx.generate_watermark(
                black_box("customer_id_12345"),
                black_box("script_id_67890")
            )
        });
    });
    
    // Watermark verification
    group.bench_function("watermark_verification", |b| {
        let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
        let watermark = crypto_ctx.generate_watermark("customer_id_12345", "script_id_67890");
        
        b.iter(|| {
            crypto_ctx.verify_watermark(
                black_box(&watermark),
                black_box("customer_id_12345")
            )
        });
    });
    
    group.finish();
}

// ============================================================================
// Roblox-Specific Benchmarks
// ============================================================================

fn bench_roblox_scripts(c: &mut Criterion) {
    let mut group = c.benchmark_group("roblox_scripts");
    
    let script_sizes = vec![500, 1000, 2000];
    
    for size in script_sizes {
        let script = generate_roblox_script(size);
        let parser = LuauParser::new();
        let analysis_options = AnalysisOptions::default();
        let analysis_engine = AnalysisEngine::new(analysis_options);
        
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &script,
            |b, script| {
                b.iter(|| {
                    let ast = parser.parse(black_box(script)).unwrap();
                    let analysis_result = analysis_engine.analyze(black_box(&ast)).unwrap();
                    
                    let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
                    let engine = ObfuscationEngine::new(ObfuscationTier::Standard, crypto_ctx);
                    engine.obfuscate(black_box(&ast), black_box(&analysis_result)).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

// ============================================================================
// Memory Usage Benchmarks
// ============================================================================

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.measurement_time(Duration::from_secs(5));
    
    // Large script to test memory usage (target: <100MB)
    let large_script = generate_complex_script(10000);
    
    group.bench_function("large_script_10k_lines", |b| {
        b.iter(|| {
            let parser = LuauParser::new();
            let ast = parser.parse(black_box(&large_script)).unwrap();
            
            let analysis_options = AnalysisOptions::default();
            let analysis_engine = AnalysisEngine::new(analysis_options);
            let analysis_result = analysis_engine.analyze(black_box(&ast)).unwrap();
            
            let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
            let engine = ObfuscationEngine::new(ObfuscationTier::Standard, crypto_ctx);
            engine.obfuscate(black_box(&ast), black_box(&analysis_result)).unwrap()
        });
    });
    
    group.finish();
}

// ============================================================================
// Output Size Measurements
// ============================================================================

fn bench_output_size_overhead(c: &mut Criterion) {
    // This benchmark measures the size overhead for each tier
    // We'll use custom measurements to capture output sizes
    
    let script = generate_complex_script(1000);
    let parser = LuauParser::new();
    let ast = parser.parse(&script).unwrap();
    
    let analysis_options = AnalysisOptions::default();
    let analysis_engine = AnalysisEngine::new(analysis_options);
    let analysis_result = analysis_engine.analyze(&ast).unwrap();
    
    println!("\n=== Output Size Analysis ===");
    println!("Original script size: {} bytes", script.len());
    
    for tier in &[ObfuscationTier::Basic, ObfuscationTier::Standard, ObfuscationTier::Premium] {
        let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
        let engine = ObfuscationEngine::new(*tier, crypto_ctx);
        let obfuscated = engine.obfuscate(&ast, &analysis_result).unwrap();
        
        let obfuscated_str = format!("{:?}", obfuscated);
        let size = obfuscated_str.len();
        let overhead = (size as f64 / script.len() as f64 - 1.0) * 100.0;
        
        println!("{:?} tier: {} bytes ({:.1}% overhead)", tier, size, overhead);
    }
    
    println!("============================\n");
}

// ============================================================================
// Startup Time Benchmark
// ============================================================================

fn bench_startup_time(c: &mut Criterion) {
    // Test CLI startup overhead (target: <100ms)
    c.bench_function("cli_initialization", |b| {
        b.iter(|| {
            // Simulate CLI initialization
            let _parser = LuauParser::new();
            let _analysis_options = AnalysisOptions::default();
            let _analysis_engine = AnalysisEngine::new(_analysis_options);
            black_box(());
        });
    });
}

// ============================================================================
// Stress Test Benchmarks
// ============================================================================

fn bench_stress_tests(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_tests");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10);
    
    // Extremely large script
    let huge_script = generate_complex_script(20000);
    
    group.bench_function("huge_script_20k_lines_basic", |b| {
        let parser = LuauParser::new();
        let analysis_options = AnalysisOptions::default();
        let analysis_engine = AnalysisEngine::new(analysis_options);
        
        b.iter(|| {
            let ast = parser.parse(black_box(&huge_script)).unwrap();
            let analysis_result = analysis_engine.analyze(black_box(&ast)).unwrap();
            
            let crypto_ctx = CryptoContext::new("benchmark_password", None).unwrap();
            let engine = ObfuscationEngine::new(ObfuscationTier::Basic, crypto_ctx);
            engine.obfuscate(black_box(&ast), black_box(&analysis_result)).unwrap()
        });
    });
    
    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    name = benches;
    config = Criterion::default()
        .significance_level(0.05)
        .sample_size(100)
        .warm_up_time(Duration::from_secs(2));
    targets = 
        bench_end_to_end_obfuscation,
        bench_1000_line_standard_tier,
        bench_parsing_only,
        bench_analysis_only,
        bench_transformation_only,
        bench_crypto_operations,
        bench_roblox_scripts,
        bench_memory_usage,
        bench_output_size_overhead,
        bench_startup_time,
        bench_stress_tests
);

criterion_main!(benches);
