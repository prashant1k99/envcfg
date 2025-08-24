extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(EnvCfg, attributes(env_cfg))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    dbg!(ast);

    let expanded = quote! {};

    TokenStream::from(expanded)
}
