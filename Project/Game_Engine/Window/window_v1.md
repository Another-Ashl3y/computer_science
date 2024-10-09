# window.rs

The `window.rs` file only has `methods` as it is an add on to the `Window` struct in minifb. 
There is a private method `get_colours` which turns the supplied `&Vec<Pixels>` into the correct `DisplayColour`s which are to be drawn.
The public method used for drawing to the window is `update_window`. It takes the `Window` object from the minifb crate and updates it with a buffer after transforming the result of `get_colours` into a 1D buffer.


