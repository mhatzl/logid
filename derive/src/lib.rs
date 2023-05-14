use logid::log_id::LogLevel;
use proc_macro::TokenStream;
use quote::quote_spanned;
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
    let enum_name = input.ident;

    match input.data {
        syn::Data::Enum(enum_data) => {
            let span = enum_name.span();

            let log = match log_level {
                LogLevel::Error => quote_spanned! {span=> logid::log_id::LogLevel::Error },
                LogLevel::Warn => quote_spanned! {span=> logid::log_id::LogLevel::Warn },
                LogLevel::Info => quote_spanned! {span=> logid::log_id::LogLevel::Info },
                LogLevel::Debug => quote_spanned! {span=> logid::log_id::LogLevel::Debug },
                LogLevel::Trace => quote_spanned! {span=> logid::log_id::LogLevel::Trace },
            };

            let mut from_fields = Vec::new();
            let mut field_identifiers = Vec::new();

            for variant in enum_data.variants {
                let field_name = variant.ident;
                let full_field_name = quote_spanned! {span=>
                    #enum_name::#field_name
                };
                let full_field_name_str =
                    syn::LitStr::new(&full_field_name.to_string().replace(' ', ""), span);
                field_identifiers.push(quote_spanned! {span=>
                    #full_field_name => #full_field_name_str,
                });
                from_fields.push(quote_spanned! {span=>
                    v if v == #full_field_name_str => #full_field_name,
                });
            }

            let from_event_id = quote_spanned! {span=>
                impl From<logid::log_id::LogId> for #enum_name {
                    fn from(value: logid::log_id::LogId) -> Self {
                        if value.get_log_level() != #log
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
                            #log,
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
        _ => panic!("Derive `(Err|Warn|Info|Dbg)LogId` is only implemented for enumerations."),
    }
}
