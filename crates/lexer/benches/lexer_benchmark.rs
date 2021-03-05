use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexer::tokenize;

use std::fs::read_to_string;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tokenize", |b| {
        let data = read_to_string("../../crates/lexer/benches/data/math_data.txt")
            .expect("Failed to load benchmark data.");
        b.iter(|| tokenize(black_box(&data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
