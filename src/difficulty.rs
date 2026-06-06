//! Difficulty calculator handle and configuration.
//!
//! Provides a builder-style interface for configuring and running difficulty
//! (star rating) calculations. Each setter returns `FfiResult::Ok` and the
//! handle remains valid for further configuration.

use std::{mem, ptr};

use rosu_pp::{model::beatmap::BeatmapAttribute, Difficulty};

use crate::{
    attributes::DifficultyAttributes,
    beatmap::BeatmapHandle,
    error::FfiResult,
    gradual::difficulty::GradualDifficultyHandle,
    mods::ModsHandle,
    strains::{build_strains_handle, StrainsHandle},
};

/// Opaque handle to a difficulty calculator builder.
///
/// Created via `rosu_pp_difficulty_new`. Configure it with setter functions,
/// then calculate with `rosu_pp_difficulty_calculate`.
///
/// **Builder pattern:** Each setter consumes the handle internally (using
/// `Box::from_raw` + `mem::forget`) and returns `FfiResult::Ok`. The handle
/// pointer remains valid and can be used for subsequent setter calls.
///
/// **Must be freed** with `rosu_pp_difficulty_free` when done.
pub struct DifficultyHandle {
    pub(crate) difficulty: Difficulty,
}

/// Create a new difficulty calculator with default settings.
///
/// **Returns:** A non-null handle to a new `Difficulty` builder.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_difficulty_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_new() -> *mut DifficultyHandle {
    Box::into_raw(Box::new(DifficultyHandle {
        difficulty: Difficulty::new(),
    }))
}

/// Clone a difficulty calculator handle.
///
/// Creates an independent copy of the difficulty builder, including all
/// configured settings (mods, passed_objects, clock_rate, attribute overrides, etc.).
///
/// **Parameters:**
/// - `handle`: A valid `DifficultyHandle` pointer (must not be null).
///
/// **Returns:** A new handle on success, or `NULL` if `handle` is null.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_difficulty_free`. The original `handle` remains valid.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_clone(
    handle: *const DifficultyHandle,
) -> *mut DifficultyHandle {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let h = unsafe { &*handle };

    Box::into_raw(Box::new(DifficultyHandle {
        difficulty: h.difficulty.clone(),
    }))
}

/// Set the game mods for the difficulty calculation.
///
/// **Parameters:**
/// - `handle`: A valid `DifficultyHandle` pointer (must not be null).
/// - `mods`: A `ModsHandle` pointer containing the mods to apply (may be null
///   to clear mods, though this is equivalent to not setting any).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_mods(
    handle: *mut DifficultyHandle,
    mods: *const ModsHandle,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    let mut h = unsafe { Box::from_raw(handle) };
    let mods = unsafe { &*mods };
    h.difficulty = h.difficulty.mods(mods.mods.clone());
    mem::forget(h);

    FfiResult::Ok
}

macro_rules! setter {
    ( $fn:ident ( $arg:ident: $ty:ty $(, $args:ident: $tys:ty ),* ) ) => {
        /// Configuration setter for the difficulty calculator.
        ///
        /// **Parameters:**
        /// - `handle`: A valid `DifficultyHandle` pointer (must not be null).
        /// - `$arg`: The primary parameter value.
        /// $(, `$args`): Additional parameter values.
        ///
        /// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer`
        /// if `handle` is null.
        ///
        /// **Handle reuse:** The `handle` remains valid after this call.
        #[no_mangle]
        pub extern "C" fn $fn(
            handle: *mut DifficultyHandle,
            $arg: $ty
            $(, $args: $tys )*
        ) -> FfiResult {
            if handle.is_null() {
                return FfiResult::NullPointer;
            }

            let mut h = unsafe { Box::from_raw(handle) };
            h.difficulty = h.difficulty.$arg( $arg $(, $args )* );
            mem::forget(h);

            FfiResult::Ok
        }
    }
}

setter!(rosu_pp_difficulty_passed_objects(passed_objects: u32));
setter!(rosu_pp_difficulty_clock_rate(clock_rate: f64));
setter!(rosu_pp_difficulty_ar(ar: f32, fixed: bool));
setter!(rosu_pp_difficulty_cs(cs: f32, fixed: bool));
setter!(rosu_pp_difficulty_hp(hp: f32, fixed: bool));
setter!(rosu_pp_difficulty_od(od: f32, fixed: bool));
setter!(rosu_pp_difficulty_hardrock_offsets(hardrock_offsets: bool));
setter!(rosu_pp_difficulty_lazer(lazer: bool));

