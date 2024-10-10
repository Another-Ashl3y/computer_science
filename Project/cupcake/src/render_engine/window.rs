use crate::engine::camera::Camera;
use crate::engine::constants::*;
use crate::engine::pixel::Pixel;
use crate::render_engine::colour::DisplayColour;
use minifb::Window;

fn get_colours(camera: &Camera, pixels: &Vec<Pixel>) -> [[DisplayColour; WIDTH]; HEIGHT] {
    let mut output: [[DisplayColour; WIDTH]; HEIGHT] = [[DisplayColour::black(); WIDTH]; HEIGHT];

    for pixel in pixels {
        // Make sure pixel is not out of bounds.
        if (pixel.position.x < camera.position.x || pixel.position.y < camera.position.y)
            || (pixel.position.x >= camera.position.x + DIMENSIONS.x
                || pixel.position.y >= camera.position.y + DIMENSIONS.y)
        {
            continue;
        }

        let output_position = pixel.position - camera.position;
        let x = output_position.x as usize;
        let y = output_position.y as usize;
        let other = output[y][x];

        let display_colour = pixel.display_colour();

        if other.z > display_colour.z {
            output[y][x] = other.blend_with(display_colour);
        } else {
            output[y][x] = display_colour.blend_with(other);
        }
    }

    output
}

pub fn update_window(window: &mut Window, camera: &Camera, pixels: &Vec<Pixel>) {
    let colours = get_colours(camera, pixels);

    let mut buffer: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = colours[y][x].colour.u32_colour();
        }
    }

    let result = window.update_with_buffer(&buffer, WIDTH, HEIGHT);
    if let Err(e) = result {
        println!("{}", e);
    }
}
