use crate::game::{entity, voxels::blocks};
use rand::prelude::*;

pub struct Chunk {
    width: u32,
    height: u32,
    depth: u32,
    world_height: u32,
    blocks: Vec<blocks::Blocks>,
}

impl Chunk {
    pub fn new(width: u32, height: u32, depth: u32, world_height: u32) -> Self {
        Chunk {
            width,
            height,
            depth,
            world_height,
            blocks: vec![blocks::Blocks::AIR; (width * height * depth) as usize],
        }
    }

    pub fn generate(
        &mut self,
        rng: &mut ThreadRng,
        spawn_chunk: bool,
        world_x: i32,
        entities: &mut Vec<entity::Entity>,
        entity_dirs: &mut Vec<i32>,
    ) {
        entities.clear();
        entity_dirs.clear();

        let blocks_len = self.width * self.height * self.depth;
        let lower_border_z = 0;
        let upper_border_z = self.depth as i32 - 1;
        let lower_wall_z = 1;
        let upper_wall_z = self.depth as i32 - 2;
        let floor_y = 0;

        for i in 0..blocks_len {
            let (x, y, z) = Self::get_block_xyz(self.width, self.height, i as usize);

            let block = if z == lower_border_z || z == upper_border_z {
                blocks::BORDER_TILES.choose(rng)
            } else if z == lower_wall_z || z == upper_wall_z {
                blocks::WALL_TILES.choose(rng)
            } else {
                if y == floor_y {
                    blocks::FLOOR_TILES.choose(rng)
                } else if spawn_chunk {
                    Some(&blocks::Blocks::AIR)
                } else {
                    blocks::OBSTACLE_TILES.choose(rng)
                }
            }
            .expect("Failed to pick a block while generating");

            if !spawn_chunk && *block == blocks::Blocks::AIR && rng.gen_range(0..6) == 0 {
                let enemy = entity::Entity::new((x + world_x) as f32, z as f32, 1);
                entities.push(enemy);
                entity_dirs.push(1);
            }

            self.set_block(*block, x, y, z);
        }
    }

    pub fn get_block_xyz(chunk_width: u32, chunk_height: u32, i: usize) -> (i32, i32, i32) {
        let i_u32 = i as u32;
        let x = i_u32 % chunk_width;
        let y = (i_u32 / chunk_width) % chunk_height;
        let z = i_u32 / (chunk_width * chunk_height);

        (x as i32, y as i32, z as i32)
    }

    pub fn get_block_i(chunk_width: u32, chunk_height: u32, x: i32, y: i32, z: i32) -> usize {
        ((z as u32) * chunk_width * chunk_height + (y as u32) * chunk_width + (x as u32)) as usize
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> blocks::Blocks {
        if !self.is_block_in_bounds(x, y, z) {
            return blocks::Blocks::AIR;
        }

        self.blocks[Self::get_block_i(self.width, self.height, x, y, z)]
    }

    pub fn set_block(&mut self, block: blocks::Blocks, x: i32, y: i32, z: i32) {
        if !self.is_block_in_bounds(x, y, z) {
            return;
        }

        self.blocks[Self::get_block_i(self.width, self.height, x, y, z)] = block;
    }

    fn is_block_in_bounds(&self, x: i32, y: i32, z: i32) -> bool {
        x >= 0
            && (x as u32) < self.width
            && y >= 0
            && (y as u32) < self.height
            && z >= 0
            && (z as u32) < self.depth
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn world_height(&self) -> u32 {
        self.world_height
    }
}