/// Calculate difficulty attributes for the configured settings.
///
/// **Parameters:**
/// - `handle`: A valid `DifficultyHandle` pointer. **Consumed** by this call.
///   The handle must NOT be used or freed after this call.
/// - `map`: A valid `BeatmapHandle` pointer (must not be null).
/// - `out`: Pointer to a `DifficultyAttributes` struct where results will be written.
///   (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle`, `map`, or `out` is null.
///
/// **Ownership:** This function **consumes** the `handle`. The caller must NOT
/// call `rosu_pp_difficulty_free` on the handle, nor use it after this call.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_calculate(
    handle: *mut DifficultyHandle,
    map: *const BeatmapHandle,
    out: *mut crate::attributes::DifficultyAttributes,
) -> FfiResult {
    if handle.is_null() || map.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { Box::from_raw(handle) };
    let map = unsafe { &(*map).beatmap };
    let attrs = h.difficulty.calculate(map);
    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
}

/// Calculate difficulty attributes for the configured settings after verifying
/// the map is not too suspicious.
///
/// Same as `rosu_pp_difficulty_calculate` but checks the map for suspicious
/// hit objects first. If the map is too suspicious, returns `FfiResult::TooSuspicious`.
///
/// **Parameters:**
/// - `handle`: A valid `DifficultyHandle` pointer. **Consumed** by this call.
///   The handle must NOT be used or freed after this call.
/// - `map`: A valid `BeatmapHandle` pointer (must not be null).
/// - `out`: Pointer to a `DifficultyAttributes` struct where results will be written.
///   (must not be null).
///
/// **Returns:**
/// - `FfiResult::Ok` — Calculation succeeded.
/// - `FfiResult::TooSuspicious` — The map contains suspicious hit objects.
/// - `FfiResult::NullPointer` — `handle`, `map`, or `out` is null.
///
/// **Ownership:** This function **consumes** the `handle`. The caller must NOT
/// call `rosu_pp_difficulty_free` on the handle, nor use it after this call.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_checked_calculate(
    handle: *mut DifficultyHandle,
    map: *const BeatmapHandle,
    out: *mut DifficultyAttributes,
) -> FfiResult {
    if handle.is_null() || map.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { Box::from_raw(handle) };
    let map = unsafe { &(*map).beatmap };

    match h.difficulty.checked_calculate(map) {
        Ok(attrs) => {
            unsafe { *out = (&attrs).into() };

            FfiResult::Ok
        }
        Err(_) => FfiResult::TooSuspicious,
    }
}

/// Perform the difficulty calculation but instead of evaluating the skill
/// strains, return them as is. Suitable for plotting the difficulty of a map
/// over time.
///
/// **Parameters:**
/// - `handle`: A `DifficultyHandle` pointer. **Consumed** by this call.
///   The handle must NOT be used or freed after this call.
/// - `map`: A valid `BeatmapHandle` pointer (must not be null).
///
/// **Returns:** A non-null `StrainsHandle` on success, or `NULL` if either
/// pointer is null.
///
/// **Ownership:** This function **consumes** the `difficulty` handle. The caller
/// must NOT call `rosu_pp_difficulty_free` on the handle, nor use it after this call.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_strains_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_strains(
    handle: *mut DifficultyHandle,
    map: *const BeatmapHandle,
) -> *mut StrainsHandle {
    if handle.is_null() || map.is_null() {
        return ptr::null_mut();
    }

    let h = unsafe { Box::from_raw(handle) };
    let map = unsafe { &(*map).beatmap };
    let strains = h.difficulty.strains(map);

    Box::into_raw(build_strains_handle(strains))
}

