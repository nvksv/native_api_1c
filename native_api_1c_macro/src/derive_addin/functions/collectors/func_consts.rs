use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::Ident;

use crate::derive_addin::functions::FuncDesc;

use super::{empty_func_collector_error, FunctionCollectorMut};

pub struct FuncConstantsCollector {
    generated: Result<TokenStream, darling::Error>,
}

impl Default for FuncConstantsCollector {
    fn default() -> Self {
        Self {
            generated: Err(empty_func_collector_error()),
        }
    }
}

impl<'a> FromIterator<(usize, &'a mut FuncDesc)> for FuncConstantsCollector {
    fn from_iter<T: IntoIterator<Item = (usize, &'a mut FuncDesc)>>(iter: T) -> Self {
        let mut body = TokenStream::new();

        for (func_index, func_desc) in iter {
            let name_literal = &func_desc.name_literal;
            let name_ru_literal = &func_desc.name_ru_literal;

            let name_const = Ident::new(&format!("ADDIN_FUNC_NAME_{}", func_index + 1), func_desc.ident.span());
            let name_ru_const = Ident::new(&format!("ADDIN_FUNC_NAME_RU_{}", func_index + 1), func_desc.ident.span());
            let name_slice_const = Ident::new(&format!("ADDIN_FUNC_NAME_{}_SLICE", func_index + 1), func_desc.ident.span());
            let name_ru_slice_const = Ident::new(&format!("ADDIN_FUNC_NAME_RU_{}_SLICE", func_index + 1), func_desc.ident.span());

            body.extend(quote_spanned! { func_desc.ident.span() =>
                const #name_const: &'static native_api_1c::native_api_1c_core::widestring::U16CStr = const { native_api_1c::native_api_1c_core::widestring::u16cstr!(#name_literal) };
                const #name_slice_const: &'static [native_api_1c::native_api_1c_core::widestring::internals::core::primitive::u16] = const { Self::#name_const.as_slice_with_nul() };
                const #name_ru_const: &'static native_api_1c::native_api_1c_core::widestring::U16CStr = const { native_api_1c::native_api_1c_core::widestring::u16cstr!(#name_ru_literal) };
                const #name_ru_slice_const: &'static [native_api_1c::native_api_1c_core::widestring::internals::core::primitive::u16] = const { Self::#name_ru_const.as_slice_with_nul() };
            });

            func_desc.name_const = name_const.into_token_stream();
            func_desc.name_ru_const = name_ru_const.into_token_stream();
            func_desc.name_slice_const = name_slice_const.into_token_stream();
            func_desc.name_ru_slice_const = name_ru_slice_const.into_token_stream();
        }

        Self {
            generated: Ok(body),
        }
    }
}

impl FunctionCollectorMut<'_> for FuncConstantsCollector {
    fn release(self) -> Result<TokenStream, darling::Error> {
        self.generated
    }
}
