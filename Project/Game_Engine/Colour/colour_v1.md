# colour.rs

A pretty big part of games these days is colour so our game engine needs a method of storing colour values. That's where are `Colour` struct and `DisplayColour` struct come in.

## Colour

The `Colour` stuct is used just to store rgba colour values and has an implementation of `blend_with` and `u32_colour`.
`u32_colour` turns the colour value into a u32 type which is needed for the minifb crate to draw the colours on the screen.
`blend_with` is used to blend two colours together and returns a new colour value.

The code for the blending script was tested in python using pygame and came to the result of [this](../../Images/colour_blend_demo.png) which was blended with the script:
```python
def blend_with(self, other):
    return Colour(
        (self.r * self.a / 255.0 + other.r)/2.0,
        (self.g * self.a / 255.0 + other.g)/2.0,
        (self.b * self.a / 255.0 + other.b)/2.0,
        max(self.a, other.a)
    )
```

## DisplayColour

The `DisplayColour` struct is used when rendering the screen. It contains a `Colour` object but also contains a `z` for working with different height pixels.
The `DisplayColour` struct also has a `black()` method which returns the background colour of the game engine.
The `blend_with` method for this one handles sorting the pixels too. On the case that the z values are the same, we just treat `self` as the top value. This is because the black pixels will be drawn first and anything else with a `z` index of the same value will be a designed pixel and should be drawn in place of the background.

