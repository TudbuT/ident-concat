//! [`ident!`], [`replace!`]
extern crate proc_macro;
use proc_macro::{token_stream::IntoIter, *};

fn concat(ts: &mut IntoIter, end: Option<char>) -> Ident {

    let mut ident = String::new();
    for x in ts {
        if let TokenTree::Ident(x) = x {
            ident = ident + &x.to_string();
        } else if matches!(end, Some(e) if matches!(x, TokenTree::Punct(x) if x == e)) {
            break;
        }
    }
    Ident::new(&ident, Span::call_site())
}

#[proc_macro]
/// Concatenates identifiers.
/// ```
/// use ident_concat::ident;
/// let ident!(a b) = 4;
/// assert_eq!(ab, ident!(a b));
/// ```
pub fn ident(ts: TokenStream) -> TokenStream {
    TokenStream::from(TokenTree::Ident(concat(&mut ts.into_iter(), None)))
}

#[proc_macro]
/// Replaces identifiers with concatenated ones. This will not recurse!
/// Use multiple invocations for recursion.
/// ```
/// use ident_concat::replace;
/// replace!{ placeholder te st:
///     fn placeholder() -> u32 { 4 }
/// }
/// assert_eq!(test(), 4);
/// ```
pub fn replace(ts: TokenStream) -> TokenStream {
    let mut iter = ts.into_iter();
    let to_replace = iter
        .next()
        .expect("syntax: replace!(to_replace with _this: to_replace) => with_this");
    let ident = concat(&mut iter, Some(':'));
    TokenStream::from_iter(iter.map(|x| {
        if matches!((&x, &to_replace), (TokenTree::Ident(x), TokenTree::Ident(to_replace)) 
            if x.to_string() == to_replace.to_string())
        {
            TokenTree::Ident(ident.clone())
        } else {
            x
        }
    }))
}
