use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn columnar_write_benchmark(c: &mut Criterion) {
    c.bench_function("write_empty_rowgroup", |b| {
        b.iter(|| {
            let _data = black_box(vec![0u8; 1024]);
        })
    });
}

criterion_group!(benches, columnar_write_benchmark);
criterion_main!(benches);
