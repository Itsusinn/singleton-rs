extern crate proc_macro;
extern crate proc_macro2;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Singleton)]
pub fn singleton_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = &ast.ident;
    let uppercase_indent = Ident::new(&ident.to_string().to_ascii_uppercase(), Span::call_site());
    let expanded: proc_macro2::TokenStream = quote! {
        pub static #uppercase_indent: once_cell::sync::Lazy<#ident> = once_cell::sync::Lazy::new(|| {
            #ident::default()
        });
   }.into();
   proc_macro::TokenStream::from(expanded)
}
