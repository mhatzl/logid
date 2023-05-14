use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, DeriveInput};

// #[proc_macro_derive(FromLogId)]
// pub fn derive_from_log_id(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let enum_name = input.ident;

//     match input.data {
//         syn::Data::Enum(enum_data) => {
//             let span = enum_name.span();
//             let mut fields = Vec::new();

//             for variant in enum_data.variants {
//                 let field_name = variant.ident;
//                 let full_field_name = quote_spanned! {span=>
//                     #enum_name :: #field_name
//                 };
//                 fields.push(quote_spanned! {span=>
//                     v if v == logid::logid!(#full_field_name) => #full_field_name,
//                 });
//             }

//             let expanded = quote_spanned! {span=>
//                 impl From<logid::log_id::LogId> for #enum_name {
//                     fn from(value: logid::log_id::LogId) -> Self {
//                         match value {
//                             #(#fields)*
//                             _ => Self::default(),
//                         }
//                     }
//                 }
//             };

//             TokenStream::from(expanded)
//         }
//         _ => panic!("Derive `FromLogId` is only implemented for enumerations."),
//     }
// }

#[proc_macro_derive(ErrLogId)]
pub fn derive_err_log_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    match input.data {
        syn::Data::Enum(enum_data) => {
            let span = enum_name.span();
            let mut from_fields = Vec::new();
            let mut field_identifiers = Vec::new();

            for variant in enum_data.variants {
                let field_name = variant.ident;
                let full_field_name = quote_spanned! {span=>
                    #enum_name::#field_name
                };
                field_identifiers.push(quote_spanned! {span=>
                    #full_field_name => "#full_field_name",
                });
                from_fields.push(quote_spanned! {span=>
                    v if v == "#full_field_name" => #full_field_name,
                });
            }

            let from_event_id = quote_spanned! {span=>
                impl From<logid::log_id::LogId> for #enum_name {
                    fn from(value: logid::log_id::LogId) -> Self {
                        if value.get_log_level() != logid::log_id::LogLevel::Error
                            || value.get_crate_name() != env!("CARGO_PKG_NAME")
                            || value.get_module_path() != module_path!() {

                            return Self::default();
                        }

                        match value.get_identifier() {
                            #(#from_fields)*
                            _ => Self::default(),
                        }
                    }
                }
            };

            let from_enum = quote_spanned! {span=>
                impl From<#enum_name> for logid::log_id::LogId {
                    fn from(value: #enum_name) -> Self {
                        let field_name = match value {
                            #(#field_identifiers)*
                        };

                        logid::log_id::LogId::new(
                            env!("CARGO_PKG_NAME"),
                            module_path!(),
                            field_name,
                            logid::log_id::LogLevel::Error,
                        )
                    }
                }
            };

            let froms = quote_spanned! {span=>
                #from_event_id

                #from_enum
            };

            TokenStream::from(froms)
        }
        _ => panic!("Derive `ErrLogId` is only implemented for enumerations."),
    }
}
