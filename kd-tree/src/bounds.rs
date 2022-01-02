use std::cmp::Ordering;

use multi_dimension::{MultiDimension, distances::DissimilarityMeasure};

struct Bounds<T> {
    upper: T,
    lower: T,
}

impl<T, O> Bounds<T>
where
    T: MultiDimension + DissimilarityMeasure<Output = O>,
    O: Ord,
{
    fn contain_ball(&self, center: &T, radius: &T::Output) -> bool {
        for j in 0..T::DIM {
            let upper_distance = T::j_distance(j, center, &self.upper);
            let lower_distance = T::j_distance(j, center, &self.lower);
            if upper_distance <= *radius || lower_distance <= *radius {
                return false;
            }
        }
        true
    }
}

impl<T, O> Bounds<T>
where
    T: MultiDimension + DissimilarityMeasure<Output = O>,
    O: Ord + std::ops::AddAssign + Default,
{
    fn overlap_ball(&self, center: &T, radius: &T::Output) -> bool {
        let mut sum = O::default();
        for j in 0..T::DIM {
            if T::j_compare(j, center, &self.lower) == Ordering::Less {
                // lower than boundary
                sum += T::j_distance(j, center, &self.lower);
                if T::dissimilarity(&sum) > *radius {
                    return false;
                }
            } else if T::j_compare(j, center, &self.upper) == Ordering::Greater {
                // higher than boundary
                sum += T::j_distance(j, center, &self.upper);
                if T::dissimilarity(&sum) > *radius {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}
