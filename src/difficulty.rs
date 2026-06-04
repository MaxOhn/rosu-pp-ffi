use std::{mem, ptr};

use rosu_pp::Difficulty;

use crate::{error::FfiResult, mods::ModsHandle};

#[repr(C)]
pub struct DifficultyHandle {
    pub(crate) difficulty: Difficulty,
}

#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_new() -> *mut DifficultyHandle {
    Box::into_raw(Box::new(DifficultyHandle {
        difficulty: Difficulty::new(),
    }))
}

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

#[no_mangle]
pub extern "C" fn rosu_pp_difficulty_free(handle: *mut DifficultyHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
