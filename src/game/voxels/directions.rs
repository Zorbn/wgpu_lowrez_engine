#[derive(Copy, Clone)]
pub enum Directions {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Forward = 4,
    Backward = 5,
}

pub fn dir_to_offset(dir: Directions) -> (i32, i32, i32) {
    match dir {
        Directions::Up => (0, 1, 0),
        Directions::Down => (0, -1, 0),
        Directions::Left => (-1, 0, 0),
        Directions::Right => (1, 0, 0),
        Directions::Forward => (0, 0, 1),
        Directions::Backward => (0, 0, -1),
    }
}
