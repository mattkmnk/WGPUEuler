use tracing::debug;

use crate::renderer::GraphicsContext;

use super::window::Window;
use super::InputHandler;

pub struct App {
    window: Window,
    input_handler: Option<InputHandler>,
}

impl App {
    pub fn new() -> Self {
        let window = Window::new();
        let graphics_context = GraphicsContext::new();

        Self {
            window,
            input_handler: None,
        }
    }

    pub fn with_input(mut self, input_handler: InputHandler) -> App {
        self.input_handler = Some(input_handler);

        self
    }

    pub fn run(self) {
        if let Some(input_handler) = self.input_handler {
            debug!("Running custom input handler");
            self.window.run(input_handler);
        } else {
            self.window.run(InputHandler::new())
        }
    }
}
