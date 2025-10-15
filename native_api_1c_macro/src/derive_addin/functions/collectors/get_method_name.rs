use proc_macro2::TokenStream;
use quote::quote;

use crate::derive_addin::functions::FuncDesc;

use super::{empty_func_collector_error, FunctionCollector};

pub struct GetMethodNameCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for GetMethodNameCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_func_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a FuncDesc)> for GetMethodNameCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a FuncDesc)>>(iter: T) -> Self {
        let mut get_func_name_body = TokenStream::new();

        for (func_index, func_desc) in iter {
            let name_const = &func_desc.name_const;
            let name_ru_const = &func_desc.name_ru_const;

            get_func_name_body.extend(quote! {
                (#func_index, 0) => {
                    Some(Self::#name_const)
                },
                (#func_index, _) => {
                    Some(Self::#name_ru_const)
                },
            });
        }

        let get_func_name_definition = quote! {
            fn get_method_name(&self, num: usize, alias: usize) -> Option<&native_api_1c::native_api_1c_core::widestring::U16CStr> {
                match (num, alias) {
                    #get_func_name_body
                    _ => {
                        None
                    }
                }
            }
        };

        Self {
            generated: Ok(get_func_name_definition),
        }
    }
}

impl FunctionCollector<'_> for GetMethodNameCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
