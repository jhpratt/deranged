use crate::Sign;
use std::convert::TryInto;

pub(crate) struct Number {
    pub(crate) sign: Sign,
    pub(crate) value: u128,
}

macro_rules! to_unsigned {
    ($($name:ident($type:ty))*) => {$(
        pub(crate) fn $name(&self) -> Option<$type> {
            if self.sign == Sign::Positive {
                self.value.try_into().ok()
            } else {
                None
            }
        }
    )*};
}

macro_rules! to_signed {
    ($($name:ident($type:ty))*) => {$(
        pub(crate) fn $name(&self) -> Option<$type> {
            let Self { sign, value } = self;

            if sign == &Sign::Positive && *value <= <$type>::MAX as u128 {
                Some(*value as $type)
            } else if sign == &Sign::Negative && *value <= <$type>::MIN.wrapping_abs() as u128 {
                Some((*value as $type).wrapping_neg())
            } else {
                None
            }
        }
    )*};
}

impl Number {
    to_unsigned! {
        to_u8(u8)
        to_u16(u16)
        to_u32(u32)
        to_u64(u64)
        to_u128(u128)
    }

    to_signed! {
        to_i8(i8)
        to_i16(i16)
        to_i32(i32)
        to_i64(i64)
        to_i128(i128)
    }
}
