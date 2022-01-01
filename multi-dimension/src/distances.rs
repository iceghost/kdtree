use super::MultiDimension;

pub trait JMeasure: MultiDimension {
    type Distance;
    fn j_diff(j: usize, this: &Self, that: &Self) -> Self::Distance;
    fn j_compare(j: usize, this: &Self, that: &Self) -> std::cmp::Ordering;
}

pub trait DissimilarityMeasure: JMeasure {
    type Dissimilarity;
    fn dissimilarity_measure(this: &Self, that: &Self) -> Self::Dissimilarity;
}

pub trait Euclidian: MultiDimension + JMeasure<Distance = f32> {}

impl<T: Euclidian> DissimilarityMeasure for T {
    type Dissimilarity = f32;
    fn dissimilarity_measure(this: &Self, that: &Self) -> Self::Dissimilarity {
        let mut sum = 0f32;
        for j in 0..Self::DIM {
            let diff = JMeasure::j_diff(j, this, that);
            sum += diff * diff;
        }
        sum.sqrt()
    }
}
