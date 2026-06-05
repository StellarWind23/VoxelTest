use std::collections::HashMap;

use godot::prelude::*;

use crate::{gdres::GdRes, identifier::Id, resdb::ResourceDB};

#[derive(GodotClass, Debug, Default)]
#[class(base=RefCounted, init)]
pub struct Registry {
    hash: Option<u32>,
    id_to_object: HashMap<Id, GdRes>
}

impl Registry {

    pub fn int_register(&mut self, id: Id, entry: Gd<Resource>) -> Result<(), &str>{

        let entry_hash = ResourceDB::compute_hash(&entry);

        //Handle first
        if self.hash.is_none() {
            self.hash = Some(entry_hash)
        }

        //Validate type
        if let Some(expected) = self.hash {
            if entry_hash != expected {
                return Err("type mismatch!")
            }
        }

        //Insert if not already there
        if !self.id_to_object.contains_key(&id) {
            self.id_to_object.insert(id, GdRes::new(entry_hash, entry));
            Ok(())
        } else {
            Err("entry already exists!")
        }
    }

    pub fn int_lookup(&self, id: &Id) -> Option<&GdRes> {
        match self.id_to_object.get(id) {
            Some(r) => Some(r),
            None => None
        }
    }
}

#[godot_api]
impl Registry {

    #[func]
    fn new() -> Gd<Registry> {
        Gd::from_object(Registry { hash: None, id_to_object: HashMap::default() })
    }

    #[func]
    fn register(&mut self, id: Gd<Id>, entry: Gd<Resource>) -> bool {
        let id_ref = id.bind();
        match self.int_register(id_ref.clone(), entry) {
            Ok(_) => true,
            Err(e) => {
                godot_error!("Failed to register {}: {}", id_ref.int_as_string(), e);
                false
            }
        }
    }

    #[func]
    fn contains(&self, id: Gd<Id>) -> bool {
        let id_ref = id.bind();
        return self.id_to_object.contains_key(&*id_ref)
    }

    #[func]
    fn lookup(&self, id: Gd<Id>) -> Option<Gd<Resource>> {
        let id_ref = id.bind();
        match self.int_lookup(&*id_ref) {
            Some(v) => { return Some(v.get_resource().clone());},
            None => { None }
        }
    }
}