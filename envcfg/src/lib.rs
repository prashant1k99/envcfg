extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// ⚠️WARNING: WIP
#[proc_macro_derive(EnvCfg, attributes(env_cfg))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    dbg!(ast);

    // Do not allow Option in config struct

    let expanded = quote! {
        // Implementation comes here
    };

    TokenStream::from(expanded)
}
