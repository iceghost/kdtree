use std::cmp::Ordering;

use multi_dimension::MultiDimension;

use super::kd_tree::{KdTree, Link};

use super::bounds::Bounds;

struct Searcher<'a, T> {
    searchee: T,
    bounds: Bounds<'a, T>
}

impl<'a, T> Searcher<'a, T>
where
    T: MultiDimension,
{
    // fn search(&mut self, link: &'a Link<T>) -> Option<&'a Link<T>>
    // {
    //     let node = link.as_ref()?;

    //     if node.left.is_none() && node.right.is_none() {
    //         // update priority queue
    //         if bounds::contain_ball(&self, center, radius)
    //     }

    //     if T::j_compare(j, searchee, node) != Ordering::Greater {

    //     }
    // }

}