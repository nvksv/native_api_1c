use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

use native_api_1c_core::interface::ParamValue;
use crate::derive_addin::{functions::{FuncDesc, FuncParamType}, parsers::ParamValueWrapper};
use super::{empty_func_collector_error, FunctionCollector};

pub struct GetParamDefValueCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for GetParamDefValueCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_func_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a FuncDesc)> for GetParamDefValueCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a FuncDesc)>>(iter: T) -> Self {
        let mut body = TokenStream::new();

        for (func_index, func_desc) in iter {
            for (arg_index, arg_desc) in func_desc.get_1c_params().iter().enumerate() {
                let default_value;

                if let Some(ParamValueWrapper{ ty, value}) = &arg_desc.optional {
                    let from_type_fn = Ident::new(ParamValue::from_type_fn_name(*ty), arg_desc.span);
                    default_value = quote_spanned! { arg_desc.span => 
                        native_api_1c::native_api_1c_core::interface::ParamValue::#from_type_fn(#value) 
                    };

                } else if let Some(expr) = &arg_desc.default {
                    let FuncParamType::PlatformType(ty) = &arg_desc.ty else {
                        // Skip parameters that is not platform type
                        continue;
                    };
                    let from_type_fn = Ident::new(ParamValue::from_type_fn_name(*ty), arg_desc.span);
                    default_value = quote_spanned! { arg_desc.span => 
                        native_api_1c::native_api_1c_core::interface::ParamValue::#from_type_fn(#expr) 
                    };
                } else {
                    continue;
                };

                body.extend(quote_spanned! { func_desc.ident.span() =>
                    (#func_index, #arg_index) => {
                        Some(#default_value)
                    },
                })
            }
        }

        let definition = quote! {
            fn get_param_def_value(
                &self,
                method_num: usize,
                param_num: usize,
            ) -> Option<native_api_1c::native_api_1c_core::interface::ParamValue> {
                match (method_num, param_num) {
                    #body
                    _ => {
                        None
                    }
                }
            }
        };

        Self {
            generated: Ok(definition),
        }
    }
}

impl FunctionCollector<'_> for GetParamDefValueCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
