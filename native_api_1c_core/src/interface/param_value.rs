use std::ops::{Index, IndexMut};

use crate::ffi::{provided_types::Tm};
use widestring::{U16CString, U16CStr};
use super::param_type::ParamType;

/// Represents 1C variant values for parameters in safe Rust code.
#[derive(Clone, Debug)]
pub enum ParamValue {
    /// Empty value
    Empty,
    /// Boolean value
    Bool(bool),
    /// Integer value
    I32(i32),
    /// Float value
    F64(f64),
    /// Date-time value
    Date(Tm),
    /// UTF-16 string value
    String(U16CString),
    /// Blob value
    Blob(Vec<u8>),
}

impl ParamValue {
    pub fn new_bool(val: bool) -> Self {
        Self::Bool(val)
    }

    pub fn new_i32(&mut self, val: i32) -> Self {
        Self::I32(val)
    }

    pub fn new_f64(&mut self, val: f64) -> Self {
        Self::F64(val)
    }

    pub fn new_date(&mut self, val: Tm) -> Self {
        Self::Date(val)
    }

    pub fn new_str(&mut self, val: U16CString) -> Self {
        Self::String(val)
    }

    pub fn new_blob(&mut self, val: Vec<u8>) -> Self {
        Self::Blob(val)
    }


    pub fn set_bool(&mut self, val: bool) {
        *self = Self::Bool(val);
    }

    pub fn set_i32(&mut self, val: i32) {
        *self = Self::I32(val);
    }

    pub fn set_f64(&mut self, val: f64) {
        *self = Self::F64(val);
    }

    pub fn set_date(&mut self, val: Tm) {
        *self = Self::Date(val);
    }

    pub fn set_str(&mut self, val: U16CString) {
        *self = Self::String(val);
    }

    pub fn set_blob(&mut self, val: Vec<u8>) {
        *self = Self::Blob(val);
    }
}

impl ParamValue {
    fn to_type_fn_name(ty: ParamType) -> &'static str {
        match ParamType {
            ParamType::Bool => {
                "to_bool"
            }
            ParamType::I32 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::I32 }
            }
            ParamType::F64 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::F64 }
            }
            ParamType::Date => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Date }
            }
            ParamType::String => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::String }
            }
            ParamType::Blob => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Blob }
            }
        }
    }
}

impl PartialEq for ParamValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::I32(a), Self::I32(b)) => a == b,
            (Self::F64(a), Self::F64(b)) => a == b,
            (Self::Date(a), Self::Date(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Blob(a), Self::Blob(b)) => a == b,
            _ => false,
        }
    }
}

/// Represents 1C variant values for return values in safe Rust code.
/// Only creator of the object can set the initial value, therefor has
/// control over count of values.
#[derive(Clone)]
pub struct ParamValues {
    values: Vec<ParamValue>,
}

impl ParamValues {
    pub fn new(values: Vec<ParamValue>) -> Self {
        Self { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ParamValue> {
        self.values.iter()
    }
}

impl Index<usize> for ParamValues {
    type Output = ParamValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for ParamValues {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}
