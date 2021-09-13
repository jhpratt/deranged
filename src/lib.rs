#![cfg_attr(not(feature = "std"), no_std)]
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

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::convert::{TryFrom, TryInto};
use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromIntError;

impl fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("out of range integral type conversion attempted")
    }
}
#[cfg(feature = "std")]
impl Error for TryFromIntError {}

macro_rules! const_try_opt {
    ($e:expr) => {
        match $e {
            Some(value) => value,
            None => return None,
        }
    };
}

macro_rules! if_signed {
    (true $($x:tt)*) => { $($x)*};
    (false $($x:tt)*) => {};
}

macro_rules! article {
    (true) => {
        "An"
    };
    (false) => {
        "A"
    };
}

macro_rules! impl_ranged {
    ($(
        $type:ident {
            internal: $internal:ident
            signed: $is_signed:ident
            into: [$($into:ident),* $(,)?]
            try_into: [$($try_into:ident),* $(,)?]
            try_from: [$($try_from:ident),* $(,)?]
        }
    )*) => {$(
        #[doc = concat!(
            article!($is_signed),
            " `",
            stringify!($internal),
            "` that is known to be in the range `MIN..=MAX`.",
        )]
        #[repr(transparent)]
        #[derive(Clone, Copy, Eq, Ord, Hash)]
        pub struct $type<const MIN: $internal, const MAX: $internal>(
            $internal,
        );

        impl<const MIN: $internal, const MAX: $internal> $type<MIN, MAX> {
            /// The smallest value that can be represented by this type.
            pub const MIN: Self = Self(MIN);

            /// The largest value that can be represented by this type.
            pub const MAX: Self = Self(MAX);

            /// Creates a ranged integer without checking the value.
            ///
            /// # Safety
            ///
            /// The value must be within the range `MIN..=MAX`.
            pub const unsafe fn new_unchecked(value: $internal) -> Self {
                Self(value)
            }

            /// Creates a ranged integer if the given value is in the range
            /// `MIN..=MAX`.
            pub const fn new(value: $internal) -> Option<Self> {
                if value < MIN || value > MAX {
                    None
                } else {
                    Some(Self(value))
                }
            }

            const fn new_saturating(value: $internal) -> Self {
                Self(if value < MIN {
                    MIN
                } else if value > MAX {
                    MAX
                } else {
                    value
                })
            }

            /// Returns the value as a primitive type.
            pub const fn get(self) -> $internal {
                self.0
            }

            /// Checked integer addition. Computes `self + rhs`, returning
            /// `None` if the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_add(self, rhs: $internal) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_add(rhs)))
            }

            /// Checked integer addition. Computes `self - rhs`, returning
            /// `None` if the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_sub(self, rhs: $internal) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_sub(rhs)))
            }

            /// Checked integer addition. Computes `self * rhs`, returning
            /// `None` if the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_mul(self, rhs: $internal) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_mul(rhs)))
            }

            /// Checked integer addition. Computes `self / rhs`, returning
            /// `None` if `rhs == 0` or if the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_div(self, rhs: $internal) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_div(rhs)))
            }

            /// Checked Euclidean division. Computes `self.div_euclid(rhs)`,
            /// returning `None` if `rhs == 0` or if the resulting value is out
            /// of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_div_euclid(self, rhs: $internal) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_div_euclid(rhs)))
            }

            /// Checked integer remainder. Computes `self % rhs`, returning
            /// `None` if `rhs == 0` or if the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_rem(self, rhs: $internal) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_rem(rhs)))
            }

            /// Checked Euclidean remainder. Computes `self.rem_euclid(rhs)`,
            /// returning `None` if `rhs == 0` or if the resulting value is out
            /// of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_rem_euclid(self, rhs: $internal) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_rem_euclid(rhs)))
            }

            /// Checked negation. Computes `-self`, returning `None` if the
            /// resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_neg(self) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_neg()))
            }

            /// Checked shift left. Computes `self << rhs`, returning `None` if
            /// the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_shl(rhs)))
            }

            /// Checked shift right. Computes `self >> rhs`, returning `None` if
            /// the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_shr(rhs)))
            }

            if_signed!($is_signed
            /// Checked absolute value. Computes `self.abs()`, returning `None`
            /// if the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_abs(self) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_abs()))
            });

            /// Checked exponentiation. Computes `self.pow(exp)`, returning
            /// `None` if the resulting value is out of range.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_pow(self, exp: u32) -> Option<Self> {
                Self::new(const_try_opt!(self.0.checked_pow(exp)))
            }

            /// Saturating integer addition. Computes `self + rhs`, saturating
            /// at the numeric bounds.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_add(self, rhs: $internal) -> Self {
                Self::new_saturating(self.0.saturating_add(rhs))
            }

            /// Saturating integer subtraction. Computes `self - rhs`,
            /// saturating at the numeric bounds.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_sub(self, rhs: $internal) -> Self {
                Self::new_saturating(self.0.saturating_sub(rhs))
            }

            if_signed!($is_signed
            /// Saturating integer negation. Computes `self - rhs`, saturating
            /// at the numeric bounds.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_neg(self) -> Self {
                Self::new_saturating(self.0.saturating_neg())
            });

            if_signed!($is_signed
            /// Saturating absolute value. Computes `self.abs()`, saturating at
            /// the numeric bounds.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_abs(self) -> Self {
                Self::new_saturating(self.0.saturating_abs())
            });

            /// Saturating integer multiplication. Computes `self * rhs`,
            /// saturating at the numeric bounds.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_mul(self, rhs: $internal) -> Self {
                Self::new_saturating(self.0.saturating_mul(rhs))
            }

            /// Saturating integer exponentiation. Computes `self.pow(exp)`,
            /// saturating at the numeric bounds.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_pow(self, exp: u32) -> Self {
                Self::new_saturating(self.0.saturating_pow(exp))
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::Debug for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::Display for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> AsRef<$internal> for $type<MIN, MAX> {
            fn as_ref(&self) -> &$internal {
                &self.0
            }
        }

        impl<const MIN: $internal, const MAX: $internal> Borrow<$internal> for $type<MIN, MAX> {
            fn borrow(&self) -> &$internal {
                &self.0
            }
        }

        impl<
            const MIN_A: $internal,
            const MAX_A: $internal,
            const MIN_B: $internal,
            const MAX_B: $internal,
        > PartialEq<$type<MIN_B, MAX_B>> for $type<MIN_A, MAX_A> {
            fn eq(&self, other: &$type<MIN_B, MAX_B>) -> bool {
                self.0 == other.0
            }
        }

        impl<const MIN: $internal, const MAX: $internal> PartialEq<$internal> for $type<MIN, MAX> {
            fn eq(&self, other: &$internal) -> bool {
                self.0 == *other
            }
        }

        impl<const MIN: $internal, const MAX: $internal> PartialEq<$type<MIN, MAX>> for $internal {
            fn eq(&self, other: &$type<MIN, MAX>) -> bool {
                *self == other.0
            }
        }

        impl<
            const MIN_A: $internal,
            const MAX_A: $internal,
            const MIN_B: $internal,
            const MAX_B: $internal,
        > PartialOrd<$type<MIN_B, MAX_B>> for $type<MIN_A, MAX_A> {
            fn partial_cmp(&self, other: &$type<MIN_B, MAX_B>) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> PartialOrd<$internal> for $type<MIN, MAX> {
            fn partial_cmp(&self, other: &$internal) -> Option<Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> PartialOrd<$type<MIN, MAX>> for $internal {
            fn partial_cmp(&self, other: &$type<MIN, MAX>) -> Option<Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::Binary for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::LowerHex for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::UpperHex for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::LowerExp for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::UpperExp for $type<MIN, MAX> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<const MIN: $internal, const MAX: $internal> fmt::Octal for $type<MIN, MAX> {
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

        #[cfg(feature = "serde")]
        impl<const MIN: $internal, const MAX: $internal> serde::Serialize for $type<MIN, MAX> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.get().serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<
            'de,
            const MIN: $internal,
            const MAX: $internal,
        > serde::Deserialize<'de> for $type<MIN, MAX> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let internal = <$internal>::deserialize(deserializer)?;
                Self::new(internal).ok_or_else(|| <D::Error as serde::de::Error>::invalid_value(
                    serde::de::Unexpected::Other("integer"),
                    &format!("an integer in the range {}..={}", MIN, MAX).as_ref()
                ))
            }
        }

        #[cfg(feature = "rand")]
        impl<
            const MIN: $internal,
            const MAX: $internal,
        > rand::distributions::Distribution<$type<MIN, MAX>> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $type<MIN, MAX> {
                $type(rng.gen_range(MIN..=MAX))
            }
        }
    )*};
}

