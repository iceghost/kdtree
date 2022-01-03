pub trait MultiDimension {
    const DIM: usize;

    // fn j_clone(j: usize, this: &mut Self, that: &Self);
    fn j_compare(j: usize, this: &Self, that: &Self) -> std::cmp::Ordering;
}

// blanket implementations for numbers for testings
macro_rules! dimension_1 {
    ($type:ty) => {
        impl MultiDimension for $type {
            const DIM: usize = 1;
            fn j_compare(_: usize, this: &Self, that: &Self) -> std::cmp::Ordering {
                Self::cmp(this, that)
            }
        }
    };
}

macro_rules! dimension_2 {
    ($type1:ty, $type2:ty) => {
        impl MultiDimension for ($type1, $type2) {
            const DIM: usize = 2;
            fn j_compare(j: usize, this: &Self, that: &Self) -> std::cmp::Ordering {
                if j % 2 == 0 {
                    <$type1>::cmp(&this.0, &that.0)
                } else {
                    <$type2>::cmp(&this.1, &that.1)
                }
            }
        }
    };
}

dimension_1!(isize);
dimension_1!(usize);
dimension_2!(isize, isize);
dimension_2!(usize, usize);
