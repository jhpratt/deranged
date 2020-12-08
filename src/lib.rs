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

use std::{convert::TryFrom, error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutOfRange;

impl fmt::Display for OutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("value out of range")
    }
}
impl Error for OutOfRange {}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangedU8<const MIN: u8, const MAX: u8>(u8);

impl<const MIN: u8, const MAX: u8> fmt::Debug for RangedU8<MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const MIN: u8, const MAX: u8> From<RangedU8<MIN, MAX>> for u8 {
    fn from(value: RangedU8<MIN, MAX>) -> Self {
        value.0
    }
}

impl<const MIN: u8, const MAX: u8> TryFrom<u8> for RangedU8<MIN, MAX> {
    type Error = OutOfRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < MIN || value > MAX {
            Err(OutOfRange)
        } else {
            Ok(Self(value))
        }
    }
}
