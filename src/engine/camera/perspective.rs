use glam::{Mat4, Vec4};

use crate::Rad;

pub struct Perspective;

impl Perspective {
    pub fn from(fovy: Rad, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
        let f: f32 = (fovy.0 / 2.0).cos() * (fovy.0 / 2.0).sin();
        let _uh = 1.0 / (fovy.0 / 2.0).tan();
        let _uw = 1.0 / aspect;

        // Mat4::from_cols(
        //     Vec4::new(uw, 0.0, 0.0, 0.0),
        //     Vec4::new(0.0, uh, 0.0, 0.0),
        //     Vec4::new(0.0, 0.0, zfar / (zfar - znear), 1.0),
        //     Vec4::new(0.0, 0.0, -zfar * znear / (zfar - znear), 0.0),
        // )

        Mat4::from_cols(
            Vec4::new(f, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, (zfar + znear) / (znear - zfar), -1.0),
            Vec4::new(0.0, 0.0, (2.0 * zfar * znear) / (znear - zfar), 0.0),
        )
    }
}
