use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Physics {
    velocity: Vec2,
    mass: f64,
}

impl Physics {
    pub fn new(velocity: Vec2, mass: f64) -> Self {
        Self { velocity, mass }
    }
}
