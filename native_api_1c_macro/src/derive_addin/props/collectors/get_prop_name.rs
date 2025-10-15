use proc_macro2::TokenStream;
use quote::quote;

use crate::derive_addin::props::PropDesc;

use super::{empty_prop_collector_error, PropCollector};

pub struct GetPropNameCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for GetPropNameCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_prop_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a PropDesc)> for GetPropNameCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a PropDesc)>>(iter: T) -> Self {
        let mut get_prop_name_body = TokenStream::new();

        for (prop_index, prop_desc) in iter {
            let name_const = &prop_desc.name_const;
            let name_ru_const = &prop_desc.name_ru_const;

            get_prop_name_body.extend(quote! {
                (#prop_index, 0) => { 
                    Some(Self::#name_const) 
                },
                (#prop_index, _) => {
                    Some(Self::#name_ru_const) 
                },
            });
        }

        let _definition = quote! {
            fn get_prop_name(&self, num: usize, alias: usize) -> Option<&native_api_1c::native_api_1c_core::widestring::U16CStr> {
                match (num, alias) {
                    #get_prop_name_body
                    _ => {
                        None
                    }
                }
            }
        };

        Self {
            generated: Ok(_definition),
        }
    }
}

impl PropCollector<'_> for GetPropNameCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
