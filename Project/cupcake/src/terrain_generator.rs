use noise::{NoiseFn, OpenSimplex};

use crate::physics_engine::simulation_type::SimulationType::Static;
use crate::{colour::Colour, pixel::Pixel, vec2::Vec2};

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
    let perlin = OpenSimplex::new(42069);
    (0..(width * height))
        .map(|i| {
            let x = i % height;
            let y = i / width;
            let val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
            println!("{:?}", val);
            if val > 0.2 {
                return Pixel::new(Vec2::new(x as f64, y as f64), 3, FRONT_COLOUR, Static);
            } else if val > 0.0 {
                return Pixel::new(Vec2::new(x as f64, y as f64), 3, MIDDLE_COLOUR, Static);
            }
            Pixel::new(Vec2::new(x as f64, y as f64), 3, BACK_COLOUR, Static)
        })
        .collect()
}
