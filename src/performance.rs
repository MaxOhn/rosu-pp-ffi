//! Performance calculator handle and configuration.
//!
//! Provides a builder-style interface for configuring and running performance
//! calculations.

use std::ptr;

use rosu_pp::{Performance, any::HitResultPriority};

use crate::{
    attributes::performance::PerformanceAttributes,
    beatmap::BeatmapHandle,
    error::FfiResult,
    handle::{HandleOwned, HandleRef},
    mods::ModsHandle,
    score_state::ScoreState,
};

handle! {
    /// Opaque handle to a performance calculator builder.
    ///
    /// Created via `rosu_pp_performance_new`. Configure it with setter functions,
    /// then calculate with `rosu_pp_performance_calculate`.
    ///
    /// **Builder pattern:** Each setter consumes the handle internally and
    /// returns `FfiResult::Ok`. The handle pointer remains valid and can be
    /// used for subsequent setter calls.
    ///
    /// **Lifetime requirement:** The `BeatmapHandle` passed to
    /// `rosu_pp_performance_new` **must remain valid for the entire lifetime**
    /// of this `PerformanceHandle`. Do NOT free the beatmap handle until after
    /// you have called `rosu_pp_performance_free`. Using this handle after the
    /// beatmap has been freed results in undefined behavior.
    ///
    /// **Must be freed** with `rosu_pp_performance_free` when done.
    #[cheadergen::config(rename = "rosu_pp_PerformanceHandle")]
    PerformanceHandle -> Performance<'static>
}

/// Create a new performance calculator for the given beatmap.
///
/// **Parameters:**
/// - `map`: A valid `BeatmapHandle` pointer (may be null).
///
/// **Returns:** A non-null handle on success, or `NULL` if `map` is null.
///
/// **Lifetime requirement:** The `map` handle **must remain valid** for the
/// entire lifetime of the returned `PerformanceHandle`. Do NOT call
/// `rosu_pp_beatmap_free` on the map handle until after you have called
/// `rosu_pp_performance_free`.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_performance_free`.
///
/// # Safety
///
/// `map` must be a valid pointer to a `BeatmapHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_performance_new(
    map: *const BeatmapHandle,
) -> *mut PerformanceHandle {
    let Some(map) = map.checked_by_ref() else {
        return ptr::null_mut();
    };

    Box::into_raw(Box::new(PerformanceHandle::from(Performance::new(map))))
}

/// Set the game mods for the performance calculation.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer (may be null).
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
/// `handle` must be a valid pointer to a `PerformanceHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_performance_mods(
    handle: *mut PerformanceHandle,
    mods: *const ModsHandle,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    handle.by_owned(|perf| {
        if let Some(mods) = mods.checked_by_ref() {
            perf.mods(mods.to_owned())
        } else {
            perf.mods(0)
        }
    });

    FfiResult::Ok
}

