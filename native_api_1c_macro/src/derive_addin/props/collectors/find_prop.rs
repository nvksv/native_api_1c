use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use crate::derive_addin::props::PropDesc;

use super::{empty_prop_collector_error, PropCollector};

pub struct FindPropCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for FindPropCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_prop_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a PropDesc)> for FindPropCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a PropDesc)>>(iter: T) -> Self {
        let mut find_prop_body = TokenStream::new();

        for (prop_index, prop_desc) in iter {
            let name_slice_const = &prop_desc.name_slice_const;
            let name_ru_slice_const = &prop_desc.name_ru_slice_const;

            find_prop_body.extend(quote_spanned! { prop_desc.ident.span() =>
                Self::#name_slice_const | Self::#name_ru_slice_const => { 
                    Some(#prop_index) 
                },
            });
        }

        let _definition = quote! {
            fn find_prop(&self, name: &native_api_1c::native_api_1c_core::widestring::U16CStr) -> Option<usize> {
                match name.as_slice_with_nul() {
                    #find_prop_body
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

impl PropCollector<'_> for FindPropCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
