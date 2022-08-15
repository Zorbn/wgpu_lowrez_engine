#[derive(Copy, Clone)]
pub enum Directions {
    Up = 0,
    Forward = 4,
}

pub fn dir_to_offset(dir: Directions) -> (i32, i32, i32) {
    match dir {
        Directions::Up => (0, 1, 0),
        Directions::Forward => (0, 0, 1),
    }
}
