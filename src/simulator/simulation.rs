use std::time::Duration;

use crate::{
    CameraController2D, CameraDescriptor, Controller, Deg, Engine, EulerSimulation, Point3,
    Projection, Window, WindowEvents,
};

use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode},
};

struct Simulation {
    engine: Engine,

    camera_controller: CameraController2D,
    mouse_pressed: bool,
    simulation: EulerSimulation,

    stopped: bool,
}

impl Simulation {
    pub async fn new(window: &Window) -> Self {
        let (width, height) = (window.get_width(), window.get_height());
        let projection = Projection::new(width, height, Deg(90.0), 0.1, 10000.0);
        let camera_controller = CameraController2D::new(100.0, 0.5, 2.0, 1000.0); // 2.0, 2000.0
        let mut engine = Engine::new(
            window,
            &CameraDescriptor {
                position: Point3::from(100.0, 50.0, 50.0),
                yaw: Deg::new(-90.0).into(),
                pitch: Deg::new(0.0).into(),
                projection,
            },
        )
        .await;

        engine.add_render_pass();

        let mut simulation = EulerSimulation::new(1000.0, 200, 100, 1.0 / 200.0 * 100.0);
        simulation.init();

        Simulation {
            engine,
            camera_controller,
            mouse_pressed: false,
            simulation,
            stopped: true,
        }
    }

    pub fn process_input(&mut self, state: ElementState, key: VirtualKeyCode) {
        if key == VirtualKeyCode::Space && state == ElementState::Pressed {
            self.stopped = !self.stopped;
        }
        self.camera_controller.process_keyboard(key, state);
    }

    pub fn process_mouse(&mut self, position: PhysicalPosition<f32>) {
        if self.mouse_pressed {
            self.camera_controller.process_mouse(position.x, position.y);
        }
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.camera_controller.process_scroll(delta);
    }

    fn process_mouse_input(&mut self, state: ElementState, mouse_button: MouseButton) {
        if mouse_button == MouseButton::Left && state == ElementState::Pressed {
            self.mouse_pressed = true;
        } else {
            self.mouse_pressed = false;
        }
    }

    fn update(&mut self, dt: Duration) {
        self.camera_controller.update(&mut self.engine.camera, dt);

        if !self.stopped {
            self.simulation.update(dt);
        }

        self.engine.update_instances(&self.simulation.instances);
        self.engine.update(dt);
    }

    fn render(&mut self) {
        self.engine.render();
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.engine.resize(new_size);
    }
}

pub async fn run() {
    let window = Window::new();
    let mut game = Simulation::new(&window).await;

    window.run(move |event| match event {
        WindowEvents::Unknown => todo!(),
        WindowEvents::Keyboard {
            state,
            virtual_keycode,
        } => game.process_input(state, virtual_keycode),
        WindowEvents::Mouse { state, button } => {
            game.process_mouse_input(state, button);
        }
        WindowEvents::MouseMoved { delta } => {
            game.process_mouse(delta);
        }
        WindowEvents::MouseWheel { delta } => {
            game.process_scroll(&delta);
        }
        WindowEvents::Resized { width, height } => game.resize(PhysicalSize::new(width, height)),
        WindowEvents::Draw { dt } => {
            game.update(dt);
            game.render();
        }
    })
}
