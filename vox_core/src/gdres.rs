
use std::hash::{DefaultHasher, Hash, Hasher};

use godot::prelude::*;
use godot::classes::Resource;


#[derive(GodotClass, Debug, Default)]
#[class(base=Resource, init)]
pub struct GdRes {
    hash: u32,
    resource: Gd<Resource>
}

impl GdRes {

    pub fn new(hash: u32, resource: Gd<Resource>) -> GdRes {
       GdRes { hash, resource }
    }

    pub fn get_hash(&self) -> u32 {
        self.hash
    }

    pub fn get_resource(&self) -> &Gd<Resource> {
        &self.resource
    }

    pub fn get_deep_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let r = &*self.resource;

        //Include type hash
        self.get_hash().hash(&mut hasher);

        //Hash names & values of things
        for p in r.get_property_list().iter_shared() {
            match p.get("name") {
                Some(name) => {
                    name.hash_u32().hash(&mut hasher);
                    //Grap value using name
                    r.get(&StringName::from(&name.to_string())).hash_u32().hash(&mut hasher);
                }
                None => continue
            }
        }

        return hasher.finish()
    }
}