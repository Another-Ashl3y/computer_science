# window.rs

There was a bug with the initial window where it would draw pixels off screen. This was because I was checking if the pixel position was less than the camera position or greater than the camera position plus the dimensions of the screen using `<` and `>=` on the vectors which was defined as `x1 > x2 && y1 > y2` which meant it would only stop drawing objects if they were out of bounds of both axis instead of either of them. Because of this the check was changed to:
```rust
        // Make sure pixel is not out of bounds.
        if (pixel.position.x < camera.position.x || pixel.position.y < camera.position.y)
            || (pixel.position.x >= camera.position.x + DIMENSIONS.x
                || pixel.position.y >= camera.position.y + DIMENSIONS.y)
        {
            continue;
        }
```


