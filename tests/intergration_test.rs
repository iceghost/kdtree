use kdtree::kdtree::KdTree;
use kdtree::multidimension::distances::JMeasure;
use kdtree::multidimension::MultiDimension;

#[test]
fn test() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Scalar(isize);

    impl MultiDimension for Scalar {
        const DIM: usize = 1;
    }

    impl JMeasure for Scalar {
        type Distance = isize;
        fn j_diff(_: usize, this: &Self, that: &Self) -> Self::Distance {
            this.0 - that.0
        }
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
    #[derive(Debug)]
    struct Point(isize, isize);

    impl MultiDimension for Point {
        const DIM: usize = 2;
    }
    impl JMeasure for Point {
        type Distance = isize;
        fn j_diff(j: usize, this: &Self, that: &Self) -> Self::Distance {
            if j % 2 == 0 {
                this.0 - that.0
            } else {
                this.1 - that.1
            }
        }
        fn j_compare(j: usize, this: &Self, that: &Self) -> std::cmp::Ordering {
            if j % 2 == 0 {
                this.0.cmp(&that.0)
            } else {
                this.1.cmp(&that.1)
            }
        }
    }

    let nodes = [(40, 45), (70, 10), (15, 70), (69, 50), (66, 85), (85, 90)] as [(isize, isize); 6];
    let tree = nodes
        .into_iter()
        .map(|(a, b)| Point(a, b))
        .collect::<KdTree<_>>();
    println!("{:?}", tree);
}
