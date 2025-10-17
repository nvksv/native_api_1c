use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

use functions::{collectors::*, parse::parse_functions};
use props::{collectors::*, parse::parse_props};
use utils::{macros::tkn_err, str_literal_token};

mod constants;
mod functions;
mod parsers;
mod props;
mod utils;

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    match derive_result(&derive_input) {
        Ok(tokens) => tokens.into(),
        Err(tokens) => tokens.into(),
    }
}

fn derive_result(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let impl_block = build_impl_block(input).map_err(|darling_error| {
        let error_tokens = darling_error.write_errors();
        let error_tokens = quote! {
            compile_error!(#error_tokens);
        };
        error_tokens
    })?;

    Ok(quote! {
        #impl_block
    })
}

fn get_addin_name_from_attribute( input: &DeriveInput ) -> Result<Option<TokenStream>, syn::Error> {

    for attr in &input.attrs {
        if attr.path().is_ident("add_in") {
            let nested = match attr.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            for meta in nested {
                match &meta {
                    syn::Meta::NameValue(syn::MetaNameValue{path, value, ..}) if path.is_ident("name") => {
                        return Ok(Some( quote!{ #value } ));
                    },
                    _ => {},
                }
            }
        }
    };

    Ok(None)
}

// fn generate_addin_const

fn build_impl_block(input: &DeriveInput) -> Result<proc_macro2::TokenStream, darling::Error> {
    let struct_ident = &input.ident;
    let syn::Data::Struct(struct_data) = &input.data else {
        return tkn_err!(
            "AddIn can only be derived for structs",
            &struct_ident.span()
        );
    };

    let addin_name = if let Some(addin_name) = get_addin_name_from_attribute(input)? {
        addin_name
    } else {
        str_literal_token(&struct_ident.to_string(), struct_ident)?
    };

    let mut props = parse_props(struct_data)?;
    let mut functions = parse_functions(struct_data)?;

    let addin_name_const = Ident::new("ADDIN_NAME", Span::call_site());
    let addin_consts = quote! {
        const #addin_name_const: &'static native_api_1c::native_api_1c_core::widestring::U16CStr = const { native_api_1c::native_api_1c_core::widestring::u16cstr!(#addin_name) };
    };

    let pi = props.iter_mut().enumerate();
    let prop_consts = pi.collect::<PropConstantsCollector>().release()?;

    let pi = functions.iter_mut().enumerate();
    let func_consts = pi.collect::<FuncConstantsCollector>().release()?;

    let pi = props.iter().enumerate();
    let prop_definitions = [
        pi.clone().collect::<FindPropCollector>().release()?,
        pi.clone().collect::<GetNPropsCollector>().release()?,
        pi.clone().collect::<GetPropNameCollector>().release()?,
        pi.clone().collect::<IsPropReadableCollector>().release()?,
        pi.clone().collect::<IsPropWritableCollector>().release()?,
        pi.clone().collect::<GetPropValCollector>().release()?,
        pi.clone().collect::<SetPropValCollector>().release()?,
    ];

    let fi = functions.iter().enumerate();
    let func_definitions = [
        fi.clone().collect::<FindMethodCollector>().release()?,
        fi.clone().collect::<GetMethodNameCollector>().release()?,
        fi.clone().collect::<GetNMethodsCollector>().release()?,
        fi.clone().collect::<GetNParamsCollector>().release()?,
        fi.clone().collect::<HasReturnValueCollector>().release()?,
        fi.clone().collect::<CallAsProcCollector>().release()?,
        fi.clone().collect::<CallAsFuncCollector>().release()?,
        fi.clone()
            .collect::<GetParamDefValueCollector>()
            .release()?,
    ];

    let result = quote! {
        impl #struct_ident {
            #addin_consts
            #prop_consts
            #func_consts
        }

        impl native_api_1c::native_api_1c_core::interface::AddInWrapper for #struct_ident {
            fn init(&mut self, interface: &'static native_api_1c::native_api_1c_core::ffi::connection::Connection) -> bool {
                self.connection = std::sync::Arc::new(Some(interface));
                true
            }

            fn get_info(&self) -> u16 {
                2000
            }

            fn done(&mut self) {}
            
            fn register_extension_as(&mut self) -> &native_api_1c::native_api_1c_core::widestring::U16CStr {
                Self::#addin_name_const
            }

            #(#prop_definitions)*
            #(#func_definitions)*

            fn set_locale(&mut self, loc: &native_api_1c::native_api_1c_core::widestring::U16CStr) {
            }

            fn set_user_interface_language_code(&mut self, lang: &native_api_1c::native_api_1c_core::widestring::U16CStr) {
            }
        }
    };
    Ok(result)
}
