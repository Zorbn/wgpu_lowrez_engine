use crate::engine::{
    asset_handle, camera, engine_handle, game, input, model, render_handle, resource_handles,
    vertex,
};

const VERTICES: &[vertex::Vertex] = &[
    vertex::Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    },
    vertex::Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    },
    vertex::Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
    },
    vertex::Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
    },
    vertex::Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    },
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

const SCREEN_VERTICES: &[vertex::Vertex] = &[
    vertex::Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
    },
    vertex::Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
    vertex::Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
    vertex::Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    },
];

const SCREEN_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

const SCREEN_SIZE: u32 = 64;

pub struct LowRezGameState {
    fixed_update_count: u32,
    camera: resource_handles::CameraHandle,
    v_camera: resource_handles::CameraHandle,
    diffuse_texture: resource_handles::TextureHandle,
    render_texture: resource_handles::TextureHandle,
    tree_model: resource_handles::ModelHandle,
    v_screen_model: resource_handles::ModelHandle,
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

    fn render_game(
        state: &LowRezGameState,
        render_handle: &mut render_handle::RenderHandle,
        asset_handle: &asset_handle::AssetHandle,
    ) {
        let model = asset_handle.get_model(state.tree_model);
        let render_texture = asset_handle.get_texture(state.render_texture);
        let mut render_pass = render_handle.begin_render_pass(
            state.v_camera,
            wgpu::Color {
                r: 1.0,
                g: 0.5,
                b: 0.0,
                a: 1.0,
            },
            Some(render_texture),
        );
        render_pass.set_vertex_buffer(0, model.vertices().slice(..));
        render_pass.set_index_buffer(model.indices().slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..model.num_indices(), 0, 0..1);
    }

    fn render_screen(
        state: &LowRezGameState,
        render_handle: &mut render_handle::RenderHandle,
        asset_handle: &asset_handle::AssetHandle,
    ) {
        let screen_model = asset_handle.get_model(state.v_screen_model);
        let mut render_pass = render_handle.begin_render_pass(
            state.camera,
            wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            None,
        );
        render_pass.set_vertex_buffer(0, screen_model.vertices().slice(..));
        render_pass.set_index_buffer(screen_model.indices().slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..screen_model.num_indices(), 0, 0..1);
    }
}

impl game::Game for LowRezGame {
    fn start(&mut self, handle: &mut engine_handle::EngineHandle) {
        let diffuse_texture = handle.load_texture("happy-tree.png");
        let render_texture = handle.create_texture(
            SCREEN_SIZE,
            SCREEN_SIZE,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
            Some("render_texture"),
        );

        self.state = Some(LowRezGameState {
            fixed_update_count: 0,
            camera: handle.create_camera(
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
                render_texture,
                None,
                None,
            ),
            v_camera: handle.create_camera(
                (0.0, 0.0, 2.0).into(),
                (0.0, 0.0, 0.0).into(),
                cgmath::Vector3::unit_y(),
                Box::new(camera::OrthographicProjection {
                    width: 1.0,
                    height: 1.0,
                    fixed_aspect_ratio: true,
                    z_near: 0.1,
                    z_far: 100.0,
                }),
                diffuse_texture,
                Some(SCREEN_SIZE),
                Some(SCREEN_SIZE),
            ),
            tree_model: handle.create_model(VERTICES, INDICES),
            v_screen_model: handle.create_model(SCREEN_VERTICES, SCREEN_INDICES),
            diffuse_texture,
            render_texture,
        });
    }

    fn update(
        &mut self,
        input: &input::Input,
        handle: &mut engine_handle::EngineHandle,
        delta_time: f32,
    ) {
    }

    fn fixed_update(&mut self, input: &input::Input, handle: &mut engine_handle::EngineHandle) {
        if let Some(state) = &mut self.state {
            state.fixed_update_count = state.fixed_update_count.overflowing_add(1).0;

            let camera = handle.get_camera(state.v_camera);

            if state.fixed_update_count % 5 == 0 {
                camera.pan(input, 1.0 / 64.0);
            }
        }
    }

    fn render(
        &mut self,
        render_handle: &mut render_handle::RenderHandle,
        asset_handle: &asset_handle::AssetHandle,
    ) {
        if let Some(state) = &mut self.state {
            LowRezGame::render_game(state, render_handle, asset_handle);
            LowRezGame::render_screen(state, render_handle, asset_handle);
        }
    }

    fn get_fixed_update_rate(&self) -> u32 {
        self.fixed_update_rate
    }
}
