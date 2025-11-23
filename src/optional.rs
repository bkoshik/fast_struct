use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Ident, ItemStruct, Type, Visibility};

pub fn optional_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(item as ItemStruct);

    let name: Ident = input.ident;
    let vis: &Visibility = &input.vis;
    let attrs: Vec<_> = input.attrs.iter().filter(|attr| !attr.path().is_ident("optional")).collect();
    let result: TokenStream = match &input.fields {
        Fields::Named(fields) => {
            let result: Vec<_> = fields.named.iter().map(|f| {
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
            }).collect();

            quote! {
                #(#attrs)*
                #vis struct #name {
                    #(#result),*
                }
            }.into()
        }
        Fields::Unnamed(fields) => {
            let result: Vec<_> = fields.unnamed.iter().map(|f| {
                let f_type: &Type = &f.ty;

                quote! {
                    Option<#f_type>
                }
            }).collect();

            quote! {
                #(#attrs)*
                #vis struct #name(#(#result),*);
            }.into()
        }
        _ => unimplemented!("Optional can only be derived for structs with named and unnamed fields"),
    };

    result
}