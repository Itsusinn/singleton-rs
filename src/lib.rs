extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Singleton)]
pub fn singleton_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = &ast.ident;

    let uppercase: String = ident
        .to_string()
        .char_indices()
        .flat_map(|(index, c)| {
            if index != 0 && c.is_uppercase() {
                let mut v = "_".to_string();
                v.push(c);
                v.chars().collect::<Vec<_>>()
            } else {
                c.to_uppercase().to_string().chars().collect::<Vec<_>>()
            }
        })
        .collect();
    let uppercase_indent = Ident::new(&uppercase, Span::call_site());
    let expanded: proc_macro2::TokenStream = quote! {
        pub static #uppercase_indent: once_cell::sync::Lazy<#ident> = once_cell::sync::Lazy::new(|| {
            #ident::default()
        });
   }.into();
    proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let ident = "DotTest";
        let uppercase: String = ident
            .to_string()
            .char_indices()
            .flat_map(|(index, c)| {
                if index != 0 && c.is_uppercase() {
                    let mut v = "_".to_string();
                    v.push(c);
                    v.chars().collect::<Vec<_>>()
                } else {
                    c.to_uppercase().to_string().chars().collect::<Vec<_>>()
                }
            })
            .collect();
        assert_eq!("DOT_TEST", uppercase);
    }
}
