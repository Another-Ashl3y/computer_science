use crate::camera::Camera;
use crate::colour::DisplayColour;
use crate::constants::*;
use crate::pixel::Pixel;
use minifb::Window;

fn get_colours(camera: Camera, pixels: Vec<Pixel>) -> [[DisplayColour; WIDTH]; HEIGHT] {
    let mut output: [[DisplayColour; WIDTH]; HEIGHT] = [[DisplayColour::black(); WIDTH]; HEIGHT];

    for pixel in pixels {
        // Make sure pixel is not out of bounds.
        if pixel.position < camera.position || pixel.position > camera.position + DIMENSIONS {
            continue;
        }

        let output_position = pixel.position - camera.position;
        let other = output[output_position.y as usize][output_position.x as usize];

        let display_colour = pixel.display_colour();

        if other.z > display_colour.z {
            output[output_position.y as usize][output_position.x as usize] =
                other.blend_with(display_colour);
        } else {
            output[output_position.y as usize][output_position.x as usize] =
                display_colour.blend_with(other);
        }
    }

    output
}

pub fn update_window(window: &mut Window, camera: Camera, pixels: Vec<Pixel>) {
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
