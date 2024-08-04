use std::{
    f32::consts::PI,
    ops::{AddAssign, Mul, Neg},
};

use crate::Deg;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rad(pub f32);

impl Rad {
    pub fn new(deg: f32) -> Self {
        return Self(deg);
    }
}

impl Neg for Rad {
    type Output = Rad;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul<f32> for Rad {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl AddAssign<f32> for Rad {
    fn add_assign(&mut self, rhs: f32) {
        *self = Self(self.0 + rhs);
    }
}

impl From<Deg> for Rad {
    fn from(deg: Deg) -> Self {
        Rad(deg.0 * (PI / 180.0))
    }
}

impl From<f32> for Rad {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Into<f32> for Rad {
    fn into(self) -> f32 {
        self.0
    }
}
