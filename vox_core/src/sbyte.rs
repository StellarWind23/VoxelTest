use godot::prelude::*;
use std::ops::*;
use crate::byte::Byte;

#[derive(GodotClass, Debug)]
#[class(base=RefCounted, init)]
pub struct SByte {
    v: i8
}

impl SByte {

    pub fn int_add(&self, other: &SByte) -> SByte {
        SByte {
            v: self.v.add(other.v),
        }
    }

    pub fn int_add_wrap(&self, other: &SByte) -> SByte {
        SByte {
            v: self.v.wrapping_add(other.v),
        }
    }

    pub fn int_sub(&self, other: &SByte) -> SByte {
        SByte {
            v: self.v.sub(other.v),
        }
    }

    pub fn int_sub_wrap(&self, other: &SByte) -> SByte {
        SByte {
            v: self.v.wrapping_sub(other.v),
        }
    }

    pub fn int_and(&self, other: &SByte) -> SByte {
        SByte { v: self.v.bitand(other.v) }
    }

    pub fn int_or(&self, other: &SByte) -> SByte {
        SByte { v: self.v.bitor(other.v) }
    }

    pub fn int_xor(&self, other: &SByte) -> SByte {
        SByte { v: self.v.bitxor(other.v) }
    }

    pub fn int_not(&self) -> SByte {
        SByte { v: self.v.not() }
    }

    pub fn int_shl(&self, n: u32) -> SByte {
        SByte { v: self.v.shl(n) }
    }

    pub fn int_shr(&self, n: u32) -> SByte {
        SByte { v: self.v.shr(n) }
    }

    pub fn int_get_bit(&self, bit: u8) -> bool {
        (self.v >> bit) & 1 != 0
    }

    pub fn int_set_bit(&self, bit: u8, val: bool) -> SByte {
        if val {
            SByte { v: self.v | 1 << bit}
        } else {
            SByte { v: self.v & !(1 << bit)}
        }
    }
}

impl GodotConvert for SByte{
    type Via = i8;
}

impl ToGodot for SByte {
    type ToVia<'v> = i8;

    fn to_godot(&self) -> Self::ToVia<'_> {
        return self.v
    }
}

impl FromGodot for SByte {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        return Ok(SByte { v: via });
    }
}

#[godot_api]
impl SByte {

    #[func]
    pub fn of(v: i8) -> Gd<SByte> {
        Gd::from_object(SByte { v })
    }

    #[func]
    pub fn v(&self) -> i8 { self.v }

    #[func]
    pub fn set(&mut self, v: i8) {
        self.v = v;
    }

    #[func]
    pub fn to_string(&self) -> GString {
        format!("{}", self.v).into()
    }

    #[func]
    pub fn to_byte(&self) -> Gd<Byte> {
        Byte::of(self.v as u8)
    }

    // ---- Arithmetic ops ----

    #[func]
    pub fn add(&self, other: Gd<SByte>) -> Gd<SByte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add(&*other_ref))
    }

    #[func]
    pub fn add_wrap(&self, other: Gd<SByte>) -> Gd<SByte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_add_wrap(&*other_ref))
    }

    #[func]
    pub fn sub(&self, other: Gd<SByte>) -> Gd<SByte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub(&*other_ref))
    }

    #[func]
    pub fn sub_wrap(&self, other: Gd<SByte>) -> Gd<SByte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_sub_wrap(&*other_ref))
    }

    // ---- Bitwise ops ----

    #[func]
    pub fn and(&self, other: Gd<SByte>) -> Gd<SByte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_and(&*other_ref))
    }

    #[func]
    pub fn or(&self, other: Gd<SByte>) -> Gd<SByte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_or(&*other_ref))
    }

    #[func]
    pub fn xor(&self, other: Gd<SByte>) -> Gd<SByte> {
        let other_ref = other.bind();
        Gd::from_object(self.int_xor(&*other_ref))
    }

    #[func]
    pub fn not(&self) -> Gd<SByte> {
        Gd::from_object(self.int_not())
    }

    #[func]
    pub fn shl(&self, n: u32) -> Gd<SByte> {
        Gd::from_object(self.int_shl(n))
    }

    #[func]
    pub fn shr(&self, n: u32) -> Gd<SByte> {
        Gd::from_object(self.int_shr(n))
    }

    #[func]
    pub fn get_bit(&self, bit: u32) ->  bool {
        self.int_get_bit(bit as u8)
    }

    #[func]
    pub fn set_bit(&self, bit: u32, val: bool) -> Gd<SByte> {
        Gd::from_object(self.int_set_bit(bit as u8, val))
    }
}
