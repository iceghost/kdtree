use kd_tree::KdTree;
use lsh::EuclidianLSHSearcher;
use point_3d::Point;
use rand::random;

pub fn random_point() -> Point {
    Point::new(
        random::<f32>() * 100f32,
        random::<f32>() * 100f32,
        random::<f32>() * 100f32,
    )
}

#[test]
pub fn test() {
    let size = 10_000;
    let mut agree = 0;
    for _ in 0..100 {
        let points = (0..size).map(|_| random_point()).collect::<Vec<_>>();
        let points_clone = points.clone();
        let seachee = random_point();
        let seachee_clone = seachee.clone();

        let seacher = points_clone.iter().collect::<EuclidianLSHSearcher>();
        let mut result2 = seacher.search(&seachee);

        let tree = points.into_iter().collect::<KdTree<_>>();
        let mut result1 = tree.k_nearest_neighbor(seachee_clone, 1);
        if result1.next() == result2.next() {
            agree += 1;
        }
    }
    println!("Agree: {}/100", agree);
    assert!(agree > 90);
}
