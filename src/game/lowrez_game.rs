use crate::engine::{
    camera, engine_handle, game, input, model, render_handle, texture, texture_array, vertex,
};
use crate::game::voxels::{chunk, mesh_generator};

const SCREEN_VERTICES: &[vertex::Vertex] = &[
    vertex::Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
    vertex::Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
    vertex::Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
    vertex::Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
];

const SCREEN_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

const SCREEN_SIZE: u32 = 64;

pub struct LowRezGameState {
    fixed_update_count: u32,
    camera: camera::CameraHandle,
    v_camera: camera::CameraHandle,
    block_tex_array: texture_array::TextureArray,
    render_texture: texture::Texture,
    screen_model: model::Model,
    chunk_model: model::Model,
    screen_pipeline: wgpu::RenderPipeline,
    pipeline: wgpu::RenderPipeline,
}

pub struct LowRezGame {
    fixed_update_rate: u32,
    state: Option<LowRezGameState>,
}

impl LowRezGame {
    pub fn new(fixed_update_rate: u32) -> Self {
        Self {
            fixed_update_rate,
            state: None,
        }
    }

    fn render_game(state: &LowRezGameState, handle: &mut render_handle::RenderHandle) {
        let (mut render_pass, camera) = handle.begin_render_pass(
            state.v_camera,
            wgpu::Color {
                r: 1.0,
                g: 0.5,
                b: 0.0,
                a: 1.0,
            },
            Some(&state.render_texture),
        );
        render_pass.set_pipeline(&state.pipeline);
        render_pass.set_bind_group(0, state.block_tex_array.bind_group(), &[]);
        render_pass.set_bind_group(1, camera.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, state.chunk_model.vertices().slice(..));
        render_pass.set_index_buffer(
            state.chunk_model.indices().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..state.chunk_model.num_indices(), 0, 0..1);
    }

    fn render_screen(state: &LowRezGameState, handle: &mut render_handle::RenderHandle) {
        let (mut render_pass, camera) = handle.begin_render_pass(
            state.camera,
            wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            None,
        );
        render_pass.set_pipeline(&state.screen_pipeline);
        render_pass.set_bind_group(0, state.render_texture.bind_group().unwrap(), &[]);
        render_pass.set_bind_group(1, camera.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, state.screen_model.vertices().slice(..));
        render_pass.set_index_buffer(
            state.screen_model.indices().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..state.screen_model.num_indices(), 0, 0..1);
    }
}

impl game::Game for LowRezGame {
    fn start(&mut self, handle: &mut engine_handle::EngineHandle) {
        let camera = handle.create_camera(
            (0.0, 0.0, 2.0).into(),
            (0.0, 0.0, 0.0).into(),
            cgmath::Vector3::unit_y(),
            Box::new(camera::OrthographicProjection {
                width: 1.0,
                height: 1.0,
                fixed_aspect_ratio: false,
                z_near: 0.1,
                z_far: 100.0,
            }),
            None,
            None,
        );

        let cam_start_offset = (3.5, 4.25);

        let v_camera = handle.create_camera(
            (cam_start_offset.0, 8.0, 4.0 + cam_start_offset.1).into(),
            (cam_start_offset.0, 0.0, cam_start_offset.1).into(),
            cgmath::Vector3::unit_y(),
            Box::new(camera::OrthographicProjection {
                width: 8.0,
                height: 8.0,
                fixed_aspect_ratio: true,
                z_near: 0.1,
                z_far: 100.0,
            }),
            Some(SCREEN_SIZE),
            Some(SCREEN_SIZE),
        );

        let block_textures = vec![
            handle.load_texture("grass.png"),
            handle.load_texture("dirt.png"),
            handle.load_texture("obsidian.png"),
        ];

        let block_tex_array = handle.create_texture_array(block_textures);

        let render_texture = handle.create_texture(
            SCREEN_SIZE,
            SCREEN_SIZE,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
            Some("render_texture"),
        );

        let mut chunk = chunk::Chunk::new(8, 2, 11, 2);
        chunk.generate(&mut rand::thread_rng());

        let chunk_model_data = mesh_generator::generate_mesh_data(&chunk);
        let chunk_model = handle.create_model(
            chunk_model_data.vertices.as_slice(),
            chunk_model_data.indices.as_slice(),
        );

        self.state = Some(LowRezGameState {
            fixed_update_count: 0,
            camera,
            v_camera,
            screen_pipeline: handle.create_pipeline(
                "shader.wgsl",
                &[render_texture.bind_group_layout().unwrap()],
                Some(camera),
            ),
            pipeline: handle.create_pipeline(
                "shader.wgsl",
                &[block_tex_array.bind_group_layout()],
                Some(camera),
            ),
            screen_model: handle.create_model(SCREEN_VERTICES, SCREEN_INDICES),
            block_tex_array,
            render_texture,
            chunk_model,
        });
    }

    fn fixed_update(&mut self, input: &input::Input, handle: &mut engine_handle::EngineHandle) {
        if let Some(state) = &mut self.state {
            state.fixed_update_count = state.fixed_update_count.overflowing_add(1).0;

            let camera = handle.get_camera(state.v_camera);

            if state.fixed_update_count % 2 == 0 {
                use winit::event::VirtualKeyCode;

                let mut dir_x = 0;

                if input.is_key_held(VirtualKeyCode::Left) {
                    dir_x = -1;
                }

                if input.is_key_held(VirtualKeyCode::Right) {
                    dir_x = 1;
                }

                let speed = 8.0 / 64.0;

                camera.pan(dir_x as f32 * speed, 0.0, 0.0);
            }
        }
    }

    fn render(&mut self, handle: &mut render_handle::RenderHandle) {
        if let Some(state) = &mut self.state {
            LowRezGame::render_game(state, handle);
            LowRezGame::render_screen(state, handle);
        }
    }

    fn get_fixed_update_rate(&self) -> u32 {
        self.fixed_update_rate
    }
}
