use crate::{pixel::Pixel, vec2::Vec2};

#[derive(Clone, Copy)]
pub struct Free {
    pub velocity: Vec2,
    pub mass: f64,
}
impl Free {
    pub fn new(velocity: Vec2, mass: f64) -> Self {
        Self { velocity, mass }
    }
}

#[derive(Clone)]
pub struct Glue {
    pub free: Free,
    pub glued: Vec<Pixel>,
}
impl Glue {
    pub fn new(free: Free, glued: Vec<Pixel>) -> Self {
        Self { free, glued }
    }
}
