//! Tests for null pointer handling across all modules.

mod common;

use rosu_pp_ffi::{
    rosu_pp_DifficultyAttributes, rosu_pp_FfiResult, rosu_pp_GameMode,
    rosu_pp_PerformanceAttributes, rosu_pp_beatmap_attrs_apply_clock_rate,
    rosu_pp_beatmap_attrs_ar, rosu_pp_beatmap_attrs_builder_ar,
    rosu_pp_beatmap_attrs_builder_clock_rate, rosu_pp_beatmap_attrs_builder_cs,
    rosu_pp_beatmap_attrs_builder_difficulty, rosu_pp_beatmap_attrs_builder_free,
    rosu_pp_beatmap_attrs_builder_hp, rosu_pp_beatmap_attrs_builder_map,
    rosu_pp_beatmap_attrs_builder_mode, rosu_pp_beatmap_attrs_builder_mods,
    rosu_pp_beatmap_attrs_builder_od, rosu_pp_beatmap_attrs_clock_rate, rosu_pp_beatmap_attrs_cs,
    rosu_pp_beatmap_attrs_free, rosu_pp_beatmap_attrs_hit_windows, rosu_pp_beatmap_attrs_hp,
    rosu_pp_beatmap_attrs_od, rosu_pp_beatmap_free, rosu_pp_difficulty_ar,
    rosu_pp_difficulty_calculate, rosu_pp_difficulty_clock_rate, rosu_pp_difficulty_cs,
    rosu_pp_difficulty_free, rosu_pp_difficulty_hardrock_offsets, rosu_pp_difficulty_hp,
    rosu_pp_difficulty_lazer, rosu_pp_difficulty_mods, rosu_pp_difficulty_od,
    rosu_pp_difficulty_passed_objects, rosu_pp_gradual_difficulty_free,
    rosu_pp_gradual_difficulty_next, rosu_pp_gradual_performance_free,
    rosu_pp_gradual_performance_next, rosu_pp_mods_free, rosu_pp_mods_free_string,
    rosu_pp_performance_accuracy, rosu_pp_performance_calculate, rosu_pp_performance_combo,
    rosu_pp_performance_free, rosu_pp_performance_misses, rosu_pp_performance_mods,
    rosu_pp_performance_state, rosu_pp_score_state_new, rosu_pp_score_state_total_hits,
    rosu_pp_strains_free, AdjustedBeatmapAttributes, HitWindows,
};

#[test]
fn beatmap_free_null() {
    rosu_pp_beatmap_free(std::ptr::null_mut());
}

#[test]
fn difficulty_free_null() {
    rosu_pp_difficulty_free(std::ptr::null_mut());
}

#[test]
fn performance_free_null() {
    rosu_pp_performance_free(std::ptr::null_mut());
}

#[test]
fn mods_free_null() {
    rosu_pp_mods_free(std::ptr::null_mut());
}

#[test]
fn mods_free_string_null() {
    rosu_pp_mods_free_string(std::ptr::null_mut());
}

#[test]
fn strains_free_null() {
    rosu_pp_strains_free(std::ptr::null_mut());
}

#[test]
fn beatmap_attrs_builder_free_null() {
    rosu_pp_beatmap_attrs_builder_free(std::ptr::null_mut());
}

#[test]
fn beatmap_attrs_free_null() {
    rosu_pp_beatmap_attrs_free(std::ptr::null_mut());
}

#[test]
fn gradual_difficulty_free_null() {
    rosu_pp_gradual_difficulty_free(std::ptr::null_mut());
}

#[test]
fn gradual_performance_free_null() {
    rosu_pp_gradual_performance_free(std::ptr::null_mut());
}

#[test]
fn setters_null_handle() {
    let result = rosu_pp_difficulty_mods(std::ptr::null_mut(), std::ptr::null_mut());
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_passed_objects(std::ptr::null_mut(), 0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_clock_rate(std::ptr::null_mut(), 0.0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_ar(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_cs(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_hp(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_od(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_hardrock_offsets(std::ptr::null_mut(), false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_difficulty_lazer(std::ptr::null_mut(), false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_performance_mods(std::ptr::null_mut(), std::ptr::null_mut());
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_performance_accuracy(std::ptr::null_mut(), 0.0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_performance_misses(std::ptr::null_mut(), 0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_performance_combo(std::ptr::null_mut(), 0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let state = rosu_pp_score_state_new();
    let result = rosu_pp_performance_state(std::ptr::null_mut(), &state);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_beatmap_attrs_builder_map(std::ptr::null_mut(), std::ptr::null_mut());
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_beatmap_attrs_builder_mods(std::ptr::null_mut(), std::ptr::null_mut());
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_beatmap_attrs_builder_clock_rate(std::ptr::null_mut(), 0.0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result =
        rosu_pp_beatmap_attrs_builder_mode(std::ptr::null_mut(), rosu_pp_GameMode::Osu, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_beatmap_attrs_builder_ar(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_beatmap_attrs_builder_od(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_beatmap_attrs_builder_cs(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result = rosu_pp_beatmap_attrs_builder_hp(std::ptr::null_mut(), 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let result =
        rosu_pp_beatmap_attrs_builder_difficulty(std::ptr::null_mut(), std::ptr::null_mut());
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn calculate_null_handles() {
    let null_map: *mut std::ffi::c_void = std::ptr::null_mut();
    let null_diff: *mut std::ffi::c_void = std::ptr::null_mut();
    let null_perf: *mut std::ffi::c_void = std::ptr::null_mut();

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result =
        rosu_pp_difficulty_calculate(null_diff as *mut _, null_map as *const _, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let mut perf_attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let result = rosu_pp_performance_calculate(null_perf as *mut _, &mut perf_attrs);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn gradual_next_null() {
    let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };

    let result = rosu_pp_gradual_difficulty_next(null_handle as *mut _, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let mut perf_attrs = unsafe { std::mem::zeroed::<rosu_pp_PerformanceAttributes>() };
    let state = rosu_pp_score_state_new();
    let result = rosu_pp_gradual_performance_next(null_handle as *mut _, &state, &mut perf_attrs);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn beatmap_attrs_getters_null() {
    let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
    let cast = null_handle as *const _;

    let _ = rosu_pp_beatmap_attrs_ar(cast);
    let _ = rosu_pp_beatmap_attrs_od(cast);
    let _ = rosu_pp_beatmap_attrs_cs(cast);
    let _ = rosu_pp_beatmap_attrs_hp(cast);
    let _ = rosu_pp_beatmap_attrs_clock_rate(cast);

    let mut hw = unsafe { std::mem::zeroed::<HitWindows>() };
    let result = rosu_pp_beatmap_attrs_hit_windows(cast, &mut hw);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    let mut adj = unsafe { std::mem::zeroed::<AdjustedBeatmapAttributes>() };
    let result = rosu_pp_beatmap_attrs_apply_clock_rate(cast, &mut adj);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn score_state_total_hits_null() {
    let null_state: *const std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_score_state_total_hits(null_state as *const _, rosu_pp_GameMode::Osu);
    assert_eq!(result, 0);
}
