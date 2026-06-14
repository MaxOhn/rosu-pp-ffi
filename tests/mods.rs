//! Tests for mod parsing and manipulation.

mod common;

use common::mods;

use rosu_pp_ffi::{
    rosu_pp_FfiResult, rosu_pp_GameMode, rosu_pp_mods_free, rosu_pp_mods_free_string,
    rosu_pp_mods_from_acronym, rosu_pp_mods_from_bits, rosu_pp_mods_from_json,
    rosu_pp_mods_from_json_with_mode, rosu_pp_mods_to_bits, rosu_pp_mods_to_string,
};

// --- rosu_pp_mods_from_acronym ---

#[test]
fn from_acronym_hd_hr() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new("HDHR").unwrap();
        let result = rosu_pp_mods_from_acronym(s.as_ptr(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, mods::HD | mods::HR);

        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_acronym_dt() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new("DT").unwrap();
        let result = rosu_pp_mods_from_acronym(s.as_ptr(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, mods::DT);

        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_acronym_invalid() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new("{\"acronym\":\"HD\"}").unwrap();
        let result = rosu_pp_mods_from_acronym(s.as_ptr(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        assert!(!handle.is_null());

        let c_str = rosu_pp_mods_to_string(handle);
        assert!(!c_str.is_null());
        let s = std::ffi::CStr::from_ptr(c_str).to_str().unwrap();
        assert_eq!(s, "ACHD\"}:\"M\"NYRO{\"");
        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, 8); // only HD has a bitvalue
        rosu_pp_mods_free_string(c_str);
        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_acronym_empty() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new("").unwrap();
        let result = rosu_pp_mods_from_acronym(s.as_ptr(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, 0);

        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_acronym_null_string() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let result = rosu_pp_mods_from_acronym(std::ptr::null_mut(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn from_acronym_null_out() {
    unsafe {
        let s = std::ffi::CString::new("HD").unwrap();
        let result = rosu_pp_mods_from_acronym(s.as_ptr(), std::ptr::null_mut());
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

// --- rosu_pp_mods_from_json ---

#[test]
fn from_json_simple() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new(r#"{"acronym":"HD"}"#).unwrap();
        let result = rosu_pp_mods_from_json(s.as_ptr(), false, &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, mods::HD);

        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_json_array() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new(
            r#"[
                {
                    "acronym": "HD"
                },
                1024,
                {
                    "acronym": "DT",
                    "settings": {
                        "speed_change": 1.2
                    }
                }
            ]"#,
        )
        .unwrap();
        let result = rosu_pp_mods_from_json(s.as_ptr(), false, &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        let c_str = rosu_pp_mods_to_string(handle);
        assert!(!c_str.is_null());
        let s = std::ffi::CStr::from_ptr(c_str).to_str().unwrap();
        assert_eq!(s, "DTFLHD");
        rosu_pp_mods_free_string(c_str);
        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_json_int_value() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new("72").unwrap();
        let result = rosu_pp_mods_from_json(s.as_ptr(), false, &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);

        let c_str = rosu_pp_mods_to_string(handle);
        assert!(!c_str.is_null());
        let s = std::ffi::CStr::from_ptr(c_str).to_str().unwrap();
        assert_eq!(s, "DTHD");
        rosu_pp_mods_free_string(c_str);
        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_json_invalid() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new("invalid json").unwrap();
        let result = rosu_pp_mods_from_json(s.as_ptr(), false, &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::ParseError);
        assert!(handle.is_null());
    }
}

#[test]
fn from_json_null_string() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let result = rosu_pp_mods_from_json(std::ptr::null_mut(), false, &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn from_json_null_out() {
    unsafe {
        let s = std::ffi::CString::new("[]").unwrap();
        let result = rosu_pp_mods_from_json(s.as_ptr(), false, std::ptr::null_mut());
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

// --- rosu_pp_mods_from_json_with_mode ---

#[test]
fn from_json_with_mode_osu() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new(r#"[{"acronym":"HD"},{"acronym":"FI"}]"#).unwrap();
        let result = rosu_pp_mods_from_json_with_mode(
            s.as_ptr(),
            false,
            rosu_pp_GameMode::Mania,
            &mut handle,
        );
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, mods::HD | mods::FI);

        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_json_with_mode_array() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let s = std::ffi::CString::new(
            r#"[{"acronym":"HD"},{"acronym":"DT","settings":{"speed_change":1.2}}]"#,
        )
        .unwrap();
        let result = rosu_pp_mods_from_json_with_mode(
            s.as_ptr(),
            false,
            rosu_pp_GameMode::Taiko,
            &mut handle,
        );
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!handle.is_null());

        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_json_with_mode_null_string() {
    unsafe {
        let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
        let result = rosu_pp_mods_from_json_with_mode(
            std::ptr::null_mut(),
            false,
            rosu_pp_GameMode::Osu,
            &mut handle,
        );
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn from_json_with_mode_null_out() {
    unsafe {
        let s = std::ffi::CString::new("HD").unwrap();
        let result = rosu_pp_mods_from_json_with_mode(
            s.as_ptr(),
            false,
            rosu_pp_GameMode::Osu,
            std::ptr::null_mut(),
        );
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

// --- rosu_pp_mods_from_bits ---

#[test]
fn from_bits_zero() {
    unsafe {
        let handle = rosu_pp_mods_from_bits(0);
        assert!(!handle.is_null());
        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, 0);
        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_bits_hd_hr() {
    unsafe {
        let handle = rosu_pp_mods_from_bits(mods::HD | mods::HR);
        assert!(!handle.is_null());
        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, mods::HD | mods::HR);
        rosu_pp_mods_free(handle);
    }
}

#[test]
fn from_bits_dt() {
    unsafe {
        let handle = rosu_pp_mods_from_bits(mods::DT);
        assert!(!handle.is_null());
        let bits = rosu_pp_mods_to_bits(handle);
        assert_eq!(bits, mods::DT);
        rosu_pp_mods_free(handle);
    }
}

// --- rosu_pp_mods_to_string ---

#[test]
fn to_string_simple() {
    unsafe {
        let handle = rosu_pp_mods_from_bits(mods::HD | mods::HR);
        let c_str = rosu_pp_mods_to_string(handle);
        assert!(!c_str.is_null());
        let s = std::ffi::CStr::from_ptr(c_str).to_str().unwrap();
        assert_eq!(s, "HDHR");
        rosu_pp_mods_free_string(c_str);
        rosu_pp_mods_free(handle);
    }
}

#[test]
fn to_string_dt() {
    unsafe {
        let handle = rosu_pp_mods_from_bits(mods::HD | mods::HR | mods::DT);
        let c_str = rosu_pp_mods_to_string(handle);
        assert!(!c_str.is_null());
        let s = std::ffi::CStr::from_ptr(c_str).to_str().unwrap();
        assert!(s.contains("HD"));
        assert!(s.contains("HR"));
        assert!(s.contains("DT"));
        rosu_pp_mods_free_string(c_str);
        rosu_pp_mods_free(handle);
    }
}

#[test]
fn to_string_null_handle() {
    unsafe {
        let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
        let result = rosu_pp_mods_to_string(null_handle as *const _);
        assert!(result.is_null());
    }
}

// --- Free functions ---

#[test]
fn free_string_null() {
    unsafe {
        rosu_pp_mods_free_string(std::ptr::null_mut());
    }
}

#[test]
fn free_null_handle() {
    unsafe {
        rosu_pp_mods_free(std::ptr::null_mut());
    }
}
