use std::{ffi, ptr, slice};

use rosu_pp::{model::mode::GameMode, Beatmap};

#[repr(C)]
pub struct BeatmapHandle {
    pub(crate) beatmap: Beatmap,
}

#[no_mangle]
pub extern "C" fn rosu_pp_beatmap_from_path(path: *const ffi::c_char) -> *mut BeatmapHandle {
    if path.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe { ffi::CStr::from_ptr(path) };

    let Ok(path) = c_str.to_str() else {
        return ptr::null_mut();
    };

    let beatmap = match Beatmap::from_path(path) {
        Ok(b) => b,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(BeatmapHandle { beatmap }))
}

#[no_mangle]
pub extern "C" fn rosu_pp_beatmap_from_bytes(bytes: *const u8, len: usize) -> *mut BeatmapHandle {
    if bytes.is_null() {
        return ptr::null_mut();
    }

    let slice = unsafe { slice::from_raw_parts(bytes, len) };

    let beatmap = match Beatmap::from_bytes(slice) {
        Ok(b) => b,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(BeatmapHandle { beatmap }))
}

macro_rules! getter {
    ( $fn:ident( $field:ident ) -> $ty:ty ) => {
        #[no_mangle]
        pub extern "C" fn $fn(handle: *const BeatmapHandle) -> $ty {
            if handle.is_null() {
                return <$ty>::default();
            }

            unsafe { (*handle).beatmap.$field }
        }
    };
    ( $fn:ident( $expr:expr ) -> $ty:ty ) => {
        #[no_mangle]
        pub extern "C" fn $fn(handle: *const BeatmapHandle) -> $ty {
            if handle.is_null() {
                return <$ty>::default();
            }

            unsafe { ($expr)(&(*handle).beatmap) }
        }
    };
}

getter!(rosu_pp_beatmap_version(version) -> i32);
getter!(rosu_pp_beatmap_mode(|map: &Beatmap| GameMode::from(map.mode) as i32) -> i32);
getter!(rosu_pp_beatmap_ar(ar) -> f32);
getter!(rosu_pp_beatmap_cs(cs) -> f32);
getter!(rosu_pp_beatmap_hp(hp) -> f32);
getter!(rosu_pp_beatmap_od(od) -> f32);
getter!(rosu_pp_beatmap_slider_multiplier(slider_multiplier) -> f64);
getter!(rosu_pp_beatmap_slider_tick_rate(slider_tick_rate) -> f64);
getter!(rosu_pp_beatmap_stack_leniency(stack_leniency) -> f32);
getter!(rosu_pp_beatmap_is_convert(is_convert) -> bool);
getter!(rosu_pp_beatmap_hit_object_count(|map: &Beatmap| map.hit_objects.len()) -> usize);
getter!(rosu_pp_beatmap_total_break_time(|map: &Beatmap| map.total_break_time()) -> f64);
getter!(rosu_pp_beatmap_bpm(|map: &Beatmap| map.bpm()) -> f64);

#[no_mangle]
pub extern "C" fn rosu_pp_beatmap_free(handle: *mut BeatmapHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
