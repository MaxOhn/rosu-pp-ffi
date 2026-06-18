use std::ptr;

use rosu_pp::model::beatmap::BeatmapAttributesBuilder;

use crate::{
    attributes::beatmap::BeatmapAttributesHandle,
    beatmap::BeatmapHandle,
    difficulty::DifficultyHandle,
    error::FfiResult,
    handle::{HandleMut, HandleOwned, HandleRef},
    mode::GameMode,
    mods::ModsHandle,
};

handle! {
    /// Opaque handle to a `BeatmapAttributesBuilder`.
    ///
    /// Created via `rosu_pp_beatmap_attrs_builder_new`. Configure it with setter
    /// functions, then build with `rosu_pp_beatmap_attrs_builder_build`.
    ///
    /// **Must be freed** with `rosu_pp_beatmap_attrs_builder_free` when done.
    BeatmapAttributesBuilderHandle -> BeatmapAttributesBuilder
}

/// Create a new beatmap attributes builder with default settings.
///
/// **Returns:** A non-null handle to a new `BeatmapAttributesBuilder`.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_beatmap_attrs_builder_free`.
///
/// # Safety
///
/// This function is safe to call from any context. It takes no raw pointer
/// arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_new() -> *mut BeatmapAttributesBuilderHandle
{
    Box::into_raw(Box::new(BeatmapAttributesBuilderHandle::from(
        BeatmapAttributesBuilder::new(),
    )))
}

/// Populate the builder from a beatmap's attributes (AR, OD, CS, HP, mode,
/// convert status).
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `map`: A valid `BeatmapHandle` pointer (may be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// either pointer is null.
///
/// **Handle reuse:** The `handle` remains valid after this call. Individual
/// setters (ar, od, cs, hp) can be called after `map` to override specific
/// values.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
/// `map` must be a valid pointer to a `BeatmapHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_map(
    handle: *mut BeatmapAttributesBuilderHandle,
    map: *const BeatmapHandle,
) -> FfiResult {
    if handle.is_null() || map.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().map(map.by_ref());

    FfiResult::Ok
}

/// Override the approach rate.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `ar`: The approach rate value.
/// - `fixed`: If `true`, the value is used as-is with no mod/clock-rate
///   adjustment. If `false`, the value may be adjusted by mods and clock rate.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_ar(
    handle: *mut BeatmapAttributesBuilderHandle,
    ar: f32,
    fixed: bool,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().ar(ar, fixed);

    FfiResult::Ok
}

/// Override the overall difficulty.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `od`: The overall difficulty value.
/// - `fixed`: If `true`, the value is used as-is with no mod/clock-rate
///   adjustment. If `false`, the value may be adjusted by mods and clock rate.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_od(
    handle: *mut BeatmapAttributesBuilderHandle,
    od: f32,
    fixed: bool,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().od(od, fixed);

    FfiResult::Ok
}

/// Override the circle size.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `cs`: The circle size value.
/// - `fixed`: If `true`, the value is used as-is with no mod/clock-rate
///   adjustment. If `false`, the value may be adjusted by mods and clock rate.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_cs(
    handle: *mut BeatmapAttributesBuilderHandle,
    cs: f32,
    fixed: bool,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().cs(cs, fixed);

    FfiResult::Ok
}

/// Override the HP drain rate.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `hp`: The HP drain rate value.
/// - `fixed`: If `true`, the value is used as-is with no mod/clock-rate
///   adjustment. If `false`, the value may be adjusted by mods and clock rate.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_hp(
    handle: *mut BeatmapAttributesBuilderHandle,
    hp: f32,
    fixed: bool,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().hp(hp, fixed);

    FfiResult::Ok
}

/// Set the game mods for the beatmap attributes calculation.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `mods`: A `ModsHandle` pointer containing the mods to apply (may be null
///   to clear mods).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_mods(
    handle: *mut BeatmapAttributesBuilderHandle,
    mods: *const ModsHandle,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    let handle = handle.by_mut();

    if let Some(mods) = mods.checked_by_ref() {
        handle.mods(mods.to_owned());
    } else {
        handle.mods(0);
    }

    FfiResult::Ok
}

/// Set a custom clock rate.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `clock_rate`: The clock rate value.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_clock_rate(
    handle: *mut BeatmapAttributesBuilderHandle,
    clock_rate: f64,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().clock_rate(clock_rate);

    FfiResult::Ok
}

/// Set the game mode and convert status.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `mode`: The game mode.
/// - `is_convert`: Whether this is a converted map.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_mode(
    handle: *mut BeatmapAttributesBuilderHandle,
    mode: GameMode,
    is_convert: bool,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().mode(mode.into(), is_convert);

    FfiResult::Ok
}

/// Populate the builder from a difficulty calculator's settings.
///
/// Copies the map difficulty attributes (AR, OD, CS, HP), mods, and clock
/// rate from the difficulty calculator.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer (may be null).
/// - `difficulty`: A valid `DifficultyHandle` pointer (may be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// either pointer is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
/// `difficulty` must be a valid pointer to a `DifficultyHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_difficulty(
    handle: *mut BeatmapAttributesBuilderHandle,
    difficulty: *const DifficultyHandle,
) -> FfiResult {
    if handle.is_null() || difficulty.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_mut().difficulty(difficulty.by_ref());

    FfiResult::Ok
}

/// Build the `BeatmapAttributes` from the configured builder.
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesBuilderHandle` pointer. **Consumed**
///   by this call. The handle must NOT be used or freed after this call.
///
/// **Returns:** A non-null `BeatmapAttributesHandle` on success, or `NULL` if
/// `handle` is null.
///
/// **Ownership:** This function **consumes** the `handle`. The caller must NOT
/// call `rosu_pp_beatmap_attrs_free` on the builder handle, nor use it after
/// this call.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_beatmap_attrs_free`.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesBuilderHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_build(
    handle: *mut BeatmapAttributesBuilderHandle,
) -> *mut BeatmapAttributesHandle {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let attrs = handle.by_mut().build();

    Box::into_raw(Box::new(BeatmapAttributesHandle::from(attrs)))
}

/// Free a beatmap attributes builder handle and release its memory.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_beatmap_attrs_builder_new`.
///   May be null (null is a no-op).
///
/// **Note:** Do NOT call this function if the handle was passed to
/// `rosu_pp_beatmap_attrs_builder_build` — that function consumes the
/// builder handle.
///
/// # Safety
///
/// `handle` must be a null pointer, or a valid handle previously returned by
/// `rosu_pp_beatmap_attrs_builder_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_builder_free(
    handle: *mut BeatmapAttributesBuilderHandle,
) {
    handle.drop_handle();
}
