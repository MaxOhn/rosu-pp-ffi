//! Tests for rosu_pp_mode_from_str and rosu_pp_mode_to_str.

mod common;

use rosu_pp_ffi::{
    rosu_pp_GameMode, rosu_pp_FfiResult, rosu_pp_mode_from_str, rosu_pp_mode_to_str,
};

#[test]
fn to_str_osu() {
    let s = unsafe { rosu_pp_mode_to_str(rosu_pp_GameMode::Osu) };
    let c_str = unsafe { &std::ffi::CStr::from_ptr(s) };
    assert_eq!(c_str.to_str().unwrap(), "osu");
}

#[test]
fn to_str_taiko() {
    let s = unsafe { rosu_pp_mode_to_str(rosu_pp_GameMode::Taiko) };
    let c_str = unsafe { &std::ffi::CStr::from_ptr(s) };
    assert_eq!(c_str.to_str().unwrap(), "taiko");
}

#[test]
fn to_str_catch() {
    let s = unsafe { rosu_pp_mode_to_str(rosu_pp_GameMode::Catch) };
    let c_str = unsafe { &std::ffi::CStr::from_ptr(s) };
    assert_eq!(c_str.to_str().unwrap(), "catch");
}

#[test]
fn to_str_mania() {
    let s = unsafe { rosu_pp_mode_to_str(rosu_pp_GameMode::Mania) };
    let c_str = unsafe { &std::ffi::CStr::from_ptr(s) };
    assert_eq!(c_str.to_str().unwrap(), "mania");
}

#[test]
fn from_str_osu_aliases() {
    for alias in ["osu", "std", "0"] {
        unsafe {
            let c_str = std::ffi::CString::new(alias).unwrap();
            let mut mode: rosu_pp_GameMode = rosu_pp_GameMode::Taiko;
            let result = rosu_pp_mode_from_str(c_str.as_ptr(), &mut mode);
            assert_eq!(result, rosu_pp_FfiResult::Ok);
            assert_eq!(mode, rosu_pp_GameMode::Osu);
        }
    }
}

#[test]
fn from_str_taiko_aliases() {
    for alias in ["taiko", "tko", "1"] {
        unsafe {
            let c_str = std::ffi::CString::new(alias).unwrap();
            let mut mode: rosu_pp_GameMode = rosu_pp_GameMode::Osu;
            let result = rosu_pp_mode_from_str(c_str.as_ptr(), &mut mode);
            assert_eq!(result, rosu_pp_FfiResult::Ok);
            assert_eq!(mode, rosu_pp_GameMode::Taiko);
        }
    }
}

#[test]
fn from_str_catch_aliases() {
    for alias in ["catch", "ctb", "fruits", "2"] {
        unsafe {
            let c_str = std::ffi::CString::new(alias).unwrap();
            let mut mode: rosu_pp_GameMode = rosu_pp_GameMode::Osu;
            let result = rosu_pp_mode_from_str(c_str.as_ptr(), &mut mode);
            assert_eq!(result, rosu_pp_FfiResult::Ok);
            assert_eq!(mode, rosu_pp_GameMode::Catch);
        }
    }
}

#[test]
fn from_str_mania_aliases() {
    for alias in ["mania", "mna", "3"] {
        unsafe {
            let c_str = std::ffi::CString::new(alias).unwrap();
            let mut mode: rosu_pp_GameMode = rosu_pp_GameMode::Osu;
            let result = rosu_pp_mode_from_str(c_str.as_ptr(), &mut mode);
            assert_eq!(result, rosu_pp_FfiResult::Ok);
            assert_eq!(mode, rosu_pp_GameMode::Mania);
        }
    }
}

#[test]
fn from_str_invalid() {
    unsafe {
        let c_str = std::ffi::CString::new("invalid").unwrap();
        let mut mode: rosu_pp_GameMode = rosu_pp_GameMode::Osu;
        let result = rosu_pp_mode_from_str(c_str.as_ptr(), &mut mode);
        assert_eq!(result, rosu_pp_FfiResult::InvalidArgument);
    }
}

#[test]
fn from_str_null_input() {
    unsafe {
        let mut mode: rosu_pp_GameMode = rosu_pp_GameMode::Osu;
        let result = rosu_pp_mode_from_str(std::ptr::null_mut(), &mut mode);
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn from_str_null_out() {
    unsafe {
        let c_str = std::ffi::CString::new("osu").unwrap();
        let result = rosu_pp_mode_from_str(c_str.as_ptr(), std::ptr::null_mut());
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn roundtrip_all_modes() {
    let modes = [
        (rosu_pp_GameMode::Osu, "osu"),
        (rosu_pp_GameMode::Taiko, "taiko"),
        (rosu_pp_GameMode::Catch, "catch"),
        (rosu_pp_GameMode::Mania, "mania"),
    ];

    for (expected_mode, expected_str) in modes {
        // to_str
        let s = unsafe { rosu_pp_mode_to_str(expected_mode) };
        let c_str = unsafe { &std::ffi::CStr::from_ptr(s) };
        assert_eq!(c_str.to_str().unwrap(), expected_str);

        // from_str
        let c_str = std::ffi::CString::new(expected_str).unwrap();
        let mut mode: rosu_pp_GameMode = rosu_pp_GameMode::Osu;
        let result = unsafe { rosu_pp_mode_from_str(c_str.as_ptr(), &mut mode) };
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert_eq!(mode, expected_mode);
    }
}
