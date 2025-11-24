use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Ident, ItemStruct, Type, parse_macro_input};
use syn::spanned::Spanned;

pub fn auto_setters_impl(input: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(input as ItemStruct);

    let name: Ident = input.ident;
    let output: TokenStream = match &input.fields {
        Fields::Named(fields) => {
            let result: Vec<_> = fields
                .named
                .iter()
                .map(|f| {
                    let f_name: &Ident = f.ident.as_ref().unwrap();
                    let f_set_name: Ident = Ident::new(
                        &format!("set_{}", f_name),
                        f.span()
                    );
                    let f_type: &Type = &f.ty;

                    quote! {
                        pub fn #f_set_name<T: Into<#f_type>>(&mut self, value: T) {
                            self.#f_name = value.into();
                        }
                    }
                })
                .collect();

            quote! {
                impl #name {
                    #(#result)*
                }
            }
            .into()
        }
        _ => unimplemented!("AutoSetters can only be derived for structs with named fields"),
    };

    output
}