/// Create a gradual difficulty calculator for incremental star rating calculation.
///
/// The gradual difficulty calculator processes hit objects one at a time,
/// returning updated star ratings after each object.
///
/// **Parameters:**
/// - `handle`: A `DifficultyHandle` pointer. **Consumed** by this call.
///   The caller must NOT use or free this handle afterward.
/// - `map`: A valid `BeatmapHandle` pointer (must not be null).
///
/// **Returns:** A non-null handle on success, or `NULL` if either pointer is null.
///
/// **Ownership:** This function **consumes** the `difficulty` handle. The caller
/// must NOT call `rosu_pp_difficulty_free` on the difficulty handle, nor use it
/// after this call. The `map` handle is only borrowed and remains valid.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_gradual_difficulty_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_gradual_difficulty(
    handle: *mut DifficultyHandle,
    map: *const BeatmapHandle,
) -> *mut GradualDifficultyHandle {
    if handle.is_null() || map.is_null() {
        return ptr::null_mut();
    }

    let difficulty = unsafe { Box::from_raw(handle) };
    let map = unsafe { &(*map).beatmap };
    let gradual = difficulty.difficulty.gradual_difficulty(map);

    Box::into_raw(Box::new(GradualDifficultyHandle { gradual }))
}

/// Turn the difficulty calculator into an inspector to view its configured values.
///
/// **Parameters:**
/// - `handle`: A `DifficultyHandle` pointer. **Consumed** by this call.
///   The handle must NOT be used or freed after this call.
///
/// **Returns:** A non-null `InspectDifficultyHandle` pointer on success, or `NULL`
/// if `handle` is null.
///
/// **Ownership:** This function **consumes** the `difficulty` handle. The caller
/// must NOT call `rosu_pp_difficulty_free` on the handle, nor use it after this call.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_inspect_difficulty_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_inspect(
    handle: *mut DifficultyHandle,
) -> *mut InspectDifficultyHandle {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let h = unsafe { Box::from_raw(handle) };
    let inspect = h.difficulty.inspect();

    Box::into_raw(Box::new(InspectDifficultyHandle { inspect }))
}

/// Opaque handle to an inspected difficulty calculator.
///
/// Created via `rosu_pp_difficulty_inspect`. Use getter functions to inspect
/// the configured values.
pub struct InspectDifficultyHandle {
    inspect: rosu_pp::any::InspectDifficulty,
}

/// Inspect the mods configured on a difficulty calculator.
///
/// **Parameters:**
/// - `handle`: A valid `InspectDifficultyHandle` pointer (must not be null).
///
/// **Returns:** A `ModsHandle` pointer on success, or `NULL` if `handle` is null.
///
/// **Memory:** The returned handle is owned by the inspector and will be freed
/// when the inspector is freed. The caller must NOT free it separately.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_mods(
    handle: *const InspectDifficultyHandle,
) -> *mut ModsHandle {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let h = unsafe { &*handle };

    Box::into_raw(Box::new(ModsHandle {
        mods: h.inspect.mods.clone(),
    }))
}

/// Inspect the passed objects count configured on a difficulty calculator.
///
/// **Parameters:**
/// - `handle`: A valid `InspectDifficultyHandle` pointer (must not be null).
/// - `out`: Pointer to store the result (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null. A value of `0` with `FfiResult::Ok` means
/// no passed objects were set (defaults to processing all objects).
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_passed_objects(
    handle: *const InspectDifficultyHandle,
    out: *mut u32,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };
    unsafe { *out = h.inspect.passed_objects.unwrap_or(0) };

    FfiResult::Ok
}

/// Inspect the clock rate configured on a difficulty calculator.
///
/// **Parameters:**
/// - `handle`: A valid `InspectDifficultyHandle` pointer (must not be null).
/// - `out`: Pointer to store the result (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null. A value of `0.0` means no custom clock rate was set.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_clock_rate(
    handle: *const InspectDifficultyHandle,
    out: *mut f64,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };
    unsafe { *out = h.inspect.clock_rate.unwrap_or(0.0) };

    FfiResult::Ok
}

/// Inspect the AR override configured on a difficulty calculator.
///
/// **Parameters:**
/// - `handle`: A valid `InspectDifficultyHandle` pointer (must not be null).
/// - `out`: Pointer to store the result (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null. A value of `0.0` means no AR override was set.
///
/// The `fixed` flag indicates whether the value is fixed (true) or relative to mods (false).
/// Returns `0.0` for `fixed` if no AR override was set.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_ar(
    handle: *const InspectDifficultyHandle,
    out: *mut f32,
    fixed: *mut bool,
) -> FfiResult {
    if handle.is_null() || out.is_null() || fixed.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };

    match h.inspect.ar {
        BeatmapAttribute::None | BeatmapAttribute::Value(_) => unsafe {
            *out = 0.0;
            *fixed = false;
        },
        BeatmapAttribute::Given(v) => unsafe {
            *out = v;
            *fixed = false;
        },
        BeatmapAttribute::Fixed(v) => unsafe {
            *out = v;
            *fixed = true;
        },
    }

    FfiResult::Ok
}

