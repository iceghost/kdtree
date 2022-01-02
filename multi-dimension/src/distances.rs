pub trait DissimilarityMeasure {
    type Output;
    fn j_distance(j: usize, this: &Self, that: &Self) -> Self::Output;
    fn dissimilarity(sum: &Self::Output) -> Self::Output;
}
