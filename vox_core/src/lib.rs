use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl VoxCore for MyExtension {}
