use std::ops::AddAssign;

use glam::{Vec3, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_homogeneous(&self) -> Vec4 {
        Vec4::from_array([self.x, self.y, self.z, 1.0])
    }
}

impl From<Point3> for Vec3 {
    fn from(point: Point3) -> Self {
        Vec3 {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl From<(f32, f32, f32)> for Point3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Point3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
