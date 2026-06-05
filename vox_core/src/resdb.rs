use std::collections::HashMap;
use std::hash::{Hash};
use godot::{classes::ClassDb, prelude::*};
use hash32::FnvHasher;
use hash32::Hasher;


#[derive(GodotClass, Debug, Default)]
#[class(base=Resource, init)]
pub struct ResourceDB {
    names: HashMap<String, u32>,
    types: HashMap<u32, HashMap<String, VariantType>>,
}

impl ResourceDB {

    pub fn int_insert(&mut self, resource: &Gd<Resource>) -> Option<u32> {
        //Gather
        let name = resource.get_class();
        let n_str = name.to_string();
        let types = Self::get_types(&name);
        let hash = Self::int_compute_hash(&n_str, &types);
        
        //Insert
        if !self.names.contains_key(&n_str) && self.types.contains_key(&hash) {

            self.names.insert(n_str, hash);
            self.types.insert(hash, types);

            Some(hash)
        } else {

            None
        }
    }

    pub fn int_lookup(&self, name: &str) -> Option<&u32> {
        self.names.get(name)
    }

    pub fn compute_hash(resource: &Gd<Resource>) -> u32 {
        let name = resource.get_class();
        let types = Self::get_types(&name);
        Self::int_compute_hash(&name.to_string(), &types)
    }

    fn int_compute_hash(name: &str, types: &HashMap<String, VariantType>) -> u32 {
        let mut hasher: FnvHasher = Default::default();
        name.hash(&mut hasher);

        let mut keys: Vec<&String> = types.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(variant_type) = types.get(key) {
                key.hash(&mut hasher);
                variant_type.ord().hash(&mut hasher);
            }
        }

        hasher.finish32()
    }

    pub fn get_types(class_name: &GString) -> HashMap<String, VariantType> {
        let class_db = ClassDb::singleton();
        let prop_list: Array<Dictionary<Variant, Variant>> = class_db.class_get_property_list(&StringName::from(class_name));
        let mut types: HashMap<String, VariantType> = HashMap::new();
        
        for i in 0..prop_list.len() {
            if let Some(prop_dict) = prop_list.get(i) {
                let name = prop_dict.get("name").map(|v| v.to::<String>());
                let type_val = prop_dict.get("type").map(|v| v.to::<i32>());
                
                if let (Some(n), Some(t)) = (name, type_val) {
                    types.insert(n, VariantType::from_ord(t));
                }
            }
        }
        types
    }
}

#[godot_api]
impl ResourceDB {

    #[func]
    fn new() -> Gd<ResourceDB> {
        Gd::from_object(ResourceDB::default())
    }

    #[func]
    pub fn insert(&mut self, resource: Gd<Resource>) -> u32{
        let r = self.int_insert(&resource).unwrap_or(0);
        r
    }

    #[func]
    pub fn lookup(&self, name: GString) -> u32 {
        let r = self.int_lookup(&name.to_string()).unwrap_or(&0);
        *r
    }
}