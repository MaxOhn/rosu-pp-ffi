use std::{mem, ptr};

use rosu_pp::Performance;

use crate::{
    attributes::PerformanceAttributes, beatmap::BeatmapHandle, error::FfiResult, mods::ModsHandle,
    score_state::ScoreState,
};

#[repr(C)]
pub struct PerformanceHandle<'map> {
    performance: Performance<'map>,
}

#[no_mangle]
pub extern "C" fn rosu_pp_performance_new(
    map: *const BeatmapHandle,
) -> *mut PerformanceHandle<'static> {
    if map.is_null() {
        return ptr::null_mut();
    }

    let map = unsafe { &(*map).beatmap };

    Box::into_raw(Box::new(PerformanceHandle {
        performance: Performance::new(map),
    }))
}

#[no_mangle]
pub extern "C" fn rosu_pp_performance_mods(
    handle: *mut PerformanceHandle<'static>,
    mods: *const ModsHandle,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    let mut h = unsafe { Box::from_raw(handle) };
    let mods = unsafe { &*mods };
    h.performance = h.performance.mods(mods.mods.clone());
    mem::forget(h);

    FfiResult::Ok
}

macro_rules! setter {
    ( $fn:ident ( $arg:ident: $ty:ty $(, $args:ident: $tys:ty ),* ) ) => {
        #[no_mangle]
        pub extern "C" fn $fn(
            handle: *mut PerformanceHandle,
            $arg: $ty
            $(, $args: $tys )*
        ) -> FfiResult {
            if handle.is_null() {
                return FfiResult::NullPointer;
            }

            let mut h = unsafe { Box::from_raw(handle) };
            h.performance = h.performance.$arg( $arg $(, $args )* );
            mem::forget(h);

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

#[no_mangle]
pub extern "C" fn rosu_pp_performance_state(
    handle: *mut PerformanceHandle<'static>,
    state: &ScoreState,
) -> FfiResult {
    if handle.is_null() {
        return FfiResult::NullPointer;
    }

    let mut h = unsafe { Box::from_raw(handle) };
    h.performance = h.performance.state(state.into());
    mem::forget(h);

    FfiResult::Ok
}

#[no_mangle]
pub extern "C" fn rosu_pp_performance_calculate(
    handle: *mut PerformanceHandle<'static>,
    out: *mut PerformanceAttributes,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { Box::from_raw(handle) };
    let attrs = h.performance.calculate();
    unsafe { *out = (&attrs).into() };

    FfiResult::Ok
}

#[no_mangle]
pub extern "C" fn rosu_pp_performance_free(handle: *mut PerformanceHandle<'static>) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
