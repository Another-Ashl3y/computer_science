use noise::{NoiseFn, OpenSimplex};

use crate::engine::pixel::Pixel;
use crate::physics_engine::physics::Free;
use crate::physics_engine::simulation_type::SimulationType::Static;
use crate::physics_engine::vec2::Vec2;
use crate::render_engine::colour::Colour;

const FRONT_COLOUR: Colour = Colour {
    r: 77,
    g: 38,
    b: 4,
    a: 255,
};
const MIDDLE_COLOUR: Colour = Colour {
    r: 90,
    g: 50,
    b: 10,
    a: 255,
};
const BACK_COLOUR: Colour = Colour {
    r: 50,
    g: 20,
    b: 4,
    a: 255,
};

pub fn new_terrain(width: usize, height: usize) -> Vec<Pixel> {
    let mut output: Vec<Pixel> = Vec::new();
    let perlin = OpenSimplex::new(42069);
    for x in 0..width {
        for y in 0..height {
            let val = perlin.get([x as f64 * 0.02, y as f64 * 0.02]);
            println!("{:?}", val);
            if val > 0.2 {
                output.push(Pixel::new(
                    Vec2::new(x as f64, y as f64),
                    3,
                    FRONT_COLOUR,
                    Static,
                ));
            } else if val > 0.0 {
                output.push(Pixel::new(
                    Vec2::new(x as f64, y as f64),
                    3,
                    MIDDLE_COLOUR,
                    Static,
                ));
            } else if val > -0.2 {
                output.push(Pixel::new(
                    Vec2::new(x as f64, y as f64),
                    3,
                    BACK_COLOUR,
                    crate::physics_engine::simulation_type::SimulationType::Free(Free::new(
                        Vec2::ZERO(),
                        1.0,
                    )),
                ))
            }
        }
    }
    output
}
