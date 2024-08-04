use std::time::Duration;

use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode};

use crate::Camera;

pub trait Controller {
    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool;
    fn process_mouse(&mut self, mouse_dx: f32, mouse_dy: f32);
    fn process_scroll(&mut self, delta: &MouseScrollDelta);
    fn update(&mut self, camera: &mut Camera, dt: Duration);
}
