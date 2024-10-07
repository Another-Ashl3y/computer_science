# Game Engine

For my project I have decided to make a game engine. This is because it is a large scale, deep project as mentioned in my `Starting A New Project` file.

The stakeholders will be game developers online and some friends in class. People online will be more critical and I can get faster feedback with friends, however, getting feedback from a wider range of people online will create better feedback.

The bulk of the program will be written in rust as "Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages".

## Overview

The game engine is similar to the falling everything engine which sadly isn't available for public use, however, it's an incredibly cool game engine where each pixel is simulated. This is quite an innovative idea. Public game engines today don't have this feature so it would be useful for many people to make interesting games.

## The design process

| Components of the program |
| ------------------------- |
| Name      | Function      |
| --------- | ------------- |
| Renderer  | Display pixels on the screen |
| Physics Engine | Simulate the pixels |
| Built-in language | Allow developers to create their own code in the product |
| Art editor | Allow developers to create art in the program |
| UI | Allow the consumer to interact with the program |

Aside from the UI which will be developed in almost every stage except the physics engine, these features will be created in this order:

1. The Renderer
2. The Physics Engine
3. The Art Editor
4. The Built-In Language

## Explanation of each component

### The Renderer

  The Renderer will be a program that takes pixels and draws them to the screen. The program will be 2D so this feature isn't as performance-critical. The main display of the program will be a 16:9 resolution scaled from `256x144` pixels (may be change later). The pixels have to be limited as a higher number of pixels will decrease performance.
  The Renderer will take a `Pixel` struct which will have an `x`, `y` and `z` component to justify its position. If two values are in the same position then the `z` value will be compared. A higher `z` value means it takes priority. If the `z` value is the same then the first one to come in will be drawn.
  There will also be a camera object that defines the offset of the pixels.

Example:
```rust
fn get_window(camera: Camera, pixels: Vec<Pixel>) -> [[Pixel; WIDTH]; HEIGHT] {

    // The window with the pixel data that will be returned.
    let mut window: [[Pixel; WIDTH]; HEIGHT] = [[Pixel::Empty; WIDTH]; HEIGHT];

    for pixel in pixels {

        // Don't process the pixel if it's off-screen
        if
            pixel.x < camera.x || pixel.x > camera.x + WIDTH  // Pixel is off-screen on the x-axis
         || pixel.y < camera.y || pixel.y > camera.y + HEIGHT // Pixel is off-screen on the y-axis
        {
            continue;
        }

        // Grab the pixel in that position
        let pixel: Pixel = window[pixel.y][pixel.x];

        match pixel {
            // Immediately replace the pixel if it's empty
            Pixel::Empty => window[pixel.y][pixel.x] = pixel,
            // If the pixel is solid, compare the z values.
            Pixel::Solid(other) => {
                if other.z >= self.z {
                    window[pixel.y][pixel.x] = pixel;
                }
            }
            _=>todo!(),
        }
    }
}
```

  The renderer will also have to draw to a minifb window as that is the window handling crate we are using. The minifb crate uses a one dimensional buffer of u32 types so we first have to convert the pixels into that. Then we update the window.

```rust
fn update_window(window: Window, pixels: Vec<Pixel>) {

    // First we get the pixels that need to be displayed
    let display: [[Pixel; WIDTH]; HEIGHT] = get_window(pixels);

    let mut buffer: [u32; WIDTH*HEIGHT] = [0; WIDTH*HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match display[y][x] {
                // Change the pixel to be the colour we want
                Pixel::Solid(pixel) => buffer[y*WIDTH + x] = pixel.colour.to_u32();
            }
        }
    }

    window.update_with_buffer(&buffer, WIDTH*HEIGHT);
}
```






