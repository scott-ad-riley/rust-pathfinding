use criterion::{criterion_group, criterion_main, Criterion};
use pathfinding_core::{build_path, Coordinate};

pub fn criterion_benchmark(c: &mut Criterion) {
    let source = Coordinate { x: 15., y: 15. };
    let target = Coordinate { x: 22., y: 12. };
    let box_size = 10.;
    c.bench_function("build_path adjacent_boxes", |b| {
        b.iter(|| build_path(source, target, box_size, vec![]))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
