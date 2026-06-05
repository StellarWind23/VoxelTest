use godot::prelude::*;
use std::ops::*;
use crate::sbyte::SByte;

#[derive(GodotClass, GodotConvert, Debug)]
#[class(base=RefCounted, init)]
#[godot(transparent)]
pub struct Byte {
    v: u8
}

impl Byte {

    pub fn int_add(&self, other: &Byte) -> Byte {
        Byte {
            v: self.v.add(other.v),
        }
    }

    pub fn int_add_wrap(&self, other: &Byte) -> Byte {
        Byte {
            v: self.v.wrapping_add(other.v),
        }
    }

    pub fn int_sub(&self, other: &Byte) -> Byte {
        Byte {
            v: self.v.sub(other.v),
        }
    }

    pub fn int_sub_wrap(&self, other: &Byte) -> Byte {
        Byte {
            v: self.v.wrapping_sub(other.v),
        }
    }

    pub fn int_and(&self, other: &Byte) -> Byte {
        Byte { v: self.v.bitand(other.v) }
    }

    pub fn int_or(&self, other: &Byte) -> Byte {
        Byte { v: self.v.bitor(other.v) }
    }

    pub fn int_xor(&self, other: &Byte) -> Byte {
        Byte { v: self.v.bitxor(other.v) }
    }

    pub fn int_not(&self) -> Byte {
        Byte { v: self.v.not() }
    }

    pub fn int_shl(&self, n: u32) -> Byte {
        Byte { v: self.v.shl(n) }
    }

    pub fn int_shr(&self, n: u32) -> Byte {
        Byte { v: self.v.shr(n) }
    }

    pub fn int_get_bit(&self, bit: u8) -> bool {
        (self.v >> bit) & 1 != 0
    }

    pub fn int_set_bit(&self, bit: u8, val: bool) -> Byte {
        if val {
            Byte { v: self.v | 1 << bit}
        } else {
            Byte { v: self.v & !(1 << bit)}
        }
    }
}

#[godot_api]
impl Byte {

    #[func]
    pub fn of(v: u8) -> Gd<Byte> {
        Gd::from_object(Byte { v })
    }

    #[func]
    pub fn v(&self) -> u8 { self.v }

    #[func]
    pub fn set(&mut self, v: u8) {
        self.v = v;
    }

    #[func]
    pub fn to_string(&self) -> GString {
        GString::from(&format!("{}", self.v))
    }

    #[func]
    pub fn to_sbyte(&self) -> Gd<SByte> {
        SByte::of(self.v as i8)
    }

    // ---- Arithmetic ops ----

    #[func]
    pub fn add(&self, other: Gd<Byte>) -> Gd<Byte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add(&*other_ref))
    }

    #[func]
    pub fn add_wrap(&self, other: Gd<Byte>) -> Gd<Byte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add_wrap(&*other_ref))
    }

    #[func]
    pub fn sub(&self, other: Gd<Byte>) -> Gd<Byte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub(&*other_ref))
    }

    #[func]
    pub fn sub_wrap(&self, other: Gd<Byte>) -> Gd<Byte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub_wrap(&*other_ref))
    }

    // ---- Bitwise ops ----

    #[func]
    pub fn and(&self, other: Gd<Byte>) -> Gd<Byte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_and(&*other_ref))
    }

    #[func]
    pub fn or(&self, other: Gd<Byte>) -> Gd<Byte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_or(&*other_ref))
    }

    #[func]
    pub fn xor(&self, other: Gd<Byte>) -> Gd<Byte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_xor(&*other_ref))
    }

    #[func]
    pub fn not(&self) -> Gd<Byte> {
        Gd::from_object(self.int_not())
    }

    #[func]
    pub fn shl(&self, n: u32) -> Gd<Byte> {
        Gd::from_object(self.int_shl(n))
    }

    #[func]
    pub fn shr(&self, n: u32) -> Gd<Byte> {
        Gd::from_object(self.int_shr(n))
    }

    #[func]
    pub fn get_bit(&self, bit: u32) ->  bool {
        self.int_get_bit(bit as u8)
    }

    #[func]
    pub fn set_bit(&self, bit: u32, val: bool) -> Gd<Byte> {
        Gd::from_object(self.int_set_bit(bit as u8, val))
    }
}
