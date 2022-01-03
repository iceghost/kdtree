use crate::MultiDimension;

pub trait DissimilarityMeasure {
    type Output;
    fn j_distance(j: usize, this: &Self, that: &Self) -> Self::Output;
    fn dissimilarity(sum: &Self::Output) -> Self::Output;
}

pub fn dissimilarity_between<T, O>(this: &T, that: &T) -> O
where
    T: MultiDimension + DissimilarityMeasure<Output = O>,
    O: Default + std::ops::AddAssign
{
    let mut sum = O::default();
    for j in 0..T::DIM {
        sum += T::j_distance(j, this, that);
    }
    T::dissimilarity(&sum)
}
