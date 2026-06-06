use std::{collections::HashMap, hash::{DefaultHasher, Hasher}};

use godot::prelude::*;
use std::hash::{Hash};

use crate::{gdres::GdRes, identifier::Id, resdb::ResourceDB};

#[derive(GodotClass, Debug, Default)]
#[class(base=RefCounted, init)]
pub struct Registry {
    hash: Option<u32>,
    cache_hash: u64,
    id_to_object: HashMap<Id, GdRes>,
    locked: bool
}

impl Registry {

    pub fn int_register(&mut self, id: Id, entry: Gd<Resource>) -> Result<(), &str>{

        if self.locked {
            return Err("registry has been locked!")
        }

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

    pub fn int_get_type_hash(&self) -> u32 {
        match self.hash {
            Some(h) => {return h}
            None => {return 0}
        }
    }

    pub fn int_cache_hash(&mut self) -> u64 {

        //If locked don't recalculate immediately return.
        if self.locked {
            return self.cache_hash;
        }

        let mut hasher = DefaultHasher::new();

        for i in self.id_to_object.iter() {
            i.0.hash(&mut hasher);
            i.1.get_deep_hash().hash(&mut hasher);
        }

        self.cache_hash = hasher.finish();
        return self.cache_hash;
    }
    
    pub fn int_lock(&mut self) {
        self.int_cache_hash();
        self.locked = true;
    }

    pub fn int_is_locked(&self) -> bool {
        return self.locked;
    }
}

#[godot_api]
impl Registry {

    #[func]
    fn new() -> Gd<Registry> {
        Gd::from_object(Registry { hash: None, cache_hash: 0, id_to_object: HashMap::default(), locked: false })
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
        self.id_to_object.contains_key(&*id_ref)
    }

    #[func]
    fn lookup(&self, id: Gd<Id>) -> Option<Gd<Resource>> {
        let id_ref = id.bind();
        match self.int_lookup(&*id_ref) {
            Some(v) => { return Some(v.get_resource().clone())},
            None => { None }
        }
    }

    #[func]
    fn get_type_hash(&self) -> u32 {
        self.int_get_type_hash()
    }

    #[func]
    fn get_cache_hash(&mut self) -> i64 {
        self.int_cache_hash() as i64
    }

    #[func]
    fn lock(&mut self) {
        self.int_lock();
    }

    #[func]
    fn is_locked(&self) -> bool {
        self.int_is_locked()
    }
}