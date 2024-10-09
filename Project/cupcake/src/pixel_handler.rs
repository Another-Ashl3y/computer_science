use crate::{pixel::Pixel, vec2::Vec2};

#[derive(Clone)]
pub struct PixelHander {
    pub pixels: Vec<Pixel>,
}

impl PixelHander {
    pub fn new() -> Self {
        Self { pixels: Vec::new() }
    }
    pub fn apply_gravity(&mut self, gravity: f64) {
        self.pixels
            .iter_mut()
            .for_each(|i| i.apply_force(Vec2::new(0.0, 9.8)));
    }
}
