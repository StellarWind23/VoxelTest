use godot::prelude::*;

struct VoxCore;

#[gdextension]
unsafe impl ExtensionLibrary for VoxCore {}

mod sstr;