use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use crate::{CameraUniform, GraphicsContext, Point3, Projection, Rad};

pub struct CameraDescriptor {
    pub position: Point3,
    pub yaw: Rad,
    pub pitch: Rad,
    pub projection: Projection,
}

#[derive(Debug)]
pub struct Camera {
    pub position: Point3,
    pub yaw: Rad,
    pub pitch: Rad,
    pub projection: Projection,

    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new<V: Into<Point3>, Y: Into<Rad>, P: Into<Rad>>(
        graphics_context: &GraphicsContext,
        position: V,
        yaw: Y,
        pitch: P,
        projection: Projection,
    ) -> Self {
        let camera_uniform = CameraUniform::new();

        let camera_buffer =
            graphics_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[camera_uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let camera_bind_group_layout =
            graphics_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("camera_bind_group_layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let camera_bind_group =
            graphics_context
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }],
                    label: Some("camera_bind_group"),
                });

        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            camera_uniform,
            projection,
            camera_buffer,
            camera_bind_group_layout,
            camera_bind_group,
        }
    }

    pub fn position(&self) -> Point3 {
        self.position
    }

    pub fn calc_view_matrix(&self) -> Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Mat4::look_to_rh(
            Vec3::from(self.position),
            Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vec3::new(0.0, 1.0, 0.0),
        )
    }

    pub fn resize_projection(&mut self, new_size: PhysicalSize<u32>) {
        let (width, height) = new_size.into();
        self.projection.resize(width, height);
    }

    pub fn update_view_proj(&mut self) {
        self.camera_uniform.update_view_proj(
            &self.position,
            &self.projection,
            self.calc_view_matrix(),
        );
    }
}
