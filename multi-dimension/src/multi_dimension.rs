pub trait MultiDimension {
    const DIM: usize;

    fn j_clone(j: usize, this: &mut Self, that: &Self);
    fn j_compare(j: usize, this: &Self, that: &Self) -> std::cmp::Ordering;
}