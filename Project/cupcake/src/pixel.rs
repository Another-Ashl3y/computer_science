use crate::colour::{Colour, DisplayColour};
use crate::simulation_type::SimulationType;
use crate::vec2::Vec2;

#[derive(Clone)]
pub struct Pixel {
    pub position: Vec2,
    pub z: usize,
    pub colour: Colour,
    pub simulation_type: SimulationType,
}

impl Pixel {
    pub fn new(position: Vec2, z: usize, colour: Colour, simulation_type: SimulationType) -> Self {
        Self {
            position,
            z,
            colour,
            simulation_type,
        }
    }
    pub fn display_colour(&self) -> DisplayColour {
        DisplayColour::new(self.colour, self.z)
    }
    pub fn apply_force(&mut self, force: Vec2) {
        self.simulation_type.apply_force(force);
    }
}