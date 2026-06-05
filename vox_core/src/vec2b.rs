use godot::prelude::*;
use std::ops::{Add, Sub};
use crate::{byte::Byte, vec2sb::Vec2sb};

#[derive(GodotClass, Debug, Clone, Copy, PartialEq, Eq)]
#[class(base=RefCounted, init)]
pub struct Vec2b {
    x: u8,
    y: u8,
}

impl Vec2b {

    pub fn int_of(x: u8, y: u8) -> Vec2b {
        Vec2b { x: x, y: y}
    } 

    fn int_add(&self, other: &Vec2b) -> Vec2b {
        Vec2b {
            x: self.x.add(other.x),
            y: self.y.add(other.y),
        }
    }

    fn int_add_wrap(&self, other: &Vec2b) -> Vec2b {
        Vec2b {
            x: self.x.wrapping_add(other.x),
            y: self.y.wrapping_add(other.y)
        }
    }

    fn int_sub(&self, other: &Vec2b) -> Vec2b {
        Vec2b {
            x: self.x.sub(other.x),
            y: self.y.sub(other.y),
        }
    }

    fn int_sub_wrap(&self, other: &Vec2b) -> Vec2b {
        Vec2b {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y)
        }
    }
}

#[godot_api]
impl Vec2b {

    #[func]
    pub fn of(x: u8, y: u8) -> Gd<Vec2b> {
        Gd::from_object(Vec2b { x, y })
    }

    #[func]
    pub fn of_bytes(x: Gd<Byte>, y: Gd<Byte>) -> Gd<Vec2b> {
        let x_ref = x.bind().v();
        let y_ref = y.bind().v();
        Gd::from_object(Vec2b {x: x_ref, y: y_ref})
    }

    #[func]
    pub fn of_byte_array(bytes: PackedByteArray) -> Gd<Vec2b> {
        Gd::from_object(Vec2b {
            x: bytes.get(0).unwrap_or_default(),
            y: bytes.get(1).unwrap_or_default()
        })
    }

    #[func]
    pub fn to_byte_array(&self) -> PackedByteArray {
       let mut pa =  PackedByteArray::new();
       pa.push(self.x);
       pa.push(self.y);
       return pa;
    }

    #[func]
    pub fn x(&self) -> u8 { self.x }

    #[func]
    pub fn y(&self) -> u8 { self.y }

    #[func]
    pub fn set(&mut self, x: u8, y: u8) {
        self.x = x;
        self.y = y;
    }

    #[func]
    pub fn to_string(&self) -> GString {
        GString::from(&format!("{},{}", self.x, self.y))
    }

    #[func]
    pub fn to_i8(&self) -> Gd<Vec2sb> {
        Vec2sb::of(self.x as i8, self.y as i8)
    }

    #[func]
    pub fn add(&self, other: Gd<Vec2b>) -> Gd<Vec2b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add(&*other_ref))
    }

    #[func]
    pub fn add_wrap(&self, other: Gd<Vec2b>) -> Gd<Vec2b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add_wrap(&*other_ref))
    }

    #[func]
    pub fn sub(&self, other: Gd<Vec2b>) -> Gd<Vec2b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub(&*other_ref))
    }

    #[func]
    pub fn sub_wrap(&self, other: Gd<Vec2b>) -> Gd<Vec2b> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub_wrap(&*other_ref))
    }
}
