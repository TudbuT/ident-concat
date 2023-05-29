//! [`concat_idents!`]
#![no_std]

extern crate proc_macro;

use proc_macro::*;

#[proc_macro]
/// Concatenates identifiers.
/// ```
/// use ident_concat::ident;
/// let ident!(a b) = 4;
/// assert_eq!(ab, ident!(a b));
/// ```
pub fn ident(ts: TokenStream) -> TokenStream {
    extern crate alloc;
    use alloc::string::String;
    use alloc::string::ToString;

    let mut ident = String::new();
    for x in ts {
        if let TokenTree::Ident(x) = x {
            ident = ident + &x.to_string();
        }
    }

    TokenStream::from(TokenTree::Ident(Ident::new(&ident, Span::call_site())))
}
