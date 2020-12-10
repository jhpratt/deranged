#![feature(min_const_generics)]
#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate
)]
#![doc(test(attr(deny(warnings))))]

use core::{
    convert::{TryFrom, TryInto},
    fmt,
};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromIntError;

impl fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("out of range integral type conversion attempted")
    }
}
impl Error for TryFromIntError {}

macro_rules! impl_ranged {
    ($(
        $type:ident($internal:ident): {
            into: [$($into:ident),* $(,)?]
            try_into: [$($try_into:ident),* $(,)?]
            try_from: [$($try_from:ident),* $(,)?]
        }
    )*) => {$(
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type<const MIN: $internal, const MAX: $internal>(
            $internal,
        );

        impl<const MIN: $internal, const MAX: $internal> fmt::Debug for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        $(impl<const MIN: $internal, const MAX: $internal> From<$type<MIN,MAX>> for $into {
            fn from(value: $type<MIN, MAX>) -> Self {
                value.0.into()
            }
        })*

        $(impl<const MIN: $internal, const MAX: $internal> TryFrom<$type<MIN, MAX>> for $try_into {
            type Error = TryFromIntError;

            fn try_from(value: $type<MIN, MAX>) -> Result<Self, Self::Error> {
                if (MIN..=MAX).contains(&value.0) {
                    Ok(value.try_into()?)
                } else {
                    Err(TryFromIntError)
                }
            }
        })*

        $(impl<const MIN: $internal, const MAX: $internal> TryFrom<$try_from> for $type<MIN, MAX> {
            type Error = TryFromIntError;

            fn try_from(value: $try_from) -> Result<Self, Self::Error> {
                let value = match TryInto::<$internal>::try_into(value) {
                    Ok(value) => value,
                    Err(_) => return Err(TryFromIntError)
                };

                if value < MIN || value > MAX {
                    Err(TryFromIntError)
                } else {
                    match TryFrom::try_from(value) {
                        Ok(value) => Ok(value),
                        Err(_) => Err(TryFromIntError),
                    }
                }
            }
        })*
    )*};
}

impl_ranged! {
    RangedU8(u8): {
        into: [u8, u16, u32, u64, u128, i16, i32, i64, i128]
        try_into: [i8]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedU16(u16): {
        into: [u16, u32, u64, u128, i32, i64, i128]
        try_into: [u8, i8, i16]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedU32(u32): {
        into: [u32, u64, u128, i64, i128]
        try_into: [u8, u16, i8, i16, i32]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedU64(u64): {
        into: [u64, u128, i128]
        try_into: [u8, u16, u32, i8, i16, i32, i64]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedU128(u128): {
        into: [u128]
        try_into: [u8, u16, u32, u64, i8, i16, i32, i64, i128]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedI8(i8): {
        into: [i8, i16, i32, i64, i128]
        try_into: [u8, u16, u32, u64, u128]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedI16(i16): {
        into: [i16, i32, i64, i128]
        try_into: [u8, u16, u32, u64, u128, i8]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedI32(i32): {
        into: [i32, i64, i128]
        try_into: [u8, u16, u32, u64, u128, i8, i16]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedI64(i64): {
        into: [i64, i128]
        try_into: [u8, u16, u32, u64, u128, i8, i16, i32]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
    RangedI128(i128): {
        into: [i128]
        try_into: [u8, u16, u32, u64, u128, i8, i16, i32, i64]
        try_from: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
    }
}
