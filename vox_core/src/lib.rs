use godot::prelude::*;

struct VoxCore;

#[gdextension]
unsafe impl ExtensionLibrary for VoxCore {}

mod sbyte;
mod byte;
mod vec2b;
mod vec2sb;
mod vec3b;
mod vec3sb;
mod registry;
mod identifier;
mod gdres;
mod resdb;