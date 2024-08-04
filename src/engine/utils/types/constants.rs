use std::f32::consts::FRAC_PI_2;

use glam::{Mat4, Vec4};

pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    Vec4::new(1.0, 0.0, 0.0, 0.0),
    Vec4::new(0.0, 1.0, 0.0, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 1.0),
);

pub const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;
