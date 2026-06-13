//! Tests for BeatmapAttributesBuilder and BeatmapAttributes

mod common;

use common::{assert_float_eq, beatmap_path, Mode};

use rosu_pp_ffi::{
    rosu_pp_FfiResult, rosu_pp_GameMode, rosu_pp_beatmap_attrs_apply_clock_rate,
    rosu_pp_beatmap_attrs_ar, rosu_pp_beatmap_attrs_builder_ar,
    rosu_pp_beatmap_attrs_builder_build, rosu_pp_beatmap_attrs_builder_clock_rate,
    rosu_pp_beatmap_attrs_builder_cs, rosu_pp_beatmap_attrs_builder_free,
    rosu_pp_beatmap_attrs_builder_hp, rosu_pp_beatmap_attrs_builder_map,
    rosu_pp_beatmap_attrs_builder_mode, rosu_pp_beatmap_attrs_builder_mods,
    rosu_pp_beatmap_attrs_builder_new, rosu_pp_beatmap_attrs_builder_od,
    rosu_pp_beatmap_attrs_clock_rate, rosu_pp_beatmap_attrs_cs, rosu_pp_beatmap_attrs_free,
    rosu_pp_beatmap_attrs_hit_windows, rosu_pp_beatmap_attrs_hp, rosu_pp_beatmap_attrs_od,
    rosu_pp_beatmap_free, rosu_pp_beatmap_from_path, rosu_pp_mods_free, rosu_pp_mods_from_bits,
    AdjustedBeatmapAttributes, HitWindows,
};

#[test]
fn builder_new_and_free() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    assert!(!builder.is_null());
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn builder_free_null() {
    rosu_pp_beatmap_attrs_builder_free(std::ptr::null_mut());
}

#[test]
fn build_null_builder() {
    let null_builder: *mut std::ffi::c_void = std::ptr::null_mut();
    let attrs = rosu_pp_beatmap_attrs_builder_build(null_builder as *mut _);
    assert!(attrs.is_null());
}

