use rosu_pp::model::beatmap::AdjustedBeatmapAttributes as RosuAdjustedBeatmapAttributes;

use crate::{attributes::beatmap::BeatmapAttributesHandle, error::FfiResult, handle::HandleRef};

/// Adjusted beatmap attributes with clock rate applied to AR and OD.
#[repr(C)]
#[cheadergen::config(export)]
pub struct AdjustedBeatmapAttributes {
    /// Approach rate
    pub ar: f64,
    /// Circle size
    pub cs: f32,
    /// HP drain rate
    pub hp: f32,
    /// Overall difficulty
    pub od: f64,
}

impl From<RosuAdjustedBeatmapAttributes> for AdjustedBeatmapAttributes {
    fn from(attrs: RosuAdjustedBeatmapAttributes) -> Self {
        AdjustedBeatmapAttributes {
            ar: attrs.ar,
            cs: attrs.cs,
            hp: attrs.hp,
            od: attrs.od,
        }
    }
}

/// Apply the clock rate to get adjusted AR and OD values.
///
/// The returned struct has AR and OD adjusted for the clock rate, while CS
/// and HP remain unchanged (they are not affected by clock rate).
///
/// **Parameters:**
/// - `handle`: A valid `BeatmapAttributesHandle` pointer (may be null).
/// - `out`: Pointer to an `AdjustedBeatmapAttributes` struct where results
///   will be written (may be null).
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
/// `handle` or `out` is null.
///
/// # Safety
///
/// `handle` must be a valid pointer to a `BeatmapAttributesHandle`, or null.
/// `out` must point to a valid `AdjustedBeatmapAttributes` struct, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_beatmap_attrs_apply_clock_rate(
    handle: *const BeatmapAttributesHandle,
    out: *mut AdjustedBeatmapAttributes,
) -> FfiResult {
    if handle.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    unsafe { *out = AdjustedBeatmapAttributes::from(handle.by_ref().apply_clock_rate()) };

    FfiResult::Ok
}
