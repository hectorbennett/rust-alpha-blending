use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_alpha_blending::v1;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("v1::blend_rgba", |b| {
        b.iter(|| v1::blend_rgba(black_box([255, 0, 0, 255]), black_box([0, 0, 255, 127])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
