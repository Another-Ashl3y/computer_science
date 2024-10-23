use minifb::{Window, WindowOptions};

pub struct Main {
    alive: bool,
    window: Window,
    widgets: Vec<Widget>,
}

impl Main {
    pub fn new(name: &str) -> Self {
        let error_message = &format!("failed to open main window: {}", name);
        Self {
            alive: true,
            window: Window::new(
                "name",
                800,
                600,
                WindowOptions {
                    resize: true,
                    ..WindowOptions::default()
                },
            )
            .expect(error_message),
            widgets: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        let size: (usize, usize) = self.window.get_size();
        let mut buffer: Vec<u32> = vec![0_u32; size.0 * size.1];
        for widget in self.widgets.clone() {
            widget.update()
        }
        self.window
            .update_with_buffer(&buffer, size.0, size.1)
            .unwrap();
    }
}

pub struct Vec2<T> {
    x: T,
    y: T,
}

impl Vec2<f64> {
    pub fn to_usize(&self) -> Vec2<usize> {
        Vec2::new(self.x as usize, self.y as usize)
    }
}

impl Vec2<T> {
    pub fn new(x: T, y: T) -> Self<T> {
        Self { x, y }
    }
}

pub struct WidgetOptions {
    pos: Vec2<f64>,
}

#[derive(Clone)]
pub enum Widget {}
impl Widget {
    pub fn update(&self) {}
}

struct Buffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
    pub fn set(&mut self, pos: Vec2<f64>, colour: u32) {
        // convert given position to an index
        let index = pos.y * self.width as f64 + pos.x;

        // sanity check
        if index > self.buffer.len() as f64 || index < 0.0 {
            return;
        }

        // assign the index with the colour
        self.buffer[index as usize] = colour;
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_opens() {
        Main::new("Window opening test");
    }

    #[test]
    fn main() {
        let mut window = Main::new("main test");
        while window.alive {
            window.update();
        }
    }
}

// Charles is gay (he'll never admit it but he is)
