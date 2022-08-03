use crate::engine::vertex::Vertex;
use wgpu::util::DeviceExt;

pub struct Model {
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub num_indices: u32,
}

impl Model {
    pub fn new(device: &wgpu::Device, vertex_array: &[Vertex], index_array: &[u16]) -> Self {
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex_array),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(index_array),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = index_array.len() as u32;

        Self {
            vertices,
            indices,
            num_indices,
        }
    }

    pub fn vertices(&self) -> &wgpu::Buffer {
        &self.vertices
    }

    pub fn indices(&self) -> &wgpu::Buffer {
        &self.indices
    }

    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}
