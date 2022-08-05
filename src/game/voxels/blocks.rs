#[derive(Copy, Clone, PartialEq)]
pub enum Blocks {
    AIR,
    GRASS,
    DIRT,
    OBSIDIAN,
}

pub const FLOOR_TILES: &[Blocks] = &[ Blocks::GRASS, Blocks::DIRT ];
pub const WALL_TILES: &[Blocks] = &[ Blocks::GRASS, Blocks::DIRT ];
pub const OBSTACLE_TILES: &[Blocks] = &[ Blocks::AIR, Blocks::GRASS, Blocks::DIRT ];
pub const BORDER_TILES: &[Blocks] = &[ Blocks::OBSIDIAN ];