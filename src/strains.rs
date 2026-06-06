//! Strain calculation results for plotting difficulty over time.
//!
//! Provides the `StrainsHandle` type and `StrainsData` struct for accessing
//! strain peak data for all four osu! game modes.

use std::ptr;

use rosu_pp::{
    any::Strains as RosuStrains, catch::CatchStrains, mania::ManiaStrains, osu::OsuStrains,
    taiko::TaikoStrains,
};

use crate::error::FfiResult;

/// The result of calculating the strains on a map.
///
/// Suitable to plot the difficulty of a map over time. The `mode` field
/// indicates which game mode the strains belong to, and the corresponding
/// strain arrays will be populated.
///
/// **osu! (mode=0):** `aim`, `aim_no_sliders`, `speed`, `flashlight`
/// **taiko (mode=1):** `color`, `reading`, `rhythm`, `stamina`, `single_color_stamina`
/// **catch (mode=2):** `movement`
/// **mania (mode=3):** `strains`
#[repr(C)]
pub struct StrainsData {
    /// Game mode: 0=osu!, 1=taiko, 2=catch, 3=mania
    pub mode: i32,
    /// Time between two strain values in milliseconds (valid for all modes)
    pub section_len: f64,
    /// Number of strain values in each array
    pub len: usize,
    /// Aim strain peaks (osu! only)
    pub aim: *const f64,
    /// Aim strain peaks without sliders (osu! only)
    pub aim_no_sliders: *const f64,
    /// Speed strain peaks (osu! only)
    pub speed: *const f64,
    /// Flashlight strain peaks (osu! only)
    pub flashlight: *const f64,
    /// Stamina strain peaks (taiko only)
    pub stamina: *const f64,
    /// Rhythm strain peaks (taiko only)
    pub rhythm: *const f64,
    /// Color strain peaks (taiko only)
    pub color: *const f64,
    /// Reading strain peaks (taiko only)
    pub reading: *const f64,
    /// Single color stamina strain peaks (taiko only)
    pub single_color_stamina: *const f64,
    /// Movement strain peaks (catch only)
    pub movement: *const f64,
    /// Strain peaks (mania only)
    pub strains: *const f64,
}

/// Opaque handle to strain data.
///
/// Created via `rosu_pp_difficulty_strains` (in difficulty.rs). The handle owns
/// the strain arrays and must be freed with `rosu_pp_strains_free`.
pub struct StrainsHandle {
    _strains: RosuStrains,
    /// Boxed slices for owned data (kept alive by the handle)
    data: StrainsDataHolder,
}

enum StrainsDataHolder {
    Osu {
        aim: Box<[f64]>,
        aim_no_sliders: Box<[f64]>,
        speed: Box<[f64]>,
        flashlight: Box<[f64]>,
    },
    Taiko {
        color: Box<[f64]>,
        reading: Box<[f64]>,
        rhythm: Box<[f64]>,
        stamina: Box<[f64]>,
        single_color_stamina: Box<[f64]>,
    },
    Catch {
        movement: Box<[f64]>,
    },
    Mania {
        strains: Box<[f64]>,
    },
}

/// Get strain data from a strains handle.
///
/// **Parameters:**
/// - `handle`: A valid `StrainsHandle` pointer (must not be null).
/// - `out`: Pointer to a `StrainsData` struct where results will be written.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null.
#[no_mangle]
pub extern "C" fn rosu_pp_strains_data(
    handle: *const StrainsHandle,
    out: *mut StrainsData,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let h = unsafe { &*handle };
    unsafe { *out = data_to_strains_data(&h.data) };

    FfiResult::Ok
}

/// Free a strains handle and release its memory.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_difficulty_strains`. May be null
///   (null is a no-op).
#[no_mangle]
pub extern "C" fn rosu_pp_strains_free(handle: *mut StrainsHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}

/// Build a StrainsHandle from a RosuStrains value.
///
/// Internal function called by difficulty.rs.
pub fn build_strains_handle(strains: RosuStrains) -> Box<StrainsHandle> {
    Box::new(StrainsHandle {
        _strains: strains.clone(),
        data: build_data_holder(&strains),
    })
}

