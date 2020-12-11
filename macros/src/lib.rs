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
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate
)]
#![doc(test(attr(deny(warnings))))]

mod number;

use number::Number;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::Peekable;

macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(err) => return err,
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
}

impl Type {
    fn from_min_max(min: &Number, max: &Number) -> Result<Self, TokenStream> {
        if min.to_u8().is_some() && max.to_u8().is_some() {
            Ok(Self::U8)
        } else if min.to_u16().is_some() && max.to_u16().is_some() {
            Ok(Self::U16)
        } else if min.to_u32().is_some() && max.to_u32().is_some() {
            Ok(Self::U32)
        } else if min.to_u64().is_some() && max.to_u64().is_some() {
            Ok(Self::U64)
        } else if min.to_u128().is_some() && max.to_u128().is_some() {
            Ok(Self::U128)
        } else if min.to_i8().is_some() && max.to_i8().is_some() {
            Ok(Self::I8)
        } else if min.to_i16().is_some() && max.to_i16().is_some() {
            Ok(Self::I16)
        } else if min.to_i32().is_some() && max.to_i32().is_some() {
            Ok(Self::I32)
        } else if min.to_i64().is_some() && max.to_i64().is_some() {
            Ok(Self::I64)
        } else if min.to_i128().is_some() && max.to_i128().is_some() {
            Ok(Self::I128)
        } else {
            Err(compile_error(
                "minimum-maximum pair cannot be represented by a single primitive integer",
            ))
        }
    }

    fn tokens_from_min_max(min: &Number, max: &Number) -> Result<TokenStream, TokenStream> {
        let (type_name_token, min_token, max_token);

        // The validity of the casts are verified by the enum discriminant.
        #[allow(clippy::unwrap_used)]
        match Self::from_min_max(min, max)? {
            Self::U8 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedU8", Span::call_site()));
                min_token = TokenTree::Literal(Literal::u8_unsuffixed(min.to_u8().unwrap()));
                max_token = TokenTree::Literal(Literal::u8_unsuffixed(max.to_u8().unwrap()));
            }
            Self::U16 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedU16", Span::call_site()));
                min_token = TokenTree::Literal(Literal::u16_unsuffixed(min.to_u16().unwrap()));
                max_token = TokenTree::Literal(Literal::u16_unsuffixed(max.to_u16().unwrap()));
            }
            Self::U32 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedU32", Span::call_site()));
                min_token = TokenTree::Literal(Literal::u32_unsuffixed(min.to_u32().unwrap()));
                max_token = TokenTree::Literal(Literal::u32_unsuffixed(max.to_u32().unwrap()));
            }
            Self::U64 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedU64", Span::call_site()));
                min_token = TokenTree::Literal(Literal::u64_unsuffixed(min.to_u64().unwrap()));
                max_token = TokenTree::Literal(Literal::u64_unsuffixed(max.to_u64().unwrap()));
            }
            Self::U128 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedU128", Span::call_site()));
                min_token = TokenTree::Literal(Literal::u128_unsuffixed(min.to_u128().unwrap()));
                max_token = TokenTree::Literal(Literal::u128_unsuffixed(max.to_u128().unwrap()));
            }
            Self::I8 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedI8", Span::call_site()));
                min_token = TokenTree::Literal(Literal::i8_unsuffixed(min.to_i8().unwrap()));
                max_token = TokenTree::Literal(Literal::i8_unsuffixed(max.to_i8().unwrap()));
            }
            Self::I16 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedI16", Span::call_site()));
                min_token = TokenTree::Literal(Literal::i16_unsuffixed(min.to_i16().unwrap()));
                max_token = TokenTree::Literal(Literal::i16_unsuffixed(max.to_i16().unwrap()));
            }
            Self::I32 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedI32", Span::call_site()));
                min_token = TokenTree::Literal(Literal::i32_unsuffixed(min.to_i32().unwrap()));
                max_token = TokenTree::Literal(Literal::i32_unsuffixed(max.to_i32().unwrap()));
            }
            Self::I64 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedI64", Span::call_site()));
                min_token = TokenTree::Literal(Literal::i64_unsuffixed(min.to_i64().unwrap()));
                max_token = TokenTree::Literal(Literal::i64_unsuffixed(max.to_i64().unwrap()));
            }
            Self::I128 => {
                type_name_token = TokenTree::Ident(Ident::new("RangedI128", Span::call_site()));
                min_token = TokenTree::Literal(Literal::i128_unsuffixed(min.to_i128().unwrap()));
                max_token = TokenTree::Literal(Literal::i128_unsuffixed(max.to_i128().unwrap()));
            }
        }

        Ok([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("deranged", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            type_name_token,
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Punct(Punct::new('<', Spacing::Alone)),
            min_token,
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            max_token,
            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
        ]
        .iter()
        .cloned()
        .collect())
    }
}

fn compile_error(message: &str) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", Span::mixed_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from(TokenTree::Literal(Literal::string(message))),
        )),
    ]
    .iter()
    .cloned()
    .collect()
}

/// Return if a sign is positive or negative. If a sign is invalid, a
/// `TokenStream` describing the error is returned.
fn parse_sign(iter: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Sign, TokenStream> {
    match iter.peek() {
        Some(TokenTree::Punct(punct)) => {
            if punct.as_char() == '-' {
                let _ = iter.next();
                Ok(Sign::Negative)
            } else if punct.as_char() == '+' {
                let _ = iter.next();
                Ok(Sign::Positive)
            } else {
                Err(compile_error("unexpected symbol"))
            }
        }
        _ => Ok(Sign::Positive),
    }
}

fn parse_integer(iter: &mut impl Iterator<Item = TokenTree>) -> Result<u128, TokenStream> {
    match iter.next() {
        Some(TokenTree::Literal(literal)) => {
            let value = literal.to_string();

            // Integers cannot begin or end with an underscore, but may have
            // internal underscores.
            if value.starts_with('_')
                || value.ends_with('_')
                || value.chars().any(|c| !c.is_digit(10) && c != '_')
            {
                Err(compile_error("expected integer"))
            } else {
                match value.replace('_', "").parse::<u128>() {
                    Ok(value) => Ok(value),
                    Err(_) => Err(compile_error(
                        "value cannot be represented by any primitive integer",
                    )),
                }
            }
        }
        _ => Err(compile_error("expected integer")),
    }
}

/// Consume the provided character, returning a `TokenStream` describing the
/// error upon failure.
fn parse_punct(iter: &mut impl Iterator<Item = TokenTree>, c: char) -> Result<(), TokenStream> {
    match iter.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == c => Ok(()),
        _ => Err(compile_error(&format!("expected `{}`", c))),
    }
}

#[proc_macro]
pub fn ranged_int(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter().peekable();

    let min_sign = unwrap_or_return!(parse_sign(&mut iter));
    let min = unwrap_or_return!(parse_integer(&mut iter));
    unwrap_or_return!(parse_punct(&mut iter, '.'));
    unwrap_or_return!(parse_punct(&mut iter, '.'));
    unwrap_or_return!(parse_punct(&mut iter, '='));
    let max_sign = unwrap_or_return!(parse_sign(&mut iter));
    let max = unwrap_or_return!(parse_integer(&mut iter));

    unwrap_or_return!(Type::tokens_from_min_max(
        &Number {
            sign: min_sign,
            value: min,
        },
        &Number {
            sign: max_sign,
            value: max,
        }
    ))
}
