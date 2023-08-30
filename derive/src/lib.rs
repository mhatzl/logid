use logid_core::log_id::LogLevel;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ErrLogId)]
pub fn derive_err_log_id(input: TokenStream) -> TokenStream {
    derive_log_id(input, LogLevel::Error)
}

#[proc_macro_derive(WarnLogId)]
pub fn derive_warn_log_id(input: TokenStream) -> TokenStream {
    derive_log_id(input, LogLevel::Warn)
}

#[proc_macro_derive(InfoLogId)]
pub fn derive_info_log_id(input: TokenStream) -> TokenStream {
    derive_log_id(input, LogLevel::Info)
}

#[proc_macro_derive(DbgLogId)]
pub fn derive_dbg_log_id(input: TokenStream) -> TokenStream {
    derive_log_id(input, LogLevel::Debug)
}

#[proc_macro_derive(TraceLogId)]
pub fn derive_trace_log_id(input: TokenStream) -> TokenStream {
    derive_log_id(input, LogLevel::Trace)
}

fn derive_log_id(input: TokenStream, log_level: LogLevel) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident_name = input.ident;

    let log_token = log_level_as_tokenstream(log_level);

    match input.data {
        syn::Data::Enum(enum_data) => {
            let span = ident_name.span();

            let mut field_identifiers = Vec::new();

            for variant in enum_data.variants {
                let field_name = variant.ident;
                let full_field_name = match variant.fields {
                    syn::Fields::Named(_) => {
                        quote_spanned! {span=>
                            #ident_name::#field_name{..}
                        }
                    }
                    syn::Fields::Unnamed(_) => {
                        quote_spanned! {span=>
                            #ident_name::#field_name(_)
                        }
                    }
                    _ => quote_spanned! {span=>
                        #ident_name::#field_name
                    },
                };
                let full_field_name_str =
                    syn::LitStr::new(&full_field_name.to_string().replace(' ', ""), span);
                field_identifiers.push(quote_spanned! {span=>
                    #full_field_name => #full_field_name_str,
                });
            }

            let from_enum = quote_spanned! {span=>
                impl From<#ident_name> for logid::log_id::LogId {
                    fn from(value: #ident_name) -> Self {
                        let field_name = match value {
                            #(#field_identifiers)*
                        };

                        logid::log_id::LogId::new(
                            module_path!(),
                            field_name,
                            #log_token,
                        )
                    }
                }
            };

            TokenStream::from(from_enum)
        }
        syn::Data::Struct(struct_data) => {
            let span = struct_data.struct_token.span;
            from_struct_or_union(ident_name, log_token, span)
        }
        syn::Data::Union(union_data) => {
            let span = union_data.union_token.span;
            from_struct_or_union(ident_name, log_token, span)
        }
    }
}

fn from_struct_or_union(
    ident_name: proc_macro2::Ident,
    log_token: proc_macro2::TokenStream,
    span: proc_macro2::Span,
) -> TokenStream {
    let ident_name_str = syn::LitStr::new(&ident_name.to_string(), span);

    let from = quote_spanned! {span=>
        impl From<#ident_name> for logid::log_id::LogId {
            fn from(value: #ident_name) -> Self {
                logid::log_id::LogId::new(
                    module_path!(),
                    #ident_name_str,
                    #log_token,
                )
            }
        }
    };

    TokenStream::from(from)
}

#[proc_macro_derive(FromLogId)]
pub fn derive_from_log_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    match input.data {
        syn::Data::Enum(enum_data) => {
            let span = enum_name.span();

            let mut from_fields = Vec::new();

            for variant in enum_data.variants {
                let field_name = variant.ident;
                let full_field_name = quote_spanned! {span=>
                    #enum_name::#field_name
                };
                let full_field_name_str =
                    syn::LitStr::new(&full_field_name.to_string().replace(' ', ""), span);
                from_fields.push(quote_spanned! {span=>
                    v if v == #full_field_name_str => #full_field_name,
                });
            }

            let from_log_id = quote_spanned! {span=>
                impl From<logid::log_id::LogId> for #enum_name {
                    fn from(value: logid::log_id::LogId) -> Self {
                        if value.get_module_path() != module_path!() {

                            return Self::default();
                        }

                        match value.get_identifier() {
                            #(#from_fields)*
                            _ => Self::default(),
                        }
                    }
                }

                impl From<logid::logging::intermediary_event::IntermediaryLogEvent> for #enum_name {
                    fn from(value: logid::logging::intermediary_event::IntermediaryLogEvent) -> Self {
                        value.finalize().into_event_id().into()
                    }
                }
            };

            TokenStream::from(from_log_id)
        }
        _ => panic!("Derive `FromLogId` is only implemented for enumerations where no item has fields (e.g. SomeEnum::Item(Field) is **not** allowed)."),
    }
}

fn log_level_as_tokenstream(level: LogLevel) -> proc_macro2::TokenStream {
    match level {
        LogLevel::Error => quote! { logid::log_id::LogLevel::Error },
        LogLevel::Warn => quote! { logid::log_id::LogLevel::Warn },
        LogLevel::Info => quote! { logid::log_id::LogLevel::Info },
        LogLevel::Debug => quote! { logid::log_id::LogLevel::Debug },
        LogLevel::Trace => quote! { logid::log_id::LogLevel::Trace },
    }
}
