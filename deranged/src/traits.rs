//! Declaration and implementation of traits used for const assertions.

use crate::{
    RangedI128, RangedI16, RangedI32, RangedI64, RangedI8, RangedIsize, RangedU128, RangedU16,
    RangedU32, RangedU64, RangedU8, RangedUsize,
};

/// Declare a series of traits that will be used for const assertions.
macro_rules! declare_traits {
    ($($trait_name:ident),* $(,)?) => {$(
        pub(crate) trait $trait_name {
            const ASSERT: ();
        }
    )*};
}

/// Implements traits that are common to all integer types.
macro_rules! impl_traits_for_all {
    ($($ranged_ty:ident $inner_ty:ident),* $(,)?) => {$(
        impl<const MIN: $inner_ty, const MAX: $inner_ty> RangeIsValid for $ranged_ty<MIN, MAX> {
            const ASSERT: () = assert!(MIN <= MAX);
        }

        impl<
            const CURRENT_MIN: $inner_ty,
            const CURRENT_MAX: $inner_ty,
            const NEW_MIN: $inner_ty,
            const NEW_MAX: $inner_ty,
        > ExpandIsValid for ($ranged_ty<CURRENT_MIN, CURRENT_MAX>, $ranged_ty<NEW_MIN, NEW_MAX>) {
            const ASSERT: () = {
                assert!(NEW_MIN <= CURRENT_MIN);
                assert!(NEW_MAX >= CURRENT_MAX);
            };
        }

        impl<
            const CURRENT_MIN: $inner_ty,
            const CURRENT_MAX: $inner_ty,
            const NEW_MIN: $inner_ty,
            const NEW_MAX: $inner_ty,
        > NarrowIsValid for ($ranged_ty<CURRENT_MIN, CURRENT_MAX>, $ranged_ty<NEW_MIN, NEW_MAX>) {
            const ASSERT: () = {
                assert!(NEW_MIN >= CURRENT_MIN);
                assert!(NEW_MAX <= CURRENT_MAX);
            };
        }

        impl<
            const VALUE: $inner_ty,
            const MIN: $inner_ty,
            const MAX: $inner_ty,
        > StaticIsValid for ($ranged_ty<MIN, VALUE>, $ranged_ty<VALUE, MAX>) {
            const ASSERT: () = {
                assert!(VALUE >= MIN);
                assert!(VALUE <= MAX);
            };
        }
    )*};
}

/// Implement traits that are common to all signed integer types.
macro_rules! impl_traits_for_signed {
    ($($ranged_ty:ident $inner_ty:ident),* $(,)?) => {$(
        impl<const MIN: $inner_ty, const MAX: $inner_ty> AbsIsSafe for $ranged_ty<MIN, MAX> {
            const ASSERT: () = {
                assert!(MIN != <$inner_ty>::MIN);
                assert!(-MIN <= MAX);
            };
        }

        impl<const MIN: $inner_ty, const MAX: $inner_ty> NegIsSafe for $ranged_ty<MIN, MAX> {
            const ASSERT: () = {
                assert!(MIN != <$inner_ty>::MIN);
                assert!(-MIN <= MAX);
                assert!(-MAX >= MIN);
            };
        }

        impl_traits_for_all!($ranged_ty $inner_ty);
    )*};
}

/// Implement traits that are common to all unsigned integer types.
macro_rules! impl_traits_for_unsigned {
    ($($ranged_ty:ident $inner_ty:ident),* $(,)?) => {$(
        impl<const MIN: $inner_ty, const MAX: $inner_ty> AbsIsSafe for $ranged_ty<MIN, MAX> {
            const ASSERT: () = ();
        }

        impl<const MIN: $inner_ty, const MAX: $inner_ty> NegIsSafe for $ranged_ty<MIN, MAX> {
            const ASSERT: () = assert!(MAX == 0);
        }

        impl_traits_for_all!($ranged_ty $inner_ty);
    )*};
}

declare_traits![
    RangeIsValid,
    AbsIsSafe,
    NegIsSafe,
    ExpandIsValid,
    NarrowIsValid,
    StaticIsValid,
];

impl_traits_for_signed! {
    RangedI8 i8,
    RangedI16 i16,
    RangedI32 i32,
    RangedI64 i64,
    RangedI128 i128,
    RangedIsize isize,
}

impl_traits_for_unsigned! {
    RangedU8 u8,
    RangedU16 u16,
    RangedU32 u32,
    RangedU64 u64,
    RangedU128 u128,
    RangedUsize usize,
}

/// Macro to implement non-fallible `From` conversion traits for ranged types to other wider/equal
/// ranged type with different bit sizes.
///
/// This allows you to convert, for example, a RangedI8<-100, 100> to a RangedI32<-100, 100> or
/// vice-versa without having to cast the value manually and avoid unnecessary range checks.
///
/// The macro checks that the source ranged type's range fits within the destination ranged type's
/// range, and it uses the larger of the two types for the comparison to ensure that the conversion
/// is valid. It also asserts that the source and destination ranges are valid at compile time.
macro_rules! impl_ranged_from {
    ($src_ty:ident, $src_ty_base:ty, $dst_ty:ident, $dst_ty_base:ty) => {
        impl<
                const MIN: $src_ty_base,
                const MAX: $src_ty_base,
                const MIN2: $dst_ty_base,
                const MAX2: $dst_ty_base,
            > From<$src_ty<MIN, MAX>> for $dst_ty<MIN2, MAX2>
        {
            #[inline(always)]
            #[allow(trivial_numeric_casts)]
            fn from(value: $src_ty<MIN, MAX>) -> Self {
                const {
                    assert!(MIN <= MAX, "MIN must be less than or equal to MAX");
                    assert!(MIN2 <= MAX2, "MINU32 must be less than or equal to MAXU32");

                    // compare using the larger of the two types. This doesn't work for
                    // signed/unsigned

                    if <$src_ty_base>::BITS >= <$dst_ty_base>::BITS {
                        assert!(
                            MIN >= MIN2 as $src_ty_base && MAX <= MAX2 as $src_ty_base,
                            "Source ranged values must fit in destination ranged values"
                        );
                    } else {
                        assert!(
                            MIN as $dst_ty_base >= MIN2 && MAX as $dst_ty_base <= MAX2,
                            "Source ranged values must fit in destination ranged values"
                        );
                    }
                };

                // Safety: The conversion is valid because the source range fits within the
                // destination
                unsafe { $dst_ty::new_unchecked(value.get() as $dst_ty_base) }
            }
        }
    };
}

