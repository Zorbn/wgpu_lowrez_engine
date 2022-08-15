use crate::engine::{
    camera, engine_handle, game, input, instance, model, render_handle, texture, texture_array,
    vertex,
};
use crate::game::voxels::blocks;
use crate::game::{
    entity,
    voxels::{chunk, mesh_generator},
};
use cgmath::prelude::*;

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

pub const SPRITE_HALF_HEIGHT: f32 = 1.14;
pub const SPRITE_HALF_WIDTH: f32 = 0.5;
const SPRITE_VERTICES: &[vertex::Vertex] = &[
    vertex::Vertex {
        position: [-SPRITE_HALF_WIDTH, SPRITE_HALF_HEIGHT, 0.0],
        tex_coords: [0.0, 0.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
    vertex::Vertex {
        position: [-SPRITE_HALF_WIDTH, -SPRITE_HALF_HEIGHT, 0.0],
        tex_coords: [0.0, 1.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
    vertex::Vertex {
        position: [SPRITE_HALF_WIDTH, -SPRITE_HALF_HEIGHT, 0.0],
        tex_coords: [1.0, 1.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
    vertex::Vertex {
        position: [SPRITE_HALF_WIDTH, SPRITE_HALF_HEIGHT, 0.0],
        tex_coords: [1.0, 0.0],
        tex_index: 0,
        color: [1.0, 1.0, 1.0],
    },
];

const SPRITE_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

const CAM_OFFSET: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 0.0, 4.25);
const CAM_POS_OFFSET: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 8.0, 4.0);

const SCREEN_SIZE: u32 = 64;
const BLOCK_SIZE: u32 = 8;

pub struct LowRezGameState {
    fixed_update_count: u32,
    camera: camera::CameraHandle,
    v_camera: camera::CameraHandle,
    v_camera_pos: cgmath::Vector3<f32>,
    v_camera_target: cgmath::Vector3<f32>,
    block_tex_array: texture_array::TextureArray,
    sprite_tex_array: texture_array::TextureArray,
    render_texture: texture::Texture,
    screen_model: model::Model,
    sprite_model: model::Model,
    screen_pipeline: wgpu::RenderPipeline,
    chunk_pipeline: wgpu::RenderPipeline,
    sprite_pipeline: wgpu::RenderPipeline,
    screen_instance_buffer: wgpu::Buffer,
    entities: Vec<entity::Entity>,
    entity_instance_buffer: wgpu::Buffer,
    chunks: [chunk::Chunk; 2],
    chunk_models: Vec<model::Model>,
    chunk_instances: Vec<instance::Instance>,
    chunk_instance_buffers: Vec<wgpu::Buffer>,
    chunk_entities: Vec<Vec<entity::Entity>>,
    chunk_entity_dirs: Vec<Vec<i32>>,
    pan_distance: f32,
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
        render_pass.set_pipeline(&state.chunk_pipeline);
        render_pass.set_bind_group(0, state.block_tex_array.bind_group(), &[]);
        render_pass.set_bind_group(1, camera.bind_group(), &[]);

        for i in 0..state.chunk_models.len() {
            render_pass.set_vertex_buffer(0, state.chunk_models[i].vertices().slice(..));
            render_pass.set_vertex_buffer(1, state.chunk_instance_buffers[i].slice(..));
            render_pass.set_index_buffer(
                state.chunk_models[i].indices().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..state.chunk_models[i].num_indices(), 0, 0..1);
        }

        render_pass.set_pipeline(&state.sprite_pipeline);
        render_pass.set_bind_group(0, state.sprite_tex_array.bind_group(), &[]);
        render_pass.set_bind_group(1, camera.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, state.sprite_model.vertices().slice(..));
        render_pass.set_index_buffer(
            state.sprite_model.indices().slice(..),
            wgpu::IndexFormat::Uint16,
        );

        assert_eq!(state.chunks.len(), 2);
        let num_entities = (state.entities.len()
            + state.chunk_entities[0].len()
            + state.chunk_entities[1].len()) as u32;
        render_pass.set_vertex_buffer(1, state.entity_instance_buffer.slice(..));
        render_pass.draw_indexed(0..state.screen_model.num_indices(), 0, 0..num_entities);
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
        render_pass.set_vertex_buffer(1, state.screen_instance_buffer.slice(..));
        render_pass.set_index_buffer(
            state.screen_model.indices().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..state.screen_model.num_indices(), 0, 0..1);
    }

    pub fn round_to_pixel(x: f32) -> f32 {
        (x * (BLOCK_SIZE as f32)).floor() / (BLOCK_SIZE as f32)
    }

    fn create_chunk_models(
        chunks: &[chunk::Chunk],
        handle: &mut engine_handle::EngineHandle,
    ) -> Vec<model::Model> {
        let mut models = Vec::new();

        for chunk in chunks {
            let chunk_model_data = mesh_generator::generate_mesh_data(chunk);
            let chunk_model = handle.create_model(
                chunk_model_data.vertices.as_slice(),
                chunk_model_data.indices.as_slice(),
            );

            models.push(chunk_model);
        }

        models
    }

    fn create_chunk_instance_buffers(
        chunk_instances: &Vec<instance::Instance>,
        handle: &mut engine_handle::EngineHandle,
    ) -> Vec<wgpu::Buffer> {
        chunk_instances
            .iter()
            .map(|i| handle.create_instance_buffer(&vec![i]))
            .collect()
    }

    fn create_entity_instance_buffer(
        entities: &Vec<entity::Entity>,
        chunks: &[chunk::Chunk; 2],
        chunk_entities: &Vec<Vec<entity::Entity>>,
        handle: &mut engine_handle::EngineHandle,
    ) -> wgpu::Buffer {
        let mut raw_entities = std::collections::HashMap::new();

        fn hash_depth(depth: f32, x: f32) -> i32 {
            ((depth * 1000.0).floor() + (x / (SPRITE_HALF_WIDTH * 10.0)).round()) as i32
        }

        fn add_raw_entities(
            entities: &Vec<entity::Entity>,
            raw_entities: &mut std::collections::HashMap<i32, instance::InstanceRaw>,
        ) {
            for e in entities {
                let mut z_off = 0.0;

                while raw_entities.contains_key(&hash_depth(e.pos.z + z_off, e.pos.x)) {
                    z_off += 0.001;
                }

                raw_entities.insert(
                    hash_depth(e.pos.z + z_off, e.pos.x),
                    e.instance
                        .to_raw_with_offset(cgmath::Vector3::new(0.0, 0.0, z_off)),
                );
            }
        }

        add_raw_entities(entities, &mut raw_entities);

        for i in 0..chunks.len() {
            add_raw_entities(&chunk_entities[i], &mut raw_entities);
        }

        let mut sorted_raw_entities = raw_entities.iter().collect::<Vec<_>>();
        sorted_raw_entities.sort_by_key(|i| i.0);

        let raw_instances = sorted_raw_entities.iter().map(|i| *i.1).collect::<Vec<_>>();

        handle.create_instance_buffer_from_raw(&raw_instances)
    }

    fn update_camera(v_camera_pos: &mut cgmath::Vector3<f32>, v_camera_target: &mut cgmath::Vector3<f32>, v_camera: camera::CameraHandle, handle: &mut engine_handle::EngineHandle, player_pos: cgmath::Vector3<f32>) {
        *v_camera_pos = player_pos + CAM_OFFSET + CAM_POS_OFFSET;
        *v_camera_target = player_pos + CAM_OFFSET;

        let camera = handle.get_camera(v_camera);
        camera.viewpoint.pos = *v_camera_pos;
        camera.viewpoint.target = *v_camera_target;
        camera.viewpoint.pos.x = Self::round_to_pixel(v_camera_pos.x);
        camera.viewpoint.target.x = Self::round_to_pixel(v_camera_target.x);
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

        let v_camera_pos = CAM_OFFSET + CAM_POS_OFFSET;
        let v_camera_target = CAM_OFFSET;

        let v_camera = handle.create_camera(
            v_camera_pos,
            v_camera_target,
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

        let sprite_textures = vec![
            handle.load_texture("player.png"),
            handle.load_texture("octopus.png"),
            handle.load_texture("acorn.png"),
        ];

        let sprite_tex_array = handle.create_texture_array(sprite_textures);

        let render_texture = handle.create_texture(
            SCREEN_SIZE,
            SCREEN_SIZE,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
            Some("render_texture"),
        );

        let mut chunks = [
            chunk::Chunk::new(8, 2, 11, 2),
            chunk::Chunk::new(8, 2, 11, 2),
        ];

        let chunk_instances = vec![
            instance::Instance {
                position: cgmath::Vector3::new(0.0, 0.0, 0.0),
                rotation: cgmath::Quaternion::one(),
                tex_index: 0,
            },
            instance::Instance {
                position: cgmath::Vector3::new(8.0, 0.0, 0.0),
                rotation: cgmath::Quaternion::one(),
                tex_index: 0,
            },
        ];

        let mut chunk_entities: Vec<Vec<entity::Entity>> = Vec::<Vec<_>>::new();
        chunk_entities.push(Vec::<entity::Entity>::new());
        chunk_entities.push(Vec::<entity::Entity>::new());
        let mut chunk_entity_dirs: Vec<Vec<i32>> = Vec::<Vec<_>>::new();
        chunk_entity_dirs.push(Vec::<i32>::new());
        chunk_entity_dirs.push(Vec::<i32>::new());

        chunks[0].generate(
            &mut rand::thread_rng(),
            true,
            chunk_instances[0].position.x as i32,
            &mut chunk_entities[0],
            &mut chunk_entity_dirs[0],
        );
        chunks[1].generate(
            &mut rand::thread_rng(),
            false,
            chunk_instances[1].position.x as i32,
            &mut chunk_entities[1],
            &mut chunk_entity_dirs[1],
        );

        let chunk_models = Self::create_chunk_models(&chunks, handle);

        let chunk_instance_buffers = Self::create_chunk_instance_buffers(&chunk_instances, handle);

        let screen_instances = vec![instance::Instance {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::one(),
            tex_index: 0,
        }];
        let screen_instance_buffer = handle.create_instance_buffer(&screen_instances);

        let entities = vec![entity::Entity::new(3.5, 4.5, 0)];
        let entity_instance_buffer =
            Self::create_entity_instance_buffer(&entities, &chunks, &chunk_entities, handle);

        let screen_pipeline = handle.create_pipeline(
            "shader.wgsl",
            &[render_texture.bind_group_layout().unwrap()],
            Some(camera),
        );
        let chunk_pipeline = handle.create_pipeline(
            "shader.wgsl",
            &[block_tex_array.bind_group_layout()],
            Some(camera),
        );
        let sprite_pipeline = handle.create_pipeline(
            "shader.wgsl",
            &[sprite_tex_array.bind_group_layout()],
            Some(camera),
        );

        let screen_model = handle.create_model(SCREEN_VERTICES, SCREEN_INDICES);
        let sprite_model = handle.create_model(SPRITE_VERTICES, SPRITE_INDICES);

        self.state = Some(LowRezGameState {
            fixed_update_count: 0,
            pan_distance: 0.0,
            chunk_entities,
            chunk_entity_dirs,
            camera,
            v_camera,
            v_camera_pos,
            v_camera_target,
            screen_pipeline,
            chunk_pipeline,
            sprite_pipeline,
            screen_model,
            sprite_model,
            block_tex_array,
            sprite_tex_array,
            render_texture,
            screen_instance_buffer,
            entities,
            entity_instance_buffer,
            chunks,
            chunk_models,
            chunk_instances,
            chunk_instance_buffers,
        });
    }

    fn fixed_update(&mut self, input: &input::Input, handle: &mut engine_handle::EngineHandle) {
        if let Some(state) = &mut self.state {
            use winit::event::VirtualKeyCode;

            state.fixed_update_count = state.fixed_update_count.overflowing_add(1).0;

            let mut dir_x = 0;
            let mut dir_z = 0;

            if input.is_key_held(VirtualKeyCode::Left) {
                dir_x = -1;
            }

            if input.is_key_held(VirtualKeyCode::Right) {
                dir_x = 1;
            }

            if input.is_key_held(VirtualKeyCode::Down) {
                dir_z = 1;
            }

            if input.is_key_held(VirtualKeyCode::Up) {
                dir_z = -1;
            }

            let speed = 4.0 / 64.0;
            let move_x = dir_x as f32 * speed;

            if state.entities[0].pos.x + move_x > state.v_camera_pos.x - 3.5 {
                state.entities[0].move_x(move_x, &state.chunks);
            }

            state.entities[0].move_z(dir_z as f32 * speed, &state.chunks);

            let player_pos_vec = cgmath::Vector3::new(state.entities[0].pos.x, 0.0, 0.0);

            if player_pos_vec.x > state.v_camera_target.x {
                state.pan_distance += move_x;

                if state.pan_distance >= 8.0 {
                    state.pan_distance = 0.0;
                    let last_chunk_i = (((state.entities[0].pos.x as i32 >> 3) + 1) % 2) as usize;
                    state.chunk_instances[last_chunk_i].position.x += 16.0;

                    let LowRezGameState {
                        chunks,
                        chunk_entities,
                        chunk_entity_dirs,
                        ..
                    } = state;
                    chunks[last_chunk_i].generate(
                        &mut rand::thread_rng(),
                        false,
                        state.chunk_instances[last_chunk_i].position.x as i32,
                        &mut chunk_entities[last_chunk_i],
                        &mut chunk_entity_dirs[last_chunk_i],
                    );

                    state.chunk_instance_buffers =
                        Self::create_chunk_instance_buffers(&state.chunk_instances, handle);
                    state.chunk_models = Self::create_chunk_models(&state.chunks, handle);
                }

                Self::update_camera(&mut state.v_camera_pos, &mut state.v_camera_target, state.v_camera, handle, player_pos_vec);
            }

            if input.was_key_pressed(VirtualKeyCode::Space) {
                state.entities.push(entity::Entity::new(
                    state.entities[0].pos.x,
                    state.entities[0].pos.z,
                    2,
                ));
            }

            let LowRezGameState {
                chunks,
                chunk_entities,
                chunk_entity_dirs,
                ..
            } = state;

            // Check bullet collisions.
            for i in (1..state.entities.len()).rev() {
                if let Some(hit) =
                    entity::Entity::check_entity_collisions(state.entities[i].pos, &chunk_entities)
                {
                    chunk_entities[hit.0].remove(hit.1);
                    chunk_entity_dirs[hit.0].remove(hit.1);
                    state.entities.remove(i);
                    break;
                }

                if !state.entities[i].move_x(0.2, chunks) {
                    let hit_block_x =
                        (state.entities[i].pos.x + SPRITE_HALF_WIDTH * 2.0).floor() as i32;

                    for hit_z_corner in 0..3 {
                        let norm_hit_z_corner = (hit_z_corner - 1) as f32;
                        let hit_block_z = (state.entities[i].pos.z
                            + SPRITE_HALF_WIDTH + norm_hit_z_corner * entity::PADDED_SPRITE_WIDTH)
                            .floor() as i32;
                        let hit_chunk = ((hit_block_x >> 3) % 2) as usize;

                        if chunks[hit_chunk].get_block(hit_block_x % 8, 1, hit_block_z)
                            != blocks::Blocks::AIR
                        {
                            chunks[hit_chunk].set_block(
                                blocks::Blocks::AIR,
                                hit_block_x % 8,
                                1,
                                hit_block_z,
                            );
                            state.chunk_instance_buffers =
                                Self::create_chunk_instance_buffers(&state.chunk_instances, handle);
                            state.chunk_models = Self::create_chunk_models(chunks, handle);
                            break;
                        }
                    }

                    state.entities.remove(i);
                } else {
                    if state.entities[i].pos.x - 8.0 > state.entities[0].pos.x {
                        state.entities.remove(i);
                    }
                }
            }

            for ci in 0..chunks.len() {
                for i in 0..chunk_entities[ci].len() {
                    if !chunk_entities[ci][i].move_z(chunk_entity_dirs[ci][i] as f32 * 0.1, chunks) {
                        chunk_entity_dirs[ci][i] *= -1;
                    }
                }
            }

            // Check player collisions.
            if let Some(_) = entity::Entity::check_entity_collisions(state.entities[0].pos, &chunk_entities) {
                state.chunk_instances[0].position = cgmath::Vector3::new(0.0, 0.0, 0.0);
                state.chunk_instances[1].position = cgmath::Vector3::new(8.0, 0.0, 0.0);

                chunks[0].generate(
                    &mut rand::thread_rng(),
                    true,
                    state.chunk_instances[0].position.x as i32,
                    &mut chunk_entities[0],
                    &mut chunk_entity_dirs[0],
                );

                chunks[1].generate(
                    &mut rand::thread_rng(),
                    false,
                    state.chunk_instances[1].position.x as i32,
                    &mut chunk_entities[1],
                    &mut chunk_entity_dirs[1],
                );

                state.entities[0].pos.x = 3.5;
                state.entities[0].pos.z = 4.5;
                state.pan_distance = 0.0;

                let player_pos_vec = cgmath::Vector3::new(state.entities[0].pos.x, 0.0, 0.0);
                Self::update_camera(&mut state.v_camera_pos, &mut state.v_camera_target, state.v_camera, handle, player_pos_vec);

                state.chunk_instance_buffers =
                    Self::create_chunk_instance_buffers(&state.chunk_instances, handle);
                state.chunk_models = Self::create_chunk_models(chunks, handle);
            }

            state.entity_instance_buffer = Self::create_entity_instance_buffer(
                &state.entities,
                &chunks,
                &chunk_entities,
                handle,
            );
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
