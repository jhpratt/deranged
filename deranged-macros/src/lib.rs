//! Macros for the `deranged` crate.

#![doc(test(attr(deny(warnings))))]

mod integer;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::integer::Integer;

/// Unwrap a `Result` or return the error directly.
macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(err) => return err,
        }
    };
}

/// Attach a [`Span`] to a [`TokenTree`].
fn with_span(mut tree: TokenTree, span: Span) -> TokenTree {
    tree.set_span(span);
    tree
}

/// The minimum and maximum values for a primitive integer.
#[derive(Debug)]
enum MinMax {
    #[allow(clippy::missing_docs_in_private_items)]
    U8(u8, u8),
    #[allow(clippy::missing_docs_in_private_items)]
    U16(u16, u16),
    #[allow(clippy::missing_docs_in_private_items)]
    U32(u32, u32),
    #[allow(clippy::missing_docs_in_private_items)]
    U64(u64, u64),
    #[allow(clippy::missing_docs_in_private_items)]
    U128(u128, u128),
    #[allow(clippy::missing_docs_in_private_items)]
    I8(i8, i8),
    #[allow(clippy::missing_docs_in_private_items)]
    I16(i16, i16),
    #[allow(clippy::missing_docs_in_private_items)]
    I32(i32, i32),
    #[allow(clippy::missing_docs_in_private_items)]
    I64(i64, i64),
    #[allow(clippy::missing_docs_in_private_items)]
    I128(i128, i128),
}

/// The spans for the minimum and maximum values in [`MinMax`].
#[derive(Debug)]
struct MinMaxSpan(Span, Span);

/// A min-max spanned pair of primitive integers.
#[derive(Debug)]
struct Type<const OPTIONAL: bool>(MinMax, MinMaxSpan);

