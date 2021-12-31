//! Our custom float with accuracy of 0.001

use std::cmp::Ordering;

const ACCURACY: f32 = 0.001;

#[derive(Clone, Copy)]
pub struct Float(f32);

impl Float {
    pub fn new(f: f32) -> Self {
        Self(f)
    }
}

impl PartialEq for Float {
    fn eq(&self, y: &Self) -> bool {
        self.cmp(y) == Ordering::Equal
    }
}

impl Eq for Float {}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if (self.0 - other.0).abs() <= ACCURACY {
            Ordering::Equal
        } else if self.0 - other.0 > ACCURACY {
            Ordering::Greater
        } else {
            Ordering::Less
        })
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn equal() {
        assert!(Float(0.1) == Float(0.11));
    }
}
