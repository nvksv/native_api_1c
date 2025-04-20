use super::{get_str, offset, provided_types::TVariant};
use crate::interface::{AddInWrapper, ParamValue, ParamValues};
use std::{
    ffi::c_long,
    ptr::{self},
    slice::from_raw_parts_mut,
};

type This<T> = super::This<{ offset::LANG_EXTENDER }, T>;

#[repr(C)]
pub struct LanguageExtenderBaseVTable<T: AddInWrapper> {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    register_extension_as:
        unsafe extern "system" fn(&mut This<T>, *mut *mut u16) -> bool,
    get_n_props: unsafe extern "system" fn(&mut This<T>) -> c_long,
    find_prop: unsafe extern "system" fn(&mut This<T>, *const u16) -> c_long,
    get_prop_name:
        unsafe extern "system" fn(&mut This<T>, c_long, c_long) -> *const u16,
    get_prop_val:
        unsafe extern "system" fn(&mut This<T>, c_long, &mut TVariant) -> bool,
    set_prop_val:
        unsafe extern "system" fn(&mut This<T>, c_long, &TVariant) -> bool,
    is_prop_readable: unsafe extern "system" fn(&mut This<T>, c_long) -> bool,
    is_prop_writable: unsafe extern "system" fn(&mut This<T>, c_long) -> bool,
    get_n_methods: unsafe extern "system" fn(&mut This<T>) -> c_long,
    find_method: unsafe extern "system" fn(&mut This<T>, *const u16) -> c_long,
    get_method_name:
        unsafe extern "system" fn(&mut This<T>, c_long, c_long) -> *const u16,
    get_n_params: unsafe extern "system" fn(&mut This<T>, c_long) -> c_long,
    get_param_def_value: unsafe extern "system" fn(
        &mut This<T>,
        c_long,
        c_long,
        &mut TVariant,
    ) -> bool,
    has_ret_val: unsafe extern "system" fn(&mut This<T>, c_long) -> bool,
    call_as_proc: unsafe extern "system" fn(
        &mut This<T>,
        c_long,
        *mut TVariant,
        c_long,
    ) -> bool,
    call_as_func: unsafe extern "system" fn(
        &mut This<T>,
        c_long,
        &mut TVariant,
        *mut TVariant,
        c_long,
    ) -> bool,
}

unsafe extern "system" fn register_extension_as<T: AddInWrapper>(
    this: &mut This<T>,
    name: *mut *mut u16,
) -> bool {
    let component = this.get_component();
    let Some(allocator) = component.memory_manager_ptr else {
        return false;
    };

    let extension_name = component.addin.register_extension_as();

    let Ok(ptr) = allocator.alloc_str(extension_name.len()) else {
        return false;
    };
    ptr::copy_nonoverlapping(
        extension_name.as_ptr(),
        ptr.as_ptr(),
        extension_name.len(),
    );
    *name = ptr.as_ptr();

    true
}

unsafe extern "system" fn get_n_props<T: AddInWrapper>(
    this: &mut This<T>,
) -> c_long {
    let component = this.get_component();
    component.addin.get_n_props() as c_long
}

unsafe extern "system" fn find_prop<T: AddInWrapper>(
    this: &mut This<T>,
    name: *const u16,
) -> c_long {
    let component = this.get_component();
    let name = get_str(name);
    match component.addin.find_prop(name) {
        Some(i) => i as c_long,
        None => -1,
    }
}

unsafe extern "system" fn get_prop_name<T: AddInWrapper>(
    this: &mut This<T>,
    num: c_long,
    alias: c_long,
) -> *const u16 {
    let component = this.get_component();
    let Some(allocator) = component.memory_manager_ptr else {
        return ptr::null();
    };
    let Some(prop_name) =
        component.addin.get_prop_name(num as usize, alias as usize)
    else {
        return ptr::null();
    };
    let Ok(ptr) = allocator.alloc_str(prop_name.len()) else {
        return ptr::null();
    };
    ptr::copy_nonoverlapping(prop_name.as_ptr(), ptr.as_ptr(), prop_name.len());

    ptr.as_ptr()
}

unsafe extern "system" fn get_prop_val<T: AddInWrapper>(
    this: &mut This<T>,
    num: c_long,
    val: &mut TVariant,
) -> bool {
    let component = this.get_component();
    let Some(mem_mngr) = component.memory_manager_ptr else {
        return false;
    };

    let prop_val_result = component.addin.get_prop_val(num as usize);
    match prop_val_result {
        Ok(prop_val) => {
            val.update_from_return(mem_mngr, &prop_val);
            true
        }
        Err(_) => false,
    }
}

unsafe extern "system" fn set_prop_val<T: AddInWrapper>(
    this: &mut This<T>,
    num: c_long,
    val: &TVariant,
) -> bool {
    this.get_component()
        .addin
        .set_prop_val(num as usize, val.into())
        .is_ok()
}