impl<const OPTIONAL: bool> Type<OPTIONAL> {
    /// Obtain the primitive integer type from a minimum-maximum pair.
    fn from_min_max(min: &Integer, max: &Integer) -> Result<Self, TokenStream> {
        let spans = MinMaxSpan(min.span, max.span);

        if let (Some(min_value), Some(max_value)) = (min.to_unsigned(), max.to_unsigned()) {
            Ok(Self(MinMax::U8(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_unsigned(), max.to_unsigned()) {
            Ok(Self(MinMax::U16(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_unsigned(), max.to_unsigned()) {
            Ok(Self(MinMax::U32(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_unsigned(), max.to_unsigned()) {
            Ok(Self(MinMax::U64(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_unsigned(), max.to_unsigned()) {
            Ok(Self(MinMax::U128(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_signed(), max.to_signed()) {
            Ok(Self(MinMax::I8(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_signed(), max.to_signed()) {
            Ok(Self(MinMax::I16(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_signed(), max.to_signed()) {
            Ok(Self(MinMax::I32(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_signed(), max.to_signed()) {
            Ok(Self(MinMax::I64(min_value, max_value), spans))
        } else if let (Some(min_value), Some(max_value)) = (min.to_signed(), max.to_signed()) {
            Ok(Self(MinMax::I128(min_value, max_value), spans))
        } else {
            Err(compile_error(
                "minimum and maximum values cannot be represented by any one primitive integer",
                None,
            ))
        }
    }

    /// Convert a type into a `TokenStream`.
    fn into_tokens(self) -> TokenStream {
        #[allow(clippy::missing_docs_in_private_items)]
        macro_rules! maybe_optional {
            ($name:literal) => {
                if OPTIONAL {
                    concat!("Option", $name)
                } else {
                    $name
                }
            };
        }

        let Self(min_max, span) = self;
        let MinMaxSpan(min_span, max_span) = span;

        let (type_name, min_token, max_token) = match min_max {
            MinMax::U8(min, max) => (
                maybe_optional!("RangedU8"),
                Literal::u8_unsuffixed(min),
                Literal::u8_unsuffixed(max),
            ),
            MinMax::U16(min, max) => (
                maybe_optional!("RangedU16"),
                Literal::u16_unsuffixed(min),
                Literal::u16_unsuffixed(max),
            ),
            MinMax::U32(min, max) => (
                maybe_optional!("RangedU32"),
                Literal::u32_unsuffixed(min),
                Literal::u32_unsuffixed(max),
            ),
            MinMax::U64(min, max) => (
                maybe_optional!("RangedU64"),
                Literal::u64_unsuffixed(min),
                Literal::u64_unsuffixed(max),
            ),
            MinMax::U128(min, max) => (
                maybe_optional!("RangedU128"),
                Literal::u128_unsuffixed(min),
                Literal::u128_unsuffixed(max),
            ),
            MinMax::I8(min, max) => (
                maybe_optional!("RangedI8"),
                Literal::i8_unsuffixed(min),
                Literal::i8_unsuffixed(max),
            ),
            MinMax::I16(min, max) => (
                maybe_optional!("RangedI16"),
                Literal::i16_unsuffixed(min),
                Literal::i16_unsuffixed(max),
            ),
            MinMax::I32(min, max) => (
                maybe_optional!("RangedI32"),
                Literal::i32_unsuffixed(min),
                Literal::i32_unsuffixed(max),
            ),
            MinMax::I64(min, max) => (
                maybe_optional!("RangedI64"),
                Literal::i64_unsuffixed(min),
                Literal::i64_unsuffixed(max),
            ),
            MinMax::I128(min, max) => (
                maybe_optional!("RangedI128"),
                Literal::i128_unsuffixed(min),
                Literal::i128_unsuffixed(max),
            ),
        };

        TokenStream::from_iter([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("deranged", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new(type_name, Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Punct(Punct::new('<', Spacing::Alone)),
            with_span(TokenTree::Literal(min_token), min_span),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            with_span(TokenTree::Literal(max_token), max_span),
            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
        ])
    }
}

/// Construct a compilation error with the provided message.
fn compile_error(message: &str, span: Option<(Span, Span)>) -> TokenStream {
    let span_start = span.map_or_else(Span::call_site, |span| span.0);
    let span_end = span.map_or(span_start, |span| span.1);

    TokenStream::from_iter([
        with_span(TokenTree::from(Punct::new(':', Spacing::Joint)), span_start),
        with_span(TokenTree::from(Punct::new(':', Spacing::Alone)), span_start),
        TokenTree::from(Ident::new("core", span_start)),
        with_span(TokenTree::from(Punct::new(':', Spacing::Joint)), span_start),
        with_span(TokenTree::from(Punct::new(':', Spacing::Alone)), span_start),
        with_span(
            TokenTree::from(Ident::new("compile_error", Span::mixed_site())),
            span_start,
        ),
        with_span(TokenTree::from(Punct::new('!', Spacing::Alone)), span_start),
        with_span(
            TokenTree::from(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from(TokenTree::Literal(Literal::string(message))),
            )),
            span_end,
        ),
    ])
}

/// Consume a comma, returning a `TokenStream` describing the error upon failure.
fn parse_comma(iter: &mut impl Iterator<Item = TokenTree>) -> Result<(), TokenStream> {
    match iter.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => Ok(()),
        Some(TokenTree::Punct(punct)) => {
            let first_span = punct.span();
            let last_span = iter
                .take_while(|token| matches!(token, TokenTree::Punct(_)))
                .last()
                .map_or(first_span, |token| token.span());
            Err(compile_error(
                "minimum and maximum value must be separated by a comma",
                Some((first_span, last_span)),
            ))
        }
        Some(token) => Err(compile_error(
            "minimum and maximum value must be separated by a comma",
            Some((token.span(), token.span())),
        )),
        None => Err(compile_error("expected maximum value", None)),
    }
}

#[allow(missing_docs)] // documented in re-export in `deranged`
#[proc_macro]
pub fn int(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();

    let min = unwrap_or_return!(Integer::try_from_tokens(&mut iter, "minimum value"));
    unwrap_or_return!(parse_comma(&mut iter));
    let max = unwrap_or_return!(Integer::try_from_tokens(&mut iter, "maximum value"));
    unwrap_or_return!(Type::<false>::from_min_max(&min, &max)).into_tokens()
}

#[allow(missing_docs)] // documented in re-export in `deranged`
#[proc_macro]
pub fn opt_int(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();

    let min = unwrap_or_return!(Integer::try_from_tokens(&mut iter, "minimum value"));
    unwrap_or_return!(parse_comma(&mut iter));
    let max = unwrap_or_return!(Integer::try_from_tokens(&mut iter, "maximum value"));
    unwrap_or_return!(Type::<true>::from_min_max(&min, &max)).into_tokens()
}
