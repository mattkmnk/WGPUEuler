use std::any::TypeId;

use tracing::debug;
use winit::event::{
    ElementState, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent,
};

pub struct Callback<T> {
    cb: Box<dyn FnMut(T) -> ()>,
}

impl<T> Callback<T> {
    pub fn new(callback: impl 'static + FnMut(T) -> ()) -> Self {
        Self {
            cb: Box::new(callback),
        }
    }
}

pub enum Inputs {
    Unknown,
    KeyboardInput {
        state: ElementState,
        virtual_keycode: Option<VirtualKeyCode>,
    },
    MouseInput {
        state: ElementState,
        button: MouseButton,
    },
}

enum InputType {
    Unknown,
    KeyboardInput,
    MouseInput,
}

trait CallbackType {
    fn get_type(&self) -> InputType;
}

impl<T: 'static> CallbackType for Callback<T> {
    fn get_type(&self) -> InputType {
        let type_id = TypeId::of::<T>();

        if type_id == TypeId::of::<KeyboardInput>() {
            return InputType::KeyboardInput;
        }

        if type_id == TypeId::of::<MouseButton>() {
            return InputType::MouseInput;
        }

        return InputType::Unknown;
    }
}

pub struct InputHandler {
    keyboard_handlers: Callback<KeyboardInput>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keyboard_handlers: Callback::new(|input| {}),
        }
    }

    pub fn process(&mut self, event: WindowEvent) -> bool {
        return false;
    }

    pub fn add_input<T: 'static>(mut self, callback: impl 'static + FnMut(T) -> ()) -> Self {
        let callback: Callback<T> = Callback::new(callback);
        match callback.get_type() {
            InputType::Unknown => todo!(),
            InputType::KeyboardInput => {
                debug!("KeyboardInput");
                // self.keyboard_handlers = callback;
            }
            InputType::MouseInput => {
                debug!("MouseInput");
            }
        }

        self
    }
}
