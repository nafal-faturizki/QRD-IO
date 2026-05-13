use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn columnar_read_benchmark(c: &mut Criterion) {
    c.bench_function("read_empty_column", |b| {
        b.iter(|| {
            let _data = black_box(vec![0u8; 1024]);
        })
    });
}

criterion_group!(benches, columnar_read_benchmark);
criterion_main!(benches);
