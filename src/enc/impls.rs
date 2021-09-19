use super::{Encode, Encodeable};
use crate::error::Error;

impl Encodeable for u8 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u8(*self)
    }
}

impl Encodeable for u16 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u16(*self)
    }
}

impl Encodeable for u32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u32(*self)
    }
}

impl Encodeable for u64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u64(*self)
    }
}

impl Encodeable for u128 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u128(*self)
    }
}

impl Encodeable for usize {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_usize(*self)
    }
}

impl Encodeable for i8 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_i8(*self)
    }
}

impl Encodeable for i16 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_i16(*self)
    }
}

impl Encodeable for i32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_i32(*self)
    }
}

impl Encodeable for i64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_i64(*self)
    }
}

impl Encodeable for i128 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_i128(*self)
    }
}

impl Encodeable for isize {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_isize(*self)
    }
}

impl Encodeable for f32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_f32(*self)
    }
}

impl Encodeable for f64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_f64(*self)
    }
}

impl Encodeable for &'_ [u8] {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_blob(*self)
    }
}

impl<'a, T> Encode for &'a mut T
where
    T: Encode,
{
    fn encode_u8(&mut self, val: u8) -> Result<(), Error> {
        T::encode_u8(self, val)
    }
    fn encode_u16(&mut self, val: u16) -> Result<(), Error> {
        T::encode_u16(self, val)
    }
    fn encode_u32(&mut self, val: u32) -> Result<(), Error> {
        T::encode_u32(self, val)
    }
    fn encode_u64(&mut self, val: u64) -> Result<(), Error> {
        T::encode_u64(self, val)
    }
    fn encode_u128(&mut self, val: u128) -> Result<(), Error> {
        T::encode_u128(self, val)
    }
    fn encode_usize(&mut self, val: usize) -> Result<(), Error> {
        T::encode_usize(self, val)
    }

    fn encode_i8(&mut self, val: i8) -> Result<(), Error> {
        T::encode_i8(self, val)
    }
    fn encode_i16(&mut self, val: i16) -> Result<(), Error> {
        T::encode_i16(self, val)
    }
    fn encode_i32(&mut self, val: i32) -> Result<(), Error> {
        T::encode_i32(self, val)
    }
    fn encode_i64(&mut self, val: i64) -> Result<(), Error> {
        T::encode_i64(self, val)
    }
    fn encode_i128(&mut self, val: i128) -> Result<(), Error> {
        T::encode_i128(self, val)
    }
    fn encode_isize(&mut self, val: isize) -> Result<(), Error> {
        T::encode_isize(self, val)
    }

    fn encode_f32(&mut self, val: f32) -> Result<(), Error> {
        T::encode_f32(self, val)
    }
    fn encode_f64(&mut self, val: f64) -> Result<(), Error> {
        T::encode_f64(self, val)
    }
    fn encode_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        T::encode_blob(self, val)
    }
}
