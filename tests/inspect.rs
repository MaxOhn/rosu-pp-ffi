//! Tests for difficulty inspect functionality.

mod common;

use common::beatmap_path;

use rosu_pp_ffi::{
    rosu_pp_FfiResult, rosu_pp_beatmap_free, rosu_pp_beatmap_from_path,
    rosu_pp_difficulty_inspect_new, rosu_pp_difficulty_new, rosu_pp_inspect_difficulty_free,
};

#[test]
fn new_and_free() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(common::Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let handle = rosu_pp_difficulty_inspect_new(diff);
    assert!(!handle.is_null());
    rosu_pp_inspect_difficulty_free(handle);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn free_null_handle() {
    rosu_pp_inspect_difficulty_free(std::ptr::null_mut());
}

#[test]
fn null_difficulty_handle() {
    let result = rosu_pp_difficulty_inspect_new(std::ptr::null_mut());
    assert!(result.is_null());
}

#[test]
fn null_map_handle() {
    let _null_map: *mut std::ffi::c_void = std::ptr::null_mut();
    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_inspect_new(diff);
    // The function doesn't check map handle, it checks difficulty handle
    // So this should still create a valid inspect handle
    assert!(!result.is_null());
    rosu_pp_inspect_difficulty_free(result);
}
