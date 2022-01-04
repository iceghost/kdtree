use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use kd_tree::KdTree;
use lsh::EuclidianLSHSearcher;
use point_3d::Point;
use rand::random;

fn full(c: &mut Criterion) {
    let mut group = c.benchmark_group("full n points");
    for size in (100000..=1000000).step_by(400000) {
        group.bench_with_input(BenchmarkId::new("tree build", size), &size, |b, &size| {
            b.iter(|| {
                let tree = (0..size).map(|_| random_point()).collect::<KdTree<_>>();
                let mut it = tree.k_nearest_neighbor(black_box(random_point()), black_box(1));
                it.next();
                // println!("{:?}", result.next());
            })
        });
        group.bench_with_input(
            BenchmarkId::new("hash build", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let points = (0..size).map(|_| random_point()).collect::<Vec<_>>();
                    let searcher = points.iter().collect::<EuclidianLSHSearcher>();
                    let searchee = random_point();
                    let mut it = searcher.search(&searchee);
                    it.next();
                });
            },
        );
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

fn hash_build(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("hash build", "10 000"),
        &10_000,
        |b, &size| {
            let points = (0..10_000).map(|_| random_point()).collect::<Vec<_>>();
            b.iter(|| {
                let _ = points.iter().collect::<EuclidianLSHSearcher>();
            });
        },
    );
}

pub fn random_point() -> Point {
    Point::new(
        random::<f32>() * 100f32,
        random::<f32>() * 100f32,
        random::<f32>() * 100f32,
    )
}

fn hash_nns(c: &mut Criterion) {
    let size = 10_000;
    let points = (0..size).map(|_| random_point()).collect::<Vec<_>>();
    let seacher = points.iter().collect::<EuclidianLSHSearcher>();
    c.bench_with_input(BenchmarkId::new("lsh", size), &size, |b, i| {
        b.iter(|| {
            let _ = seacher.search(black_box(&random_point()));
        })
    });
}

// criterion_group!(benches, kd_tree_nns, hash_nns, kd_tree_build, hash_build);
criterion_group!(benches, full);
criterion_main!(benches);
