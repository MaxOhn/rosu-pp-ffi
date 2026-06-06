//! Beatmap loading and inspection handles.
//!
//! Provides functions to load `.osu` beatmap files from disk or raw bytes,
//! and inspect their properties (AR, CS, HP, OD, timing points, etc.).

use std::{ffi, ptr, slice};

use rosu_pp::{model::mode::GameMode, Beatmap};

/// Opaque handle to a loaded osu! beatmap.
///
/// Created via `rosu_pp_beatmap_from_path` or `rosu_pp_beatmap_from_bytes`.
/// Must be freed with `rosu_pp_beatmap_free` when no longer needed.
pub struct BeatmapHandle {
    pub(crate) beatmap: Beatmap,
}

/// Load a beatmap from a file path.
///
/// **Parameters:**
/// - `path`: Null-terminated C string containing the file path to the `.osu` file.
///
/// **Returns:** A non-null handle on success, or `NULL` if:
/// - The path pointer is null
/// - The path contains invalid UTF-8
/// - The file cannot be read or parsed
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_beatmap_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_beatmap_from_path(path: *const ffi::c_char) -> *mut BeatmapHandle {
    if path.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe { ffi::CStr::from_ptr(path) };

    let Ok(path) = c_str.to_str() else {
        return ptr::null_mut();
    };

    let beatmap = match Beatmap::from_path(path) {
        Ok(b) => b,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(BeatmapHandle { beatmap }))
}

/// Load a beatmap from raw bytes.
///
/// **Parameters:**
/// - `bytes`: Pointer to a buffer containing the `.osu` file contents.
/// - `len`: Length of the buffer in bytes.
///
/// **Returns:** A non-null handle on success, or `NULL` if:
/// - The bytes pointer is null
/// - The bytes cannot be parsed as a valid beatmap
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_beatmap_free`. The `bytes` buffer is only borrowed during this call
/// and may be freed immediately after.
#[no_mangle]
pub extern "C" fn rosu_pp_beatmap_from_bytes(bytes: *const u8, len: usize) -> *mut BeatmapHandle {
    if bytes.is_null() {
        return ptr::null_mut();
    }

    let slice = unsafe { slice::from_raw_parts(bytes, len) };

    let beatmap = match Beatmap::from_bytes(slice) {
        Ok(b) => b,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(BeatmapHandle { beatmap }))
}

macro_rules! getter {
    ( $fn:ident( $field:ident ) -> $ty:ty ) => {
        /// Returns the value of the `$field` field from the beatmap.
        ///
        /// **Parameters:**
        /// - `handle`: A valid beatmap handle pointer (must not be null).
        ///
        /// **Returns:** The field value, or the type's default value if `handle` is null.
        #[no_mangle]
        pub extern "C" fn $fn(handle: *const BeatmapHandle) -> $ty {
            if handle.is_null() {
                return <$ty>::default();
            }

            unsafe { (*handle).beatmap.$field }
        }
    };
    ( $fn:ident( $expr:expr ) -> $ty:ty ) => {
        /// Returns a computed value derived from the beatmap.
        ///
        /// **Parameters:**
        /// - `handle`: A valid beatmap handle pointer (must not be null).
        ///
        /// **Returns:** The computed value, or the type's default value if `handle` is null.
        #[no_mangle]
        pub extern "C" fn $fn(handle: *const BeatmapHandle) -> $ty {
            if handle.is_null() {
                return <$ty>::default();
            }

            unsafe { ($expr)(&(*handle).beatmap) }
        }
    };
}

getter!(rosu_pp_beatmap_version(version) -> i32);
getter!(rosu_pp_beatmap_mode(|map: &Beatmap| GameMode::from(map.mode) as i32) -> i32);
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

/// Free a beatmap handle and release its memory.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_beatmap_from_path` or
///   `rosu_pp_beatmap_from_bytes`. May be null (null is a no-op).
///
/// After calling this function, the handle must NOT be used again.
#[no_mangle]
pub extern "C" fn rosu_pp_beatmap_free(handle: *mut BeatmapHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
