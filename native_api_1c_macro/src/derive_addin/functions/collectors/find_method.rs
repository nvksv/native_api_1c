use proc_macro2::TokenStream;
use quote::quote;

use crate::derive_addin::functions::FuncDesc;

use super::{empty_func_collector_error, FunctionCollector};

pub struct FindMethodCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for FindMethodCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_func_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a FuncDesc)> for FindMethodCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a FuncDesc)>>(iter: T) -> Self {
        let mut find_method_body = TokenStream::new();

        for (func_index, func_desc) in iter {
            let name_slice_const = &func_desc.name_slice_const;
            let name_ru_slice_const = &func_desc.name_ru_slice_const;

            find_method_body.extend(quote! {
                Self::#name_slice_const | Self::#name_ru_slice_const => { 
                    Some(#func_index)
                },
            });
        }

        let find_method_definition = quote! {
            fn find_method(&self, name: &native_api_1c::native_api_1c_core::widestring::U16CStr) -> Option<usize> {
                match name.as_slice_with_nul() {
                    #find_method_body
                    _ => {
                        None
                    }
                }
            }
        };

        Self {
            generated: Ok(find_method_definition),
        }
    }
}

impl FunctionCollector<'_> for FindMethodCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
