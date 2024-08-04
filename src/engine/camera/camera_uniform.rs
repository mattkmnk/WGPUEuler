use glam::Mat4;

use crate::{Point3, Projection};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, position: &Point3, projection: &Projection, view: Mat4) {
        self.view_position = [position.x, position.y, position.z, 1.0];
        self.view_proj = (projection.calc_matrix() * view).to_cols_array_2d();
    }
}
