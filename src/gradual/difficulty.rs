//! Gradual (per-object) difficulty calculator.
//!
//! Computes star ratings incrementally as each hit object is processed,
//! enabling real-time difficulty progression tracking.

use std::ptr;

use rosu_pp::GradualDifficulty;

use crate::{
    attributes::difficulty::DifficultyAttributes,
    beatmap::BeatmapHandle,
    difficulty::DifficultyHandle,
    error::FfiResult,
    handle::{HandleMut, HandleOwned, HandleRef},
};

/// Opaque handle to a gradual difficulty calculator.
///
/// Created via `rosu_pp_gradual_difficulty_new`. Iterate through hit objects
/// using `rosu_pp_gradual_difficulty_next` until it returns `FfiResult::Done`.
///
/// **Must be freed** with `rosu_pp_gradual_difficulty_free` when done.
pub struct GradualDifficultyHandle(GradualDifficulty);

handle!(GradualDifficultyHandle -> GradualDifficulty);

/// Create a new gradual difficulty calculator.
///
/// **Parameters:**
/// - `difficulty`: A `DifficultyHandle` pointer. **Consumed** by this call.
///   The caller must NOT use or free this handle afterward.
/// - `map`: A valid `BeatmapHandle` pointer (may be null).
///
/// **Returns:** A non-null handle on success, or `NULL` if either pointer is null.
///
/// **Ownership:** This function **consumes** the `difficulty` handle. The caller
/// must NOT call `rosu_pp_difficulty_free` on the difficulty handle, nor use it
/// after this call. The `map` handle is only borrowed and remains valid.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_gradual_difficulty_free`.
///
/// # Safety
///
/// `difficulty` must be a valid pointer to a `DifficultyHandle`, or null.
/// `map` must be a valid pointer to a `BeatmapHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_gradual_difficulty_new(
    difficulty: *mut DifficultyHandle,
    map: *const BeatmapHandle,
) -> *mut GradualDifficultyHandle {
    if difficulty.is_null() || map.is_null() {
        return ptr::null_mut();
    }

    let gradual = difficulty.into_owned().gradual_difficulty(map.by_ref());

    Box::into_raw(Box::new(GradualDifficultyHandle::from(gradual)))
}

/// Process the next hit object and return incremental difficulty attributes.
///
/// Call this function repeatedly until it returns `FfiResult::Done` (all objects processed).
///
/// **Parameters:**
/// - `handle`: A valid `GradualDifficultyHandle` pointer (may be null).
/// - `out`: Pointer to a `DifficultyAttributes` struct where results will be
///   written (may be null).
///
/// **Returns:**
/// - `FfiResult::Ok` — More objects remain; call `next` again.
/// - `FfiResult::Done` — All objects have been processed. No more calls needed.
/// - `FfiResult::NullPointer` — `handle` or `out` is null.
///
/// **Handle reuse:** The `handle` remains valid after `Ok` and can be used for
/// subsequent calls.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `GradualDifficultyHandle`, or null.
/// `out` must point to a valid `DifficultyAttributes` struct, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_gradual_difficulty_next(
    handle: *mut GradualDifficultyHandle,
    out: *mut DifficultyAttributes,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let Some(attrs) = handle.by_mut().next() else {
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
///
/// # Safety
///
/// `handle` must be a null pointer, or a valid handle previously returned by
/// `rosu_pp_gradual_difficulty_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_gradual_difficulty_free(handle: *mut GradualDifficultyHandle) {
    handle.drop_handle();
}
