//! Representation and conversion of integer literals.

use proc_macro::{token_stream, Span, TokenStream, TokenTree};

use crate::helpers::compile_error;

/// The suffix of an integer literal.
#[derive(Debug)]
pub(crate) enum Suffix {
    /// The integer's suffix was `u`.
    Unsigned,
    /// The integer's suffix was `i`.
    Signed,
    /// The integer's suffix was omitted.
    Either,
}

/// An unsigned integer.
pub(crate) trait Unsigned: TryFrom<u128> {}
/// A signed integer.
pub(crate) trait Signed: TryFrom<i128> {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for i128 {}

/// An integer literal.
#[derive(Debug)]
pub(crate) struct Integer {
    /// Whether the integer is negative.
    pub(crate) is_negative: bool,
    /// The value after casting to `u128`.
    pub(crate) raw_value: u128,
    /// The suffix, whether `u`, `i`, or omitted.
    pub(crate) suffix: Suffix,
    /// The span of the integer literal.
    pub(crate) span: Span,
}

impl Integer {
    /// Whether the integer is permitted to be unsigned.
    const fn can_be_unsigned(&self) -> bool {
        !self.is_negative && matches!(self.suffix, Suffix::Unsigned | Suffix::Either)
    }

    /// Whether the integer is permitted to be signed.
    const fn can_be_signed(&self) -> bool {
        matches!(self.suffix, Suffix::Signed | Suffix::Either)
    }

    /// Attempt to cast the integer to the requested unsigned integer.
    pub(crate) fn to_unsigned<T>(&self) -> Option<T>
    where
        T: Unsigned,
    {
        self.can_be_unsigned()
            .then(|| self.raw_value.try_into().ok())
            .flatten()
    }

    /// Attempt to cast the integer to the requested signed integer.
    pub(crate) fn to_signed<T>(&self) -> Option<T>
    where
        T: Signed,
    {
        self.can_be_signed()
            .then(|| {
                if self.is_negative {
                    (-(self.raw_value as i128)).try_into().ok()
                } else {
                    (self.raw_value as i128).try_into().ok()
                }
            })
            .flatten()
    }

    /// Parse an integer literal from a token stream.
    #[allow(clippy::unwrap_in_result)] // `expect` on negative sign
    pub(crate) fn try_from_tokens(
        token: &mut token_stream::IntoIter,
        what: &str,
    ) -> Result<Self, TokenStream> {
        let (is_negative, negative_span, token) = match token.next() {
            Some(TokenTree::Literal(token)) => (false, None, token),
            Some(TokenTree::Punct(punct)) if punct.as_char() == '-' => {
                let token = match token.next() {
                    Some(TokenTree::Literal(token)) => token,
                    Some(token) => {
                        return Err(compile_error(
                            &format!("expected {what}"),
                            (punct.span(), token.span()),
                        ));
                    }
                    None => {
                        return Err(compile_error(
                            &format!("expected {what}"),
                            (punct.span(), punct.span()),
                        ));
                    }
                };
                (true, Some(punct.span()), token)
            }
            Some(token) => {
                return Err(compile_error(
                    &format!("expected {what}"),
                    (token.span(), token.span()),
                ));
            }
            None => {
                return Err(compile_error(&format!("expected {what}"), None));
            }
        };

        let repr = token.to_string();
        parse_lit_int(is_negative, &repr, what).map_or_else(
            |err| {
                Err(compile_error(
                    &err,
                    (negative_span.unwrap_or_else(|| token.span()), token.span()),
                ))
            },
            |(raw_value, suffix)| {
                Ok(Self {
                    is_negative,
                    raw_value,
                    suffix,
                    span: token.span(),
                })
            },
        )
    }
}

// Returns base 10 digits and suffix.
fn parse_lit_int(is_negative: bool, s: &str, what: &str) -> Result<(u128, Suffix), String> {
    let s = s.as_bytes();

    let (base, mut s) = match s {
        [b'0', b'x', rest @ ..] => (16, rest),
        [b'0', b'o', rest @ ..] => (8, rest),
        [b'0', b'b', rest @ ..] => (2, rest),
        [b'0'..=b'9', ..] => (10, s),
        _ => return Err(format!("{what} must be an integer literal")),
    };

    let mut value = 0u128;
    let mut has_digit = false;
    loop {
        let digit;
        (digit, s) = match s {
            [b @ b'0'..=b'9', rest @ ..] => (b - b'0', rest),
            [b @ b'a'..=b'f', rest @ ..] if base > 10 => (b - b'a' + 10, rest),
            [b @ b'A'..=b'F', rest @ ..] if base > 10 => (b - b'A' + 10, rest),
            [b'_', rest @ ..] => {
                s = rest;
                continue;
            }
            // only present in floats
            [b'.' | b'e' | b'E', ..] => return Err(format!("{what} must be an integer literal")),
            _ => break,
        };

        if digit >= base {
            return Err("invalid digit for base".to_owned());
        }

        has_digit = true;

        // Check for overflow depending on the sign.
        value = if is_negative {
            value
                .checked_mul(base as u128)
                .and_then(|value| value.checked_add(digit as u128))
                .ok_or("value too small to be represented by a primitive integer")?
        } else {
            (value as i128)
                .checked_mul(base as i128)
                .and_then(|value| value.checked_add(digit as i128))
                .ok_or("value too large to be represented by a primitive integer")?
                as u128
        };
    }

    if !has_digit {
        return Err(format!("{what} must be an integer literal"));
    }

    let suffix = match s {
        b"" => Suffix::Either,
        b"u" if is_negative => return Err("unsigned integer cannot be negative".to_owned()),
        b"u" => Suffix::Unsigned,
        b"i" => Suffix::Signed,
        _ => return Err("integer suffix must be `u`, `i`, or omitted".to_owned()),
    };

    Ok((value, suffix))
}