// U8 to other types
impl_ranged_from!(RangedU16, u16, RangedU8, u8);
impl_ranged_from!(RangedU32, u32, RangedU8, u8);
impl_ranged_from!(RangedU64, u64, RangedU8, u8);
impl_ranged_from!(RangedU128, u128, RangedU8, u8);
impl_ranged_from!(RangedUsize, usize, RangedU8, u8);

// U16 to other types
impl_ranged_from!(RangedU8, u8, RangedU16, u16);
impl_ranged_from!(RangedU32, u32, RangedU16, u16);
impl_ranged_from!(RangedU64, u64, RangedU16, u16);
impl_ranged_from!(RangedU128, u128, RangedU16, u16);
impl_ranged_from!(RangedUsize, usize, RangedU16, u16);

// U32 to other types
impl_ranged_from!(RangedU8, u8, RangedU32, u32);
impl_ranged_from!(RangedU16, u16, RangedU32, u32);
impl_ranged_from!(RangedU64, u64, RangedU32, u32);
impl_ranged_from!(RangedU128, u128, RangedU32, u32);
impl_ranged_from!(RangedUsize, usize, RangedU32, u32);

// U64 to other types
impl_ranged_from!(RangedU8, u8, RangedU64, u64);
impl_ranged_from!(RangedU16, u16, RangedU64, u64);
impl_ranged_from!(RangedU32, u32, RangedU64, u64);
impl_ranged_from!(RangedU128, u128, RangedU64, u64);
impl_ranged_from!(RangedUsize, usize, RangedU64, u64);

// U128 to other types
impl_ranged_from!(RangedU8, u8, RangedU128, u128);
impl_ranged_from!(RangedU16, u16, RangedU128, u128);
impl_ranged_from!(RangedU32, u32, RangedU128, u128);
impl_ranged_from!(RangedU64, u64, RangedU128, u128);
impl_ranged_from!(RangedUsize, usize, RangedU128, u128);

// usize to other types
impl_ranged_from!(RangedU8, u8, RangedUsize, usize);
impl_ranged_from!(RangedU16, u16, RangedUsize, usize);
impl_ranged_from!(RangedU32, u32, RangedUsize, usize);
impl_ranged_from!(RangedU64, u64, RangedUsize, usize);
impl_ranged_from!(RangedU128, u128, RangedUsize, usize);

// I8 to other types
impl_ranged_from!(RangedI16, i16, RangedI8, i8);
impl_ranged_from!(RangedI32, i32, RangedI8, i8);
impl_ranged_from!(RangedI64, i64, RangedI8, i8);
impl_ranged_from!(RangedI128, i128, RangedI8, i8);
impl_ranged_from!(RangedIsize, isize, RangedI8, i8);

// I16 to other types
impl_ranged_from!(RangedI8, i8, RangedI16, i16);
impl_ranged_from!(RangedI32, i32, RangedI16, i16);
impl_ranged_from!(RangedI64, i64, RangedI16, i16);
impl_ranged_from!(RangedI128, i128, RangedI16, i16);
impl_ranged_from!(RangedIsize, isize, RangedI16, i16);

// I32 to other types
impl_ranged_from!(RangedI8, i8, RangedI32, i32);
impl_ranged_from!(RangedI16, i16, RangedI32, i32);
impl_ranged_from!(RangedI64, i64, RangedI32, i32);
impl_ranged_from!(RangedI128, i128, RangedI32, i32);
impl_ranged_from!(RangedIsize, isize, RangedI32, i32);

// I64 to other types
impl_ranged_from!(RangedI8, i8, RangedI64, i64);
impl_ranged_from!(RangedI16, i16, RangedI64, i64);
impl_ranged_from!(RangedI32, i32, RangedI64, i64);
impl_ranged_from!(RangedI128, i128, RangedI64, i64);
impl_ranged_from!(RangedIsize, isize, RangedI64, i64);

// I128 to other types
impl_ranged_from!(RangedI8, i8, RangedI128, i128);
impl_ranged_from!(RangedI16, i16, RangedI128, i128);
impl_ranged_from!(RangedI32, i32, RangedI128, i128);
impl_ranged_from!(RangedI64, i64, RangedI128, i128);
impl_ranged_from!(RangedIsize, isize, RangedI128, i128);

// usize to other types
impl_ranged_from!(RangedI8, i8, RangedIsize, isize);
impl_ranged_from!(RangedI16, i16, RangedIsize, isize);
impl_ranged_from!(RangedI32, i32, RangedIsize, isize);
impl_ranged_from!(RangedI64, i64, RangedIsize, isize);
impl_ranged_from!(RangedI128, i128, RangedIsize, isize);
