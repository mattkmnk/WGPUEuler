use glam::Vec3;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta, VirtualKeyCode},
};

use crate::{Controller, Rad, SAFE_FRAC_PI_2};

pub struct CameraController2D {
    amount_left: f32,
    amount_right: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
    min_zoom: f32,
    max_zoom: f32,
}

impl CameraController2D {
    pub fn new(speed: f32, sensitivity: f32, min_zoom: f32, max_zoom: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            min_zoom,
            max_zoom,
        }
    }
}

impl Controller for CameraController2D {
    fn process_keyboard(
        &mut self,
        key: winit::event::VirtualKeyCode,
        state: winit::event::ElementState,
    ) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };

        match key {
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::W => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::S => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }

    fn process_mouse(&mut self, mouse_dx: f32, mouse_dy: f32) {
        self.rotate_horizontal = mouse_dx;
        self.rotate_vertical = mouse_dy;
    }

    fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        }
    }

    fn update(&mut self, camera: &mut crate::Camera, dt: std::time::Duration) {
        let dt = dt.as_secs_f32();

        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        camera.position.x += (self.amount_right - self.amount_left) * self.speed * dt;
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward = Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        camera.position.z = if camera.position.z < self.min_zoom {
            self.min_zoom
        } else {
            camera.position.z
        };

        camera.position.z = if camera.position.z > self.max_zoom {
            self.max_zoom
        } else {
            camera.position.z
        };
    }
}
