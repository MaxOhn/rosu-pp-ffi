//! Gradual (per-frame) difficulty calculator.
//!
//! Computes star ratings incrementally as each hit object is processed,
//! enabling real-time difficulty progression tracking.

use std::ptr;

use rosu_pp::GradualDifficulty as RosuGradualDifficulty;

use crate::{
    attributes::DifficultyAttributes, beatmap::BeatmapHandle, difficulty::DifficultyHandle,
    error::FfiResult,
};

/// Opaque handle to a gradual difficulty calculator.
///
/// Created via `rosu_pp_gradual_difficulty_new`. Iterate through hit objects
/// using `rosu_pp_gradual_difficulty_next` until it returns `FfiResult::Done`.
///
/// **Must be freed** with `rosu_pp_gradual_difficulty_free` when done.
pub struct GradualDifficultyHandle {
    pub(crate) gradual: RosuGradualDifficulty,
}

/// Create a new gradual difficulty calculator.
///
/// **Parameters:**
/// - `difficulty`: A `DifficultyHandle` pointer. **Consumed** by this call.
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
pub extern "C" fn rosu_pp_gradual_difficulty_new(
    difficulty: *mut DifficultyHandle,
    map: *const BeatmapHandle,
) -> *mut GradualDifficultyHandle {
    if difficulty.is_null() || map.is_null() {
        return ptr::null_mut();
    }

    let difficulty = unsafe { Box::from_raw(difficulty) };
    let map = unsafe { &(*map).beatmap };
    let gradual = difficulty.difficulty.gradual_difficulty(map);

    Box::into_raw(Box::new(GradualDifficultyHandle { gradual }))
}

/// Process the next hit object and return incremental difficulty attributes.
///
/// Call this function repeatedly until it returns `FfiResult::Done` (all objects processed).
///
/// **Parameters:**
/// - `handle`: A valid `GradualDifficultyHandle` pointer (must not be null).
/// - `out`: Pointer to a `DifficultyAttributes` struct where results will be
///   written (must not be null).
///
/// **Returns:**
/// - `FfiResult::Ok` — More objects remain; call `next` again.
/// - `FfiResult::Done` — All objects have been processed. No more calls needed.
/// - `FfiResult::NullPointer` — `handle` or `out` is null.
///
/// **Handle reuse:** The `handle` remains valid after `Ok` and can be used for
/// subsequent calls.
#[no_mangle]
pub extern "C" fn rosu_pp_gradual_difficulty_next(
    handle: *mut GradualDifficultyHandle,
    out: *mut DifficultyAttributes,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &mut *handle };

    let Some(attrs) = h.gradual.next() else {
        return FfiResult::Done;
    };

    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
}

/// Free a gradual difficulty calculator handle.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_gradual_difficulty_new`. May be
///   null (null is a no-op).
#[no_mangle]
pub extern "C" fn rosu_pp_gradual_difficulty_free(handle: *mut GradualDifficultyHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
