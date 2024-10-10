use crate::physics_engine::vec2::Vec2;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 144;
pub const DIMENSIONS: Vec2 = Vec2 {
    x: WIDTH as f64,
    y: HEIGHT as f64,
};
