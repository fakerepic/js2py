extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GetSpan)]
pub fn derive_get_span(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let data = input.data; // may be enum or struct

    let expanded = match data {
        // if data is struct, get self.span
        syn::Data::Struct(_) => {
            let span = quote! {
                self.span
            };
            quote! {
                impl #generics GetSpan for #name #generics {
                    fn span(&self) -> Span {
                        #span
                    }
                }
            }
        }
        // else if data is enum, get self.span from each variant (Box<Struct>)
        syn::Data::Enum(data) => {
            let list = data.variants.iter().map(|variant| {
                let ident = &variant.ident;
                quote! {
                    #name::#ident(inner) => inner.span(),
                }
            });
            let span = quote! {
                match self {
                    #(#list)*
                }
            };
            quote! {
                impl #generics GetSpan for #name #generics {
                    fn span(&self) -> Span {
                        #span
                    }
                }
            }
        }

        // if data is union, return error
        _ => {
            quote! {
                compile_error!("GetSpan can only be derived for structs and enums");
            }
        }
    };

    TokenStream::from(expanded)
}
