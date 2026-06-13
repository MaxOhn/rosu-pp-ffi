//! Tests for gradual difficulty and performance calculation.

mod common;

use common::{beatmap_path, Mode};

use rosu_pp_ffi::{
    rosu_pp_DifficultyAttributes, rosu_pp_FfiResult, rosu_pp_PerformanceAttributes,
    rosu_pp_beatmap_free, rosu_pp_beatmap_from_path, rosu_pp_difficulty_new,
    rosu_pp_gradual_difficulty_free, rosu_pp_gradual_difficulty_new,
    rosu_pp_gradual_difficulty_next, rosu_pp_gradual_performance_free,
    rosu_pp_gradual_performance_new, rosu_pp_gradual_performance_next, rosu_pp_score_state_new,
};

#[test]
fn gradual_difficulty_new() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let gradual = rosu_pp_gradual_difficulty_new(diff, map_handle);

    assert!(!gradual.is_null());
    rosu_pp_gradual_difficulty_free(gradual);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn gradual_difficulty_next() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let gradual = rosu_pp_gradual_difficulty_new(diff, map_handle);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_gradual_difficulty_next(gradual, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(attrs.stars > 0.0);

    rosu_pp_gradual_difficulty_free(gradual);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn gradual_difficulty_free_null() {
    rosu_pp_gradual_difficulty_free(std::ptr::null_mut());
}

#[test]
fn gradual_difficulty_next_null_gradual() {
    let null_gradual: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_gradual_difficulty_next(null_gradual as *mut _, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn gradual_difficulty_next_null_attrs() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let gradual = rosu_pp_gradual_difficulty_new(diff, map_handle);

    let result = rosu_pp_gradual_difficulty_next(gradual, std::ptr::null_mut());
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    rosu_pp_gradual_difficulty_free(gradual);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn gradual_performance_new() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let gradual = rosu_pp_gradual_performance_new(diff, map_handle);

    assert!(!gradual.is_null());
    rosu_pp_gradual_performance_free(gradual);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn gradual_performance_next() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let gradual = rosu_pp_gradual_performance_new(diff, map_handle);

    let state = rosu_pp_score_state_new();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_gradual_performance_next(gradual, &state, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(attrs.pp > 0.0);

    rosu_pp_gradual_performance_free(gradual);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn gradual_performance_free_null() {
    rosu_pp_gradual_performance_free(std::ptr::null_mut());
}

#[test]
fn gradual_performance_next_null_gradual() {
    let null_gradual: *mut std::ffi::c_void = std::ptr::null_mut();
    let state = rosu_pp_score_state_new();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_gradual_performance_next(null_gradual as *mut _, &state, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn gradual_performance_next_null_state() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let gradual = rosu_pp_gradual_performance_new(diff, map_handle);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let state = rosu_pp_score_state_new();
    let result = rosu_pp_gradual_performance_next(std::ptr::null_mut(), &state, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    rosu_pp_gradual_performance_free(gradual);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn gradual_performance_next_null_attrs() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let gradual = rosu_pp_gradual_performance_new(diff, map_handle);
    let state = rosu_pp_score_state_new();

    let result = rosu_pp_gradual_performance_next(gradual, &state, std::ptr::null_mut());
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    rosu_pp_gradual_performance_free(gradual);
    rosu_pp_beatmap_free(map_handle);
}
