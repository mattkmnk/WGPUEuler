use std::time::Duration;

use glam::Vec3;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::dpi::Position;
use winit::event::ElementState;
use winit::event::MouseScrollDelta;
use winit::event::VirtualKeyCode;

use crate::renderer::GraphicsContext;
use crate::renderer::Renderer;
use crate::Camera;
use crate::CameraDescriptor;
use crate::Controller;
use crate::Cube;
use crate::Instance;
use crate::Sphere;
use crate::Vertex;

use crate::RenderPass;

use super::window::Window;

pub struct Engine {
    graphics_context: GraphicsContext,
    renderer: Renderer,
    pub camera: Camera,
}

impl Engine {
    pub async fn new(window: &Window, camera_descriptor: &CameraDescriptor) -> Engine {
        let graphics_context = GraphicsContext::new(&window.winit_window).await;
        let renderer = Renderer::new();
        let camera = Camera::new(
            &graphics_context,
            camera_descriptor.position,
            camera_descriptor.yaw,
            camera_descriptor.pitch,
            camera_descriptor.projection,
        );

        Self {
            graphics_context,
            renderer,
            camera,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.graphics_context.resize(new_size);
            self.camera.resize_projection(new_size);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&self.graphics_context, &self.camera);

        Ok(())
    }

    pub fn update(&mut self, dt: Duration) {
        self.camera.update_view_proj();
        self.graphics_context.queue.write_buffer(
            &self.camera.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.camera_uniform]),
        );
        self.graphics_context.queue.submit([]);
    }

    pub fn add_render_pass(&mut self) {
        let mut render_pass = RenderPass::new(
            &self.graphics_context,
            include_str!("../renderer/webgpu/shaders/grid_shader.wgsl"),
            &mut self.camera,
        );

        let (vertices, indices) = Cube::get_mesh();

        render_pass.set_mesh(&self.graphics_context, &vertices, &indices);

        self.renderer.add_pass(render_pass);
    }

    pub fn update_instances(&mut self, instances: &[Instance]) {
        self.renderer.render_passes[0].update_instances(&self.graphics_context, instances);
    }
}
