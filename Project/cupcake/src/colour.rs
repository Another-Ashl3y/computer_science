#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn u32_colour(&self) -> u32 {
        let (r, g, b) = (self.r as u32, self.g as u32, self.b as u32);
        (r << 16) | (g << 8) | b
    }
    pub fn blend_with(&self, below: Self) -> Self {
        if self.a == 255 {
            return *self;
        }
        let (r, g, b, a) = (self.r as f64, self.g as f64, self.b as f64, self.a as f64);
        let (br, bg, bb) = (below.r as f64, below.g as f64, below.b as f64);
        Self::new(
            ((r * a / 255.0 + br) * 0.5).clamp(0.0, 255.0) as u8,
            ((g * a / 255.0 + bg) * 0.5).clamp(0.0, 255.0) as u8,
            ((b * a / 255.0 + bb) * 0.5).clamp(0.0, 255.0) as u8,
            self.a.max(below.a),
        )
    }
}

#[derive(Clone, Copy)]
pub struct DisplayColour {
    pub colour: Colour,
    pub z: usize,
}
impl DisplayColour {
    pub fn new(colour: Colour, z: usize) -> Self {
        Self { colour, z }
    }
    pub fn blend_with(&self, other: Self) -> Self {
        if self.z < other.z {
            return Self::new(other.colour.blend_with(self.colour), other.z);
        }
        Self::new(self.colour.blend_with(other.colour), self.z)
    }
    pub fn black() -> Self {
        Self::new(Colour::new(0, 0, 0, 255), 0)
    }
}
