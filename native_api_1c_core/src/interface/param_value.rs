use std::ops::{Index, IndexMut};

use crate::ffi::{provided_types::Tm};
use widestring::U16CString;
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
    pub fn new_empty() -> Self {
        Self::Empty
    }

    pub fn from_bool(val: impl Into<bool>) -> Self {
        Self::Bool(val.into())
    }

    pub fn from_i32(val: impl Into<i32>) -> Self {
        Self::I32(val.into())
    }

    pub fn from_f64(val: impl Into<f64>) -> Self {
        Self::F64(val.into())
    }

    pub fn from_date(val: impl Into<Tm>) -> Self {
        Self::Date(val.into())
    }

    pub fn from_str(val: impl AsRef<str>) -> Self {
        Self::String(U16CString::from_str_truncate(val))
    }

    pub fn from_blob(val: impl Into<Vec<u8>>) -> Self {
        Self::Blob(val.into())
    }

    //

    pub fn into_bool(self) -> Option<bool> {
        match self {
            ParamValue::Bool(v) => Some(v),
            _ => None
        }
    }

    pub fn into_i32(self) -> Option<i32> {
        match self {
            ParamValue::I32(v) => Some(v),
            _ => None
        }
    }

    pub fn into_f64(self) -> Option<f64> {
        match self {
            ParamValue::F64(v) => Some(v),
            _ => None
        }
    }

    pub fn into_date(self) -> Option<Tm> {
        match self {
            ParamValue::Date(v) => Some(v),
            _ => None
        }
    }

    pub fn into_str(self) -> Option<String> {
        match self {
            ParamValue::String(v) => Some(v.to_string_lossy()),
            _ => None
        }
    }

    pub fn into_blob(self) -> Option<Vec<u8>> {
        match self {
            ParamValue::Blob(v) => Some(v),
            _ => None
        }
    }

    //

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            ParamValue::Bool(v) => Some(*v),
            _ => None
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            ParamValue::I32(v) => Some(*v),
            _ => None
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            ParamValue::F64(v) => Some(*v),
            _ => None
        }
    }

    pub fn to_date(&self) -> Option<Tm> {
        match self {
            ParamValue::Date(v) => Some(*v),
            _ => None
        }
    }

    pub fn to_str(&self) -> Option<String> {
        match self {
            ParamValue::String(v) => Some(v.to_string_lossy()),
            _ => None
        }
    }

    pub fn to_blob(&self) -> Option<Vec<u8>> {
        match self {
            ParamValue::Blob(v) => Some(v.clone()),
            _ => None
        }
    }

    //

    pub fn to_optional_bool(&self, none_value: &ParamValue) -> Option<Option<bool>> {
        if self == none_value {
            Some(None)
        } else {
            Self::to_bool(self)
                .map( |value| Some(value) )
        }
    }

    pub fn to_optional_i32(&self, none_value: &ParamValue) -> Option<Option<i32>> {
        if self == none_value {
            Some(None)
        } else {
            Self::to_i32(self)
                .map( |value| Some(value) )
        }
    }

    pub fn to_optional_f64(&self, none_value: &ParamValue) -> Option<Option<f64>> {
        if self == none_value {
            Some(None)
        } else {
            Self::to_f64(self)
                .map( |value| Some(value) )
        }
    }

    pub fn to_optional_date(&self, none_value: &ParamValue) -> Option<Option<Tm>> {
        if self == none_value {
            Some(None)
        } else {
            Self::to_date(self)
                .map( |value| Some(value) )
        }
    }

    pub fn to_optional_str(&self, none_value: &ParamValue) -> Option<Option<String>> {
        if self == none_value {
            Some(None)
        } else {
            Self::to_str(self)
                .map( |value| Some(value) )
        }
    }

    pub fn to_optional_blob(&self, none_value: &ParamValue) -> Option<Option<Vec<u8>>> {
        if self == none_value {
            Some(None)
        } else {
            Self::to_blob(self)
                .map( |value| Some(value) )
        }
    }

    //

    pub fn set_bool(&mut self, val: impl Into<bool>) {
        *self = Self::Bool(val.into());
    }

    pub fn set_i32(&mut self, val: impl Into<i32>) {
        *self = Self::I32(val.into());
    }

    pub fn set_f64(&mut self, val: impl Into<f64>) {
        *self = Self::F64(val.into());
    }

    pub fn set_date(&mut self, val: impl Into<Tm>) {
        *self = Self::Date(val.into());
    }

    pub fn set_str(&mut self, val: impl AsRef<str>) {
        *self = Self::String(U16CString::from_str_truncate(val));
    }

    pub fn set_blob(&mut self, val: impl Into<Vec<u8>>) {
        *self = Self::Blob(val.into());
    }

    //


}

impl ParamValue {
    pub fn from_type_fn_name(ty: ParamType) -> &'static str {
        match ty {
            ParamType::Bool => {
                "from_bool"
            }
            ParamType::I32 => {
                "from_i32"
            }
            ParamType::F64 => {
                "from_f64"
            }
            ParamType::Date => {
                "from_date"
            }
            ParamType::String => {
                "from_str"
            }
            ParamType::Blob => {
                "from_blob"
            }
        }
    }

    pub fn into_type_fn_name(ty: ParamType) -> &'static str {
        match ty {
            ParamType::Bool => {
                "into_bool"
            }
            ParamType::I32 => {
                "into_i32"
            }
            ParamType::F64 => {
                "into_f64"
            }
            ParamType::Date => {
                "into_date"
            }
            ParamType::String => {
                "into_str"
            }
            ParamType::Blob => {
                "into_blob"
            }
        }
    }

    pub fn to_type_fn_name(ty: ParamType) -> &'static str {
        match ty {
            ParamType::Bool => {
                "to_bool"
            }
            ParamType::I32 => {
                "to_i32"
            }
            ParamType::F64 => {
                "to_f64"
            }
            ParamType::Date => {
                "to_date"
            }
            ParamType::String => {
                "to_str"
            }
            ParamType::Blob => {
                "to_blob"
            }
        }
    }

    pub fn to_optional_type_fn_name(ty: ParamType) -> &'static str {
        match ty {
            ParamType::Bool => {
                "to_optional_bool"
            }
            ParamType::I32 => {
                "to_optional_i32"
            }
            ParamType::F64 => {
                "to_optional_f64"
            }
            ParamType::Date => {
                "to_optional_date"
            }
            ParamType::String => {
                "to_optional_str"
            }
            ParamType::Blob => {
                "to_optional_blob"
            }
        }
    }

    pub fn set_type_fn_name(ty: ParamType) -> &'static str {
        match ty {
            ParamType::Bool => {
                "set_bool"
            }
            ParamType::I32 => {
                "set_i32"
            }
            ParamType::F64 => {
                "set_f64"
            }
            ParamType::Date => {
                "set_date"
            }
            ParamType::String => {
                "set_str"
            }
            ParamType::Blob => {
                "set_blob"
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
