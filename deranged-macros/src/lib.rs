//! Macros for the `deranged` crate.

#![doc(test(attr(deny(warnings))))]

mod integer;
mod ty;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::integer::Integer;
use crate::ty::Type;

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
