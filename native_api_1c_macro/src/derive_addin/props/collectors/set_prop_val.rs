use native_api_1c_core::interface::ParamValue;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

use crate::derive_addin::{props::PropDesc};

use super::{empty_prop_collector_error, PropCollector};

pub struct SetPropValCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for SetPropValCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_prop_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a PropDesc)> for SetPropValCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a PropDesc)>>(iter: T) -> Self {
        let mut body = TokenStream::new();

        for (prop_index, prop_desc) in iter {
            if !prop_desc.writable {
                continue;
            }

            let prop_ident = &prop_desc.ident;
            let into_type_fn = Ident::new(ParamValue::into_type_fn_name(prop_desc.ty), prop_desc.ident.span());

            body.extend(quote_spanned! { prop_desc.ident.span() =>
                #prop_index => {
                    self.#prop_ident = native_api_1c::native_api_1c_core::interface::ParamValue::#into_type_fn(val)
                        .ok_or(())?
                        .into();
                    Ok(())
                },
            });
        }

        let definition = quote! {
            fn set_prop_val(
                &mut self,
                num: usize,
                val: native_api_1c::native_api_1c_core::interface::ParamValue,
            ) -> native_api_1c::native_api_1c_core::interface::AddInWrapperResult<()> {
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

impl PropCollector<'_> for SetPropValCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
