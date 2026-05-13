use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn encryption_benchmark(c: &mut Criterion) {
    c.bench_function("encrypt_1mb", |b| {
        b.iter(|| {
            let _data = black_box(vec![0u8; 1024 * 1024]);
        })
    });
}

criterion_group!(benches, encryption_benchmark);
criterion_main!(benches);
