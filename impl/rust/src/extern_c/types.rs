use std::{
    collections::HashMap,
    ffi::{c_char, CString},
};

use crate::parse::{FennecType, ParseError};

#[repr(C)]
#[derive(Debug)]
pub enum CFennecType {
    Object(usize, *const *const c_char, *const CFennecType),
    Array(usize, *const CFennecType),
    String(*const c_char),
    Float(f64),
    Int(i64),
    Bool(bool),
    Null,
    Error,
}

impl CFennecType {
    fn transform_str(str: &String) -> *const c_char {
        CString::new(str.as_bytes()).unwrap().into_raw()
    }

    fn from_object(obj: &HashMap<String, FennecType>) -> Self {
        let (keys, _, _) = obj
            .keys()
            .map(Self::transform_str)
            .collect::<Vec<_>>()
            .into_raw_parts();

        let (values, len, _) = obj
            .values()
            .map(Self::from)
            .collect::<Vec<_>>()
            .into_raw_parts();

        Self::Object(len, keys, values)
    }

    fn from_array(arr: &Vec<FennecType>) -> Self {
        let (ptr, len, _) = arr
            .iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .into_raw_parts();

        Self::Array(len, ptr)
    }

    fn from_string(str: &String) -> Self {
        let c_str = Self::transform_str(str);

        Self::String(c_str)
    }
}

impl From<&Result<FennecType, ParseError>> for CFennecType {
    fn from(value: &Result<FennecType, ParseError>) -> Self {
        match value {
            Ok(fen) => fen.into(),
            Err(e) => {
                println!("{e:?}");
                Self::Error
            }
        }
    }
}

impl From<Result<FennecType, ParseError>> for CFennecType {
    fn from(value: Result<FennecType, ParseError>) -> Self {
        (&value).into()
    }
}

impl From<&FennecType> for CFennecType {
    fn from(value: &FennecType) -> Self {
        match value {
            FennecType::Object(obj) => Self::from_object(obj),
            FennecType::Array(arr) => Self::from_array(arr),
            FennecType::String(str) => Self::from_string(str),
            FennecType::Float(f) => Self::Float(*f),
            FennecType::Int(i) => Self::Int(*i),
            FennecType::Bool(b) => Self::Bool(*b),
            FennecType::Null => Self::Null,
        }
    }
}

impl From<FennecType> for CFennecType {
    fn from(value: FennecType) -> Self {
        (&value).into()
    }
}

impl Drop for CFennecType {
    fn drop(&mut self) {
        unsafe {
            match self {
                Self::Object(len, keys, values) => {
                    let keys = Vec::from_raw_parts(*keys as *mut *const i8, *len, *len);
                    for key in keys {
                        let _ = CString::from_raw(key as *mut i8);
                    }
                    let _ = Vec::from_raw_parts(*values as *mut CFennecType, *len, *len);
                }
                Self::Array(len, values) => {
                    let _ = Vec::from_raw_parts(*values as *mut CFennecType, *len, *len);
                }
                Self::String(str) => {
                    let _ = CString::from_raw(*str as *mut i8);
                }
                _ => {}
            }
        }
    }
}
