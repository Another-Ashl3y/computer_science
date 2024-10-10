use minifb::Window;

use crate::engine::camera::Camera;
use crate::engine::pixel::Pixel;
use crate::physics_engine::physics_handler::apply_forces;
use crate::physics_engine::vec2::Vec2;
use crate::render_engine::window::update_window;

#[derive(Clone)]
pub struct PixelHander {
    pub pixels: Vec<Pixel>,
}

impl PixelHander {
    pub fn new() -> Self {
        Self { pixels: Vec::new() }
    }
    pub fn push_pixels(&mut self, pixels: Vec<Pixel>) {
        self.pixels.extend(pixels);
    }
    pub fn apply_gravity(&mut self, gravity: f64) {
        self.pixels
            .iter_mut()
            .for_each(|i| i.apply_force(Vec2::new(0.0, gravity)));
    }

    pub fn draw(&self, window: &mut Window, camera: &Camera) {
        update_window(window, camera, &self.pixels);
    }

    pub fn update(&mut self) {
        self.apply_gravity(9.8);
        apply_forces(&mut self.pixels);
    }
}
