use godot::prelude::*;
use std::ops::{Add, Sub};
use crate::{sbyte::SByte, vec3b::Vec3b};

#[derive(GodotClass, Debug, Clone, Copy, PartialEq, Eq)]
#[class(base=RefCounted, init)]
pub struct Vec3sb {
    x: i8,
    y: i8,
    z: i8
}

impl Vec3sb {

    pub fn int_add(&self, other: &Vec3sb) -> Vec3sb {
        Vec3sb {
            x: self.x.add(other.x),
            y: self.y.add(other.y),
            z: self.z.add(other.z),
        }
    }

    pub fn int_add_wrap(&self, other: &Vec3sb) -> Vec3sb {
        Vec3sb {
            x: self.x.wrapping_add(other.x),
            y: self.y.wrapping_add(other.y),
            z: self.z.wrapping_add(other.z),
        }
    }

    pub fn int_sub(&self, other: &Vec3sb) -> Vec3sb {
        Vec3sb {
            x: self.x.sub(other.x),
            y: self.y.sub(other.y),
            z: self.z.sub(other.z),
        }
    }

    pub fn int_sub_wrap(&self, other: &Vec3sb) -> Vec3sb {
        Vec3sb {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y),
            z: self.z.wrapping_sub(other.z),
        }
    }
}

#[godot_api]
impl Vec3sb {

    #[func]
    pub fn of(x: i8, y: i8, z: i8) -> Gd<Vec3sb> {
        Gd::from_object(Vec3sb { x, y, z})
    }

    #[func]
    pub fn of_sbytes(x: Gd<SByte>, y: Gd<SByte>, z: Gd<SByte>) -> Gd<Vec3sb> {
        let x_ref = x.bind().v();
        let y_ref = y.bind().v();
        let z_ref = z.bind().v();
        Gd::from_object(Vec3sb {x: x_ref, y: y_ref, z: z_ref})
    }

    #[func]
    pub fn x(&self) -> i8 { self.x }

    #[func]
    pub fn y(&self) -> i8 { self.y }

    #[func]
    pub fn z(&self) -> i8 { self.z }

    #[func]
    pub fn set(&mut self, x: i8, y: i8, z: i8) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    #[func]
    pub fn to_string(&self) -> GString {
        format!("{},{},{}", self.x, self.y, self.z).into()
    }

    #[func]
    pub fn to_u8(&self) -> Gd<Vec3b> {
        Vec3b::of(self.x as u8, self.y as u8, self.z as u8)
    }

    #[func]
    pub fn add(&self, other: Gd<Vec3sb>) -> Gd<Vec3sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add(&*other_ref))
    }

    #[func]
    pub fn add_wrap(&self, other: Gd<Vec3sb>) -> Gd<Vec3sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add_wrap(&*other_ref))
    }

    #[func]
    pub fn sub(&self, other: Gd<Vec3sb>) -> Gd<Vec3sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub(&*other_ref))
    }

    #[func]
    pub fn sub_wrap(&self, other: Gd<Vec3sb>) -> Gd<Vec3sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub_wrap(&*other_ref))
    }
}
