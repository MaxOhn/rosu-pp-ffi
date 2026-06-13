//! Tests for mod parsing and manipulation.

mod common;

use common::mods;

use rosu_pp_ffi::{
    rosu_pp_FfiResult, rosu_pp_GameMode, rosu_pp_mods_free, rosu_pp_mods_free_string,
    rosu_pp_mods_from_bits, rosu_pp_mods_parse, rosu_pp_mods_parse_with_mode, rosu_pp_mods_to_bits,
    rosu_pp_mods_to_string,
};

#[test]
fn parse_simple_mods() {
    let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
    let s = std::ffi::CString::new("HDHR").unwrap();
    let result = rosu_pp_mods_parse(s.as_ptr(), false, &mut handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(!handle.is_null());

    let bits = rosu_pp_mods_to_bits(handle);
    assert_eq!(bits, mods::HD | mods::HR);

    rosu_pp_mods_free(handle);
}

#[test]
fn parse_dt() {
    let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
    let s = std::ffi::CString::new("DT").unwrap();
    let result = rosu_pp_mods_parse(s.as_ptr(), false, &mut handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(!handle.is_null());

    let bits = rosu_pp_mods_to_bits(handle);
    assert_eq!(bits, mods::DT);

    rosu_pp_mods_free(handle);
}

#[test]
fn parse_no_mods() {
    let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
    let s = std::ffi::CString::new("").unwrap();
    let result = rosu_pp_mods_parse(s.as_ptr(), false, &mut handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(!handle.is_null());

    let bits = rosu_pp_mods_to_bits(handle);
    assert_eq!(bits, 0);

    rosu_pp_mods_free(handle);
}

#[test]
fn parse_with_mode() {
    let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
    let s = std::ffi::CString::new("HDHR").unwrap();
    let result =
        rosu_pp_mods_parse_with_mode(s.as_ptr(), false, rosu_pp_GameMode::Osu, &mut handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(!handle.is_null());

    let bits = rosu_pp_mods_to_bits(handle);
    assert_eq!(bits, mods::HD | mods::HR);

    rosu_pp_mods_free(handle);
}

#[test]
fn parse_invalid_string() {
    // The rosu-mods library falls back to legacy parsing for non-JSON strings,
    // so even invalid strings are accepted (unknown bits are silently dropped).
    let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
    let s = std::ffi::CString::new(r#"not valid json at all {[""#).unwrap();
    let result = rosu_pp_mods_parse(s.as_ptr(), true, &mut handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(!handle.is_null());
    rosu_pp_mods_free(handle);
}

#[test]
fn parse_null_string() {
    let mut handle = std::ptr::null_mut::<rosu_pp_ffi::rosu_pp_ModsHandle>();
    let result = rosu_pp_mods_parse(std::ptr::null_mut(), false, &mut handle);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn from_bits_zero() {
    let handle = rosu_pp_mods_from_bits(0);
    assert!(!handle.is_null());
    let bits = rosu_pp_mods_to_bits(handle);
    assert_eq!(bits, 0);
    rosu_pp_mods_free(handle);
}

#[test]
fn from_bits_hd_hr() {
    let handle = rosu_pp_mods_from_bits(mods::HD | mods::HR);
    assert!(!handle.is_null());
    let bits = rosu_pp_mods_to_bits(handle);
    assert_eq!(bits, mods::HD | mods::HR);
    rosu_pp_mods_free(handle);
}

#[test]
fn from_bits_dt() {
    let handle = rosu_pp_mods_from_bits(mods::DT);
    assert!(!handle.is_null());
    let bits = rosu_pp_mods_to_bits(handle);
    assert_eq!(bits, mods::DT);
    rosu_pp_mods_free(handle);
}

#[test]
fn to_string_simple() {
    let handle = rosu_pp_mods_from_bits(mods::HD | mods::HR);
    let c_str = rosu_pp_mods_to_string(handle);
    assert!(!c_str.is_null());
    let s = unsafe { std::ffi::CStr::from_ptr(c_str) }.to_str().unwrap();
    assert_eq!(s, "HDHR");
    rosu_pp_mods_free_string(c_str);
    rosu_pp_mods_free(handle);
}

#[test]
fn to_string_dt() {
    let handle = rosu_pp_mods_from_bits(mods::HD | mods::HR | mods::DT);
    let c_str = rosu_pp_mods_to_string(handle);
    assert!(!c_str.is_null());
    let s = unsafe { std::ffi::CStr::from_ptr(c_str) }.to_str().unwrap();
    assert!(s.contains("HD"));
    assert!(s.contains("HR"));
    assert!(s.contains("DT"));
    rosu_pp_mods_free_string(c_str);
    rosu_pp_mods_free(handle);
}

#[test]
fn to_string_null_handle() {
    let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_mods_to_string(null_handle as *const _);
    assert!(result.is_null());
}

#[test]
fn free_string_null() {
    rosu_pp_mods_free_string(std::ptr::null_mut());
}

#[test]
fn free_null_handle() {
    rosu_pp_mods_free(std::ptr::null_mut());
}
