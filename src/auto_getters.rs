use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Ident, ItemStruct, Type, parse_macro_input};
use syn::spanned::Spanned;

pub fn auto_getters_impl(input: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(input as ItemStruct);

    let name: Ident = input.ident;
    let output: TokenStream = match &input.fields {
        Fields::Named(fields) => {
            let result: Vec<_> = fields
                .named
                .iter()
                .map(|f| {
                    let f_name: &Ident = f.ident.as_ref().unwrap();
                    let f_get_name: Ident = Ident::new(
                        &format!("get_{}", f_name),
                        f.span()
                    );
                    let f_type: &Type = &f.ty;

                    quote! {
                        #[inline]
                        pub fn #f_get_name(&self) -> &#f_type {
                            &self.#f_name
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
        _ => unimplemented!("AutoGetters can only be derived for structs with named fields"),
    };

    output
}
