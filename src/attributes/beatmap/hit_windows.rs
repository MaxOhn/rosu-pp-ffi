use rosu_pp::model::beatmap::HitWindows as RosuHitWindows;

/// AR and OD hit windows for a beatmap.
///
/// Fields populated depend on the game mode (otherwise `NaN`):
/// - **osu! (0):** `ar`, `od_great`, `od_ok`, `od_meh`
/// - **taiko (1):** `od_great`, `od_ok`
/// - **catch (2):** `ar`
/// - **mania (3):** `od_perfect`, `od_great`, `od_good`, `od_ok`, `od_meh`
#[repr(C)]
#[cheadergen::config(export)]
pub struct HitWindows {
    /// Hit window for approach rate (AR) in milliseconds.
    /// Only available for osu! and catch.
    pub ar: f64,
    /// Perfect hit window (mania only).
    pub od_perfect: f64,
    /// Great hit window for OD (osu!, taiko, mania).
    pub od_great: f64,
    /// Good hit window (mania only).
    pub od_good: f64,
    /// Ok hit window for OD (osu!, taiko, mania).
    pub od_ok: f64,
    /// Meh hit window (osu!, mania only).
    pub od_meh: f64,
}

impl From<&RosuHitWindows> for HitWindows {
    fn from(hit_windows: &RosuHitWindows) -> Self {
        HitWindows {
            ar: hit_windows.ar.unwrap_or(f64::NAN),
            od_perfect: hit_windows.od_perfect.unwrap_or(f64::NAN),
            od_great: hit_windows.od_great.unwrap_or(f64::NAN),
            od_good: hit_windows.od_good.unwrap_or(f64::NAN),
            od_ok: hit_windows.od_ok.unwrap_or(f64::NAN),
            od_meh: hit_windows.od_meh.unwrap_or(f64::NAN),
        }
    }
}
