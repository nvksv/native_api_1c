use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;

use super::constants::{BLOB_TYPE, BOOL_TYPE, DATE_TYPE, F64_TYPE, I32_TYPE, STRING_TYPE};
use native_api_1c_core::interface::ParamType;

const META_TYPE_ERR: &str = "expected string literal or path";

#[derive(Debug)]
pub struct ParamTypeWrapper(pub ParamType);

impl FromMeta for ParamTypeWrapper {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        let meta_type_err = darling::Error::custom(META_TYPE_ERR);
        let expr_string = match expr {
            syn::Expr::Lit(str_lit) => match str_lit.lit {
                syn::Lit::Str(ref str) => str.value(),
                _ => return Err(meta_type_err),
            },
            syn::Expr::Path(path) => path.path.segments.first().unwrap().ident.to_string(),
            _ => return Err(meta_type_err),
        };
        Self::from_string(&expr_string)
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        Self::try_from(value).map_err(|_| {
            let joined_allowed_types = crate::derive_addin::constants::ALL_ARG_TYPES.join(", ");
            darling::Error::custom(format!(
                "unknown type `{value}`. Must be one of: {joined_allowed_types}",
            ))
        })
    }
}

impl TryFrom<&str> for ParamTypeWrapper {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            BOOL_TYPE => Ok(ParamTypeWrapper(ParamType::Bool)),
            I32_TYPE => Ok(ParamTypeWrapper(ParamType::I32)),
            F64_TYPE => Ok(ParamTypeWrapper(ParamType::F64)),
            STRING_TYPE => Ok(ParamTypeWrapper(ParamType::String)),
            DATE_TYPE => Ok(ParamTypeWrapper(ParamType::Date)),
            BLOB_TYPE => Ok(ParamTypeWrapper(ParamType::Blob)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum PropName {
    StringLiteral(syn::LitStr),
    Ident(syn::ExprPath),
}

impl FromMeta for PropName {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        match expr {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Str(str_lit) => Ok(PropName::StringLiteral(str_lit.clone())),
                _ => Err(darling::Error::custom("expected string literal").with_span(expr)),
            },
            syn::Expr::Path(path) => Ok(PropName::Ident(path.clone())),
            _ => Err(darling::Error::custom("expected string literal or path").with_span(expr)),
        }
    }
}

impl From<PropName> for proc_macro2::TokenStream {
    fn from(prop_name: PropName) -> proc_macro2::TokenStream {
        match prop_name {
            PropName::StringLiteral(str_lit) => str_lit.to_token_stream(),
            PropName::Ident(ident) => ident.to_token_stream(),
        }
    }
}

#[derive(Debug)]
pub struct ParamValueWrapper {
    pub ty: ParamType,
    pub value: TokenStream,
}

impl FromMeta for ParamValueWrapper {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        // println!("expr = {:?}", expr);
        let meta_type_err = darling::Error::custom(META_TYPE_ERR);
        
        match expr {
            syn::Expr::Call(syn::ExprCall{ func, args, ..}) => {
                let ty = match func.as_ref() {
                    syn::Expr::Path(path) => {
                        ParamTypeWrapper::from_string(&path.path.segments.first().unwrap().ident.to_string())?
                    },
                    _ => return Err(meta_type_err),
                };

                let value = if args.len() == 1 {
                    &args[0]
                } else {
                    return Err(meta_type_err);
                };

                Ok(ParamValueWrapper {
                    ty: ty.0,
                    value: value.to_token_stream(),
                })
            },
            _ => return Err(meta_type_err),
        }

        // match meta {
        //     syn::Meta::NameValue(nv) => {
        //         // let ty = ParamTypeWrapper::from_string(&nv.path.segments.first().unwrap().ident.to_string())?;
        //         let value = &nv.value;
        //         println!("value = {:?}", value);
        //         // .to_token_stream();
        //         // Ok(ParamValueWrapper {
        //         //     ty: ty.0,
        //         //     value,
        //         // })
        //     },
        //     _ => return Err(meta_type_err),
        // }
        // Err(meta_type_err)

        // Err(meta_type_err)
        // match expr {
        //     syn::Expr::Call(syn::ExprCall{ func, args, ..}) => {
        //         let ty = ParamTypeWrapper::from_string(&func.path.segments.first().unwrap().ident.to_string())?;
        //         Ok(ParamValueWrapper {
        //             ty: ty.0,
        //             value: list.tokens.clone(),
        //         })
        //     },
        //     _ => return Err(meta_type_err),
        // }
    }
}
