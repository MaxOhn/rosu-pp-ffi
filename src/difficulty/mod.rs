//! Difficulty calculator handle and configuration.
//!
//! Provides a builder-style interface for configuring and running difficulty
//! calculations. Each setter returns `FfiResult::Ok` and the handle remains
//! valid for further configuration.

pub mod inspect;

use std::ptr;

use rosu_pp::Difficulty;

use crate::{
    attributes::difficulty::DifficultyAttributes,
    beatmap::BeatmapHandle,
    error::FfiResult,
    handle::{HandleOwned, HandleRef},
    mods::ModsHandle,
    strains::StrainsData,
};

/// Opaque handle to a difficulty calculator builder.
///
/// Created via `rosu_pp_difficulty_new`. Configure it with setter functions,
/// then calculate with `rosu_pp_difficulty_calculate`.
///
/// **Builder pattern:** Each setter consumes the handle internally and
/// returns `FfiResult::Ok`. The handle pointer remains valid and can be
/// used for subsequent setter calls.
///
/// **Must be freed** with `rosu_pp_difficulty_free` when done.
pub struct DifficultyHandle(Difficulty);

handle!(DifficultyHandle -> Difficulty);

/// Create a new difficulty calculator with default settings.
///
/// **Returns:** A non-null handle to a new `Difficulty` builder.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_difficulty_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_new() -> *mut DifficultyHandle {
    Box::into_raw(Box::new(DifficultyHandle::from(Difficulty::new())))
}

/// Clone a difficulty calculator handle.
///
/// Creates an independent copy of the difficulty builder, including all
/// configured settings (mods, passed_objects, clock_rate, attribute overrides,
/// etc.).
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
    let Some(diff) = handle.checked_by_ref() else {
        return ptr::null_mut();
    };

    Box::into_raw(Box::new(DifficultyHandle::from(diff.to_owned())))
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

    handle.by_owned(|diff| diff.mods(mods.by_ref().to_owned()));

    FfiResult::Ok
}

macro_rules! setter {
    ( $fn:ident ( $arg:ident: $ty:ty $(, $args:ident: $tys:ty ),* ) ) => {
        /// Configuration setter for the difficulty calculator.
        ///
        /// **Parameters:**
        /// - `handle`: A valid `DifficultyHandle` pointer (must not be null).
        /// - `$arg`: The primary parameter value.
        // TODO: improve macro stuff
        // $(, `$args`): Additional parameter values.
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

            handle.by_owned(|diff| diff.$arg( $arg $(, $args )* ));

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
/// **Ownership:** This function **does not** consume the `handle`. The caller
/// must STILL call `rosu_pp_difficulty_free` on the handle.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_calculate(
    handle: *mut DifficultyHandle,
    map: *const BeatmapHandle,
    out: *mut DifficultyAttributes,
) -> FfiResult {
    if handle.is_null() || map.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let attrs = handle.by_ref().calculate(map.by_ref());
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
/// **Ownership:** This function **does not** consume the `handle`. The caller
/// must STILL call `rosu_pp_difficulty_free` on the handle.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_checked_calculate(
    handle: *mut DifficultyHandle,
    map: *const BeatmapHandle,
    out: *mut DifficultyAttributes,
) -> FfiResult {
    if handle.is_null() || map.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let Ok(attrs) = handle.by_ref().checked_calculate(map.by_ref()) else {
        return FfiResult::TooSuspicious;
    };

    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
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
/// **Ownership:** This function **does not** consume the `handle`. The caller
/// must STILL call `rosu_pp_difficulty_free` on the handle.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_strains_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_strains(
    handle: *mut DifficultyHandle,
    map: *const BeatmapHandle,
) -> *mut StrainsData {
    if handle.is_null() || map.is_null() {
        return ptr::null_mut();
    }

    let strains = handle.by_ref().strains(map.by_ref());

    Box::into_raw(Box::new(StrainsData::new(strains)))
}

/// Free a difficulty calculator handle.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_difficulty_new` or
///   `rosu_pp_difficulty_clone`. May be null (null is a no-op).
///
/// **Note:** Do NOT call this function if the handle was passed to
/// `rosu_pp_difficulty_gradual_difficulty` or `rosu_pp_difficulty_inspect`
/// — those functions consume the handle.
#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_free(handle: *mut DifficultyHandle) {
    handle.drop_handle();
}
