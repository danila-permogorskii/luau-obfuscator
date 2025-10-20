//! Performance benchmarks for obfuscation operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use luau_obfuscator::parser::LuauParser;
use std::fs;

fn benchmark_parsing(c: &mut Criterion) {
    let simple_script = fs::read_to_string("tests/fixtures/sample_scripts/simple.lua")
        .expect("Failed to read fixture");

    c.bench_function("parse_simple_script", |b| {
        let parser = LuauParser::new();
        b.iter(|| {
            parser.parse(black_box(&simple_script)).unwrap();
        });
    });

    let complex_script = fs::read_to_string("tests/fixtures/sample_scripts/complex.lua")
        .expect("Failed to read fixture");

    c.bench_function("parse_complex_script", |b| {
        let parser = LuauParser::new();
        b.iter(|| {
            parser.parse(black_box(&complex_script)).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_parsing);
criterion_main!(benches);
