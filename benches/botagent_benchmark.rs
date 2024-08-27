// benches/botagent_benchmark.rs

use botagent::is_bot;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_botagent(c: &mut Criterion) {
    let json_path = "src/patterns.json";

    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

    // NOTES: add more functions
    c.bench_function("is_bot", |b| {
        b.iter(|| {
            let result = is_bot(black_box(user_agent), black_box(json_path)).unwrap();
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark_botagent);
criterion_main!(benches);