impl_ranged! {
    U8 {
        internal: u8
        signed: false
        into: [u8, u16, u32, u64, u128, i16, i32, i64, i128]
        try_into: [usize, i8, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    U16 {
        internal: u16
        signed: false
        into: [u16, u32, u64, u128, i32, i64, i128]
        try_into: [u8, usize, i8, i16, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    U32 {
        internal: u32
        signed: false
        into: [u32, u64, u128, i64, i128]
        try_into: [u8, u16, usize, i8, i16, i32, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    U64 {
        internal: u64
        signed: false
        into: [u64, u128, i128]
        try_into: [u8, u16, u32, usize, i8, i16, i32, i64, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    U128 {
        internal: u128
        signed: false
        into: [u128]
        try_into: [u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    Usize {
        internal: usize
        signed: false
        into: [usize]
        try_into: [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    I8 {
        internal: i8
        signed: true
        into: [i8, i16, i32, i64, i128]
        try_into: [u8, u16, u32, u64, u128, usize, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    I16 {
        internal: i16
        signed: true
        into: [i16, i32, i64, i128]
        try_into: [u8, u16, u32, u64, u128, usize, i8, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    I32 {
        internal: i32
        signed: true
        into: [i32, i64, i128]
        try_into: [u8, u16, u32, u64, u128, usize, i8, i16, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    I64 {
        internal: i64
        signed: true
        into: [i64, i128]
        try_into: [u8, u16, u32, u64, u128, usize, i8, i16, i32, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    I128 {
        internal: i128
        signed: true
        into: [i128]
        try_into: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
    Isize {
        internal: isize
        signed: true
        into: [isize]
        try_into: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128]
        try_from: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
    }
}