/// Inspect the CS override configured on a difficulty calculator.
///
/// Same pattern as `rosu_pp_inspect_difficulty_ar`.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_cs(
    handle: *const InspectDifficultyHandle,
    out: *mut f32,
    fixed: *mut bool,
) -> FfiResult {
    if handle.is_null() || out.is_null() || fixed.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };

    match h.inspect.cs {
        BeatmapAttribute::None | BeatmapAttribute::Value(_) => unsafe {
            *out = 0.0;
            *fixed = false;
        },
        BeatmapAttribute::Given(v) => unsafe {
            *out = v;
            *fixed = false;
        },
        BeatmapAttribute::Fixed(v) => unsafe {
            *out = v;
            *fixed = true;
        },
    }

    FfiResult::Ok
}

/// Inspect the HP override configured on a difficulty calculator.
///
/// Same pattern as `rosu_pp_inspect_difficulty_ar`.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_hp(
    handle: *const InspectDifficultyHandle,
    out: *mut f32,
    fixed: *mut bool,
) -> FfiResult {
    if handle.is_null() || out.is_null() || fixed.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };

    match h.inspect.hp {
        BeatmapAttribute::None | BeatmapAttribute::Value(_) => unsafe {
            *out = 0.0;
            *fixed = false;
        },
        BeatmapAttribute::Given(v) => unsafe {
            *out = v;
            *fixed = false;
        },
        BeatmapAttribute::Fixed(v) => unsafe {
            *out = v;
            *fixed = true;
        },
    }

    FfiResult::Ok
}

/// Inspect the OD override configured on a difficulty calculator.
///
/// Same pattern as `rosu_pp_inspect_difficulty_ar`.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_od(
    handle: *const InspectDifficultyHandle,
    out: *mut f32,
    fixed: *mut bool,
) -> FfiResult {
    if handle.is_null() || out.is_null() || fixed.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };

    match h.inspect.od {
        BeatmapAttribute::None | BeatmapAttribute::Value(_) => unsafe {
            *out = 0.0;
            *fixed = false;
        },
        BeatmapAttribute::Given(v) => unsafe {
            *out = v;
            *fixed = false;
        },
        BeatmapAttribute::Fixed(v) => unsafe {
            *out = v;
            *fixed = true;
        },
    }

    FfiResult::Ok
}

/// Inspect whether hardrock offsets are configured on a difficulty calculator.
///
/// **Parameters:**
/// - `handle`: A valid `InspectDifficultyHandle` pointer (must not be null).
/// - `out`: Pointer to store the result (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null. A value of `false` means no custom setting was applied.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_hardrock_offsets(
    handle: *const InspectDifficultyHandle,
    out: *mut bool,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };
    unsafe { *out = h.inspect.hardrock_offsets.unwrap_or(false) };

    FfiResult::Ok
}

/// Inspect whether lazer mode is configured on a difficulty calculator.
///
/// **Parameters:**
/// - `handle`: A valid `InspectDifficultyHandle` pointer (must not be null).
/// - `out`: Pointer to store the result (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null. A value of `false` means no custom setting was applied.
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_lazer(
    handle: *const InspectDifficultyHandle,
    out: *mut bool,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };
    unsafe { *out = h.inspect.lazer.unwrap_or(false) };

    FfiResult::Ok
}

/// Free an inspected difficulty handle.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_difficulty_inspect`. May be null
///   (null is a no-op).
#[no_mangle]
pub extern "C" fn rosu_pp_inspect_difficulty_free(handle: *mut InspectDifficultyHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}

/// Free a difficulty calculator handle.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_difficulty_new` or
///   `rosu_pp_difficulty_clone`. May be null (null is a no-op).
///
/// **Note:** Do NOT call this function if the handle was passed to
/// `rosu_pp_difficulty_calculate`, `rosu_pp_difficulty_checked_calculate`,
/// `rosu_pp_difficulty_strains`, `rosu_pp_difficulty_gradual_difficulty`,
/// or `rosu_pp_difficulty_inspect` — those functions consume the handle.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_free(handle: *mut DifficultyHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
