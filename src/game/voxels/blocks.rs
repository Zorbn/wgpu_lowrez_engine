use rand::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Blocks {
    AIR,
    GRASS,
    DIRT,
}

pub struct Chunk {
    width: u32,
    height: u32,
    depth: u32,
    blocks: Vec<Blocks>,
}

impl Chunk {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Chunk {
            width,
            height,
            depth,
            blocks: vec![Blocks::AIR; (width * height * depth) as usize],
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        let block_count = self.width * self.height * self.depth;
        for i in 0..block_count {
            let (x, y, z) = Self::get_block_xyz(self.width, self.height, i as usize);
            let block = match rng.gen_range(0..3) {
                1 => Blocks::GRASS,
                2 => Blocks::DIRT,
                _ => Blocks::AIR,
            };
            self.set_block(block, x, y, z);
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

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Blocks {
        if !self.is_block_in_bounds(x, y, z) {
            return Blocks::AIR;
        }

        self.blocks[Self::get_block_i(self.width, self.height, x, y, z)]
    }

    pub fn set_block(&mut self, block: Blocks, x: i32, y: i32, z: i32) {
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
}