unsafe extern "system" fn is_prop_readable<T: AddInWrapper>(
    this: &mut This<T>,
    num: c_long,
) -> bool {
    this.get_component().addin.is_prop_readable(num as usize)
}

unsafe extern "system" fn is_prop_writable<T: AddInWrapper>(
    this: &mut This<T>,
    num: c_long,
) -> bool {
    let component = this.get_component();
    component.addin.is_prop_writable(num as usize)
}

unsafe extern "system" fn get_n_methods<T: AddInWrapper>(
    this: &mut This<T>,
) -> c_long {
    let component = this.get_component();
    component.addin.get_n_methods() as c_long
}

unsafe extern "system" fn find_method<T: AddInWrapper>(
    this: &mut This<T>,
    name: *const u16,
) -> c_long {
    let component = this.get_component();
    let name = get_str(name);
    match component.addin.find_method(name) {
        Some(i) => i as c_long,
        None => -1,
    }
}

unsafe extern "system" fn get_method_name<T: AddInWrapper>(
    this: &mut This<T>,
    num: c_long,
    alias: c_long,
) -> *const u16 {
    let component = this.get_component();
    let Some(allocator) = component.memory_manager_ptr else {
        return ptr::null();
    };
    let Some(method_name) = component
        .addin
        .get_method_name(num as usize, alias as usize)
    else {
        return ptr::null();
    };
    let Ok(ptr) = allocator.alloc_str(method_name.len()) else {
        return ptr::null();
    };

    ptr::copy_nonoverlapping(
        method_name.as_ptr(),
        ptr.as_ptr(),
        method_name.len(),
    );

    ptr.as_ptr()
}

unsafe extern "system" fn get_n_params<T: AddInWrapper>(
    this: &mut This<T>,
    num: c_long,
) -> c_long {
    let component = this.get_component();
    component.addin.get_n_params(num as usize) as c_long
}

unsafe extern "system" fn get_param_def_value<T: AddInWrapper>(
    this: &mut This<T>,
    method_num: c_long,
    param_num: c_long,
    val: &mut TVariant,
) -> bool {
    let component = this.get_component();
    let Some(mem) = component.memory_manager_ptr else {
        return false;
    };

    let def_value_result = component
        .addin
        .get_param_def_value(method_num as usize, param_num as usize);
    match def_value_result {
        Some(def_value) => {
            val.update_from_return(mem, &def_value);
            true
        }
        None => false,
    }
}

unsafe extern "system" fn has_ret_val<T: AddInWrapper>(
    this: &mut This<T>,
    method_num: c_long,
) -> bool {
    let component = this.get_component();
    component.addin.has_ret_val(method_num as usize)
}

unsafe extern "system" fn call_as_proc<T: AddInWrapper>(
    this: &mut This<T>,
    method_num: c_long,
    params: *mut TVariant,
    size_array: c_long,
) -> bool {
    let component = this.get_component();
    let Some(mem_mngr) = component.memory_manager_ptr else {
        return false;
    };

    let parameters_raw = if params.is_null() && size_array == 0 { 
        &mut [] 
    } else {
        from_raw_parts_mut(params, size_array as usize)
    };
    
    let mut parameters_values =
        ParamValues::new(parameters_raw.iter().map(ParamValue::from).collect());

    let call_result = component
        .addin
        .call_as_proc(method_num as usize, &mut parameters_values);

    if call_result.is_err() {
        return false;
    }

    for (i, param) in parameters_values.iter().enumerate() {
        parameters_raw[i].update_from_return(mem_mngr, param);
    }

    true
}

unsafe extern "system" fn call_as_func<T: AddInWrapper>(
    this: &mut This<T>,
    method_num: c_long,
    ret_value: &mut TVariant,
    params: *mut TVariant,
    size_array: c_long,
) -> bool {
    let component = this.get_component();
    let Some(mem_mngr) = component.memory_manager_ptr else {
        return false;
    };

    let parameters_raw = from_raw_parts_mut(params, size_array as usize);
    let mut parameters_values =
        ParamValues::new(parameters_raw.iter().map(ParamValue::from).collect());

    let call_result = component
        .addin
        .call_as_func(method_num as usize, &mut parameters_values);

    let Ok(ret_val) = call_result else {
        return false;
    };

    ret_value.update_from_return(mem_mngr, &ret_val);

    for (i, param) in parameters_values.iter().enumerate() {
        parameters_raw[i].update_from_return(mem_mngr, param);
    }

    true
}

impl<T: AddInWrapper> Default for LanguageExtenderBaseVTable<T> {
    fn default() -> Self {
        Self {
            dtor: 0,
            #[cfg(target_family = "unix")]
            dtor2: 0,
            register_extension_as,
            get_n_props,
            find_prop,
            get_prop_name,
            get_prop_val,
            set_prop_val,
            is_prop_readable,
            is_prop_writable,
            get_n_methods,
            find_method,
            get_method_name,
            get_n_params,
            get_param_def_value,
            has_ret_val,
            call_as_proc,
            call_as_func,
        }
    }
}
