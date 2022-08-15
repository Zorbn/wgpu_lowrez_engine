use crate::{
    engine::instance,
    game::{
        horizontal_point::HorizontalPoint,
        lowrez_game,
        voxels::{blocks, chunk},
    },
};
use cgmath::prelude::*;

pub struct Entity {
    pub pos: HorizontalPoint<f32>,
    pub instance: instance::Instance,
}

pub const PADDED_SPRITE_WIDTH: f32 = lowrez_game::SPRITE_HALF_WIDTH - 0.01;

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

    pub fn move_x(&mut self, amount: f32, chunks: &[chunk::Chunk]) -> bool {
        let max_movement = self
            .get_max_movement(HorizontalPoint::new(amount, 0.0), chunks)
            .x;

        self.pos.x += max_movement;
        self.instance.position.x = lowrez_game::LowRezGame::round_to_pixel(self.pos.x);

        max_movement != 0.0
    }

    pub fn move_z(&mut self, amount: f32, chunks: &[chunk::Chunk]) -> bool {
        let max_movement = self
            .get_max_movement(HorizontalPoint::new(0.0, amount), chunks)
            .z;

        self.pos.z += max_movement;
        self.instance.position.z = self.pos.z;

        max_movement != 0.0
    }

    pub fn check_entity_collisions(
        pos: HorizontalPoint<f32>,
        chunk_entities: &Vec<Vec<Entity>>,
    ) -> Option<(usize, usize)> {
        for i in 0..4 {
            let corner_pos = Self::get_corner_positions(pos, i);
            let block_x = corner_pos.x.floor() as i32;
            let chunk_i = ((block_x >> 3) % 2) as usize;

            for ei in 0..chunk_entities[chunk_i].len() {
                let e = &chunk_entities[chunk_i][ei];
                let mut collided = true;

                for j in 0..4 {
                    let e_x_corner = (j % 2) * 2 - 1;
                    let e_z_corner = (j >> 1) * 2 - 1;

                    let e_corner_pos_x = e.pos.x
                        + lowrez_game::SPRITE_HALF_WIDTH
                        + (e_x_corner as f32) * PADDED_SPRITE_WIDTH;
                    let e_corner_pos_z = e.pos.z
                        + lowrez_game::SPRITE_HALF_WIDTH
                        + (e_z_corner as f32) * PADDED_SPRITE_WIDTH;

                    if (e_x_corner == -1 && corner_pos.x < e_corner_pos_x)
                        || (e_z_corner == -1 && corner_pos.z < e_corner_pos_z)
                    {
                        collided = false;
                        break;
                    }

                    if (e_x_corner == 1 && corner_pos.x > e_corner_pos_x)
                        || (e_z_corner == 1 && corner_pos.z > e_corner_pos_z)
                    {
                        collided = false;
                        break;
                    }
                }

                if collided {
                    return Some((chunk_i, ei));
                }
            }
        }

        None
    }

    fn get_corner_positions(pos: HorizontalPoint<f32>, i: i32) -> HorizontalPoint<f32> {
        let x_corner = (i % 2) * 2 - 1;
        let z_corner = (i >> 1) * 2 - 1;

        let corner_pos_x =
            pos.x + lowrez_game::SPRITE_HALF_WIDTH + (x_corner as f32) * PADDED_SPRITE_WIDTH;
        let corner_pos_z =
            pos.z + lowrez_game::SPRITE_HALF_WIDTH + (z_corner as f32) * PADDED_SPRITE_WIDTH;

        HorizontalPoint::new(corner_pos_x, corner_pos_z)
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
            let corner_pos = Self::get_corner_positions(pos, i);
            let block_x = corner_pos.x.floor() as i32;
            let block_z = corner_pos.z.floor() as i32;

            let chunk_i = ((block_x >> 3) % 2) as usize;

            if chunks[chunk_i].get_block(block_x % 8, 1, block_z) != blocks::Blocks::AIR {
                return Some(HorizontalPoint::new(block_x, block_z));
            }
        }

        None
    }
}
