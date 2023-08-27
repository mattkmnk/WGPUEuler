use abstrct::{App, InputHandler, Inputs};
use tracing::debug;

fn main() {
    init_logger();

    let input_handler = InputHandler::new()
        .add_input(keyboard_input)
        .add_input(mouse_input);

    App::new().with_input(input_handler).run();
}

fn keyboard_input(input: Inputs) {
    if let Inputs::KeyboardInput {
        state,
        virtual_keycode: Some(keycode),
        ..
    } = input
    {
        debug!("{:?} {:?}", keycode, state);
    }
}

fn mouse_input(input: Inputs) {
    if let Inputs::MouseInput { state, button } = input {
        debug!("{:?} {:?}", button, state);
    }
}

fn init_logger() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_max_level(tracing::Level::TRACE)
        .with_writer(std::io::stdout)
        .init();
}
