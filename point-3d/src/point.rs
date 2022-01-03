use crate::float::Float;
use multi_dimension::MultiDimension;

#[derive(PartialEq)]
pub struct Point {
    x: Float,
    y: Float,
    z: Float,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point {
            x: Float::new(x),
            y: Float::new(y),
            z: Float::new(z),
        }
    }
}

impl MultiDimension for Point {
    const DIM: usize = 3;

    fn j_compare(j: usize, this: &Self, that: &Self) -> std::cmp::Ordering {
        let j = j % 3;
        match j {
            0 => this.x.cmp(&that.x),
            1 => this.y.cmp(&that.y),
            _ => this.z.cmp(&that.z),
        }
    }
}
