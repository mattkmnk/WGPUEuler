use crate::{Camera, GraphicsContext, RenderPass};

pub struct Renderer {
    pub render_passes: Vec<RenderPass>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            render_passes: vec![],
        }
    }

    pub fn add_pass(&mut self, render_pass: RenderPass) {
        self.render_passes.push(render_pass);
    }

    pub fn render(&mut self, ctx: &GraphicsContext, camera: &Camera) {
        for pass in self.render_passes.iter_mut() {
            match pass.render(&ctx, &camera) {
                Ok(_) => {}
                Err(_) => panic!("Failed render pass"),
            }
        }
    }
}
