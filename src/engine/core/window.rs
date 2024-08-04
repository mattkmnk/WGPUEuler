use std::time::Instant;

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::DeviceEvent;
use winit::window::WindowBuilder;
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::EventLoop,
    window::WindowId,
};

use crate::WindowEvents;

pub struct Window {
    pub event_loop: Option<EventLoop<()>>,
    pub winit_window: winit::window::Window,
    pub winit_window_id: WindowId,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let winit_window = WindowBuilder::new()
            .with_min_inner_size(PhysicalSize::new(200, 200))
            .with_title("MK_Symulator")
            .build(&event_loop)
            .unwrap();
        let winit_window_id = winit_window.id();

        Self {
            event_loop: Some(event_loop),
            winit_window,
            winit_window_id,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.winit_window.inner_size().width
    }

    pub fn get_height(&self) -> u32 {
        self.winit_window.inner_size().height
    }

    pub fn run(mut self, mut callback: impl 'static + FnMut(WindowEvents)) {
        let event_loop = self.event_loop.take().unwrap();
        let mut last_render_time = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => callback(WindowEvents::MouseMoved {
                    delta: PhysicalPosition::<f32>::new(delta.0 as f32, delta.1 as f32),
                }),
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let KeyboardInput {
                            state,
                            virtual_keycode: Some(virtual_keycode),
                            ..
                        } = input
                        {
                            callback(WindowEvents::Keyboard {
                                state,
                                virtual_keycode,
                            });
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        callback(WindowEvents::Mouse { state, button });
                    }
                    WindowEvent::CursorMoved { .. } => {}
                    WindowEvent::MouseWheel { delta, .. } => {
                        callback(WindowEvents::MouseWheel { delta })
                    }
                    WindowEvent::Resized(physical_size) => {
                        callback(WindowEvents::Resized {
                            width: physical_size.width,
                            height: physical_size.height,
                        });
                    }
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.winit_window_id => {
                    let now = Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    callback(WindowEvents::Draw { dt });
                }
                Event::RedrawEventsCleared => {
                    self.winit_window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
