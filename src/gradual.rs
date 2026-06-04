use std::ptr;

use rosu_pp::GradualPerformance as RosuGradualPerformance;

use crate::{
    attributes::PerformanceAttributes, beatmap::BeatmapHandle, difficulty::DifficultyHandle,
    error::FfiResult, score_state::ScoreState,
};

// TODO: gradual difficulty

#[repr(C)]
pub struct GradualPerformanceHandle {
    gradual: RosuGradualPerformance,
}

#[no_mangle]
pub extern "C" fn rosu_pp_gradual_performance_new(
    difficulty: *mut DifficultyHandle,
    map: *const BeatmapHandle,
) -> *mut GradualPerformanceHandle {
    if difficulty.is_null() || map.is_null() {
        return ptr::null_mut();
    }

    let difficulty = unsafe { Box::from_raw(difficulty) };
    let map = unsafe { &(*map).beatmap };
    let gradual = difficulty.difficulty.gradual_performance(map);

    Box::into_raw(Box::new(GradualPerformanceHandle { gradual }))
}

#[no_mangle]
pub extern "C" fn rosu_pp_gradual_performance_next(
    handle: *mut GradualPerformanceHandle,
    state: &ScoreState,
    out: *mut PerformanceAttributes,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &mut *handle };

    let Some(attrs) = h.gradual.next(state.into()) else {
        return FfiResult::Done;
    };

    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
}

#[no_mangle]
pub extern "C" fn rosu_pp_gradual_performance_free(handle: *mut GradualPerformanceHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
