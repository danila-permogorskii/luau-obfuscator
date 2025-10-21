//! Performance benchmarks for critical operations

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use luau_obfuscator::{
    analysis::Analyzer,
    cli::args::ObfuscationTier,
    crypto::CryptoContext,
    obfuscation::Obfuscator,
    parser::LuauParser,
};
use std::fs;

/// Benchmark Luau parsing performance
fn benchmark_parsing(c: &mut Criterion) {
    let simple_script = fs::read_to_string("tests/fixtures/sample_scripts/simple.lua")
        .expect("Failed to read simple fixture");

    let complex_script = fs::read_to_string("tests/fixtures/sample_scripts/complex.lua")
        .expect("Failed to read complex fixture");

    let roblox_script = fs::read_to_string("tests/fixtures/sample_scripts/roblox_api.lua")
        .expect("Failed to read roblox fixture");

    let mut group = c.benchmark_group("parsing");

    group.bench_function("parse_simple_script", |b| {
        let parser = LuauParser::new();
        b.iter(|| {
            parser.parse(black_box(&simple_script)).unwrap();
        });
    });

    group.bench_function("parse_complex_script", |b| {
        let parser = LuauParser::new();
        b.iter(|| {
            parser.parse(black_box(&complex_script)).unwrap();
        });
    });

    group.bench_function("parse_roblox_api_script", |b| {
        let parser = LuauParser::new();
        b.iter(|| {
            parser.parse(black_box(&roblox_script)).unwrap();
        });
    });

    group.finish();
}

/// Benchmark analysis engine performance
fn benchmark_analysis(c: &mut Criterion) {
    let complex_script = fs::read_to_string("tests/fixtures/sample_scripts/complex.lua")
        .expect("Failed to read complex fixture");

    let parser = LuauParser::new();
    let parse_result = parser.parse(&complex_script).unwrap();

    let mut group = c.benchmark_group("analysis");

    group.bench_function("analyze_complex_script", |b| {
        let analyzer = Analyzer::new();
        b.iter(|| {
            analyzer.analyze(black_box(&parse_result)).unwrap();
        });
    });

    group.finish();
}

/// Benchmark cryptography operations
fn benchmark_crypto(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto");

    // Key derivation (Argon2id)
    group.bench_function("argon2id_key_derivation", |b| {
        use luau_obfuscator::crypto::KeyDerivation;
        let kdf = KeyDerivation::new();
        let password = b"benchmark_password";
        let salt = b"0123456789abcdef";

        b.iter(|| {
            kdf.derive_key(black_box(password), black_box(salt))
                .unwrap();
        });
    });

    // AES-256-GCM encryption
    group.bench_function("aes_256_gcm_encrypt", |b| {
        let crypto_ctx = CryptoContext::new("benchmark", None).unwrap();
        let plaintext = b"Hello, Roblox! This is a test string for encryption benchmarking.";

        b.iter(|| {
            crypto_ctx.encrypt(black_box(plaintext)).unwrap();
        });
    });

    // AES-256-GCM decryption
    group.bench_function("aes_256_gcm_decrypt", |b| {
        let crypto_ctx = CryptoContext::new("benchmark", None).unwrap();
        let plaintext = b"Hello, Roblox! This is a test string for encryption benchmarking.";
        let encrypted = crypto_ctx.encrypt(plaintext).unwrap();

        b.iter(|| {
            crypto_ctx.decrypt(black_box(&encrypted)).unwrap();
        });
    });

    // Watermark generation
    group.bench_function("watermark_generation", |b| {
        let crypto_ctx = CryptoContext::new("benchmark", None).unwrap();

        b.iter(|| {
            crypto_ctx.generate_watermark(
                black_box("customer_12345"),
                black_box("script_abc"),
            );
        });
    });

    group.finish();
}

