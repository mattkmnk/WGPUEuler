use std::time::Duration;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode},
};

#[derive(Debug, Clone, Copy)]
pub enum WindowEvents {
    Unknown,
    Keyboard {
        state: ElementState,
        virtual_keycode: VirtualKeyCode,
    },
    Mouse {
        state: ElementState,
        button: MouseButton,
    },
    MouseMoved {
        delta: PhysicalPosition<f32>,
    },
    MouseWheel {
        delta: MouseScrollDelta,
    },
    Resized {
        width: u32,
        height: u32,
    },
    Draw {
        dt: Duration,
    },
}
