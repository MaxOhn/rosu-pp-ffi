//! Tests for beatmap loading and inspection.

mod common;

use common::{Mode, beatmap_bytes, beatmap_path};

use rosu_pp_ffi::{
    rosu_pp_FfiResult, rosu_pp_beatmap_ar, rosu_pp_beatmap_bpm, rosu_pp_beatmap_cs,
    rosu_pp_beatmap_free, rosu_pp_beatmap_from_bytes, rosu_pp_beatmap_from_path,
    rosu_pp_beatmap_hit_object_count, rosu_pp_beatmap_hp, rosu_pp_beatmap_is_convert,
    rosu_pp_beatmap_mode, rosu_pp_beatmap_od, rosu_pp_beatmap_slider_multiplier,
    rosu_pp_beatmap_slider_tick_rate, rosu_pp_beatmap_stack_leniency,
    rosu_pp_beatmap_timing_point_count, rosu_pp_beatmap_version,
};

#[test]
fn from_path_null_pointer() {
    unsafe {
        let mut handle = std::ptr::null_mut();
        let result = rosu_pp_beatmap_from_path(std::ptr::null_mut(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn from_path_nonexistent() {
    unsafe {
        let mut handle = std::ptr::null_mut();
        let path = std::ffi::CString::new("/nonexistent/path/to/beatmap.osu").unwrap();
        let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::ParseError);
    }
}

#[test]
fn from_path_osu() {
    unsafe {
        let mut map_handle = std::ptr::null_mut();
        let path = std::ffi::CString::new(beatmap_path(Mode::Osu).to_str().unwrap()).unwrap();
        let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);
        assert!(!map_handle.is_null());

        let version = rosu_pp_beatmap_version(map_handle);
        assert!(version >= 14);

        let mode = rosu_pp_beatmap_mode(map_handle);
        assert_eq!(mode, 0); // osu!

        let ar = rosu_pp_beatmap_ar(map_handle);
        assert!(ar > 0.0);

        let cs = rosu_pp_beatmap_cs(map_handle);
        assert!(cs > 0.0);

        let hp = rosu_pp_beatmap_hp(map_handle);
        assert!(hp > 0.0);

        let od = rosu_pp_beatmap_od(map_handle);
        assert!(od > 0.0);

        let slider_multiplier = rosu_pp_beatmap_slider_multiplier(map_handle);
        assert!(slider_multiplier > 0.0);

        let slider_tick_rate = rosu_pp_beatmap_slider_tick_rate(map_handle);
        assert!(slider_tick_rate > 0.0);

        let stack_leniency = rosu_pp_beatmap_stack_leniency(map_handle);
        assert!(stack_leniency >= 0.0 && stack_leniency <= 1.0);

        let is_convert = rosu_pp_beatmap_is_convert(map_handle);
        assert!(!is_convert);

        let hit_object_count = rosu_pp_beatmap_hit_object_count(map_handle);
        assert!(hit_object_count > 0);

        let bpm = rosu_pp_beatmap_bpm(map_handle);
        assert!(bpm > 0.0);

        let timing_point_count = rosu_pp_beatmap_timing_point_count(map_handle);
        assert!(timing_point_count > 0);

        rosu_pp_beatmap_free(map_handle);
    }
}

#[test]
fn from_path_taiko() {
    unsafe {
        let mut map_handle = std::ptr::null_mut();
        let path = std::ffi::CString::new(beatmap_path(Mode::Taiko).to_str().unwrap()).unwrap();
        let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);

        let mode = rosu_pp_beatmap_mode(map_handle);
        assert_eq!(mode, 1); // taiko

        rosu_pp_beatmap_free(map_handle);
    }
}

#[test]
fn from_path_catch() {
    unsafe {
        let mut map_handle = std::ptr::null_mut();
        let path = std::ffi::CString::new(beatmap_path(Mode::Catch).to_str().unwrap()).unwrap();
        let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);

        let mode = rosu_pp_beatmap_mode(map_handle);
        assert_eq!(mode, 2); // catch

        rosu_pp_beatmap_free(map_handle);
    }
}

#[test]
fn from_path_mania() {
    unsafe {
        let mut map_handle = std::ptr::null_mut();
        let path = std::ffi::CString::new(beatmap_path(Mode::Mania).to_str().unwrap()).unwrap();
        let result = rosu_pp_beatmap_from_path(path.as_ptr(), &mut map_handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);

        let mode = rosu_pp_beatmap_mode(map_handle);
        assert_eq!(mode, 3); // mania

        rosu_pp_beatmap_free(map_handle);
    }
}

#[test]
fn from_bytes_null_pointer() {
    unsafe {
        let mut handle = std::ptr::null_mut();
        let result = rosu_pp_beatmap_from_bytes(std::ptr::null_mut(), 0, &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn from_bytes_osu() {
    unsafe {
        let mut handle = std::ptr::null_mut();
        let bytes = beatmap_bytes(Mode::Osu);
        let result = rosu_pp_beatmap_from_bytes(bytes.as_ptr(), bytes.len(), &mut handle);
        assert_eq!(result, rosu_pp_FfiResult::Ok);

        let mode = rosu_pp_beatmap_mode(handle);
        assert_eq!(mode, 0); // osu!

        rosu_pp_beatmap_free(handle);
    }
}

#[test]
fn out_pointer_null() {
    unsafe {
        let bytes = beatmap_bytes(Mode::Osu);
        let result = rosu_pp_beatmap_from_bytes(bytes.as_ptr(), bytes.len(), std::ptr::null_mut());
        assert_eq!(result, rosu_pp_FfiResult::NullPointer);
    }
}

#[test]
fn free_null_handle() {
    unsafe {
        rosu_pp_beatmap_free(std::ptr::null_mut());
    }
}
