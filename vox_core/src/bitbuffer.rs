use godot::prelude::*;

#[derive(GodotClass, Debug, Clone)]
#[class(base=RefCounted, init)]
pub struct BitBuffer {
    bits: Vec<u8>,
    len: i32
}

#[inline]
fn idx(index: i32) -> usize {
    index as usize
}

impl BitBuffer {

    pub fn int_len(&self) -> i32 {
        return self.len
    }

    pub fn int_empty(&self) -> bool {
        return self.len <= 0;
    }

    pub fn int_as_bytes(&self) -> &[u8] {
        &self.bits
    }

    pub fn int_from_bytes(bytes: Vec<u8>, bit_len: i32) -> Self {
        BitBuffer { bits: bytes, len: bit_len }
    }

    pub fn int_get_bit(&self, index: i32) -> Option<bool> {
        if index >= self.len {
            return None;
        }
        let byte_index = idx(index / 8);
        let bit_index = index % 8;
        Some((self.bits[byte_index] >> bit_index) & 1 == 1)
    }

    pub fn int_set_bit(&mut self, index: i32, value: bool) -> bool {
        if index >= self.len {
            return false;
        }
        let byte_index = idx(index / 8);
        let bit_index = index % 8;
        if value {
            self.bits[byte_index] |= 1 << bit_index;
        } else {
            self.bits[byte_index] &= !(1 << bit_index);
        }
        true
    }

    pub fn int_push_bit(&mut self, value: bool) {
        let byte_index = idx(self.len / 8);
        let bit_index = self.len % 8;

        if byte_index >= self.bits.len() {
            self.bits.push(0);
        }

        if value {
            self.bits[byte_index] |= 1 << bit_index;
        }
        self.len += 1;
    }

    pub fn int_insert_bit(&mut self, index: i32, value: bool) -> bool {
        if index > self.len {
            return false;
        }

        self.int_push_bit(false);

        for i in (index..self.len - 1).rev() {
            let bit = self.int_get_bit(i).unwrap();
            self.int_set_bit(i + 1, bit);
        }

        self.int_set_bit(index, value);
        true
    }

    pub fn int_get_bits(&self, index: i32, len: i32) -> Option<i32> {
        if index + len > self.len || len > 64 {
            return None;
        }

        let mut value: i32 = 0;
        for i in 0..len {
            if self.int_get_bit(index + i)? {
                value |= 1 << i;
            }
        }
        Some(value)
    }

    pub fn int_set_bits(&mut self, index: i32, len: i32, mut value: i32) -> bool {
        if index + len > self.len || len > 64 {
            return false;
        }

        for i in 0..len {
            let bit = (value & 1) != 0;
            self.int_set_bit(index + i, bit);
            value >>= 1;
        }
        true
    }

    pub fn int_push_bits(&mut self, len: i32, value: i32) -> bool {
        if len > 64 {
            return false;
        }

        for i in 0..len {
            let bit = (value >> i) & 1 != 0;
            self.int_push_bit(bit);
        }
        true
    }

    pub fn int_insert_bits(&mut self, index: i32, len: i32, mut value: i32) -> bool {
        if index > self.len || len == 0 || len > 64 {
            return false;
        }

        // Step 1: push len zero bits at the end to make space
        for _ in 0..len {
            self.int_push_bit(false);
        }

        // Step 2: shift existing bits right to open the segment
        for i in (index..self.len - len).rev() {
            let bit = self.int_get_bit(i).unwrap();
            self.int_set_bit(i + len, bit);
        }

        // Step 3: set new bits from value
        for i in 0..len {
            let bit = (value & 1) != 0;
            self.int_set_bit(index + i, bit);
            value >>= 1;
        }

        true
    }

    pub fn int_remove_bits(&mut self, index: i32, len: i32) -> bool {
        if len == 0 || index >= self.len || index + len > self.len {
            return false;
        }

        // Shift remaining bits left to fill the gap
        for i in index..self.len - len {
            let bit = self.int_get_bit(i + len).unwrap();
            self.int_set_bit(i, bit);
        }

        // Reduce total length
        self.len -= len;

        // Remove full trailing bytes that are now unused
        let new_bytes = ((self.len + 7) / 8) as usize;
        self.bits.truncate(new_bytes);

        true
    }
}

