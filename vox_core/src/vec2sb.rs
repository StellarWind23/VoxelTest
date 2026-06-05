use godot::prelude::*;
use std::ops::{Add, Sub};
use crate::{sbyte::SByte, vec2b::Vec2b};

#[derive(GodotClass, Debug, Clone, Copy, PartialEq, Eq)]
#[class(base=RefCounted, init)]
pub struct Vec2sb {
    x: i8,
    y: i8,
}

impl Vec2sb {

    fn int_add(&self, other: &Vec2sb) -> Vec2sb {
        Vec2sb {
            x: self.x.add(other.x),
            y: self.y.add(other.y),
        }
    }

    fn int_add_wrap(&self, other: &Vec2sb) -> Vec2sb {
        Vec2sb {
            x: self.x.wrapping_add(other.x),
            y: self.y.wrapping_add(other.y)
        }
    }

    fn int_sub(&self, other: &Vec2sb) -> Vec2sb {
        Vec2sb {
            x: self.x.sub(other.x),
            y: self.y.sub(other.y),
        }
    }

    fn int_sub_wrap(&self, other: &Vec2sb) -> Vec2sb {
        Vec2sb {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y)
        }
    }
}

#[godot_api]
impl Vec2sb {

    #[func]
    pub fn of(x: i8, y: i8) -> Gd<Vec2sb> {
        Gd::from_object(Vec2sb { x, y })
    }

    #[func]
    pub fn of_sbytes(x: Gd<SByte>, y: Gd<SByte>) -> Gd<Vec2sb> {
        let x_ref = x.bind().v();
        let y_ref = y.bind().v();
        Gd::from_object(Vec2sb {x: x_ref, y: y_ref})
    }

    #[func]
    pub fn x(&self) -> i8 { self.x }

    #[func]
    pub fn y(&self) -> i8 { self.y }

    #[func]
    pub fn set(&mut self, x: i8, y: i8) {
        self.x = x;
        self.y = y;
    }

    #[func]
    pub fn to_string(&self) -> GString {
        GString::from(&format!("{},{}", self.x, self.y))
    }

    #[func]
    pub fn to_u8(&self) -> Gd<Vec2b> {
        Vec2b::of(self.x as u8, self.y as u8)
    }

    #[func]
    pub fn add(&self, other: Gd<Vec2sb>) -> Gd<Vec2sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add(&*other_ref))
    }

    #[func]
    pub fn add_wrap(&self, other: Gd<Vec2sb>) -> Gd<Vec2sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add_wrap(&*other_ref))
    }

    #[func]
    pub fn sub(&self, other: Gd<Vec2sb>) -> Gd<Vec2sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub(&*other_ref))
    }

    #[func]
    pub fn sub_wrap(&self, other: Gd<Vec2sb>) -> Gd<Vec2sb> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub_wrap(&*other_ref))
    }
}
