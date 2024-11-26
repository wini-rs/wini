#![doc(html_root_url = "https://docs.rs/maud_macros/0.26.0")]
// TokenStream values are reference counted, and the mental overhead of tracking
// lifetimes outweighs the marginal gains from explicit borrowing
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro;

mod ast;
mod escape;
mod generate;
mod parse;

use {
    proc_macro2::{Ident, Span, TokenStream, TokenTree},
    proc_macro_error::proc_macro_error,
    quote::quote,
};

#[proc_macro]
#[proc_macro_error]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into()).into()
}

fn expand(input: TokenStream) -> TokenStream {
    let output_ident = TokenTree::Ident(Ident::new("__maud_output", Span::mixed_site()));
    let linked_files = TokenTree::Ident(Ident::new("__maud_linked_files", Span::mixed_site()));
    // Heuristic: the size of the resulting markup tends to correlate with the
    // code size of the template itself
    let size_hint = input.to_string().len();
    let markups = parse::parse(input);
    let stmts = generate::generate(markups, output_ident.clone(), linked_files.clone());
    quote!({
        extern crate alloc;
        extern crate maud;
        let mut #output_ident = alloc::string::String::with_capacity(#size_hint);
        let mut #linked_files = maud::macro_private::HashSet::<alloc::string::String>::new();
        #stmts
        maud::Markup {
            content: maud::PreEscaped(#output_ident),
            linked_files: #linked_files,

        }
    })
}
