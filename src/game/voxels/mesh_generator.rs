use crate::{
    engine::vertex,
    game::voxels::{blocks, cube_mesh},
};

pub struct MeshData {
    pub vertices: Vec<vertex::Vertex>,
    pub indices: Vec<u16>,
}

pub fn generate_mesh_data(chunk: &blocks::Chunk) {
    let num_blocks = chunk.width() * chunk.height() * chunk.depth();
    let mut mesh_data = MeshData {
        vertices: Vec::new(),
        indices: Vec::new(),
    };

    for i in 0..num_blocks {
        let (x, y, z) = blocks::Chunk::get_block_xyz(chunk.width(), chunk.height(), i);
        generate_block(chunk, &mut mesh_data, x, y, z);
    }
}

fn generate_block(chunk: &blocks::Chunk, mesh_data: &mut MeshData, x: usize, y: usize, z: usize) {
    generate_face(chunk, mesh_data, x, y, z, cube_mesh::Directions::Up);
    generate_face(chunk, mesh_data, x, y, z, cube_mesh::Directions::Forward);
}

fn generate_face(
    chunk: &blocks::Chunk,
    mesh_data: &mut MeshData,
    x: usize,
    y: usize,
    z: usize,
    face: cube_mesh::Directions,
) {
    let block = chunk.get_block(x, y, z);
    let mesh_side = &cube_mesh::MESH_SIDES[face as usize];
    mesh_data.vertices.extend_from_slice(&mesh_side.vertices);
    mesh_data.indices.extend_from_slice(&mesh_side.indices);
}
