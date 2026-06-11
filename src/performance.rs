//! Performance calculator handle and configuration.
//!
//! Provides a builder-style interface for configuring and running performance
//! calculations.

use std::ptr;

use rosu_pp::{any::HitResultPriority, Performance};

use crate::{
    attributes::PerformanceAttributes,
    beatmap::BeatmapHandle,
    error::FfiResult,
    handle::{HandleOwned, HandleRef},
    mods::ModsHandle,
    score_state::ScoreState,
};

/// Opaque handle to a performance calculator builder.
///
/// Created via `rosu_pp_performance_new`. Configure it with setter functions,
/// then calculate with `rosu_pp_performance_calculate`.
///
/// **Builder pattern:** Each setter consumes the handle internally and
/// returns `FfiResult::Ok`. The handle pointer remains valid and can be
/// used for subsequent setter calls.
///
/// **Must be freed** with `rosu_pp_performance_free` when done.
pub struct PerformanceHandle(Performance<'static>);

handle!(PerformanceHandle -> Performance<'static>);

/// Create a new performance calculator for the given beatmap.
///
/// **Parameters:**
/// - `map`: A valid `BeatmapHandle` pointer (must not be null).
///
/// **Returns:** A non-null handle on success, or `NULL` if `map` is null.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_performance_free`. The `map` handle must remain valid for the
/// lifetime of this `PerformanceHandle` (since it borrows the beatmap data).
#[no_mangle]
pub extern "C" fn rosu_pp_performance_new(map: *const BeatmapHandle) -> *mut PerformanceHandle {
    let Some(map) = map.checked_by_ref() else {
        return ptr::null_mut();
    };

    Box::into_raw(Box::new(PerformanceHandle::from(Performance::new(map))))
}

/// Set the game mods for the performance calculation.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer (must not be null).
/// - `mods`: A `ModsHandle` pointer containing the mods to apply.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
#[no_mangle]
pub extern "C" fn rosu_pp_performance_mods(
    handle: *mut PerformanceHandle,
    mods: *const ModsHandle,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_owned(|perf| perf.mods(mods.by_ref().to_owned()));

    FfiResult::Ok
}

macro_rules! setter {
    ( $fn:ident ( $arg:ident: $ty:ty $(, $args:ident: $tys:ty ),* ) ) => {
        /// Configuration setter for the performance calculator.
        ///
        /// **Parameters:**
        /// - `handle`: A valid `PerformanceHandle` pointer (must not be null).
        /// - `$arg`: The primary parameter value.
        /// $(, `$args`): Additional parameter values.
        ///
        /// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer`
        /// if `handle` is null.
        ///
        /// **Handle reuse:** The `handle` remains valid after this call.
        #[no_mangle]
        pub extern "C" fn $fn(
            handle: *mut PerformanceHandle,
            $arg: $ty
            $(, $args: $tys )*
        ) -> FfiResult {
            if handle.is_null() {
                return FfiResult::NullPointer;
            }

            handle.by_owned(|perf| perf.$arg( $arg $(, $args )* ));

            FfiResult::Ok
        }
    }
}

setter!(rosu_pp_performance_passed_objects(passed_objects: u32));
setter!(rosu_pp_performance_clock_rate(clock_rate: f64));
setter!(rosu_pp_performance_ar(ar: f32, fixed: bool));
setter!(rosu_pp_performance_cs(cs: f32, fixed: bool));
setter!(rosu_pp_performance_hp(hp: f32, fixed: bool));
setter!(rosu_pp_performance_od(od: f32, fixed: bool));
setter!(rosu_pp_performance_hardrock_offsets(hardrock_offsets: bool));
setter!(rosu_pp_performance_lazer(lazer: bool));
setter!(rosu_pp_performance_accuracy(accuracy: f64));
setter!(rosu_pp_performance_misses(misses: u32));
setter!(rosu_pp_performance_combo(combo: u32));
setter!(rosu_pp_performance_large_tick_hits(large_tick_hits: u32));
setter!(rosu_pp_performance_small_tick_hits(small_tick_hits: u32));
setter!(rosu_pp_performance_slider_end_hits(slider_end_hits: u32));
setter!(rosu_pp_performance_n300(n300: u32));
setter!(rosu_pp_performance_n100(n100: u32));
setter!(rosu_pp_performance_n50(n50: u32));
setter!(rosu_pp_performance_n_geki(n_geki: u32));
setter!(rosu_pp_performance_n_katu(n_katu: u32));
setter!(rosu_pp_performance_legacy_total_score(legacy_total_score: u32));

/// Set the priority of hitresults when generating remaining hitresults.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer (must not be null).
/// - `priority`: The hitresult priority: `0` for BestCase (prioritize good hitresults),
///   `1` for WorstCase (prioritize bad hitresults).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
#[no_mangle]
pub extern "C" fn rosu_pp_performance_hitresult_priority(
    handle: *mut PerformanceHandle,
    priority: u32,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    let priority = match priority {
        0 => HitResultPriority::BestCase,
        1 => HitResultPriority::WorstCase,
        _ => return FfiResult::InvalidArgument,
    };

    handle.by_owned(|perf| perf.hitresult_priority(priority));

    FfiResult::Ok
}

/// Calculate performance attributes for the configured settings after verifying
/// the map is not too suspicious.
///
/// Same as `rosu_pp_performance_calculate` but checks the map for suspicious
/// hit objects first. If the map is too suspicious, returns `FfiResult::TooSuspicious`.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer. **Consumed** by this call.
///   The handle must NOT be used or freed after this call.
/// - `out`: Pointer to a `PerformanceAttributes` struct where results will be
///   written (must not be null).
///
/// **Returns:**
/// - `FfiResult::Ok` — Calculation succeeded.
/// - `FfiResult::TooSuspicious` — The map contains suspicious hit objects.
/// - `FfiResult::NullPointer` — `handle` or `out` is null.
///
/// **Ownership:** This function **consumes** the `handle`. The caller must NOT
/// call `rosu_pp_performance_free` on the handle, nor use it after this call.
#[no_mangle]
pub extern "C" fn rosu_pp_performance_checked_calculate(
    handle: *mut PerformanceHandle,
    out: *mut PerformanceAttributes,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let Ok(attrs) = handle.into_owned().checked_calculate() else {
        return FfiResult::TooSuspicious;
    };

    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
}

/// Set the full score state at once.
///
/// This is an alternative to setting individual hit counts (n300, n100, etc.)
/// and combo. Use this when you have a complete `ScoreState` struct.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer.
/// - `state`: A reference to a `ScoreState` struct with the score data.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
#[no_mangle]
pub extern "C" fn rosu_pp_performance_state(
    handle: *mut PerformanceHandle,
    state: &ScoreState,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_owned(|perf| perf.state(state.into()));

    FfiResult::Ok
}

/// Calculate performance attributes for the configured settings.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer. **Consumed** by this call.
///   The handle must NOT be used or freed after this call.
/// - `out`: Pointer to a `PerformanceAttributes` struct where results will be
///   written (must not be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null.
///
/// **Ownership:** This function **consumes** the `handle`. The caller must NOT
/// call `rosu_pp_performance_free` on the handle, nor use it after this call.
#[no_mangle]
pub extern "C" fn rosu_pp_performance_calculate(
    handle: *mut PerformanceHandle,
    out: *mut PerformanceAttributes,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let attrs = handle.into_owned().calculate();
    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
}

/// Free a performance calculator handle.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_performance_new`. May be null
///   (null is a no-op).
///
/// **Note:** Do NOT call this function if the handle was passed to
/// `rosu_pp_performance_calculate` — that function consumes the handle.
#[no_mangle]
pub extern "C" fn rosu_pp_performance_free(handle: *mut PerformanceHandle) {
    handle.drop_handle();
}
