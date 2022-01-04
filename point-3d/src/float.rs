//! Our custom float with accuracy of 0.001

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, Mul, Div, Deref};

const ACCURACY: f32 = 0.001;

#[derive(Clone, Copy, Default)]
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

impl Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Float(self.0 + rhs.0)
    }
}

impl Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Float(self.0 - rhs.0)
    }
}

impl Mul for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Float(self.0 * rhs.0)
    }
}

impl Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Float(self.0 / rhs.0)
    }
}

impl AddAssign for Float {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Deref for Float {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        assert!(Float(0.1) == Float(0.1001));
    }
}
