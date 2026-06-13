//! Tests for performance calculation.

mod common;

use common::{beatmap_path, Mode};

use rosu_pp_ffi::{
    rosu_pp_FfiResult, rosu_pp_GameMode, rosu_pp_PerformanceAttributes, rosu_pp_ScoreState,
    rosu_pp_beatmap_free, rosu_pp_beatmap_from_path, rosu_pp_mods_free, rosu_pp_mods_from_bits,
    rosu_pp_performance_accuracy, rosu_pp_performance_ar, rosu_pp_performance_calculate,
    rosu_pp_performance_checked_calculate, rosu_pp_performance_clock_rate,
    rosu_pp_performance_combo, rosu_pp_performance_cs, rosu_pp_performance_free,
    rosu_pp_performance_hardrock_offsets, rosu_pp_performance_hitresult_priority,
    rosu_pp_performance_hp, rosu_pp_performance_large_tick_hits, rosu_pp_performance_lazer,
    rosu_pp_performance_legacy_total_score, rosu_pp_performance_misses, rosu_pp_performance_mods,
    rosu_pp_performance_n100, rosu_pp_performance_n300, rosu_pp_performance_n50,
    rosu_pp_performance_n_geki, rosu_pp_performance_n_katu, rosu_pp_performance_new,
    rosu_pp_performance_od, rosu_pp_performance_passed_objects,
    rosu_pp_performance_slider_end_hits, rosu_pp_performance_small_tick_hits,
    rosu_pp_performance_state, rosu_pp_score_state_new, rosu_pp_score_state_total_hits,
};

#[test]
fn new_and_free() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    assert!(!perf.is_null());
    rosu_pp_performance_free(perf);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn free_null_handle() {
    rosu_pp_performance_free(std::ptr::null_mut());
}

#[test]
fn new_null_map() {
    let null_map: *mut std::ffi::c_void = std::ptr::null_mut();
    let perf = rosu_pp_performance_new(null_map as *mut _);
    assert!(perf.is_null());
}

#[test]
fn out_pointer_null() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let result = rosu_pp_performance_new(std::ptr::null_mut());
    assert!(result.is_null());

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_mods() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let mods = rosu_pp_mods_from_bits(common::mods::HD | common::mods::HR);
    rosu_pp_performance_mods(perf, mods);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(attrs.pp > 0.0);

    rosu_pp_mods_free(mods);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn passed_objects_zero() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_passed_objects(perf, 0);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_performance_free(perf);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_clock_rate() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_clock_rate(perf, 1.5);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_ar() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_ar(perf, 7.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_cs() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_cs(perf, 4.0, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_hp() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_hp(perf, 6.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_od() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_od(perf, 8.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn hardrock_offsets() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_hardrock_offsets(perf, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn lazer_mode() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_lazer(perf, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_accuracy() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_accuracy(perf, 95.0);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_misses() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_misses(perf, 1);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_combo() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_combo(perf, 400);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_large_tick_hits() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_large_tick_hits(perf, 50);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_small_tick_hits() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_small_tick_hits(perf, 100);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_slider_end_hits() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_slider_end_hits(perf, 40);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_n300() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_n300(perf, 400);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_n100() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_n100(perf, 50);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_n50() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_n50(perf, 10);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_n_geki() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_n_geki(perf, 0);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_n_katu() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_n_katu(perf, 20);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn legacy_total_score() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_legacy_total_score(perf, 1000000);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_performance_free(perf);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn hitresult_priority() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let result = rosu_pp_performance_hitresult_priority(perf, 0);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_performance_free(perf);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn score_state() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let state = rosu_pp_score_state_new();
    let result = rosu_pp_performance_state(perf, &state);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(attrs.pp > 0.0);

    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn score_state_null_handle() {
    let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
    let state = rosu_pp_score_state_new();
    let result = rosu_pp_performance_state(null_handle as *mut _, &state);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn score_state_total_hits() {
    let state = rosu_pp_score_state_new();
    let total = rosu_pp_score_state_total_hits(&state, rosu_pp_GameMode::Osu);
    assert_eq!(total, 0);
}

#[test]
fn score_state_total_hits_zeroed() {
    let state = unsafe { std::mem::zeroed::<rosu_pp_ScoreState>() };
    let total = rosu_pp_score_state_total_hits(&state, rosu_pp_GameMode::Osu);
    assert_eq!(total, 0);
}

#[test]
fn checked_calculate() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let perf = rosu_pp_performance_new(map_handle);
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_checked_calculate(perf, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_free(map_handle);
}
