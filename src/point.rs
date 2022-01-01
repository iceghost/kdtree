use super::float::Float;
use super::kdtree::MultiDimension;

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

impl MultiDimension<3> for Point {
    type Dimension = Float;
    fn nth_dim(&self, n: usize) -> Self::Dimension {
        let n = n % 3;
        match n {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }
}
