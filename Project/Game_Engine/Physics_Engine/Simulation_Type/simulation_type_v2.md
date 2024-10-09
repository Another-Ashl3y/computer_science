# simulation_type.rs

The `SimulationType` struct has been updated to remove the `physics` field from `Pixel` and only apply it to `Free` and `Glue`.

```rust
#[derive(Clone)]
pub enum SimulationType {
    Static,
    Free(Free),
    Glued(Glue),
}
```
