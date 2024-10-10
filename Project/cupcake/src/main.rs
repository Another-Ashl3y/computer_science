mod engine;
mod physics_engine;
mod render_engine;

use crate::engine::camera::Camera;
use crate::engine::constants::{HEIGHT, WIDTH};
use crate::engine::terrain_generator::new_terrain;
use crate::physics_engine::simulation_type::SimulationType::Static;
use crate::physics_engine::vec2::Vec2;
use crate::render_engine::colour::Colour;
use crate::render_engine::window::update_window;

use crate::engine::pixel_handler::PixelHander;
use minifb::{Scale, Window, WindowOptions};

fn main() {
    let mut window = Window::new(
        "cupcake",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..minifb::WindowOptions::default()
        },
    )
    .unwrap();

    let terrain = new_terrain(WIDTH, HEIGHT);

    let mut pixels = PixelHander::new();
    pixels.push_pixels(terrain);

    let mut camera = Camera::new(Vec2::new(0.0, 0.0));

    while window.is_open() {
        camera.position.x += 0.01;
        pixels.draw(&mut window, &camera);
        pixels.update();
    }
}
