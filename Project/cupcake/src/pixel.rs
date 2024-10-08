use crate::colour::{Colour, DisplayColour};
use crate::physics_engine::physics::Physics;
use crate::simulation_type::SimulationType;
use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub position: Vec2,
    pub physics: Physics,
    pub z: usize,
    pub colour: Colour,
    pub simulation_type: SimulationType,
}

impl Pixel {
    pub fn new(
        position: Vec2,
        physics: Physics,
        z: usize,
        colour: Colour,
        simulation_type: SimulationType,
    ) -> Self {
        Self {
            position,
            physics,
            z,
            colour,
            simulation_type,
        }
    }
    pub fn display_colour(&self) -> DisplayColour {
        DisplayColour::new(self.colour, self.z)
    }
}
