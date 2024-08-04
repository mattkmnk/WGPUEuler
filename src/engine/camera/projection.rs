use glam::Mat4;

use crate::{Rad, OPENGL_TO_WGPU_MATRIX};

#[derive(Debug, Clone, Copy)]
pub struct Projection {
    aspect: f32,
    fovy: Rad,
    znear: f32,
    zfar: f32,
    perspective: Mat4,
}

impl Projection {
    pub fn new<F: Into<Rad>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        let aspect = width as f32 / height as f32;
        let fovy = fovy.into();
        let perspective = Self::calc_perspective_matrix(fovy, aspect, znear, zfar);

        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
            perspective,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
        self.perspective =
            Self::calc_perspective_matrix(self.fovy, self.aspect, self.znear, self.zfar);
    }

    pub fn calc_perspective_matrix(fovy: Rad, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
        Mat4::perspective_rh(fovy.0.into(), aspect, znear, zfar)
    }

    pub fn calc_matrix(&self) -> Mat4 {
        OPENGL_TO_WGPU_MATRIX * self.perspective
        //
    }
}
