use std::{
    ffi::{c_char, CStr},
    fs::{self},
    ptr,
};

use self::types::CFennecType;

mod types;

#[no_mangle]
unsafe extern "C" fn FennecConfig_ParseString(str: *const c_char) -> *const CFennecType {
    if str.is_null() {
        return ptr::null();
    }

    let Ok(str) = CStr::from_ptr(str).to_str() else {
        return ptr::null();
    };

    opaque_pointer::raw(crate::parse(str).into())
}

#[no_mangle]
unsafe extern "C" fn FennecConfig_ParseFile(filename: *const c_char) -> *const CFennecType {
    if filename.is_null() {
        return ptr::null();
    }

    let Ok(rstr) = CStr::from_ptr(filename).to_str() else {
        return ptr::null();
    };

    let Ok(file) = fs::read_to_string(rstr) else {
        return ptr::null();
    };

    opaque_pointer::raw(crate::parse(&file).into())
}

#[no_mangle]
unsafe extern "C" fn FennecConfig_FennecType_Free(fen: *const CFennecType) {
    if fen.is_null() {
        return;
    }
    let _ = opaque_pointer::own_back(fen as *mut CFennecType);
}

#[cfg(test)]
mod test {
    use std::ffi::CString;

    use super::{FennecConfig_FennecType_Free, FennecConfig_ParseString};

    #[test]
    fn test_create_and_free() {
        unsafe {
            let fen = FennecConfig_ParseString(
                CString::new("test = \"owo\"\nowo = 15\nuwu = false")
                    .unwrap()
                    .into_raw(),
            );

            assert!(!fen.is_null());

            FennecConfig_FennecType_Free(fen);
        }
    }
}
