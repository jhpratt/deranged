//! Helpers to make writing macros easier.

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// Unwrap a `Result` or return the error directly.
macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(err) => return err,
        }
    };
}
pub(crate) use unwrap_or_return;

/// Attach a [`Span`] to a [`TokenTree`].
pub(crate) fn with_span(mut tree: TokenTree, span: Span) -> TokenTree {
    tree.set_span(span);
    tree
}

/// A [`Span`], [`(Span, Span]`], or [`None`].
pub(crate) trait MaybeSpan {
    /// Obtain the span as a start-end pair, falling back to [`Span::call_site()`] if necessary.
    fn into_pair(self) -> (Span, Span);
}

impl MaybeSpan for Span {
    fn into_pair(self) -> (Span, Span) {
        (self, self)
    }
}

impl MaybeSpan for (Span, Span) {
    fn into_pair(self) -> (Span, Span) {
        self
    }
}

impl MaybeSpan for Option<core::convert::Infallible> {
    fn into_pair(self) -> (Span, Span) {
        (Span::call_site(), Span::call_site())
    }
}

/// Construct a compilation error with the provided message.
pub(crate) fn compile_error(message: &str, span: impl MaybeSpan) -> TokenStream {
    let (span_start, span_end) = span.into_pair();

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
pub(crate) fn parse_comma(iter: &mut impl Iterator<Item = TokenTree>) -> Result<(), TokenStream> {
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
                (first_span, last_span),
            ))
        }
        Some(token) => Err(compile_error(
            "minimum and maximum value must be separated by a comma",
            token.span(),
        )),
        None => Err(compile_error("expected maximum value", None)),
    }
}
