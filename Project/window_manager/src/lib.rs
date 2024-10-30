use minifb::{Window, WindowOptions};

pub struct Main {
    pub alive: bool,
    pub window: Window,
    pub widgets: Vec<Widget>,
}

impl Main {
    pub fn new(name: &str) -> Self {
        let error_message = &format!("failed to open main window: {}", name);
        Self {
            alive: true,
            window: Window::new(
                name,
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
    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }
    pub fn get_size(&self) -> (usize, usize) {
        self.window.get_size()
    }
    pub fn update(&mut self) {
        self.widgets.iter_mut().for_each(|w| w.update(&self.window));

        let size: (usize, usize) = self.window.get_size();
        let mut buffer: Buffer = Buffer::new(size.0, size.1);
        for mut widget in self.widgets.clone() {
            widget.attach(&mut buffer)
        }
        self.window
            .update_with_buffer(&buffer.get_buffer(), size.0, size.1)
            .unwrap();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub enum Widget {
    SubWindow(SubWindow),
    Bar(Bar),
}

impl Widget {
    pub fn attach(&mut self, buf: &mut Buffer) {
        match self {
            Widget::SubWindow(sub_window) => sub_window.attach(buf),
            Widget::Bar(bar) => bar.attach(buf),
        }
    }
    pub fn update(&mut self, root: &Window) {
        match self {
            Widget::Bar(bar) => bar.update(root),
            _ => (),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bar {
    pub pos: Vec2,
    pub dimensions: Vec2,
    pub title: String,
    pub buffer: Buffer,
    pub children: Vec<Widget>,
    pub grabbed: bool,
}

impl Bar {
    pub fn new(title: &str, pos: Vec2, dimensions: Vec2, children: Vec<Widget>) -> Self {
        Self {
            title: title.to_string(),
            pos,
            dimensions,
            buffer: Buffer::new(dimensions.x as usize, dimensions.y as usize),
            children,
            grabbed: false,
        }
    }
    pub fn update(&mut self, root: &Window) {
        let left_down = root.get_mouse_down(minifb::MouseButton::Left);
        if left_down {
            root.get_mouse_pos(minifb::MouseMode::Clamp).map(|mouse| {
                self.pos = Vec2::new(mouse.0 as f64, mouse.1 as f64);
            });
        }
    }
    pub fn get_pos(&self) -> (usize, usize) {
        (self.pos.x as usize, self.pos.y as usize)
    }
    pub fn get_size(&self) -> (usize, usize) {
        (self.dimensions.x as usize, self.dimensions.y as usize)
    }
    pub fn attach(&mut self, buffer: &mut Buffer) {
        //for mut child in self.widgets.clone() {
        //    child.attach(&mut self.buffer);
        //}

        // places the widgets buffer onto its parents buffer
        let pos = self.get_pos();
        let size = self.get_size();
        let own_buf = self.buffer.get_buffer();
        for i in (0..own_buf.len()).zip(own_buf) {
            // i.0 - index
            // i.1 - value
            buffer.set(
                Vec2::new(
                    (pos.0 + (i.0 % size.0)) as f64,
                    (pos.1 + (i.0 / size.0)) as f64,
                ),
                i.1,
            );
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubWindow {
    pub pos: Vec2,
    pub dimensions: Vec2,
    pub buffer: Buffer,
    pub widgets: Vec<Widget>,
}

impl SubWindow {
    pub fn new(pos: Vec2, dimensions: Vec2) -> Self {
        Self {
            pos,
            dimensions,
            buffer: Buffer::new(dimensions.x as usize, dimensions.y as usize),
            widgets: Vec::new(),
        }
    }
    pub fn get_size(&self) -> (usize, usize) {
        (self.dimensions.x as usize, self.dimensions.y as usize)
    }
    pub fn get_pos(&self) -> (usize, usize) {
        (self.pos.x as usize, self.pos.y as usize)
    }
    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }
    pub fn attach(&mut self, buf: &mut Buffer) {
        for mut child in self.widgets.clone() {
            child.attach(&mut self.buffer);
        }

        // places the widgets buffer onto its parents buffer
        let pos = self.get_pos();
        let size = self.get_size();
        let own_buf = self.buffer.get_buffer();
        for i in (0..own_buf.len()).zip(own_buf) {
            // i.0 - index
            // i.1 - value
            buf.set(
                Vec2::new(
                    (pos.0 + (i.0 % size.0)) as f64,
                    (pos.1 + (i.0 / size.0)) as f64,
                ),
                i.1,
            );
        }
    }
    pub fn set_background_colour(&mut self, r: u8, g: u8, b: u8) {
        self.buffer.set_all(from_u8_rgb(r, g, b));
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
    pub fn set_all(&mut self, colour: u32) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = colour;
        }
    }
    pub fn set(&mut self, pos: Vec2, colour: u32) {
        // convert given position to an index
        let index = pos.y * self.width as f64 + pos.x;

        // sanity check
        //if index > self.buffer.len() as f64 || index < 0.0 {
        //  println!("{}", index);
        //  return;
        //}

        // assign the index with the colour
        self.buffer[index as usize] = colour;
    }
    pub fn get_buffer(&self) -> Vec<u32> {
        self.buffer.clone()
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {
    use std::arch::x86_64::_MM_FROUND_RAISE_EXC;

    use super::*;

    #[test]
    fn window_opens() {
        Main::new("Window opening test");
    }

    #[test]
    fn window_updates() {
        let mut window = Main::new("window updating test");
        for _frame in 0..1000 {
            window.update();
        }
    }

    #[test]
    fn window_with_widgets() {
        let mut window = Main::new("window with widgets test");
        let mut widget = SubWindow::new(Vec2::new(40.0, 100.0), Vec2::new(100.0, 300.0));
        widget.set_background_colour(40, 40, 40);
        window.add_widget(Widget::SubWindow(widget));
        for _frame in 0..1000 {
            window.update();
        }
    }

    #[test]
    fn nested_widgets_test() {
        let mut window = Main::new("nested_widgets_test");
        let window_size = window.get_size();
        let mut main_widget = SubWindow::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(window_size.0 as f64, window_size.1 as f64),
        );
        let mut child_widget = SubWindow::new(Vec2::new(20.0, 60.0), Vec2::new(200.0, 300.0));
        let mut grandchild_widget = SubWindow::new(Vec2::new(10.0, 10.0), Vec2::new(50.0, 20.0));
        main_widget.set_background_colour(0, 0, 255);
        child_widget.set_background_colour(0, 255, 0);
        grandchild_widget.set_background_colour(255, 0, 0);
        child_widget.add_widget(Widget::SubWindow(grandchild_widget));
        main_widget.add_widget(Widget::SubWindow(child_widget));
        window.add_widget(Widget::SubWindow(main_widget));
        for _frame in 0..1000 {
            window.update();
        }
    }

    #[test]
    fn title_bar_test() {
        let mut window = Main::new("title_bar_test");
        let mut bar = Bar::new(
            "bar",
            Vec2::new(100.0, 100.0),
            Vec2::new(100.0, 20.0),
            Vec::new(),
        );
        bar.buffer.set_all(from_u8_rgb(255, 255, 255));
        window.add_widget(Widget::Bar(bar));
        for _frame in 0..1000 {
            window.update();
        }
    }

    #[test]
    fn index_to_2d() {
        let width = 16;
        let height = 9;
        for x in 0..width {
            for y in 0..height {
                let index = y * width + x;
                assert_eq!(x, index % width);
                assert_eq!(y, index / width)
            }
        }
    }
}

// Charles is gay (he'll never admit it but he is)
