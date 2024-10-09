use crate::vec2::Vec2;

pub struct Camera {
    pub position: Vec2,
}

impl Camera {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}
