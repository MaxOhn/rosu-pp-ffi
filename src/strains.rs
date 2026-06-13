//! Strain calculation results for plotting difficulty over time.
//!
//! Provides the `StrainsHandle` type and `StrainsData` struct for accessing
//! strain peak data for all four osu! game modes.

use std::{mem::ManuallyDrop, ptr};

use rosu_pp::{
    any::Strains, catch::CatchStrains, mania::ManiaStrains, osu::OsuStrains, taiko::TaikoStrains,
};

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

impl StrainsData {
    pub fn new(strains: Strains) -> Self {
        match strains {
            Strains::Osu(OsuStrains {
                aim,
                aim_no_sliders,
                speed,
                flashlight,
            }) => {
                let len = aim.len();

                StrainsData {
                    mode: 0,
                    section_len: OsuStrains::SECTION_LEN,
                    len,
                    aim: ManuallyDrop::new(aim.into_boxed_slice()).as_ptr(),
                    aim_no_sliders: ManuallyDrop::new(aim_no_sliders.into_boxed_slice()).as_ptr(),
                    speed: ManuallyDrop::new(speed.into_boxed_slice()).as_ptr(),
                    flashlight: ManuallyDrop::new(flashlight.into_boxed_slice()).as_ptr(),
                    stamina: ptr::null(),
                    rhythm: ptr::null(),
                    color: ptr::null(),
                    reading: ptr::null(),
                    single_color_stamina: ptr::null(),
                    movement: ptr::null(),
                    strains: ptr::null(),
                }
            }
            Strains::Taiko(TaikoStrains {
                color,
                reading,
                rhythm,
                stamina,
                single_color_stamina,
            }) => {
                let len = color.len();

                StrainsData {
                    mode: 1,
                    section_len: TaikoStrains::SECTION_LEN,
                    len,
                    aim: ptr::null(),
                    aim_no_sliders: ptr::null(),
                    speed: ptr::null(),
                    flashlight: ptr::null(),
                    stamina: ManuallyDrop::new(stamina.into_boxed_slice()).as_ptr(),
                    rhythm: ManuallyDrop::new(rhythm.into_boxed_slice()).as_ptr(),
                    color: ManuallyDrop::new(color.into_boxed_slice()).as_ptr(),
                    reading: ManuallyDrop::new(reading.into_boxed_slice()).as_ptr(),
                    single_color_stamina: ManuallyDrop::new(
                        single_color_stamina.into_boxed_slice(),
                    )
                    .as_ptr(),
                    movement: ptr::null(),
                    strains: ptr::null(),
                }
            }
            Strains::Catch(CatchStrains { movement }) => {
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
                    movement: ManuallyDrop::new(movement.into_boxed_slice()).as_ptr(),
                    strains: ptr::null(),
                }
            }
            Strains::Mania(ManiaStrains { strains }) => {
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
                    strains: ManuallyDrop::new(strains.into_boxed_slice()).as_ptr(),
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_strains_free(handle: *mut StrainsData) {
    if handle.is_null() {
        return;
    }

    let StrainsData {
        mode: _,
        section_len: _,
        len,
        aim,
        aim_no_sliders,
        speed,
        flashlight,
        stamina,
        rhythm,
        color,
        reading,
        single_color_stamina,
        movement,
        strains,
    } = unsafe { &mut *handle };

    macro_rules! drop {
        ($ptr:ident) => {
            if !(*$ptr).is_null() {
                let slice = ptr::slice_from_raw_parts_mut((*$ptr).cast_mut(), *len);
                drop(unsafe { Box::from_raw(slice) });
            }
        };
    }

    drop!(aim);
    drop!(aim_no_sliders);
    drop!(speed);
    drop!(flashlight);
    drop!(stamina);
    drop!(rhythm);
    drop!(color);
    drop!(reading);
    drop!(single_color_stamina);
    drop!(movement);
    drop!(strains);
}