#[test]
fn build_default() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let ar = rosu_pp_beatmap_attrs_ar(attrs);
    assert!(ar > 0.0);

    let cs = rosu_pp_beatmap_attrs_cs(attrs);
    assert!(cs > 0.0);

    let hp = rosu_pp_beatmap_attrs_hp(attrs);
    assert!(hp > 0.0);

    let od = rosu_pp_beatmap_attrs_od(attrs);
    assert!(od > 0.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn set_ar() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_ar(builder, 7.5, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let ar = rosu_pp_beatmap_attrs_ar(attrs);
    assert_float_eq(ar as f64, 7.5);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn set_cs() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_cs(builder, 3.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let cs = rosu_pp_beatmap_attrs_cs(attrs);
    assert_float_eq(cs as f64, 3.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn set_hp() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_hp(builder, 9.0, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let hp = rosu_pp_beatmap_attrs_hp(attrs);
    assert_float_eq(hp as f64, 9.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn set_od() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_od(builder, 6.0, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let od = rosu_pp_beatmap_attrs_od(attrs);
    assert_float_eq(od as f64, 6.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn set_mode() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_mode(builder, rosu_pp_GameMode::Taiko, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let ar = rosu_pp_beatmap_attrs_ar(attrs);
    assert!(ar > 0.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn set_clock_rate() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_clock_rate(builder, 1.5);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let clock_rate = rosu_pp_beatmap_attrs_clock_rate(attrs);
    assert_float_eq(clock_rate, 1.5);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn set_mods() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let mods = rosu_pp_mods_from_bits(common::mods::HD | common::mods::HR);
    let result = rosu_pp_beatmap_attrs_builder_mods(builder, mods);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let clock_rate = rosu_pp_beatmap_attrs_clock_rate(attrs);
    assert!(clock_rate > 0.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
    rosu_pp_mods_free(mods);
}

#[test]
fn hit_windows_osu() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_ar(builder, 8.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    let result = rosu_pp_beatmap_attrs_builder_od(builder, 8.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    let result = rosu_pp_beatmap_attrs_builder_clock_rate(builder, 1.5);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let mut hw = unsafe { std::mem::zeroed::<HitWindows>() };
    let result = rosu_pp_beatmap_attrs_hit_windows(attrs, &mut hw);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(hw.od_great > 0.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn hit_windows_mania() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_mode(builder, rosu_pp_GameMode::Mania, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    let result = rosu_pp_beatmap_attrs_builder_od(builder, 8.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let mut hw = unsafe { std::mem::zeroed::<HitWindows>() };
    let result = rosu_pp_beatmap_attrs_hit_windows(attrs, &mut hw);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    assert!(hw.od_great > 0.0);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn apply_clock_rate() {
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_ar(builder, 8.0, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    let result = rosu_pp_beatmap_attrs_builder_od(builder, 8.0, true);
    assert_eq!(result, rosu_pp_FfiResult::Ok);
    let result = rosu_pp_beatmap_attrs_builder_clock_rate(builder, 1.5);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let clock_rate = rosu_pp_beatmap_attrs_clock_rate(attrs);
    assert_float_eq(clock_rate, 1.5);

    let mut adj = unsafe { std::mem::zeroed::<AdjustedBeatmapAttributes>() };
    let result = rosu_pp_beatmap_attrs_apply_clock_rate(attrs, &mut adj);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn apply_clock_rate_null_handle() {
    let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut adj = unsafe { std::mem::zeroed::<AdjustedBeatmapAttributes>() };
    let result = rosu_pp_beatmap_attrs_apply_clock_rate(null_handle as *const _, &mut adj);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn hit_windows_null_handle() {
    let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut hw = unsafe { std::mem::zeroed::<HitWindows>() };
    let result = rosu_pp_beatmap_attrs_hit_windows(null_handle as *const _, &mut hw);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn getters_null_handle() {
    let null_handle: *mut std::ffi::c_void = std::ptr::null_mut();
    let cast = null_handle as *const _;

    let ar = rosu_pp_beatmap_attrs_ar(cast);
    assert!(!ar.is_nan());

    let od = rosu_pp_beatmap_attrs_od(cast);
    assert!(!od.is_nan());

    let cs = rosu_pp_beatmap_attrs_cs(cast);
    assert!(!cs.is_nan());

    let hp = rosu_pp_beatmap_attrs_hp(cast);
    assert!(!hp.is_nan());

    let clock_rate = rosu_pp_beatmap_attrs_clock_rate(cast);
    assert!(!clock_rate.is_nan());
}

#[test]
fn override_ar() {
    let _builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_ar(std::ptr::null_mut(), 7.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn override_od() {
    let _builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_od(std::ptr::null_mut(), 7.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn override_cs() {
    let _builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_cs(std::ptr::null_mut(), 7.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn override_hp() {
    let _builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_hp(std::ptr::null_mut(), 7.0, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn override_clock_rate() {
    let _builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_clock_rate(std::ptr::null_mut(), 1.5);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn override_mode() {
    let _builder = rosu_pp_beatmap_attrs_builder_new();
    let result =
        rosu_pp_beatmap_attrs_builder_mode(std::ptr::null_mut(), rosu_pp_GameMode::Osu, false);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
}

#[test]
fn build_from_map() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_map(builder, map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mods = rosu_pp_mods_from_bits(common::mods::DT);
    let result = rosu_pp_beatmap_attrs_builder_mods(builder, mods);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let result = rosu_pp_beatmap_attrs_builder_ar(builder, 7.0, false);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let attrs = rosu_pp_beatmap_attrs_builder_build(builder);
    assert!(!attrs.is_null());

    let ar = rosu_pp_beatmap_attrs_ar(attrs);
    assert_float_eq(ar as f64, 7.0);

    let clock_rate = rosu_pp_beatmap_attrs_clock_rate(attrs);
    assert_float_eq(clock_rate, 1.5);

    let mut hw = unsafe { std::mem::zeroed::<HitWindows>() };
    let result = rosu_pp_beatmap_attrs_hit_windows(attrs, &mut hw);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let mut adj = unsafe { std::mem::zeroed::<AdjustedBeatmapAttributes>() };
    let result = rosu_pp_beatmap_attrs_apply_clock_rate(attrs, &mut adj);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    rosu_pp_beatmap_attrs_free(attrs);
    rosu_pp_mods_free(mods);
    rosu_pp_beatmap_free(map_handle);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn build_from_map_null_handle() {
    let null_map: *mut std::ffi::c_void = std::ptr::null_mut();
    let builder = rosu_pp_beatmap_attrs_builder_new();
    let result = rosu_pp_beatmap_attrs_builder_map(builder, null_map as *const _);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    rosu_pp_beatmap_attrs_builder_free(builder);
}

#[test]
fn build_from_map_null_builder() {
    let mut map_handle = std::ptr::null_mut();
    let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
    let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
    assert_eq!(result, rosu_pp_FfiResult::Ok);

    let null_builder: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = rosu_pp_beatmap_attrs_builder_map(null_builder as *mut _, map_handle);
    assert_eq!(result, rosu_pp_FfiResult::NullPointer);

    rosu_pp_beatmap_free(map_handle);
}
