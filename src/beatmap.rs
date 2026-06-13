//! Beatmap loading and inspection handles.
//!
//! Provides functions to load `.osu` beatmap files from disk or raw bytes,
//! and inspect their properties (AR, CS, HP, OD, timing points, etc.).

use std::{ffi, ptr, slice};

use rosu_pp::Beatmap;

use crate::{
    error::FfiResult,
    handle::{HandleOwned, HandleRef},
};

/// Opaque handle to a loaded osu! beatmap.
///
/// Created via `rosu_pp_beatmap_from_path` or `rosu_pp_beatmap_from_bytes`.
/// Must be freed with `rosu_pp_beatmap_free` when no longer needed.
pub struct BeatmapHandle(Beatmap);

handle!(BeatmapHandle -> Beatmap);

/// Load a beatmap from a file path.
///
/// **Parameters:**
/// - `path`: Null-terminated C string containing the file path to the `.osu` file.
/// - `out`: Pointer to store the resulting `BeatmapHandle`.
///
/// **Returns:**
/// - `FfiResult::Ok` — Success. `out` is set to a non-null handle (caller owns).
/// - `FfiResult::ParseError` — The path is invalid UTF-8 or the file could not
///   be read or parsed. `out` is set to `NULL`.
/// - `FfiResult::NullPointer` — The `path` or `out` pointer is null.
///
/// **Memory:** On `Ok`, the caller owns the handle written to `out` and must
/// free it with `rosu_pp_beatmap_free`.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_from_path(
    path: *const ffi::c_char,
    out: *mut *mut BeatmapHandle,
) -> FfiResult {
    if path.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let c_str = unsafe { ffi::CStr::from_ptr(path) };

    let Ok(path) = c_str.to_str() else {
        unsafe { *out = ptr::null_mut() };

        return FfiResult::ParseError;
    };

    let Ok(beatmap) = Beatmap::from_path(path) else {
        unsafe { *out = ptr::null_mut() };

        return FfiResult::ParseError;
    };

    unsafe { *out = Box::into_raw(Box::new(BeatmapHandle::from(beatmap))) };

    FfiResult::Ok
}

/// Load a beatmap from raw bytes.
///
/// **Parameters:**
/// - `bytes`: Pointer to a buffer containing the `.osu` file contents.
/// - `len`: Length of the buffer in bytes.
/// - `out`: Pointer to store the resulting `BeatmapHandle`.
///
/// **Returns:**
/// - `FfiResult::Ok` — Success. `out` is set to a non-null handle (caller owns).
/// - `FfiResult::ParseError` — The bytes could not be parsed as a valid beatmap.
///   `out` is set to `NULL`.
/// - `FfiResult::NullPointer` — The `bytes` or `out` pointer is null.
///
/// **Memory:** On `Ok`, the caller owns the handle written to `out` and must
/// free it with `rosu_pp_beatmap_free`. The `bytes` buffer is only borrowed
/// during this call and may be freed immediately after.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_from_bytes(
    bytes: *const u8,
    len: usize,
    out: *mut *mut BeatmapHandle,
) -> FfiResult {
    if bytes.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let slice = unsafe { slice::from_raw_parts(bytes, len) };

    let Ok(beatmap) = Beatmap::from_bytes(slice) else {
        unsafe { *out = ptr::null_mut() };

        return FfiResult::ParseError;
    };

    unsafe { *out = Box::into_raw(Box::new(BeatmapHandle::from(beatmap))) };

    FfiResult::Ok
}

macro_rules! getter {
    ( $fn:ident( $field:ident ) -> $ty:ty ) => {
        /// Returns the value of the `$field` field from the beatmap.
        ///
        /// **Parameters:**
        /// - `handle`: A valid beatmap handle pointer (must not be null).
        ///
        /// **Returns:** The field value, or the type's default value if `handle` is null.
        #[unsafe(no_mangle)]
        pub extern "C" fn $fn(handle: *const BeatmapHandle) -> $ty {
            if handle.is_null() {
                return <$ty>::default();
            }

            handle.by_ref().$field
        }
    };
    ( $fn:ident( $expr:expr ) -> $ty:ty ) => {
        /// Returns a computed value derived from the beatmap.
        ///
        /// **Parameters:**
        /// - `handle`: A valid beatmap handle pointer (must not be null).
        ///
        /// **Returns:** The computed value, or the type's default value if `handle` is null.
        #[unsafe(no_mangle)]
        pub extern "C" fn $fn(handle: *const BeatmapHandle) -> $ty {
            if handle.is_null() {
                return <$ty>::default();
            }

            ($expr)(handle.by_ref())
        }
    };
}

getter!(rosu_pp_beatmap_version(version) -> i32);
getter!(rosu_pp_beatmap_mode(|map: &Beatmap| map.mode as i32) -> i32);
getter!(rosu_pp_beatmap_ar(ar) -> f32);
getter!(rosu_pp_beatmap_cs(cs) -> f32);
getter!(rosu_pp_beatmap_hp(hp) -> f32);
getter!(rosu_pp_beatmap_od(od) -> f32);
getter!(rosu_pp_beatmap_slider_multiplier(slider_multiplier) -> f64);
getter!(rosu_pp_beatmap_slider_tick_rate(slider_tick_rate) -> f64);
getter!(rosu_pp_beatmap_stack_leniency(stack_leniency) -> f32);
getter!(rosu_pp_beatmap_is_convert(is_convert) -> bool);
getter!(rosu_pp_beatmap_hit_object_count(|map: &Beatmap| map.hit_objects.len()) -> usize);
getter!(rosu_pp_beatmap_total_break_time(|map: &Beatmap| map.total_break_time()) -> f64);
getter!(rosu_pp_beatmap_bpm(|map: &Beatmap| map.bpm()) -> f64);
getter!(rosu_pp_beatmap_timing_point_count(|map: &Beatmap| map.timing_points.len()) -> usize);
getter!(rosu_pp_beatmap_difficulty_point_count(|map: &Beatmap| map.difficulty_points.len()) -> usize);
getter!(rosu_pp_beatmap_effect_point_count(|map: &Beatmap| map.effect_points.len()) -> usize);
getter!(rosu_pp_beatmap_break_count(|map: &Beatmap| map.breaks.len()) -> usize);
getter!(rosu_pp_beatmap_hit_sound_count(|map: &Beatmap| map.hit_sounds.len()) -> usize);

/// Check whether the beatmap contains suspicious hit objects.
///
/// Some beatmaps contain hit objects that appear too suspicious for further
/// calculation (e.g., maps designed to test the limits of osu!). This function
/// checks for such cases.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapHandle` pointer (must not be null).
///
/// **Returns:** `FfiResult::Ok` if the map is safe to use, or
/// `FfiResult::TooSuspicious` if the map contains suspicious objects.
/// Returns `FfiResult::NullPointer` if `handle` is null.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_check_suspicion(handle: *const BeatmapHandle) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    match handle.by_ref().check_suspicion() {
        Ok(()) => FfiResult::Ok,
        Err(_) => FfiResult::TooSuspicious,
    }
}

/// Free a beatmap handle and release its memory.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_beatmap_from_path` or
///   `rosu_pp_beatmap_from_bytes`. May be null (null is a no-op).
///
/// After calling this function, the handle must NOT be used again.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_free(handle: *mut BeatmapHandle) {
    handle.drop_handle();
}
