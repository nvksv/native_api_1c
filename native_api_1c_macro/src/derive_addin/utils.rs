use proc_macro2::{LexError, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Ident};

use super::parsers::ParamType;

pub mod macros {
    macro_rules! tkn_err_inner {
        ($str:expr, $span:expr) => {{
            let err_inner: darling::Error = darling::Error::custom($str).with_span($span);
            err_inner
        }};
    }

    macro_rules! tkn_err {
        ($str:expr, $span:expr) => {
            Err(crate::derive_addin::utils::macros::tkn_err_inner!(
                $str, $span
            ))
        };
    }

    pub(crate) use tkn_err;
    pub(crate) use tkn_err_inner;
}

const IDENT_OPTION_ERR: &str = "Unable to get ident from option";

pub fn ident_option_to_darling_err(ident: Option<&Ident>) -> Result<&Ident, darling::Error> {
    ident.ok_or_else(|| darling::Error::custom(IDENT_OPTION_ERR))
}

pub fn str_literal_token<T>(
    str_literal: &str,
    err_ident: &T,
) -> Result<proc_macro2::TokenStream, darling::Error>
where
    T: Spanned,
{
    format!(r#""{}""#, str_literal)
        .parse()
        .map_err(|e: LexError| {
            darling::Error::custom(format!("Unable to parse string literal: {}", e))
                .with_span(err_ident)
        })
}

pub fn expr_to_os_value(
    expr: &TokenStream,
    ty: &ParamType,
    string_nil: bool,
) -> proc_macro2::TokenStream {
    let os_string_fn = if string_nil {
        quote! {native_api_1c::native_api_1c_core::widestring::U16CString::from_str_truncate}
    } else {
        quote! {native_api_1c::native_api_1c_core::widestring::U16CString::from_str_truncate}
    };
    match ty {
        ParamType::String => quote! {
            {
                let _ = "expr_to_os_value: specific case for String";
                #ty(#os_string_fn(&#expr.clone()).clone().into())
            }
        },
        _ => quote! {
            {
                let _ = "expr_to_os_value: generic case";
                #ty(#expr.clone().into())
            }
        },
    }
}

pub fn expr_from_os_value(expr: &TokenStream, ty: &ParamType) -> proc_macro2::TokenStream {
    match ty {
        ParamType::String => quote! {
            {
                let _ = "expr_from_os_value: specific case for String";
                match &#expr {
                    #ty(val) => {
                        Ok(val.to_string_lossy())
                    },
                    _ => Err(()),
                }?.clone()
            }
        },
        ParamType::Blob => quote! {
            {
                let _ = "expr_from_os_value: specific case for Blob";
                match &#expr {
                    #ty(val) => {
                        Ok(val)
                    },
                    _ => Err(()),
                }?.clone()
            }
        },
        _ => quote! {
            {
                let _ = "expr_from_os_value: generic case";
                match #expr {
                    #ty(val) => {
                        Ok(val)
                    },
                    _ => Err(()),
                }?.clone()
            }
        },
    }
}
