use crate::kd_tree::Link;
use multi_dimension::{distances::DissimilarityMeasure, MultiDimension};
use std::cmp::Ordering;

pub struct Bounds<'a, T> {
    pub upper: Vec<&'a Link<T>>,
    pub lower: Vec<&'a Link<T>>,
}

type Bound<'a, T> = Vec<&'a Link<T>>;

impl<'a, T: MultiDimension> Bounds<'a, T> {
    pub fn new() -> Self {
        Self {
            upper: vec![&None; T::DIM],
            lower: vec![&None; T::DIM],
        }
    }
}

impl<'a, T, O> Bounds<'a, T>
where
    T: MultiDimension + DissimilarityMeasure<Output = O>,
    O: PartialOrd,
{
    pub fn contain_ball(&self, center: &T, radius: &T::Output) -> bool {
        for j in 0..T::DIM {
            let out_upper = Self::out_of_bound(&self.upper, j, center, radius);
            let out_lower = Self::out_of_bound(&self.lower, j, center, radius);
            if out_upper || out_lower {
                return false;
            }
        }
        true
    }

    fn out_of_bound(bound: &Bound<'a, T>, j: usize, center: &T, radius: &O) -> bool {
        let out_upper = if let Some(node) = bound[j] {
            T::dissimilarity(&T::j_distance(j, center, node)) <= *radius
        } else {
            false
        };
        out_upper
    }
}

impl<'a, T, O> Bounds<'a, T>
where
    T: MultiDimension + DissimilarityMeasure<Output = O>,
    O: PartialOrd + std::ops::AddAssign + Default,
{
    pub fn overlap_ball(&self, center: &T, radius: &T::Output) -> bool {
        let mut sum = O::default();
        for j in 0..T::DIM {
            enum Three {
                EarlyReturn,
                AtLeastITried,
                DidntTry,
            }
            let mut func = |bound: &Bound<_>, ordering: Ordering| {
                if let Some(node) = bound[j] {
                    if T::j_compare(j, center, node) == ordering {
                        // higher than boundary
                        sum += T::j_distance(j, center, node);
                        if T::dissimilarity(&sum) > *radius {
                            return Three::EarlyReturn;
                        }
                        return Three::AtLeastITried;
                    }
                };
                Three::DidntTry
            };
            match func(&self.lower, Ordering::Less) {
                Three::EarlyReturn => return false,
                Three::AtLeastITried => continue,
                Three::DidntTry => {
                    func(&self.upper, Ordering::Greater);
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
        let lower = (0isize, 0isize);
        let upper = (5isize, 5isize);
        // let bounds = Bounds::new();
        // bounds.upper.fill(upper);
    }
}
