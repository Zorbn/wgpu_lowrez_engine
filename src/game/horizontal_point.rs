#[derive(Copy, Clone)]
pub struct HorizontalPoint<T> {
    pub x: T,
    pub z: T,
}

impl<T> HorizontalPoint<T> {
    pub fn new(x: T, z: T) -> Self {
        Self { x, z }
    }
}