macro_rules! setter {
    (
        $( #[ $meta:meta ] )*
        $fn:ident ( $arg:ident: $ty:ty $(, $arg2:ident: $ty2:ty )? )
    ) => {
        $( #[ $meta ] )*
        ///   - `handle`: A valid `PerformanceHandle` pointer (may be null).
        ///
        /// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer`
        /// if `handle` is null.
        ///
        /// **Handle reuse:** The `handle` remains valid after this call.
        ///
        /// # Safety
        ///
        /// `handle` must be a valid pointer to a `PerformanceHandle`, or null.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $fn(
            handle: *mut PerformanceHandle,
            $arg: $ty
            $(, $arg2: $ty2 )?
        ) -> FfiResult {
            if handle.is_null() {
                return FfiResult::NullPointer;
            }

            handle.by_owned(|perf| perf.$arg( $arg $(, $arg2 )? ));

            FfiResult::Ok
        }
    }
}

setter! {
    /// Amount of passed objects for partial plays, e.g. a fail.
    ///
    /// **Parameters:**
    ///   - `passed_objects`: The number of hit objects to consider.
    rosu_pp_performance_passed_objects(passed_objects: u32)
}

setter! {
    /// Adjust the clock rate used in the calculation.
    ///
    /// If none is specified, it will take the clock rate based on the mods
    /// i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.
    ///
    /// **Parameters:**
    ///   - `clock_rate`: The clock rate multiplier (must be positive).
    rosu_pp_performance_clock_rate(clock_rate: f64)
}

setter! {
    /// Override the approach rate (AR).
    ///
    /// Sets a fixed AR value, bypassing the normal AR calculation from the
    /// beatmap. If `fixed` is `true`, the value is used as-is. If `fixed` is
    /// `false`, the value may be adjusted by mods and clock rate.
    ///
    /// **Parameters:**
    ///   - `ar`: The approach rate value.
    ///   - `fixed`: If `true`, the value is used as-is. If `false`, it may be
    ///     adjusted by mods and clock rate.
    rosu_pp_performance_ar(ar: f32, fixed: bool)
}

setter! {
    /// Override the circle size (CS).
    ///
    /// Sets a fixed CS value, bypassing the normal CS calculation from the
    /// beatmap. If `fixed` is `true`, the value is used as-is. If `fixed` is
    /// `false`, the value may be adjusted by mods.
    ///
    /// **Parameters:**
    ///   - `cs`: The circle size value.
    ///   - `fixed`: If `true`, the value is used as-is. If `false`, it may be
    ///     adjusted by mods.
    rosu_pp_performance_cs(cs: f32, fixed: bool)
}

setter! {
    /// Override the HP drain rate.
    ///
    /// Sets a fixed HP value, bypassing the normal HP calculation from the
    /// beatmap. If `fixed` is `true`, the value is used as-is. If `fixed` is
    /// `false`, the value may be adjusted by mods.
    ///
    /// **Parameters:**
    ///   - `hp`: The HP drain rate value.
    ///   - `fixed`: If `true`, the value is used as-is. If `false`, it may be
    ///     adjusted by mods.
    rosu_pp_performance_hp(hp: f32, fixed: bool)
}

setter! {
    /// Override the overall difficulty (OD).
    ///
    /// Sets a fixed OD value, bypassing the normal OD calculation from the
    /// beatmap. If `fixed` is `true`, the value is used as-is. If `fixed` is
    /// `false`, the value may be adjusted by mods and clock rate.
    ///
    /// **Parameters:**
    ///   - `od`: The overall difficulty value.
    ///   - `fixed`: If `true`, the value is used as-is. If `false`, it may be
    ///     adjusted by mods and clock rate.
    rosu_pp_performance_od(od: f32, fixed: bool)
}

setter! {
    /// Adjust patterns as if the HR mod is enabled.
    ///
    /// Only relevant for osu!catch.
    ///
    /// **Parameters:**
    ///   - `hardrock_offsets`: Whether to apply hardrock-specific offsets.
    rosu_pp_performance_hardrock_offsets(hardrock_offsets: bool)
}

setter! {
    /// Whether the calculated attributes belong to an osu!lazer or osu!stable
    /// score.
    ///
    /// **Parameters:**
    ///   - `lazer`: Whether to use lazer mode calculation.
    rosu_pp_performance_lazer(lazer: bool)
}

setter! {
    /// Set the accuracy between `0.0` and `100.0`.
    ///
    /// **Parameters:**
    ///   - `accuracy`: The accuracy value (0.0–100.0).
    rosu_pp_performance_accuracy(accuracy: f64)
}

setter! {
    /// Set the number of misses.
    ///
    /// **Parameters:**
    ///   - `misses`: The number of misses in the score.
    rosu_pp_performance_misses(misses: u32)
}

setter! {
    /// Set the maximum combo achieved.
    ///
    /// **Parameters:**
    ///   - `combo`: The maximum combo achieved in the score.
    rosu_pp_performance_combo(combo: u32)
}

setter! {
    /// Specify the amount of "large tick" hits.
    ///
    /// Only relevant for osu!standard.
    ///
    /// The meaning depends on the kind of score:
    /// - if set on osu!stable, this value is irrelevant and can be `0`
    /// - if set on osu!lazer *with* slider accuracy, this value is the amount
    ///   of hit slider ticks and repeats
    /// - if set on osu!lazer *without* slider accuracy, this value is the
    ///   amount of hit slider heads, ticks, and repeats
    ///
    /// **Parameters:**
    ///   - `large_tick_hits`: The number of large tick hits.
    rosu_pp_performance_large_tick_hits(large_tick_hits: u32)
}

setter! {
    /// Specify the amount of "small tick" hits.
    ///
    /// Only relevant for osu!standard lazer scores without slider accuracy. In
    /// that case, this value is the amount of slider tail hits.
    ///
    /// **Parameters:**
    ///   - `small_tick_hits`: The number of small tick hits.
    rosu_pp_performance_small_tick_hits(small_tick_hits: u32)
}

setter! {
    /// Specify the amount of hit slider ends.
    ///
    /// Only relevant for osu!standard lazer scores with slider accuracy.
    ///
    /// **Parameters:**
    ///   - `slider_end_hits`: The number of slider end hits.
    rosu_pp_performance_slider_end_hits(slider_end_hits: u32)
}

setter! {
    /// Specify the amount of 300s of a play.
    ///
    /// **Parameters:**
    ///   - `n300`: The number of 300-score hit results.
    rosu_pp_performance_n300(n300: u32)
}

setter! {
    /// Specify the amount of 100s of a play.
    ///
    /// **Parameters:**
    ///   - `n100`: The number of 100-score hit results.
    rosu_pp_performance_n100(n100: u32)
}

setter! {
    /// Specify the amount of 50s of a play.
    ///
    /// **Parameters:**
    ///   - `n50`: The number of 50-score hit results.
    rosu_pp_performance_n50(n50: u32)
}

setter! {
    /// Specify the amount of gekis of a play.
    ///
    /// Only relevant for osu!mania for which it repesents the
    /// amount of n320.
    ///
    /// **Parameters:**
    ///   - `n_geki`: The number of geki hits.
    rosu_pp_performance_n_geki(n_geki: u32)
}

setter! {
    /// Specify the amount of katus of a play.
    ///
    /// Only relevant for osu!catch for which it represents the amount of tiny
    /// droplet misses and osu!mania for which it repesents the amount of n200.
    ///
    /// **Parameters:**
    ///   - `n_katu`: The number of katu hits.
    rosu_pp_performance_n_katu(n_katu: u32)
}

setter! {
    /// Specify the legacy total score.
    ///
    /// Only relevant for osu!standard.
    ///
    /// **Parameters:**
    ///   - `legacy_total_score`: The legacy total score value.
    rosu_pp_performance_legacy_total_score(legacy_total_score: u32)
}

/// Set the priority of hitresults when generating remaining hitresults.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer (may be null).
/// - `priority`: The hitresult priority: `0` for BestCase (prioritize good hitresults),
///   `1` for WorstCase (prioritize bad hitresults).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` is null.
///
/// **Handle reuse:** The `handle` remains valid after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `PerformanceHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_performance_hitresult_priority(
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
///   written (may be null).
///
/// **Returns:**
/// - `FfiResult::Ok` — Calculation succeeded.
/// - `FfiResult::TooSuspicious` — The map contains suspicious hit objects.
/// - `FfiResult::NullPointer` — `handle` or `out` is null.
///
/// **Ownership:** This function **consumes** the `handle`. The caller must NOT
/// call `rosu_pp_performance_free` on the handle, nor use it after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `PerformanceHandle`, or null.
/// `out` must point to a valid `PerformanceAttributes` struct, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_performance_checked_calculate(
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
/// - `handle`: A valid `PerformanceHandle` pointer (may be null).
/// - `state`: A reference to a `ScoreState` struct with the score data. (may be
///   null)
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `state` is null.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `PerformanceHandle`, or null.
/// `state` must be a valid pointer to a `ScoreState`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_performance_state(
    handle: *mut PerformanceHandle,
    state: *const ScoreState,
) -> FfiResult {
    if handle.is_null() || state.is_null() {
        return FfiResult::NullPointer;
    }

    let state = unsafe { state.as_ref_unchecked() };
    handle.by_owned(|perf| perf.state(state.into()));

    FfiResult::Ok
}

/// Calculate performance attributes for the configured settings.
///
/// **Parameters:**
/// - `handle`: A valid `PerformanceHandle` pointer. **Consumed** by this call.
///   The handle must NOT be used or freed after this call.
/// - `out`: Pointer to a `PerformanceAttributes` struct where results will be
///   written (may be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null.
///
/// **Ownership:** This function **consumes** the `handle`. The caller must NOT
/// call `rosu_pp_performance_free` on the handle, nor use it after this call.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `PerformanceHandle`, or null.
/// `out` must point to a valid `PerformanceAttributes` struct, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_performance_calculate(
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
///
/// # Safety
///
/// `handle` must be a null pointer, or a valid handle previously returned by
/// `rosu_pp_performance_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_performance_free(handle: *mut PerformanceHandle) {
    handle.drop_handle();
}
