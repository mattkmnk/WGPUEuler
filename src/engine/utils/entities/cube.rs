use crate::Vertex;

pub struct Cube {}

impl Cube {
    pub fn get_mesh() -> ([Vertex; 8], [u16; 36]) {
        let vertices: [Vertex; 8] = [
            Vertex {
                position: [-0.5, -0.5, -0.5],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
            },
        ];

        let indices: [u16; 36] = [
            0, 1, 2, 2, 3, 0, // Tył
            4, 5, 1, 1, 0, 4, // Dół
            7, 6, 5, 5, 4, 7, // Przód
            3, 2, 6, 6, 7, 3, // Góra
            0, 3, 7, 7, 4, 0, // Lewo
            1, 5, 6, 6, 2, 1, // Prawo
        ];

        return (vertices, indices);
    }
}
