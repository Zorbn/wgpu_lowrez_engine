use crate::{
    game::{
        voxels::{blocks, chunk},
        horizontal_point::HorizontalPoint,
        lowrez_game
    },
    engine::{engine_handle, instance},
};
use cgmath::prelude::*;

pub struct Entity {
    pub pos: HorizontalPoint<f32>,
    pub instance: instance::Instance,
}

const PADDED_SPRITE_WIDTH: f32 = lowrez_game::SPRITE_HALF_WIDTH - 0.01;

impl Entity {
    pub fn new(x: f32, z: f32, tex_index: i32) -> Entity {
        Entity {
            pos: HorizontalPoint::new(x, z),
            instance: instance::Instance {
                position: cgmath::Vector3::new(x, 0.5 + lowrez_game::SPRITE_HALF_HEIGHT, z),
                rotation: cgmath::Quaternion::one(),
                tex_index,
            },
        }
    }

    pub fn move_x(&mut self, amount: f32, chunks: &[chunk::Chunk]) {
        self.pos.x += self
            .get_max_movement(HorizontalPoint::new(amount, 0.0), chunks)
            .x;
        self.instance.position.x = lowrez_game::LowRezGame::round_to_pixel(self.pos.x);
    }

    pub fn move_z(&mut self, amount: f32, chunks: &[chunk::Chunk]) {
        self.pos.z += self
            .get_max_movement(HorizontalPoint::new(0.0, amount), chunks)
            .z;
        self.instance.position.z = self.pos.z;
    }

    fn get_max_movement(
        &self,
        movement: HorizontalPoint<f32>,
        chunks: &[chunk::Chunk],
    ) -> HorizontalPoint<f32> {
        let moved_pos = HorizontalPoint::new(self.pos.x + movement.x, self.pos.z + movement.z);

        if let Some(col) = Self::check_block_collisions(moved_pos, chunks) {
            HorizontalPoint::new(
                col.x as f32 - movement.x.signum() - self.pos.x,
                col.z as f32 - movement.z.signum() - self.pos.z,
            )
        } else {
            movement
        }
    }

    fn check_block_collisions(
        pos: HorizontalPoint<f32>,
        chunks: &[chunk::Chunk],
    ) -> Option<HorizontalPoint<i32>> {
        for i in 0..4 {
            let x_corner = (i % 2) * 2 - 1;
            let z_corner = (i >> 1) * 2 - 1;

            let block_x =
                (pos.x + lowrez_game::SPRITE_HALF_WIDTH + (x_corner as f32) * PADDED_SPRITE_WIDTH)
                    .floor() as i32;
            let block_z =
                (pos.z + lowrez_game::SPRITE_HALF_WIDTH + (z_corner as f32) * PADDED_SPRITE_WIDTH)
                    .floor() as i32;

            let chunk_i = ((block_x >> 3) % 2) as usize;

            if chunks[chunk_i].get_block(block_x % 8, 1, block_z) != blocks::Blocks::AIR {
                return Some(HorizontalPoint::new(block_x, block_z));
            }
        }

        None
    }

    pub fn create_instance_buffer(
        entities: &Vec<Entity>,
        handle: &mut engine_handle::EngineHandle,
    ) -> wgpu::Buffer {
        let instances = entities
            .iter()
            .map(|e| e.instance.to_raw())
            .collect::<Vec<_>>();
        handle.create_instance_buffer_from_raw(&instances)
    }
}
