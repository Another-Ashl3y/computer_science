use crate::physics_engine::vec2::Vec2;

use super::physics::{Free, Glue};

#[derive(Clone)]
pub enum SimulationType {
    Static,
    Free(Free),
    Glued(Glue),
}

impl SimulationType {
    pub fn apply_force(&mut self, force: Vec2) {
        match self {
            SimulationType::Free(p) => p.velocity = p.velocity + force,
            SimulationType::Glued(p) => todo!(),
            SimulationType::Static => (),
        }
    }
}
