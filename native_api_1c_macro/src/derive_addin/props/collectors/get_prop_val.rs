use native_api_1c_core::interface::ParamValue;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::derive_addin::props::PropDesc;

use super::{empty_prop_collector_error, PropCollector};

pub struct GetPropValCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for GetPropValCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_prop_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a PropDesc)> for GetPropValCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a PropDesc)>>(iter: T) -> Self {
        let mut body = TokenStream::new();

        for (prop_index, prop_desc) in iter {
            if !prop_desc.readable {
                // Skip non-readable properties
                continue;
            }

            let prop_ident = &prop_desc.ident;
            // let prop_setter = expr_to_os_value(&quote! {self.#prop_ident}, &prop_desc.ty, false);
            // body.extend(quote! {
            //     #prop_index => {
            //         Ok(#prop_setter)
            //     },
            // });
            let from_type_fn = Ident::new(ParamValue::from_type_fn_name(prop_desc.ty), Span::call_site());
            body.extend(quote! {
                #prop_index => {
                    Ok(native_api_1c::native_api_1c_core::interface::ParamValue::#from_type_fn(self.#prop_ident.clone()))
                },
            });
        }

        let definition = quote! {
            fn get_prop_val(&self, num: usize) -> native_api_1c::native_api_1c_core::interface::AddInWrapperResult<
                native_api_1c::native_api_1c_core::interface::ParamValue
            > {
                match num {
                    #body
                    _ => {
                        Err(())
                    }
                }
            }
        };

        Self {
            generated: Ok(definition),
        }
    }
}

impl PropCollector<'_> for GetPropValCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
