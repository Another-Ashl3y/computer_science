use crate::engine::pixel::Pixel;
use crate::physics_engine::physics::{Free, Glue};
use crate::physics_engine::simulation_type::SimulationType;
use crate::physics_engine::vec2::Vec2;

pub fn apply_forces(pixels: &mut Vec<Pixel>) {
    for pixel in pixels {
        let simulation_type: SimulationType = pixel.simulation_type.clone();
        match simulation_type {
            SimulationType::Free(f) => pixel.position = pixel.position + f.velocity,
            SimulationType::Glued(g) => pixel.position = pixel.position + g.free.velocity,
            SimulationType::Static => (),
        }
    }
}

pub fn apply_force(pixels: &mut Vec<Pixel>, force: Vec2) {
    for pixel in pixels {
        match pixel.simulation_type {
            SimulationType::Free(ref mut f) => f.velocity = f.velocity + force,
            SimulationType::Glued(ref mut g) => g.free.velocity = g.free.velocity + force,
            SimulationType::Static => (),
        }
    }
}
