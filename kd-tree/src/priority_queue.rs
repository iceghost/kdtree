use multi_dimension::{
    distances::{dissimilarity_between, DissimilarityMeasure},
    MultiDimension,
};

/// This is implemented as a custom max heap
pub struct DissimilarityQueue<T> {
    capacity: usize,
    data: Vec<T>,
}

impl<T> DissimilarityQueue<T>
where
    T: Ord,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn push(&mut self, element: T) -> bool {
        if self.data.len() < self.capacity {
            self.data.push(element);
            self.sift_up();
            true
        }
        // if it is better than the worst
        else if element < self.data[0] {
            self.pop();
            self.data.push(element);
            self.sift_up();
            true
        } else {
            false
        }
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
            if self.data[index] > self.data[daddy] {
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
                if self.data[child1] > self.data[child2] {
                    child1
                } else {
                    child2
                }
            };
            if self.data[index] < self.data[child] {
                self.data.swap(index, child);
                index = child;
            } else {
                return;
            }
        }
    }

    pub fn full(&self) -> bool {
        self.capacity == self.data.len()
    }
}

impl<T> DissimilarityQueue<T>
where
    T: Eq,
{
    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[derive(PartialEq, Eq, Ord)]
        struct Modulus(isize);
        impl Into<Modulus> for isize {
            fn into(self) -> Modulus {
                Modulus(self)
            }
        }
        impl PartialOrd for Modulus {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                isize::partial_cmp(&self.0.abs(), &other.0.abs())
            }
        }
        let mut queue = DissimilarityQueue::<Modulus>::with_capacity(3);
        queue.push(5.into());
        queue.push(3.into());
        queue.push(1.into());
        assert!(queue.contains(&5.into()));
        queue.push((-10).into());
        assert!(!queue.contains(&(-10).into()));
        assert!(queue.contains(&1.into()));
        queue.push(2.into());
        assert!(!queue.contains(&5.into()));
        queue.push(4.into());
        assert!(!queue.contains(&4.into()));
    }
}
