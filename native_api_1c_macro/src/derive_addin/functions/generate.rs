use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

use native_api_1c_core::interface::ParamValue;
use super::{FuncArgumentDesc, FuncDesc, FuncParamType};

pub fn func_call_tkn(func: &FuncDesc, set_to: Option<&Ident>) -> TokenStream {
    let func_ident = func.ident.clone();

    let mut pre_call = quote! {};
    let mut func_args = quote! {};
    let mut post_call = quote! {};

    for (param_index, param_desc) in func.get_1c_params().iter().enumerate() {
        let param_ident = Ident::new(&format!("param_{}", param_index + 1), param_desc.span);
        let param_val_ident = Ident::new(&format!("param_val_{}", param_index + 1), param_desc.span);

        let (pre_call_param, post_call_param) =
            gen_param_prep(param_desc, param_index, &param_ident, &param_val_ident);

        if !func_args.is_empty() {
            func_args.extend(quote! {,});
        }
        func_args.extend(quote_spanned! { param_desc.span => 
            #param_ident 
        });

        pre_call.extend(pre_call_param);
        post_call.extend(post_call_param);
    }

    if func.has_self_param() {
        if func_args.is_empty() {
            func_args = quote_spanned! { func.ident.span() => 
                self 
            };
        } else {
            func_args = quote_spanned! { func.ident.span() => 
                self, #func_args 
            };
        }
    }

    let func_call_fn_with_args = quote_spanned! { func.ident.span() => 
        (self.#func_ident)(#func_args)
    };

    let mut func_call = quote!{};
    if let Some(set_to) = set_to {
        if func.return_value.result {
            func_call.extend(quote_spanned! { func.ident.span() => 
                let call_result = (#func_call_fn_with_args)?;
            });
        } else {
            func_call.extend(quote_spanned! { func.ident.span() => 
                let call_result = #func_call_fn_with_args;
            })
        };

        let return_ty = func.return_value.ty.clone().unwrap();
        let from_type_fn = Ident::new(ParamValue::from_type_fn_name(return_ty), func.ident.span());
        func_call.extend(quote_spanned! { func.ident.span() =>
            let #set_to = native_api_1c::native_api_1c_core::interface::ParamValue::#from_type_fn(call_result);
        });
    } else {
        if func.return_value.result {
            func_call.extend(quote_spanned! { func.ident.span() => 
                (#func_call_fn_with_args)?;
            });
        } else {
            func_call.extend(quote_spanned! { func.ident.span() => 
                #func_call_fn_with_args;
            })
        };
    }

    quote_spanned! { func.ident.span() =>
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

    let to_type_fn = Ident::new(
        if param.optional { ParamValue::to_optional_type_fn_name(*param_ty) } else { ParamValue::to_type_fn_name(*param_ty) }, 
        param.span
    );

    let param_value = quote_spanned! { param.span =>
        native_api_1c::native_api_1c_core::interface::ParamValue::#to_type_fn(&params[#param_index])
        .ok_or(())?
        .into() 
    };

    let pre_call = if param.out_param {
        quote_spanned! { param.span =>
            let mut #param_val_ident = #param_value;
            let #param_ident = &mut #param_val_ident;
        }
    } else {
        quote_spanned! { param.span =>
            let #param_ident = #param_value;
        }
    };

    let post_call = if !param.out_param {
        quote! {}
    } else {
        let set_type_fn = Ident::new(ParamValue::set_type_fn_name(*param_ty), param.span);
        quote_spanned! { param.span =>
            params[#param_index].#set_type_fn( #param_val_ident );
        }
    };

    (pre_call, post_call)
}
