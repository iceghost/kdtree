use kd_tree::KdTree;

#[test]
fn test() {
    let nodes = [0isize, 1, 2, 3, 4, 5];
    let tree = nodes.into_iter().collect::<KdTree<_>>();
    let it = tree.k_nearest_neighbor(6, 3);
    // println!("{:?}", it.collect::<Vec<_>>());
    let mut vec = it.map(|tup| (tup.0, tup.1)).collect::<Vec<_>>();
    vec.sort();
    assert_eq!(vec, vec![(&3, 3), (&4, 2), (&5, 1)]);
}

#[test]
fn two_dim() {
    let nodes = [(40, 45), (70, 10), (15, 70), (69, 50), (66, 85), (85, 90)] as [(isize, isize); 6];
    let tree = nodes.into_iter().collect::<KdTree<_>>();
    let it = tree.k_nearest_neighbor((40, 20), 3);
    let result = it.map(|t| t.0).collect::<Vec<_>>();
    // println!("{:?}", result);
    assert!(result.contains(&&(40, 45)));
    assert!(result.contains(&&(69, 50)));
}
