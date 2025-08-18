//! Macros for the `deranged` crate.

#![doc(test(attr(deny(warnings))))]

mod helpers;
mod integer;
mod ty;

use proc_macro::TokenStream;

use crate::helpers::{parse_comma, unwrap_or_return};
use crate::integer::Integer;
use crate::ty::Type;

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
