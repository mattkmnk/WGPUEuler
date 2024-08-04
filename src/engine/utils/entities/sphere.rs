use std::f32::consts::PI;

use crate::{Deg, Vertex};

const CIRCLE_SEGMENT_COUNT: usize = 64;
const CIRCLE_VERTEX_COUNT: usize = CIRCLE_SEGMENT_COUNT + 2;
const CIRCLE_INDEX_COUNT: usize = CIRCLE_SEGMENT_COUNT * 3;

pub struct Sphere {}

impl Sphere {
    pub fn get_mesh(radius: f32) -> ([Vertex; CIRCLE_VERTEX_COUNT], [u16; CIRCLE_INDEX_COUNT]) {
        let mut vertices: [Vertex; CIRCLE_VERTEX_COUNT] = [Vertex {
            position: [0.0, 0.0, 0.0],
        }; CIRCLE_VERTEX_COUNT];
        let mut indices: [u16; CIRCLE_INDEX_COUNT] = [0; CIRCLE_INDEX_COUNT];

        let segment_width = PI * 2.0 / CIRCLE_SEGMENT_COUNT as f32;
        let mut angle: f32 = 0.0;
        vertices[0] = Vertex {
            position: [0.0, 0.0, 0.0],
        };

        for i in 1..CIRCLE_VERTEX_COUNT {
            vertices[i] = Vertex {
                position: [angle.cos() * radius, angle.sin() * radius, 0.0],
            };
            angle -= segment_width;

            if i > 1 {
                let j = (i - 2) * 3;
                indices[j] = 0;
                indices[j + 1] = i as u16;
                indices[j + 2] = i as u16 - 1;
            }
        }

        return (vertices, indices);
    }
}
