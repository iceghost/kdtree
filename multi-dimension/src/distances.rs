use super::MultiDimension;

pub trait JMeasure: MultiDimension {
    type Distance;
    fn j_diff(j: usize, this: &Self, that: &Self) -> Self::Distance;
    fn j_compare(j: usize, this: &Self, that: &Self) -> std::cmp::Ordering;
}

pub trait DissimilarityMeasure: JMeasure {
    type Dissimilarity;
    fn dissimilarity(this: &Self, that: &Self) -> Self::Dissimilarity;
}

pub trait Euclidian: JMeasure {}

impl<T: Euclidian<Distance = D>, D: From<f32> + Into<f32>> DissimilarityMeasure for T {
    type Dissimilarity = D;
    fn dissimilarity(this: &Self, that: &Self) -> Self::Dissimilarity {
        let mut sum = 0f32;
        for j in 0..Self::DIM {
            let diff: f32 = JMeasure::j_diff(j, this, that).into();
            sum += diff * diff;
        }
        sum.sqrt().into()
    }
}
