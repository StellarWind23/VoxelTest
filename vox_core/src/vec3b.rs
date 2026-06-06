use godot::prelude::*;
use std::ops::{Add, Sub};
use crate::{byte::Byte, vec3sb::Vec3sb};

#[derive(GodotClass, Debug, Clone, Copy, PartialEq, Eq)]
#[class(base=RefCounted, init)]
pub struct Vec3b {
    x: u8,
    y: u8,
    z: u8
}

impl Vec3b {

    pub fn int_add(&self, other: &Vec3b) -> Vec3b {
        Vec3b {
            x: self.x.add(other.x),
            y: self.y.add(other.y),
            z: self.z.add(other.z),
        }
    }

    pub fn int_add_wrap(&self, other: &Vec3b) -> Vec3b {
        Vec3b {
            x: self.x.wrapping_add(other.x),
            y: self.y.wrapping_add(other.y),
            z: self.z.wrapping_add(other.z),
        }
    }

    pub fn int_sub(&self, other: &Vec3b) -> Vec3b {
        Vec3b {
            x: self.x.sub(other.x),
            y: self.y.sub(other.y),
            z: self.z.sub(other.z),
        }
    }

    pub fn int_sub_wrap(&self, other: &Vec3b) -> Vec3b {
        Vec3b {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y),
            z: self.z.wrapping_sub(other.z),
        }
    }
}

#[godot_api]
impl Vec3b {

    #[func]
    pub fn of(x: u8, y: u8, z: u8) -> Gd<Vec3b> {
        Gd::from_object(Vec3b { x, y, z})
    }

    #[func]
    pub fn of_bytes(x: Gd<Byte>, y: Gd<Byte>, z: Gd<Byte>) -> Gd<Vec3b> {
        Gd::from_object(Vec3b {
            x: x.bind().v(),
            y: y.bind().v(),
            z: z.bind().v()
        })
    }

    #[func]
    pub fn of_byte_array(bytes: PackedByteArray) -> Gd<Vec3b> {
        Gd::from_object(Vec3b {
            x: bytes.get(0).unwrap_or_default(),
            y: bytes.get(1).unwrap_or_default(),
            z: bytes.get(2).unwrap_or_default()
        })
    }

    #[func]
    pub fn to_byte_array(&self) -> PackedByteArray {
       let mut pa =  PackedByteArray::new();
       pa.push(self.x);
       pa.push(self.y);
       pa.push(self.z);
       return pa;
    }

    #[func]
    pub fn x(&self) -> u8 { self.x }

    #[func]
    pub fn y(&self) -> u8 { self.y }

    #[func]
    pub fn z(&self) -> u8 { self.z }

    #[func]
    pub fn set(&mut self, x: u8, y: u8, z: u8) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    #[func]
    pub fn to_string(&self) -> GString {
        GString::from(&format!("{},{},{}", self.x, self.y, self.z))
    }

    #[func]
    pub fn to_i8(&self) -> Gd<Vec3sb> {
        Vec3sb::of(self.x as i8, self.y as i8, self.z as i8)
    }

    #[func]
    pub fn add(&self, other: Gd<Vec3b>) -> Gd<Vec3b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add(&*other_ref))
    }

    #[func]
    pub fn add_wrap(&self, other: Gd<Vec3b>) -> Gd<Vec3b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add_wrap(&*other_ref))
    }

    #[func]
    pub fn sub(&self, other: Gd<Vec3b>) -> Gd<Vec3b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub(&*other_ref))
    }

    #[func]
    pub fn sub_wrap(&self, other: Gd<Vec3b>) -> Gd<Vec3b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub_wrap(&*other_ref))
    }
}
