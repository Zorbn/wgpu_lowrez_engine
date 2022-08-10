use crate::{
    engine::vertex,
    game::voxels::{blocks, chunk, cube_mesh, directions},
};

const MIN_BLOCK_TINT_BRIGHTNESS: f32 = 0.1;

pub struct MeshData {
    pub vertices: Vec<vertex::Vertex>,
    pub indices: Vec<u16>,
}

pub fn generate_mesh_data(chunk: &chunk::Chunk) -> MeshData {
    let num_blocks = chunk.width() * chunk.height() * chunk.depth();
    let mut mesh_data = MeshData {
        vertices: Vec::new(),
        indices: Vec::new(),
    };

    for i in 0..num_blocks {
        let (x, y, z) = chunk::Chunk::get_block_xyz(chunk.width(), chunk.height(), i as usize);
        generate_block(chunk, &mut mesh_data, x, y, z);
    }

    mesh_data
}

fn generate_block(chunk: &chunk::Chunk, mesh_data: &mut MeshData, x: i32, y: i32, z: i32) {
    let block = chunk.get_block(x, y, z);

    if block == blocks::Blocks::AIR {
        return;
    }

    generate_face(chunk, mesh_data, block, x, y, z, directions::Directions::Up);
    generate_face(
        chunk,
        mesh_data,
        block,
        x,
        y,
        z,
        directions::Directions::Forward,
    );
}

fn generate_face(
    chunk: &chunk::Chunk,
    mesh_data: &mut MeshData,
    block: blocks::Blocks,
    x: i32,
    y: i32,
    z: i32,
    face: directions::Directions,
) {
    let face_offset = directions::dir_to_offset(face);
    if chunk.get_block(x + face_offset.0, y + face_offset.1, z + face_offset.2)
        != blocks::Blocks::AIR
    {
        return;
    }

    let texture_index = (block as u32) - 1;

    let mesh_side = &cube_mesh::MESH_SIDES[face as usize];
    let current_vertex_count = mesh_data.vertices.len() as u16;

    for i in 0..mesh_side.vertices.len() {
        let mut new_vertex = mesh_side.vertices[i];
        new_vertex.position[0] += x as f32;
        new_vertex.position[1] += y as f32;
        new_vertex.position[2] += z as f32;
        new_vertex.tex_index = texture_index;

        let tint = (new_vertex.position[1] / chunk.world_height() as f32
            + MIN_BLOCK_TINT_BRIGHTNESS)
            .min(1.0);

        for c in 0..3 {
            new_vertex.color[c] *= tint;
        }

        mesh_data.vertices.push(new_vertex);
    }

    for i in 0..mesh_side.indices.len() {
        mesh_data
            .indices
            .push(mesh_side.indices[i] + current_vertex_count);
    }
}
