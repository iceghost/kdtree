use super::MultiDimension;

struct Bounds<T, const K: usize>
where
    T: MultiDimension<K>
{
    upper: T,
    lower: T,
}

impl<T, D, const K: usize> Bounds<T, K>
where
    T: MultiDimension<K, Dimension = D>,
    D: Ord,
{
    fn contain_ball(&self, center: &T, radius: D) -> bool {
        
    }

    // fn intersect_ball();
}