// Marker trait for Godot
impl GodotConvert for BitBuffer {
    type Via = Dictionary; // intermediate type
}

// Convert from BitBuffer -> Godot Variant
impl ToGodot for BitBuffer {
    type ToVia<'v> = Dictionary
    where
        Self: 'v;

    fn to_godot(&self) -> Self::ToVia<'_> {
        let mut dict = Dictionary::new();
        dict.set(GString::from("bits"), PackedByteArray::from(self.int_as_bytes()));
        dict.set(GString::from("len"), self.len);
        dict
    }
}


// Convert from Godot Variant -> BitBuffer
use std::convert::TryInto;

impl FromGodot for BitBuffer {
    fn try_from_godot(via: Dictionary) -> Result<Self, ConvertError> {
        // Get the "bits" field
        let bits_variant = via.get(GString::from("bits"))
            .ok_or_else(|| ConvertError::new("Missing 'bits' field in BitBuffer dictionary"))?;
        let bits_array: PackedByteArray = bits_variant.try_to()
            .map_err(|_| ConvertError::new("'bits' field must be a PackedByteArray"))?;

        // Get the "len" field
        let len_variant = via.get(GString::from("len"))
            .ok_or_else(|| ConvertError::new("Missing 'len' field in BitBuffer dictionary"))?;
        let len_i64: i64 = len_variant.try_to()
            .map_err(|_| ConvertError::new("'len' field must be an integer"))?;
        let len: i32 = len_i64.try_into()
            .map_err(|_| ConvertError::new("'len' field must be non-negative"))?;

        Ok(BitBuffer {
            bits: bits_array.to_vec(),
            len,
        })
    }
}


#[godot_api]
impl BitBuffer {

    #[func]
    pub fn new() -> Gd<BitBuffer> {
        Gd::from_object(BitBuffer { bits: Vec::new(), len: 0 })
    }

    #[func]
    pub fn with_len(len: i32) -> Gd<BitBuffer> {
        Gd::from_object(BitBuffer { bits: Vec::new(), len })
    }

    #[func]
    pub fn len(&self) -> i32 {
        self.int_len()
    }

    #[func]
    pub fn is_empty(&self) -> bool {
        self.int_empty()
    }

    #[func]
    pub fn of(data: PackedByteArray, bit_len: i32) -> Gd<BitBuffer> {
        let vec: Vec<u8> = data.to_vec();
        Gd::from_object(BitBuffer::int_from_bytes(vec, bit_len))
    }

    #[func]
    pub fn to_bytes(&self) -> PackedByteArray {
        PackedByteArray::from(self.int_as_bytes().to_vec())
    }

    #[func]
    pub fn get(&self, index: i32) -> bool {
        self.int_get_bit(index).unwrap_or(false)
    }

    #[func]
    pub fn set(&mut self, index: i32, value: bool) {
        self.int_set_bit(index, value);
    }

    #[func]
    pub fn push(&mut self, value: bool) {
        self.int_push_bit(value);
    }

    #[func]
    pub fn insert(&mut self, index: i32, value: bool) {
        self.int_insert_bit(index, value);
    }

    #[func]
    pub fn get_bits(&self, index: i32, len: i32) -> i32 {
        self.int_get_bits(index, len).unwrap_or(0)
    }

    #[func]
    pub fn set_bits(&mut self, index: i32, len: i32, value: i32) -> bool {
        self.int_set_bits(index, len, value)
    }

    #[func]
    pub fn push_bits(&mut self, value: i32, len: i32) -> bool {
        self.int_push_bits(len, value)
    }

    #[func]
    pub fn insert_bits(&mut self, index: i32, value: i32, len: i32) -> bool {
        self.int_insert_bits(index, len, value)
    }

    #[func]
    pub fn remove_bits(&mut self, index: i32, len: i32) -> bool {
        self.int_remove_bits(index, len)
    }
}
