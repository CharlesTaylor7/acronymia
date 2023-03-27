mod sliceable;
mod tag;

use ::proc_macro::TokenStream;
use ::syn;

#[proc_macro_derive(Sliceable)]
pub fn derive_sliceable(input: TokenStream) -> TokenStream {
    sliceable::derive(syn::parse_macro_input!(input)).into()
}

#[proc_macro_derive(Tag)]
pub fn derive_tag(input: TokenStream) -> TokenStream {
    tag::derive(syn::parse_macro_input!(input)).into()
}
