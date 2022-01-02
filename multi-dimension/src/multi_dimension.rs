pub trait MultiDimension {
    const DIM: usize;

    fn j_clone(j: usize, this: &mut Self, that: &mut Self);
}