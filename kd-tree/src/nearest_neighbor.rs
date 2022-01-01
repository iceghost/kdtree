use super::{KdTree, MultiDimension, Link};

use super::bounds;

struct Searcher<T, D, const K: usize>
where
    T: MultiDimension<K, Dimension = D>,
    D: Ord,
{
    searchee: T,
    data: KdTree<T, K>,
    upper_bound: T,
    lower_bound: T,
}

impl<T, D, const K: usize> Searcher<T, D, K>
where
    T: MultiDimension<K, Dimension = D>,
    D: Ord,
{
    fn search<'a>(&mut self, link: &'a Link<T, K>)
    {
        let node = link.as_ref()?;

        if node.partition_dim() < self.searchee.nth_dim(node.depth % K) {
            self.search(&node.left);
        } else {
            self.search(&node.right);
        }
        None
    }

}