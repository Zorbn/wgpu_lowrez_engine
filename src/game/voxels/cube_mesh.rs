use crate::engine::vertex::Vertex;

#[derive(Copy, Clone)]
pub enum Directions {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Forward = 4,
    Backward = 5,
}

pub struct Plane {
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
}

pub const MESH_SIDES: [Plane; 6] = [
    Plane {
        vertices: [
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
        ],
        indices: [0, 2, 1, 0, 3, 2],
    },
    Plane {
        vertices: [
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
        ],
        indices: [0, 1, 2, 0, 2, 3],
    },
    Plane {
        vertices: [
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
        ],
        indices: [0, 2, 1, 0, 3, 2],
    },
    Plane {
        vertices: [
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
        ],
        indices: [0, 1, 2, 0, 2, 3],
    },
    Plane {
        vertices: [
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                tex_index: 1,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                tex_index: 1,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                tex_index: 1,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                tex_index: 1,
                color: [1.0, 1.0, 1.0],
            },
        ],
        indices: [0, 1, 2, 0, 2, 3],
    },
    Plane {
        vertices: [
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                tex_index: 0,
                color: [1.0, 1.0, 1.0],
            },
        ],
        indices: [0, 2, 1, 0, 3, 2],
    },
];
