use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use crate::derive_addin::props::PropDesc;

use super::{empty_prop_collector_error, PropCollector};

pub struct IsPropWritableCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for IsPropWritableCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_prop_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a PropDesc)> for IsPropWritableCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a PropDesc)>>(iter: T) -> Self {
        let mut is_prop_writable_body = TokenStream::new();

        for (prop_index, prop_desc) in iter {
            let writable = prop_desc.writable;
            is_prop_writable_body.extend(quote_spanned! { prop_desc.ident.span() =>
                #prop_index => {
                    #writable
                },
            });
        }

        let _definition = quote! {
            fn is_prop_writable(&self, num: usize) -> bool {
                match num {
                    #is_prop_writable_body
                    _ => {
                        false
                    }
                }
            }
        };

        Self {
            generated: Ok(_definition),
        }
    }
}

impl PropCollector<'_> for IsPropWritableCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
