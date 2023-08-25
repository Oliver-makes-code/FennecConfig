use std::{ffi::{c_char, CStr, CString}, ptr, fs};

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
    let fen = opaque_pointer::own_back(fen as *mut CFennecType).unwrap();

    match fen {
        CFennecType::Object(len, keys, values) => {
            let mut i = 0;
            while i < len {
                let _ = CString::from_raw(*keys.add(i) as *mut i8);
                println!("{:?} {:?}", &i as *const usize, values);
                FennecConfig_FennecType_Free(values.add(i));
                i += 1;
            }
        }
        CFennecType::Array(len, values) => {
            for i in 0..len {
                FennecConfig_FennecType_Free(values.add(i));
            }
        }
        CFennecType::String(str) => {
            let _ = CString::from_raw(str as *mut i8);
        }
        _ => {}
    }
}

#[cfg(test)]
mod test {
    use super::{FennecConfig_ParseString, FennecConfig_FennecType_Free};

    #[test]
    fn test_create_and_free() {
        unsafe {
            let fen = FennecConfig_ParseString("test = \"owo\" owo = 15 uwu = false".as_ptr() as *const i8);

            FennecConfig_FennecType_Free(fen);
        }
    }
}
