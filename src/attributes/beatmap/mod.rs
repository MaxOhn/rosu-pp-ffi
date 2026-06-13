use rosu_pp::model::beatmap::BeatmapAttributes;

use crate::{
    attributes::beatmap::hit_windows::HitWindows,
    error::FfiResult,
    handle::{HandleOwned, HandleRef},
};

pub mod adjusted;
pub mod builder;
pub mod hit_windows;

/// Opaque handle to a `BeatmapAttributes` result.
///
/// Created via `rosu_pp_beatmap_attrs_builder_build`. Query it with getter
/// functions and `rosu_pp_beatmap_attrs_hit_windows`.
///
/// **Must be freed** with `rosu_pp_beatmap_attrs_free` when done.
pub struct BeatmapAttributesHandle(BeatmapAttributes);

handle!(BeatmapAttributesHandle -> BeatmapAttributes);

/// Get the approach rate from the beatmap attributes.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesHandle` pointer (must not be null).
///
/// **Returns:** The approach rate value, or `0.0` if `handle` is null.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_attrs_ar(handle: *const BeatmapAttributesHandle) -> f32 {
    if handle.is_null() {
        return 0.0;
    }

    handle.by_ref().ar()
}

/// Get the overall difficulty from the beatmap attributes.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesHandle` pointer (must not be null).
///
/// **Returns:** The overall difficulty value, or `0.0` if `handle` is null.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_attrs_od(handle: *const BeatmapAttributesHandle) -> f32 {
    if handle.is_null() {
        return 0.0;
    }

    handle.by_ref().od()
}

/// Get the circle size from the beatmap attributes.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesHandle` pointer (must not be null).
///
/// **Returns:** The circle size value, or `0.0` if `handle` is null.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_attrs_cs(handle: *const BeatmapAttributesHandle) -> f32 {
    if handle.is_null() {
        return 0.0;
    }

    handle.by_ref().cs()
}

/// Get the HP drain rate from the beatmap attributes.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesHandle` pointer (must not be null).
///
/// **Returns:** The HP drain rate value, or `0.0` if `handle` is null.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_attrs_hp(handle: *const BeatmapAttributesHandle) -> f32 {
    if handle.is_null() {
        return 0.0;
    }

    handle.by_ref().hp()
}

/// Get the clock rate from the beatmap attributes.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesHandle` pointer (must not be null).
///
/// **Returns:** The clock rate value, or `0.0` if `handle` is null.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_attrs_clock_rate(handle: *const BeatmapAttributesHandle) -> f64 {
    if handle.is_null() {
        return 0.0;
    }

    handle.by_ref().clock_rate()
}

/// Calculate the AR and OD hit windows for the beatmap attributes.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesHandle` pointer (must not be null).
/// - `out`: Pointer to a `HitWindows` struct where results will be written
///   (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_attrs_hit_windows(
    handle: *const BeatmapAttributesHandle,
    out: *mut HitWindows,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    unsafe { *out = HitWindows::from(&handle.by_ref().hit_windows()) };

    FfiResult::Ok
}

/// Free a beatmap attributes handle and release its memory.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_beatmap_attrs_builder_build`.
///   May be null (null is a no-op).
///
/// After calling this function, the handle must NOT be used again.
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_beatmap_attrs_free(handle: *mut BeatmapAttributesHandle) {
    handle.drop_handle();
}
