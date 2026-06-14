use std::ptr;

use rosu_pp::{any::InspectDifficulty, model::beatmap::BeatmapAttribute};

use crate::{
    difficulty::DifficultyHandle,
    error::FfiResult,
    handle::{HandleOwned, HandleRef},
    mods::ModsHandle,
};

/// Opaque handle to an inspected difficulty calculator.
///
/// Created via `rosu_pp_difficulty_inspect`. Use getter functions to inspect
/// the configured values.
pub struct InspectDifficultyHandle(InspectDifficulty);

handle!(InspectDifficultyHandle -> InspectDifficulty);

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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_difficulty_inspect_new(
    handle: *mut DifficultyHandle,
) -> *mut InspectDifficultyHandle {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let inspect = handle.into_owned().inspect();

    Box::into_raw(Box::new(InspectDifficultyHandle::from(inspect)))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_inspect_difficulty_mods(
    handle: *const InspectDifficultyHandle,
) -> *mut ModsHandle {
    let Some(inspect) = handle.checked_by_ref() else {
        return ptr::null_mut();
    };

    Box::into_raw(Box::new(ModsHandle::from(inspect.mods.clone())))
}

macro_rules! getter {
    ( $fn:ident -> ar: $ty:ty ) => { getter!( @attrs $fn -> ar: $ty); };
    ( $fn:ident -> cs: $ty:ty ) => { getter!( @attrs $fn -> cs: $ty); };
    ( $fn:ident -> hp: $ty:ty ) => { getter!( @attrs $fn -> hp: $ty); };
    ( $fn:ident -> od: $ty:ty ) => { getter!( @attrs $fn -> od: $ty); };

    ( $fn:ident -> $field:ident: $ty:ty ) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $fn(handle: *const InspectDifficultyHandle, out: *mut $ty) -> FfiResult {
            if handle.is_null() || out.is_null() {
                return FfiResult::NullPointer;
            }

            let Some(value) = handle.by_ref().$field else {
                return FfiResult::None;
            };

            unsafe { *out = value };

            FfiResult::Ok
        }
    };

    ( @attrs $fn:ident -> $field:ident: $ty:ty ) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $fn(
            handle: *const InspectDifficultyHandle,
            out: *mut $ty,
            fixed: *mut bool
        ) -> FfiResult {
            if handle.is_null() || out.is_null() || fixed.is_null() {
                return FfiResult::NullPointer;
            }

            match handle.by_ref().$field {
                BeatmapAttribute::None => return FfiResult::None,
                BeatmapAttribute::Value(v) | BeatmapAttribute::Given(v) => unsafe {
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
    };
}

getter!(rosu_pp_inspect_difficulty_passed_objects -> passed_objects: u32);
getter!(rosu_pp_inspect_difficulty_clock_rate -> clock_rate: f64);
getter!(rosu_pp_inspect_difficulty_hardrock_offsets -> hardrock_offsets: bool);
getter!(rosu_pp_inspect_difficulty_lazer -> lazer: bool);
getter!(rosu_pp_inspect_difficulty_ar -> ar: f32);
getter!(rosu_pp_inspect_difficulty_cs -> cs: f32);
getter!(rosu_pp_inspect_difficulty_hp -> hp: f32);
getter!(rosu_pp_inspect_difficulty_od -> od: f32);

/// Free an inspected difficulty handle.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_difficulty_inspect`. May be null
///   (null is a no-op).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_inspect_difficulty_free(handle: *mut InspectDifficultyHandle) {
    handle.drop_handle();
}