fn build_data_holder(strains: &RosuStrains) -> StrainsDataHolder {
    match strains {
        RosuStrains::Osu(OsuStrains {
            aim,
            aim_no_sliders,
            speed,
            flashlight,
        }) => StrainsDataHolder::Osu {
            aim: aim.clone().into_boxed_slice(),
            aim_no_sliders: aim_no_sliders.clone().into_boxed_slice(),
            speed: speed.clone().into_boxed_slice(),
            flashlight: flashlight.clone().into_boxed_slice(),
        },
        RosuStrains::Taiko(TaikoStrains {
            color,
            reading,
            rhythm,
            stamina,
            single_color_stamina,
        }) => StrainsDataHolder::Taiko {
            color: color.clone().into_boxed_slice(),
            reading: reading.clone().into_boxed_slice(),
            rhythm: rhythm.clone().into_boxed_slice(),
            stamina: stamina.clone().into_boxed_slice(),
            single_color_stamina: single_color_stamina.clone().into_boxed_slice(),
        },
        RosuStrains::Catch(CatchStrains { movement }) => StrainsDataHolder::Catch {
            movement: movement.clone().into_boxed_slice(),
        },
        RosuStrains::Mania(ManiaStrains { strains: strains_1 }) => StrainsDataHolder::Mania {
            strains: strains_1.clone().into_boxed_slice(),
        },
    }
}

fn data_to_strains_data(data: &StrainsDataHolder) -> StrainsData {
    match data {
        StrainsDataHolder::Osu {
            aim,
            aim_no_sliders,
            speed,
            flashlight,
        } => {
            let len = aim.len();

            StrainsData {
                mode: 0,
                section_len: OsuStrains::SECTION_LEN,
                len,
                aim: aim.as_ptr(),
                aim_no_sliders: aim_no_sliders.as_ptr(),
                speed: speed.as_ptr(),
                flashlight: flashlight.as_ptr(),
                stamina: ptr::null(),
                rhythm: ptr::null(),
                color: ptr::null(),
                reading: ptr::null(),
                single_color_stamina: ptr::null(),
                movement: ptr::null(),
                strains: ptr::null(),
            }
        }
        StrainsDataHolder::Taiko {
            color,
            reading,
            rhythm,
            stamina,
            single_color_stamina,
        } => {
            let len = color.len();

            StrainsData {
                mode: 1,
                section_len: TaikoStrains::SECTION_LEN,
                len,
                aim: ptr::null(),
                aim_no_sliders: ptr::null(),
                speed: ptr::null(),
                flashlight: ptr::null(),
                stamina: stamina.as_ptr(),
                rhythm: rhythm.as_ptr(),
                color: color.as_ptr(),
                reading: reading.as_ptr(),
                single_color_stamina: single_color_stamina.as_ptr(),
                movement: ptr::null(),
                strains: ptr::null(),
            }
        }
        StrainsDataHolder::Catch { movement } => {
            let len = movement.len();

            StrainsData {
                mode: 2,
                section_len: CatchStrains::SECTION_LEN,
                len,
                aim: ptr::null(),
                aim_no_sliders: ptr::null(),
                speed: ptr::null(),
                flashlight: ptr::null(),
                stamina: ptr::null(),
                rhythm: ptr::null(),
                color: ptr::null(),
                reading: ptr::null(),
                single_color_stamina: ptr::null(),
                movement: movement.as_ptr(),
                strains: ptr::null(),
            }
        }
        StrainsDataHolder::Mania { strains } => {
            let len = strains.len();

            StrainsData {
                mode: 3,
                section_len: ManiaStrains::SECTION_LEN,
                len,
                aim: ptr::null(),
                aim_no_sliders: ptr::null(),
                speed: ptr::null(),
                flashlight: ptr::null(),
                stamina: ptr::null(),
                rhythm: ptr::null(),
                color: ptr::null(),
                reading: ptr::null(),
                single_color_stamina: ptr::null(),
                movement: ptr::null(),
                strains: strains.as_ptr(),
            }
        }
    }
}
