use kd_tree::KdTree;
use multi_dimension::MultiDimension;

#[test]
fn test() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Scalar(isize);

    impl MultiDimension for Scalar {
        const DIM: usize = 1;

        fn j_compare(_: usize, this: &Self, that: &Self) -> std::cmp::Ordering {
            this.0.cmp(&that.0)
        }
    }

    let nodes = [0, 1, 2, 3, 4, 5];
    let tree = nodes.into_iter().map(Scalar).collect::<KdTree<Scalar>>();
    println!("{:?}", tree);
}

#[test]
fn two_dim() {
    let nodes = [(40, 45), (70, 10), (15, 70), (69, 50), (66, 85), (85, 90)] as [(isize, isize); 6];
    let tree = nodes.into_iter().collect::<KdTree<_>>();
    let it = tree.k_nearest_neighbor((40, 20), 2);
    let result = it.collect::<Vec<_>>();
    println!("{:?}", result);
}
