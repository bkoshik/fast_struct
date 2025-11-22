mod helpers;

use helpers::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type, Visibility};

#[proc_macro_derive(AutoGetters)]
pub fn auto_getters(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: Ident = input.ident;

    let fields = get_fields(&input.data);

    let getters_methods = fields.iter().map(|f| {
        let f_name: &Ident = f.ident.as_ref().unwrap();
        let f_type: &Type = &f.ty;

        quote! {
            pub fn #f_name(&self) -> &#f_type {
                &self.#f_name
            }
        }
    });

    let output: TokenStream = quote! {
        impl #name {
            #(#getters_methods)*
        }
    }.into();

    output
}

#[proc_macro_attribute]
pub fn optional(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name: Ident = input.ident;
    let vis: &Visibility = &input.vis;

    let fields = get_fields(&input.data);

    let optional_fields = fields.iter().map(|f| {
        let f_name: &Ident = f.ident.as_ref().unwrap();
        let f_type: &Type = &f.ty;
        let f_vis: &Visibility = &f.vis;

        let mut is_except = false;

        for attr in &f.attrs {
            let path = attr.path();

            if path.is_ident("except") {
                is_except = true;
            }
        }

        if is_except {
            quote! {
                #f_vis #f_name: #f_type
            }
        } else {
            quote! {
                #f_vis #f_name: Option<#f_type>
            }
        }
    });

    let output: TokenStream = quote! {
        #vis struct #name {
            #(#optional_fields),*
        }
    }.into();

    output
}
