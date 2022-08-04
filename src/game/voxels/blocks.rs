#[derive(Copy, Clone)]
pub enum Blocks {
    AIR,
    GRASS,
    DIRT,
}

pub struct Chunk {
    width: usize,
    height: usize,
    depth: usize,
    blocks: Vec<Blocks>,
}

impl Chunk {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Chunk {
            width,
            height,
            depth,
            blocks: vec![Blocks::AIR; width * height * depth],
        }
    }

    pub fn get_block_xyz(
        chunk_width: usize,
        chunk_height: usize,
        i: usize,
    ) -> (usize, usize, usize) {
        let x = i % chunk_width;
        let y = (i / chunk_width) % chunk_height;
        let z = i / (chunk_width * chunk_height);

        (x, y, z)
    }

    pub fn get_block_i(
        chunk_width: usize,
        chunk_height: usize,
        x: usize,
        y: usize,
        z: usize,
    ) -> usize {
        z * chunk_width * chunk_height + y * chunk_width + x
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Blocks {
        self.blocks[Self::get_block_i(self.width, self.height, x, y, z)]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}
