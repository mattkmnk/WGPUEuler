use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use super::{input_handler, InputHandler};

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub window: winit::window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        Self { event_loop, window }
    }

    pub fn run(self, mut input_handler: InputHandler) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => {
                        input_handler.process(event);
                    }
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        input_handler.process(event);
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}
