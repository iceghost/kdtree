use multi_dimension::{
    distances::{dissimilarity_between, DissimilarityMeasure},
    MultiDimension,
};

/// This is implemented as a custom max heap
pub struct DissimilarityQueue<T> {
    capacity: usize,
    data: Vec<T>,
    searchee: T,
}

impl<T, O> DissimilarityQueue<T>
where
    T: MultiDimension + DissimilarityMeasure<Output = O> + Eq,
    O: Default + std::ops::AddAssign + Ord,
{
    fn dissimilarity(&self, element: &T) -> T::Output {
        dissimilarity_between(element, &self.searchee)
    }

    pub fn with_capacity(capacity: usize, searchee: T) -> Self {
        Self {
            capacity,
            searchee,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, element: T) {
        if self.data.len() < self.capacity {
            self.data.push(element);
            self.sift_up();
        }
        // if it is better than the worst
        else if self.dissimilarity(&element) < self.dissimilarity(&self.data[0]) {
            self.pop();
            self.data.push(element);
            self.sift_up();
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    fn pop(&mut self) {
        if self.data.len() == 0 {
            return;
        }
        self.data.swap_remove(0);
        self.sift_down();
    }

    fn sift_up(&mut self) {
        if self.data.len() == 0 {
            return;
        }
        let mut index = self.data.len() - 1;
        while index != 0 {
            let daddy = (index - 1) / 2;
            if self.dissimilarity(&self.data[index]) > self.dissimilarity(&self.data[daddy]) {
                self.data.swap(index, daddy);
                index = daddy;
            } else {
                return;
            }
        }
    }

    fn sift_down(&mut self) {
        if self.data.len() == 0 {
            return;
        }
        let mut index = 0usize;
        loop {
            let child1 = 2 * index + 1;
            let child2 = 2 * index + 2;
            if child1 >= self.data.len() {
                return;
            }
            let child = if child2 >= self.data.len() {
                child1
            } else {
                if self.dissimilarity(&self.data[child1]) > self.dissimilarity(&self.data[child2]) {
                    child1
                } else {
                    child2
                }
            };
            if self.dissimilarity(&self.data[index]) < self.dissimilarity(&self.data[child]) {
                self.data.swap(index, child);
                index = child;
            } else {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut queue = DissimilarityQueue::<isize>::with_capacity(3, 0);
        queue.push(5);
        queue.push(3);
        queue.push(1);
        assert!(queue.contains(&5));
        queue.push(-10);
        assert!(!queue.contains(&-10));
        assert!(queue.contains(&1));
        queue.push(2);
        assert!(!queue.contains(&5));
        queue.push(4);
        assert!(!queue.contains(&4));
    }
}
