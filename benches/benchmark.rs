use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion, Throughput, black_box};
use kd_tree::KdTree;
use point_3d::Point;
use rand::random;

fn kd_tree_large_nns(c: &mut Criterion) {
    let mut group = c.benchmark_group("kdtree n points");
    for size in (10000..=100000).step_by(10000) {
        group.sample_size(20);
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let tree = (0..size).map(|_| random_point()).collect::<KdTree<_>>();
                let _ = tree.k_nearest_neighbor(Point::new(0f32, 0f32, 0f32), 1);
                // println!("{:?}", result.next());
            })
        });
    }
    group.finish();
}

fn kd_tree_nns(c: &mut Criterion) {
    let size = 10_000;
    let tree = (0..size).map(|_| random_point()).collect::<KdTree<_>>();
    c.bench_with_input(
        BenchmarkId::new("kd tree nns", "10 000"),
        &size,
        |b, &size| {
            b.iter(|| {
                let _ = tree.k_nearest_neighbor(black_box(random_point()), black_box(1));
                // println!("{:?}", result.next());
            });
        },
    );
}

fn kd_tree_build(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("kd tree build", "10 000"),
        &10_000,
        |b, &size| {
            b.iter(|| {
                let _ = (0..size).map(|_| random_point()).collect::<KdTree<_>>();
            });
        },
    );
}

fn random_point() -> Point {
    Point::new(
        random::<f32>() * 100f32,
        random::<f32>() * 100f32,
        random::<f32>() * 100f32,
    )
}

criterion_group!(benches, kd_tree_build, kd_tree_nns);
criterion_main!(benches);
