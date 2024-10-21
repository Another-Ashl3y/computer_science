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

pub struct WidgetOptions {
    x: f64,
    y: f64,
}

#[derive(Clone)]
pub enum Widget {}
impl Widget {
    pub fn update(&self) {}
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
