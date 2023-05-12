use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromLogId)]
pub fn derive_from_log_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    match input.data {
        syn::Data::Enum(enum_data) => {
            let span = enum_name.span();
            let mut fields = Vec::new();

            for variant in enum_data.variants {
                let field_name = variant.ident;
                let full_field_name = quote_spanned! {span=>
                    #enum_name :: #field_name
                };
                fields.push(quote_spanned! {span=>
                    v if v == logid::logid!(#full_field_name) => #full_field_name,
                });
            }

            let expanded = quote_spanned! {span=>
                impl From<logid::log_id::LogId> for #enum_name {
                    fn from(value: logid::log_id::LogId) -> Self {
                        match value {
                            #(#fields)*
                            _ => Self::default(),
                        }
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => panic!("Derive `FromLogId` is only implemented for enumerations."),
    }
}
