use std::cmp::Ordering;

use multi_dimension::distances::{dissimilarity_between, DissimilarityMeasure};
use multi_dimension::MultiDimension;

use crate::priority_queue::DissimilarityQueue;

use super::kd_tree::Link;

use super::bounds::Bounds;

pub struct Searcher<'a, T>
where
    T: DissimilarityMeasure,
    T::Output: PartialOrd,
{
    searchee: T,
    dissimilarity_queue: DissimilarityQueue<Neighbor<&'a T, T::Output>>,
    bounds: Bounds<'a, T>,
    at_best: bool,
    pub visited_count: usize,
}

impl<'a, T> Searcher<'a, T>
where
    T: MultiDimension + DissimilarityMeasure,
    T::Output: Default + std::ops::AddAssign + PartialOrd,
{
    pub fn new(searchee: T, capacity: usize) -> Self {
        Self {
            searchee,
            dissimilarity_queue: DissimilarityQueue::with_capacity(capacity),
            bounds: Bounds::new(),
            at_best: false,
            visited_count: 0
        }
    }

    pub fn search(&mut self, link: &'a Link<T>, j: usize) {
        let node = if let Some(node) = link {
            node
        } else {
            return ();
        };

        self.visited_count += 1;

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
            if self.dissimilarity_queue.empty()
                || self.bounds.overlap_ball(
                    &self.searchee,
                    &self
                        .dissimilarity_queue
                        .peek()
                        .expect("can't pop empty queue")
                        .1,
                )
            {
                self.search(&node.right, (j + 1) % T::DIM);
            }
            self.bounds.lower[j] = temp;
        } else {
            let temp = self.bounds.upper[j];
            self.bounds.upper[j] = link;
            if self.dissimilarity_queue.empty()
                || self.bounds.overlap_ball(
                    &self.searchee,
                    &self
                        .dissimilarity_queue
                        .peek()
                        .expect("can't pop empty queue")
                        .1,
                )
            {
                self.search(&node.left, (j + 1) % T::DIM);
            }
            self.bounds.upper[j] = temp;
        }

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
        }
    }

    pub fn finalize(self) -> impl Iterator<Item = Neighbor<&'a T, T::Output>> {
        self.dissimilarity_queue.into_iter()
    }
}

#[derive(Debug)]
pub struct Neighbor<T, O: PartialOrd>(pub T, pub O);

impl<T, O: PartialOrd> PartialEq for Neighbor<T, O> {
    fn eq(&self, other: &Self) -> bool {
        O::eq(&self.1, &other.1)
    }
}
impl<T, O: PartialOrd> PartialOrd for Neighbor<T, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        O::partial_cmp(&self.1, &other.1)
    }
}

impl<T, O: PartialOrd> From<(T, O)> for Neighbor<T, O> {
    fn from(x: (T, O)) -> Self {
        Neighbor(x.0, x.1)
    }
}
