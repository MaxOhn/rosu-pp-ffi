//! Gradual (per-object) performance calculator.
//!
//! Computes performance points incrementally as each hit object is processed,
//! enabling real-time score progression tracking.

use std::ptr;

use rosu_pp::GradualPerformance;

use crate::{
    attributes::performance::PerformanceAttributes,
    beatmap::BeatmapHandle,
    difficulty::DifficultyHandle,
    error::FfiResult,
    handle::{HandleMut, HandleOwned, HandleRef},
    score_state::ScoreState,
};

/// Opaque handle to a gradual performance calculator.
///
/// Created via `rosu_pp_gradual_performance_new`. Iterate through hit objects
/// using `rosu_pp_gradual_performance_next` until it returns `FfiResult::Done`.
///
/// **Must be freed** with `rosu_pp_gradual_performance_free` when done.
pub struct GradualPerformanceHandle(GradualPerformance);

handle!(GradualPerformanceHandle -> GradualPerformance);

/// Create a new gradual performance calculator.
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
/// `rosu_pp_gradual_performance_free`.
///
/// # Safety
///
/// `difficulty` must be a valid pointer to a `DifficultyHandle`, or null.
/// `map` must be a valid pointer to a `BeatmapHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_gradual_performance_new(
    difficulty: *mut DifficultyHandle,
    map: *const BeatmapHandle,
) -> *mut GradualPerformanceHandle {
    if difficulty.is_null() || map.is_null() {
        return ptr::null_mut();
    }

    let gradual = difficulty.into_owned().gradual_performance(map.by_ref());

    Box::into_raw(Box::new(GradualPerformanceHandle::from(gradual)))
}

/// Process the next hit object and return incremental performance attributes.
///
/// Call this function repeatedly, passing the score state for each hit object
/// in order, until it returns `FfiResult::Done` (all objects processed).
///
/// **Parameters:**
/// - `handle`: A valid `GradualPerformanceHandle` pointer (may be null).
/// - `state`: A reference to a `ScoreState` struct describing the current hit.
///   (may be null)
/// - `out`: Pointer to a `PerformanceAttributes` struct where results will be
///   written (may be null).
///
/// **Returns:**
/// - `FfiResult::Ok` — More objects remain; call `next` again.
/// - `FfiResult::Done` — All objects have been processed. No more calls needed.
/// - `FfiResult::NullPointer` — `handle`, `state`, or `out` is null.
///
/// **Handle reuse:** The `handle` remains valid after `Ok` and can be used for
/// subsequent calls.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `GradualPerformanceHandle`, or null.
/// `state` must be a valid pointer to a `ScoreState`, or null.
/// `out` must point to a valid `PerformanceAttributes` struct, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_gradual_performance_next(
    handle: *mut GradualPerformanceHandle,
    state: *const ScoreState,
    out: *mut PerformanceAttributes,
) -> FfiResult {
    if handle.is_null() || state.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let state = unsafe { state.as_ref_unchecked() };

    let Some(attrs) = handle.by_mut().next(state.into()) else {
        return FfiResult::Done;
    };

    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
}

/// Free a gradual performance calculator handle.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_gradual_performance_new`. May be
///   null (null is a no-op).
///
/// # Safety
///
/// `handle` must be a null pointer, or a valid handle previously returned by
/// `rosu_pp_gradual_performance_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_gradual_performance_free(handle: *mut GradualPerformanceHandle) {
    handle.drop_handle();
}
