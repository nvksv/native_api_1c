use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use native_api_1c_core::interface::ParamValue;
use super::{FuncArgumentDesc, FuncDesc, FuncParamType};

pub fn func_call_tkn(func: &FuncDesc, set_to: Option<&Ident>) -> TokenStream {
    let func_ident = func.ident.clone();

    let mut pre_call = quote! {};
    let mut func_args = quote! {};
    let mut post_call = quote! {};

    for (param_index, param_desc) in func.get_1c_params().iter().enumerate() {
        let param_ident = Ident::new(&format!("param_{}", param_index + 1), Span::call_site());
        let param_val_ident = Ident::new(&format!("param_val_{}", param_index + 1), Span::call_site());

        let (pre_call_param, post_call_param) =
            gen_param_prep(param_desc, param_index, &param_ident, &param_val_ident);

        if func_args.is_empty() {
            func_args.extend(quote! {#param_ident})
        } else {
            func_args.extend(quote! {, #param_ident});
        }

        pre_call.extend(pre_call_param);
        post_call.extend(post_call_param);
    }

    if func.has_self_param() {
        if func_args.is_empty() {
            func_args = quote! {self};
        } else {
            func_args = quote! {self, #func_args};
        }
    }

    let mut func_call = quote! {
        let call_result = (self.#func_ident)(#func_args);
    };

    if func.return_value.result {
        func_call.extend(quote! {
            if call_result.is_err() {
                return Err(());
            }
            let call_result = call_result.unwrap();
        });
    };

    if let Some(set_to) = set_to {
        let return_ty = func.return_value.ty.clone().unwrap();
        let from_type_fn = Ident::new(ParamValue::from_type_fn_name(return_ty), Span::call_site());
        func_call.extend(quote! {
            let #set_to = native_api_1c::native_api_1c_core::interface::ParamValue::#from_type_fn(call_result);
        });
    }

    quote! {
        #pre_call
        #func_call
        #post_call
    }
}

fn gen_param_prep(
    param: &FuncArgumentDesc,
    param_index: usize,
    param_ident: &Ident,
    param_val_ident: &Ident,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let FuncParamType::PlatformType(param_ty) = &param.ty else {
        panic!("SelfType is not allowed here");
    };

    let to_type_fn = Ident::new(ParamValue::to_type_fn_name(*param_ty), Span::call_site());

    // let param_unwrap = expr_from_os_value(&quote! { params[#param_index] }, param_ty);
    let param_value = quote! { 
        native_api_1c::native_api_1c_core::interface::ParamValue::#to_type_fn(&params[#param_index])
        .ok_or(())?
        .into() 
    };
    let pre_call = if param.out_param {
        quote! {
            let mut #param_val_ident = #param_value;
            let #param_ident = &mut #param_val_ident;
        }
    } else {
        quote! {
            let #param_ident = #param_value;
        }
    };

    let post_call = if !param.out_param {
        quote! {}
    } else {
        let set_type_fn = Ident::new(ParamValue::set_type_fn_name(*param_ty), Span::call_site());
        // let param_wrap = expr_to_os_value(&param_ident.to_token_stream(), param_ty, false);
        quote! {
            params[#param_index].#set_type_fn( #param_val_ident );
        }
    };

    (pre_call, post_call)
}
