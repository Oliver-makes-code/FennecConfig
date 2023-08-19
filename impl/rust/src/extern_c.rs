use std::{
    fs,
    ptr,
    ffi::{c_char, CStr, CString}
};

use crate::parse::FennecType;

#[repr(C)]
pub enum CParseError {
    NullValue = -1,
    NullFileName = 0,
    BrokenFileName = 1,
    FileError = 2,
    FennecParse = 3
}

pub type CParseResult = Result<FennecType, CParseError>;

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_LoadFile(filename: *const c_char) -> *const CParseResult {
    if filename.is_null() {
        return opaque_pointer::raw(Err(CParseError::NullFileName));
    }

    let Ok(rstr) = CStr::from_ptr(filename).to_str() else {
        return opaque_pointer::raw(Err(CParseError::BrokenFileName));
    };

    let Ok(file) = fs::read_to_string(rstr) else {
        return opaque_pointer::raw(Err(CParseError::FileError));
    };

    let Ok(fen) = crate::parse(&file) else {
        return opaque_pointer::raw(Err(CParseError::FennecParse))
    };

    opaque_pointer::raw(Ok(fen))
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_ParseResult_Drop(result: *const CParseResult) {
    if !result.is_null() {
        let _ = opaque_pointer::own_back(result as *mut CParseResult);
    }
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_ParseResult_IsErr(result: *const CParseResult) -> bool {
    if result.is_null() {
        return true
    }

    (*result).is_err()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_ParseResult_IsOk(result: *const CParseResult) -> bool {
    if result.is_null() {
        return true
    }

    (*result).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_ParseResult_GetErr(result: *const CParseResult) -> CParseError {
    if result.is_null() {
        return CParseError::NullValue
    }

    let Ok(owned) = opaque_pointer::own_back(result as *mut CParseResult) else {
        return CParseError::NullValue
    };

    let Err(err) = owned else {
        return CParseError::NullValue
    };
    
    err
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_ParseResult_GetOk(result: *const CParseResult) -> *const FennecType {
    if result.is_null() {
        return ptr::null()
    }

    let Ok(owned) = opaque_pointer::own_back(result as *mut CParseResult) else {
        return ptr::null()
    };

    let Ok(fen) = owned else {
        return ptr::null()
    };
    
    opaque_pointer::raw(fen)
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsObject(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }

    (*fen).as_object().is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_Object_HasKey(fen: *const FennecType, key: *const c_char) -> bool {
    if fen.is_null() {
        return false
    }
    
    if key.is_null() {
        return false
    }

    let Ok(key) = CStr::from_ptr(key).to_str() else {
        return false
    };
    
    (*fen).get_key(key).is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_Object_GetKey(fen: *const FennecType, key: *const c_char) -> *const FennecType {
    if fen.is_null() {
        return ptr::null()
    }
    
    if key.is_null() {
        return ptr::null()
    }

    let Ok(key) = CStr::from_ptr(key).to_str() else {
        return ptr::null()
    };

    let Some(fen) = (*fen).get_key(key) else {
        return ptr::null()
    };

    fen as *const FennecType
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsArray(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }
    
    (*fen).as_array().is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_Array_Len(fen: *const FennecType) -> usize {
    if fen.is_null() {
        return 0
    }
    
    let Some(fen) = (*fen).as_array() else {
        return 0
    };

    fen.len()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_Array_GetIdx(fen: *const FennecType, idx: usize) -> *const FennecType {
    if fen.is_null() {
        return ptr::null()
    }

    let Some(fen) = (*fen).as_array() else {
        return ptr::null()
    };

    let Some(fen) = (*fen).get(idx) else {
        return ptr::null()
    };
    
    return fen as *const FennecType
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsNumber(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }

    (*fen).as_int().is_some() || (*fen).as_float().is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsInt(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }
    
    (*fen).as_int().is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_GetInt(fen: *const FennecType) -> i64 {
    if fen.is_null() {
        return 0;
    }

    let Some(num) = (*fen).as_int() else {
        return 0;
    };

    num
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_GetInt_Float(fen: *const FennecType) -> f64 {
    FennecConfig_FennecType_GetInt(fen) as f64
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsFloat(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }
    
    (*fen).as_float().is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_GetFloat(fen: *const FennecType) -> f64 {
    if fen.is_null() {
        return 0.0
    }

    let Some(num) = (*fen).as_float() else {
        return FennecConfig_FennecType_GetInt_Float(fen)
    };

    num
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsString(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false;
    }
    
    (*fen).as_string().is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_GetString(fen: *const FennecType) -> *const c_char {
    if fen.is_null() {
        return ptr::null();
    }
    
    let Some(string) = (*fen).as_string() else {
        return ptr::null();
    };

    let Ok(cstr) = CString::new(string.as_bytes()) else {
        return ptr::null();
    };

    cstr.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsBool(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }
    
    (*fen).as_bool().is_some()
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_GetBool(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }

    let Some(bool) = (*fen).as_bool() else {
        return false
    };

    bool
}

#[no_mangle]
pub unsafe extern "C" fn FennecConfig_FennecType_IsNull(fen: *const FennecType) -> bool {
    if fen.is_null() {
        return false
    }
    
    (*fen).as_null().is_some()
}
