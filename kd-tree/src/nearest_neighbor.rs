use std::cmp::Ordering;
use std::ops::RangeBounds;

use multi_dimension::distances::{dissimilarity_between, DissimilarityMeasure};
use multi_dimension::MultiDimension;

use crate::priority_queue::{DissimilarityQueue, self};

use super::kd_tree::{KdTree, Link};

use super::bounds::Bounds;

struct Searcher<'a, T>
where
    T: DissimilarityMeasure,
    T::Output: Ord,
{
    searchee: T,
    dissimilarity_queue: DissimilarityQueue<Neighbor<&'a T, T::Output>>,
    bounds: Bounds<'a, T>,
    at_best: bool,
}

impl<'a, T> Searcher<'a, T>
where
    T: MultiDimension + DissimilarityMeasure,
    T::Output: Default + std::ops::AddAssign + Ord,
{
    fn new(searchee: T, capacity: usize) -> Self {
        Self {
            searchee,
            dissimilarity_queue: DissimilarityQueue::with_capacity(capacity),
            bounds: Bounds::new(),
            at_best: false,
        }
    }

    fn search(&mut self, link: &'a Link<T>, j: usize) {
        let node = if let Some(node) = link {
            node
        } else {
            return ();
        };

        // terminal node
        if node.left.is_none() && node.right.is_none() {
            // update priority queue
            self.dissimilarity_queue.push(Neighbor(
                &***node,
                dissimilarity_between(&self.searchee, &node),
            ));

            if self.dissimilarity_queue.full()
                && self
                    .bounds
                    .contain_ball(&self.searchee, &self.dissimilarity_queue.peek().unwrap().1)
            {
                self.at_best = true;
                return;
            }
            return;
        }

        // search closer son
        if T::j_compare(j, &self.searchee, &node) != Ordering::Greater {
            let temp = self.bounds.upper[j];
            self.bounds.upper[j] = link;
            self.search(&node.left, (j + 1) % T::DIM);
            self.bounds.upper[j] = temp;
        } else {
            let temp = self.bounds.lower[j];
            self.bounds.lower[j] = link;
            self.search(&node.right, (j + 1) % T::DIM);
            self.bounds.lower[j] = temp;
        }

        if self.at_best {
            return;
        }

        // search farther son
        if T::j_compare(j, &self.searchee, &node) != Ordering::Greater {
            let temp = self.bounds.lower[j];
            self.bounds.lower[j] = link;
            if self.bounds.overlap_ball(&self.searchee, &self.dissimilarity_queue.peek().unwrap().1) {
                self.search(&node.right, (j + 1) % T::DIM);
            }
            self.bounds.lower[j] = temp;
        } else {
            let temp = self.bounds.upper[j];
            self.bounds.upper[j] = link;
            if self.bounds.overlap_ball(&self.searchee, &self.dissimilarity_queue.peek().unwrap().1) {
                self.search(&node.left, (j + 1) % T::DIM);
            }
            self.bounds.upper[j] = temp;
        }
        if self.bounds.contain_ball(&self.searchee, &self.dissimilarity_queue.peek().unwrap().1) {
            self.at_best = true;
        }
    }
}

struct Neighbor<T, O: Ord>(pub T, pub O);

impl<T, O: Ord> PartialEq for Neighbor<T, O> {
    fn eq(&self, other: &Self) -> bool {
        O::eq(&self.1, &other.1)
    }
}
impl<T, O: Ord> Eq for Neighbor<T, O> {}
impl<T, O: Ord> PartialOrd for Neighbor<T, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        O::partial_cmp(&self.1, &other.1)
    }
}
impl<T, O: Ord> Ord for Neighbor<T, O> {
    fn cmp(&self, other: &Self) -> Ordering {
        O::cmp(&self.1, &other.1)
    }
}
