# simulation_type.rs

## SimulationType

This file contains a `SimulationType` with 3 variations: `Static`, `Free` and `Glued`.

`Static` means that the pixel cannot move, it has collision but cannot be effected by any forces.
`Free` means that the pixel is not bound to anything but it experiences gravity and collision.
`Glued` means it sticks to another pixel. They are bound together and any forces that apply to it also effect the one that it is bound to.

