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

    use crate::extern_c::types::CFennecType;

    use super::{FennecConfig_FennecType_Free, FennecConfig_ParseString, FennecConfig_ParseFile};

    #[test]
    fn parse_file() {
        unsafe {
            let fen = FennecConfig_ParseFile(
                CString::new("../../specification.fennec")
                    .unwrap()
                    .into_raw(),
            );

            assert!(!fen.is_null());

            assert!(if let CFennecType::Object(_, _, _, _, _) = *fen { true } else { false });

            FennecConfig_FennecType_Free(fen);
        }
    }

    #[test]
    fn parse_string() {
        unsafe {
            let fen = FennecConfig_ParseString(
                CString::new("owo = 15 uwu = \"nya\" nya [false null]")
                    .unwrap()
                    .into_raw(),
            );

            assert!(!fen.is_null());

            assert!(if let CFennecType::Object(_, _, _, _, _) = *fen { true } else { false });

            FennecConfig_FennecType_Free(fen);
        }
    }

    #[test]
    fn parse_free_parse() {
        unsafe {
            let fen_1 = FennecConfig_ParseString(
                CString::new("owo = 15 uwu = \"nya\" nya [false null]")
                    .unwrap()
                    .into_raw(),
            );

            assert!(!fen_1.is_null());

            assert!(if let CFennecType::Object(_, _, _, _, _) = *fen_1 { true } else { false });

            FennecConfig_FennecType_Free(fen_1);

            let fen_2 = FennecConfig_ParseString(
                CString::new("owo = 15 uwu = \"nya\" nya [false null]")
                    .unwrap()
                    .into_raw(),
            );

            assert!(!fen_2.is_null());

            assert!(if let CFennecType::Object(_, _, _, _, _) = *fen_2 { true } else { false });

            println!("{fen_1:?} {fen_2:?}");
        }
    }
}
