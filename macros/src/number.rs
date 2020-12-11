use crate::Sign;
use std::convert::TryInto;

pub(crate) struct Number {
    pub(crate) sign: Sign,
    pub(crate) value: u128,
}

impl Number {
    pub(crate) fn to_u8(&self) -> Option<u8> {
        if self.sign == Sign::Positive {
            self.value.try_into().ok()
        } else {
            None
        }
    }

    pub(crate) fn to_u16(&self) -> Option<u16> {
        if self.sign == Sign::Positive {
            self.value.try_into().ok()
        } else {
            None
        }
    }

    pub(crate) fn to_u32(&self) -> Option<u32> {
        if self.sign == Sign::Positive {
            self.value.try_into().ok()
        } else {
            None
        }
    }

    pub(crate) fn to_u64(&self) -> Option<u64> {
        if self.sign == Sign::Positive {
            self.value.try_into().ok()
        } else {
            None
        }
    }

    pub(crate) fn to_u128(&self) -> Option<u128> {
        if self.sign == Sign::Positive {
            Some(self.value)
        } else {
            None
        }
    }

    pub(crate) fn to_i8(&self) -> Option<i8> {
        if self.sign == Sign::Positive && self.value <= i8::MAX as u128 {
            Some(self.value as i8)
        } else if self.sign == Sign::Negative && self.value <= i8::MIN.wrapping_abs() as u128 {
            Some((self.value as i8).wrapping_neg())
        } else {
            None
        }
    }

    pub(crate) fn to_i16(&self) -> Option<i16> {
        if self.sign == Sign::Positive && self.value <= i16::MAX as u128 {
            Some(self.value as i16)
        } else if self.sign == Sign::Negative && self.value <= i16::MIN.wrapping_abs() as u128 {
            Some((self.value as i16).wrapping_neg())
        } else {
            None
        }
    }

    pub(crate) fn to_i32(&self) -> Option<i32> {
        if self.sign == Sign::Positive && self.value <= i32::MAX as u128 {
            Some(self.value as i32)
        } else if self.sign == Sign::Negative && self.value <= i32::MIN.wrapping_abs() as u128 {
            Some((self.value as i32).wrapping_neg())
        } else {
            None
        }
    }

    pub(crate) fn to_i64(&self) -> Option<i64> {
        if self.sign == Sign::Positive && self.value <= i64::MAX as u128 {
            Some(self.value as i64)
        } else if self.sign == Sign::Negative && self.value <= i64::MIN.wrapping_abs() as u128 {
            Some((self.value as i64).wrapping_neg())
        } else {
            None
        }
    }

    pub(crate) fn to_i128(&self) -> Option<i128> {
        if self.sign == Sign::Positive && self.value <= i128::MAX as u128 {
            Some(self.value as i128)
        } else if self.sign == Sign::Negative && self.value <= i128::MIN.wrapping_abs() as u128 {
            Some((self.value as i128).wrapping_neg())
        } else {
            None
        }
    }
}
