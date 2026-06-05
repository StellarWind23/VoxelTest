use std::hash::Hash;

use godot::prelude::*;

#[derive(GodotClass, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[class(base=Resource, init)]
pub struct Id {
    namespace: StringName,
    path: StringName
}

impl Id {

    pub fn int_as_string(&self) -> String {
        return self.namespace.clone().to_string() + ":" + &self.path.clone().to_string();
    }
}

#[godot_api]
impl Id {

    #[func]
    fn of(namespace: GString, path: GString) -> Gd<Id>{
        return Gd::from_object(
            Id { 
                namespace: StringName::from(&namespace),
                path: StringName::from(&path)
            }
        )
    }

    #[func]
    fn namespace(&self) -> GString {
        return GString::from(&self.namespace);
    }

    #[func]
    fn path(&self) -> GString {
        return GString::from(&self.path);
    }

    #[func]
    fn as_string(&self) -> GString {
        return GString::from(&self.int_as_string())
    }

    #[func]
    fn equals(&self, other: Gd<Id>) -> bool {
        let other_ref = other.bind();
        return self == &*other_ref;
    }
}