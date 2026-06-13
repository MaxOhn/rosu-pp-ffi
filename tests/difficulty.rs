//! Tests for difficulty calculation.

mod common;

use common::{beatmap_path, Mode};

use rosu_pp_ffi::{
    rosu_pp_DifficultyAttributes, rosu_pp_FfiResult, rosu_pp_beatmap_free,
    rosu_pp_beatmap_from_path, rosu_pp_difficulty_ar, rosu_pp_difficulty_calculate,
    rosu_pp_difficulty_clock_rate, rosu_pp_difficulty_clone, rosu_pp_difficulty_cs,
    rosu_pp_difficulty_free, rosu_pp_difficulty_hardrock_offsets, rosu_pp_difficulty_hp,
    rosu_pp_difficulty_lazer, rosu_pp_difficulty_mods, rosu_pp_difficulty_new,
    rosu_pp_difficulty_od, rosu_pp_difficulty_passed_objects, rosu_pp_difficulty_strains,
    rosu_pp_mods_free, rosu_pp_mods_from_bits, rosu_pp_strains_free,
};

#[test]
fn new_and_free() {
    let handle = rosu_pp_difficulty_new();
    assert!(!handle.is_null());
    rosu_pp_difficulty_free(handle);
}

#[test]
fn clone_and_free() {
    let handle = rosu_pp_difficulty_new();
    let cloned = rosu_pp_difficulty_clone(handle);
    assert!(!cloned.is_null());
    rosu_pp_difficulty_free(handle);
    rosu_pp_difficulty_free(cloned);
}

#[test]
fn free_null_handle() {
    rosu_pp_difficulty_free(std::ptr::null_mut());
}

#[test]
fn set_mods_dt() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let mods = rosu_pp_mods_from_bits(common::mods::HD | common::mods::HR);
    rosu_pp_difficulty_mods(diff, mods);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(attrs.stars > 0.0);

    rosu_pp_difficulty_free(diff);
    rosu_pp_mods_free(mods);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn passed_objects_zero() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_passed_objects(diff, 0);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_clock_rate() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_clock_rate(diff, 1.5);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_ar() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_ar(diff, 7.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_cs() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_cs(diff, 4.0, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_hp() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_hp(diff, 6.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn set_od() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_od(diff, 8.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn hardrock_offsets() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_hardrock_offsets(diff, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn lazer_mode() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let result = rosu_pp_difficulty_lazer(diff, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };
    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn calculate_osu() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };

    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    assert!(attrs.stars > 0.0);
    assert!(attrs.max_combo > 0);
    assert_eq!(attrs.mode, 0); // osu!

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn calculate_taiko() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Taiko).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };

    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    assert!(attrs.stars > 0.0);
    assert_eq!(attrs.mode, 1); // taiko

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn calculate_catch() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Catch).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };

    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    assert!(attrs.stars > 0.0);
    assert_eq!(attrs.mode, 2); // catch

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn calculate_mania() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Mania).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let mut attrs = unsafe { std::mem::zeroed::<rosu_pp_DifficultyAttributes>() };

    let result = rosu_pp_difficulty_calculate(diff, map_handle, &mut attrs);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    assert!(attrs.stars > 0.0);
    assert_eq!(attrs.mode, 3); // mania

    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn strains() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let diff = rosu_pp_difficulty_new();
    let strains = rosu_pp_difficulty_strains(diff, map_handle);

    assert!(!strains.is_null());
    assert!(unsafe { strains.as_ref().unwrap().mode } == 0);
    assert!(unsafe { strains.as_ref().unwrap().len } > 0);

    rosu_pp_strains_free(strains);
    rosu_pp_difficulty_free(diff);
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn strains_null_map() {
    let null_map: *mut std::ffi::c_void = std::ptr::null_mut();
    let diff = rosu_pp_difficulty_new();
    let strains = rosu_pp_difficulty_strains(diff, null_map as *const _);
    assert!(strains.is_null());
    rosu_pp_difficulty_free(diff);
}

#[test]
fn strains_null_diff() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let strains = rosu_pp_difficulty_strains(std::ptr::null_mut(), map_handle);
    assert!(strains.is_null());
    rosu_pp_beatmap_free(map_handle);
}

#[test]
fn passed_objects_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_passed_objects(cast as *mut _, 0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn clock_rate_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_clock_rate(cast as *mut _, 0.0);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn ar_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_ar(cast as *mut _, 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn cs_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_cs(cast as *mut _, 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn hp_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_hp(cast as *mut _, 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn od_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_od(cast as *mut _, 0.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn hardrock_offsets_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_hardrock_offsets(cast as *mut _, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn lazer_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_difficulty_lazer(cast as *mut _, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn mods_null_handle() {
    let cast: *mut std::ffi::c_void = std::ptr::null_mut();
    let mods = rosu_pp_mods_from_bits(common::mods::HD);
    let result = rosu_pp_difficulty_mods(cast as *mut _, mods);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    rosu_pp_mods_free(mods);
}
