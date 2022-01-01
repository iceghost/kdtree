use criterion::*;
use kdtree::{kdtree::KdTree, float::Float, point::Point};
use rand::prelude::*;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn rfloat(rng: &mut ThreadRng) -> f32 {
    rng.gen_range(0.0..100.0)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_elem");
    for size in (1..50).step_by(10) {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut rng = thread_rng();
            b.iter(|| {
                let mut data = vec![];
                for _ in 0..size {
                    let x = rfloat(&mut rng);
                    let y = rfloat(&mut rng);
                    let z = rfloat(&mut rng);
                    data.push(Point::new(x, y, z));
                }
                let tree = data.into_iter().collect::<KdTree<_>>();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
