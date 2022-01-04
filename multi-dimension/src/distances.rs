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

macro_rules! dimension_1 {
    ($type:ty) => {
        impl DissimilarityMeasure for $type {
            type Output = $type;
            fn j_distance(_: usize, this: &Self, that: &Self) -> Self::Output {
                (this - that).abs()
            }
            fn dissimilarity(sum: &Self::Output) -> Self::Output {
                *sum
            }
        }
    };
}

macro_rules! dimension_2 {
    ($type:ty) => {
        impl DissimilarityMeasure for ($type, $type) {
            type Output = $type;
            fn j_distance(j: usize, this: &Self, that: &Self) -> Self::Output {
                if j % 2 == 0 {
                    let diff = this.0 - that.0;
                    diff * diff
                } else {
                    let diff = this.1 - that.1;
                    diff * diff
                }
            }
            fn dissimilarity(sum: &Self::Output) -> Self::Output {
                *sum
            }
        }
    };
}

dimension_1!(isize);
dimension_2!(isize);