/// Benchmark obfuscation tiers
fn benchmark_obfuscation_tiers(c: &mut Criterion) {
    let complex_script = fs::read_to_string("tests/fixtures/sample_scripts/complex.lua")
        .expect("Failed to read complex fixture");

    let parser = LuauParser::new();
    let parse_result = parser.parse(&complex_script).unwrap();

    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();

    let mut group = c.benchmark_group("obfuscation_tiers");

    // Basic tier
    group.bench_function("obfuscate_basic_tier", |b| {
        let crypto_ctx = CryptoContext::new("benchmark", None).unwrap();
        let obfuscator = Obfuscator::new(ObfuscationTier::Basic, crypto_ctx);

        b.iter(|| {
            obfuscator
                .obfuscate(black_box(&parse_result), black_box(&analysis))
                .unwrap();
        });
    });

    // Standard tier
    group.bench_function("obfuscate_standard_tier", |b| {
        let crypto_ctx = CryptoContext::new("benchmark", None).unwrap();
        let obfuscator = Obfuscator::new(ObfuscationTier::Standard, crypto_ctx);

        b.iter(|| {
            obfuscator
                .obfuscate(black_box(&parse_result), black_box(&analysis))
                .unwrap();
        });
    });

    // Premium tier
    group.bench_function("obfuscate_premium_tier", |b| {
        let crypto_ctx = CryptoContext::new("benchmark", None).unwrap();
        let obfuscator = Obfuscator::new(ObfuscationTier::Premium, crypto_ctx);

        b.iter(|| {
            obfuscator
                .obfuscate(black_box(&parse_result), black_box(&analysis))
                .unwrap();
        });
    });

    group.finish();
}

/// Benchmark string encryption at scale
fn benchmark_string_encryption_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_encryption_scale");

    for string_count in [10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*string_count as u64));

        group.bench_with_input(
            format!("{}_strings", string_count),
            string_count,
            |b, &count| {
                let crypto_ctx = CryptoContext::new("benchmark", None).unwrap();

                // Generate test strings
                let strings: Vec<String> = (0..count)
                    .map(|i| format!("Test string number {}", i))
                    .collect();

                b.iter_batched(
                    || strings.clone(),
                    |strings| {
                        for s in strings {
                            crypto_ctx.encrypt(s.as_bytes()).unwrap();
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark large script handling
fn benchmark_large_scripts(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_scripts");

    // Generate large scripts of different sizes
    let sizes = [1000, 5000, 10000, 20000];

    for &size in sizes.iter() {
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(format!("{}_lines", size), &size, |b, &size| {
            let large_script = format!(
                "-- Large script test\n{}",
                "local x = 1\nprint(x)\n".repeat(size as usize)
            );

            let parser = LuauParser::new();

            b.iter(|| {
                parser.parse(black_box(&large_script)).unwrap();
            });
        });
    }

    group.finish();
}

/// Benchmark name mangling performance
fn benchmark_name_mangling(c: &mut Criterion) {
    let complex_script = fs::read_to_string("tests/fixtures/sample_scripts/complex.lua")
        .expect("Failed to read complex fixture");

    let parser = LuauParser::new();
    let parse_result = parser.parse(&complex_script).unwrap();

    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&parse_result).unwrap();

    let mut group = c.benchmark_group("name_mangling");

    group.bench_function("mangle_identifiers", |b| {
        use luau_obfuscator::obfuscation::NameMangler;
        
        b.iter(|| {
            let mut mangler = NameMangler::new(&analysis.preserved_identifiers, true);
            mangler.generate_mappings(black_box(&analysis)).unwrap();
        });
    });

    group.finish();
}

/// Benchmark dead code injection
fn benchmark_dead_code_injection(c: &mut Criterion) {
    let complex_script = fs::read_to_string("tests/fixtures/sample_scripts/complex.lua")
        .expect("Failed to read complex fixture");

    let parser = LuauParser::new();
    let parse_result = parser.parse(&complex_script).unwrap();

    let mut group = c.benchmark_group("dead_code_injection");

    for &density in [0.1, 0.3, 0.5, 1.0].iter() {
        group.bench_with_input(format!("density_{}", density), &density, |b, &density| {
            use luau_obfuscator::obfuscation::DeadCodeInjector;
            let injector = DeadCodeInjector::new(density);

            b.iter(|| {
                injector.generate(black_box(&parse_result)).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(10));
    targets = 
        benchmark_parsing,
        benchmark_analysis,
        benchmark_crypto,
        benchmark_obfuscation_tiers,
        benchmark_string_encryption_scale,
        benchmark_large_scripts,
        benchmark_name_mangling,
        benchmark_dead_code_injection
}

criterion_main!(benches);
