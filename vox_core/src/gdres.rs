
use godot::prelude::*;
use godot::classes::Resource;


#[derive(GodotClass, Debug, Default)]
#[class(base=Resource, init)]
pub struct GdRes {
    hash: u64,
    resource: Gd<Resource>
}

impl GdRes {

    pub fn new(hash: u64, resource: Gd<Resource>) -> GdRes {
       GdRes { hash, resource }
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }

    pub fn get_resource(&self) -> &Gd<Resource> {
        &self.resource
    }
}