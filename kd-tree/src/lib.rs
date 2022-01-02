//! This KdTree implementation use `str::collections::BTreeMap` from the standard library

use multi_dimension::distances::JMeasure;

// mod nearest_neighbor;
// mod bounds;
use multi_dimension::distances;
use multi_dimension::MultiDimension;

/// KdTree is essentially a binary tree with k-dimensions node
#[derive(Debug)]
pub struct KdTree<T>
where
    T: MultiDimension,
{
    root: Link<T>,
}

#[derive(Debug)]
struct Node<T>
where
    T: MultiDimension,
{
    value: T,
    left: Link<T>,
    right: Link<T>,
    depth: usize,
}

type Link<T> = Option<Box<Node<T>>>;

///
impl<T> Node<T>
where
    T: MultiDimension + JMeasure
{
    fn from_iter_help<I>(iter: I, depth: usize) -> Link<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut vec = iter.into_iter().collect::<Vec<_>>();
        if vec.len() == 0 {
            return None;
        }

        vec.sort_unstable_by(|a, b| JMeasure::j_compare(depth % T::DIM, a, b));

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
    T: JMeasure
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        KdTree {
            root: Node::from_iter_help(iter, 0),
        }
    }
}

// impl<T, const K: usize> KdTree<T, K>
// where
//     T: MultiDimension<K> + DistanceBetween,
// {
//     fn search_nearest_neighbor() {

//     }
// }