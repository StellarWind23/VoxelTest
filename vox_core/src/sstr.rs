use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
struct SStr {
    s_str: PackedByteArray
}

pub impl SStr {
    fn init(str: String) -> Self {
        SStr { s_str: PackedByteArray::new() }
    }

    fn encode(str: String) {
        let c_arr = str.chars();

        for c in c_arr {
            
        }
    }
}