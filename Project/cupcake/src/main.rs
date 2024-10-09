mod camera;
mod colour;
mod constants;
mod physics_engine;
mod pixel;
mod pixel_handler;
mod terrain_generator;
mod vec2;
mod window;

use camera::Camera;
use constants::{HEIGHT, WIDTH};
use minifb;
use physics_engine::{physics::Free, simulation_type};
use pixel::Pixel;
use terrain_generator::new_terrain;
use vec2::Vec2;

fn main() {
    let mut window = minifb::Window::new(
        "cupcake",
        constants::WIDTH,
        constants::HEIGHT,
        minifb::WindowOptions {
            scale: minifb::Scale::X4,
            ..minifb::WindowOptions::default()
        },
    )
    .unwrap();
    let mut pixels = new_terrain(WIDTH - 1, HEIGHT - 1);
    while window.is_open() {
        window::update_window(&mut window, Camera::new(Vec2::new(0.0, 0.0)), &pixels);
    }
}
