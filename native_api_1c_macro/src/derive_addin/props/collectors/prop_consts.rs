use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::Ident;

use crate::derive_addin::props::PropDesc;

use super::{empty_prop_collector_error, PropCollectorMut};

pub struct PropConstantsCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for PropConstantsCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_prop_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a mut PropDesc)> for PropConstantsCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a mut PropDesc)>>(iter: T) -> Self {
        let mut body = TokenStream::new();

        for (prop_index, prop_desc) in iter {
            let name_literal = &prop_desc.name_literal;
            let name_ru_literal = &prop_desc.name_ru_literal;

            let name_const = Ident::new(&format!("ADDIN_PROP_NAME_{}", prop_index + 1), prop_desc.ident.span());
            let name_ru_const = Ident::new(&format!("ADDIN_PROP_NAME_RU_{}", prop_index + 1), prop_desc.ident.span());
            let name_slice_const = Ident::new(&format!("ADDIN_PROP_NAME_{}_SLICE", prop_index + 1), prop_desc.ident.span());
            let name_ru_slice_const = Ident::new(&format!("ADDIN_PROP_NAME_RU_{}_SLICE", prop_index + 1), prop_desc.ident.span());

            body.extend(quote_spanned! { prop_desc.ident.span() =>
                const #name_const: &'static native_api_1c::native_api_1c_core::widestring::U16CStr = const { native_api_1c::native_api_1c_core::widestring::u16cstr!(#name_literal) };
                const #name_ru_const: &'static native_api_1c::native_api_1c_core::widestring::U16CStr = const { native_api_1c::native_api_1c_core::widestring::u16cstr!(#name_ru_literal) };
                const #name_slice_const: &'static [native_api_1c::native_api_1c_core::widestring::internals::core::primitive::u16] = const { Self::#name_const.as_slice_with_nul() };
                const #name_ru_slice_const: &'static [native_api_1c::native_api_1c_core::widestring::internals::core::primitive::u16] = const { Self::#name_ru_const.as_slice_with_nul() };
            });

            prop_desc.name_const = name_const.into_token_stream();
            prop_desc.name_ru_const = name_ru_const.into_token_stream();
            prop_desc.name_slice_const = name_slice_const.into_token_stream();
            prop_desc.name_ru_slice_const = name_ru_slice_const.into_token_stream();
        }

        Self {
            generated: Ok(body),
        }
    }
}

impl PropCollectorMut<'_> for PropConstantsCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
