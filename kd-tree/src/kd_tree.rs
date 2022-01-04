use std::ops::AddAssign;

use crate::nearest_neighbor::{Neighbor, Searcher};

// mod nearest_neighbor;
// mod bounds;
use multi_dimension::{distances::DissimilarityMeasure, MultiDimension};

/// KdTree is essentially a binary tree with k-dimensions node
#[derive(Debug)]
pub struct KdTree<T> {
    root: Link<T>,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    pub left: Link<T>,
    pub right: Link<T>,
    depth: usize,
}

pub type Link<T> = Option<Box<Node<T>>>;

///
impl<T> Node<T>
where
    T: MultiDimension,
{
    fn from_iter_help<I>(iter: I, depth: usize) -> Link<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut vec = iter.into_iter().collect::<Vec<_>>();
        if vec.len() == 0 {
            return None;
        }

        vec.sort_unstable_by(|a, b| T::j_compare(depth % T::DIM, a, b));

        // if length == 0, there shouldn't be any median:
        let mid = (vec.len() - 1) / 2;
        let mut it = vec.into_iter();

        let left = if mid != 0 {
            Node::from_iter_help(it.by_ref().take(mid), depth + 1)
        } else {
            None
        };

        let value = it.next()?;

        let right = Node::from_iter_help(it, depth + 1);

        Some(Box::new(Self {
            value,
            left,
            right,
            depth,
        }))
    }
}

///
impl<T> FromIterator<T> for KdTree<T>
where
    T: MultiDimension,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        KdTree {
            root: Node::from_iter_help(iter, 0),
        }
    }
}

impl<T> KdTree<T>
where
    T: MultiDimension + DissimilarityMeasure,
    T::Output: PartialOrd + AddAssign + Default,
{
    pub fn k_nearest_neighbor(
        &self,
        searchee: T,
        k: usize,
    ) -> impl Iterator<Item = Neighbor<&T, T::Output>> {
        let mut searcher = Searcher::new(searchee, k);
        searcher.search(&self.root, 0);
        // println!("{:?}", searcher.visited_count);
        searcher.finalize()
    }
}

impl<T> std::ops::Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
