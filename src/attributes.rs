//! Unified attribute structs for difficulty and performance results.
//!
//! These `#[repr(C)]` structs provide a single cross-mode representation of
//! difficulty and performance attributes for all four osu! game modes
//! (osu!, taiko, catch, mania). Fields not applicable to a given mode are
//! zeroed. Use the `mode` field to determine which attributes are valid.

use rosu_pp::{
    any::{
        DifficultyAttributes as RosuDifficultyAttributes,
        PerformanceAttributes as RosuPerformanceAttributes,
    },
    catch::{CatchDifficultyAttributes, CatchPerformanceAttributes},
    mania::{ManiaDifficultyAttributes, ManiaPerformanceAttributes},
    osu::{OsuDifficultyAttributes, OsuPerformanceAttributes},
    taiko::{TaikoDifficultyAttributes, TaikoPerformanceAttributes},
};

/// Unified difficulty attributes for all osu! game modes.
///
/// After a difficulty calculation, inspect the `mode` field to determine which
/// attributes are valid:
///
/// - **`0` (osu!):** `aim`, `speed`, `flashlight`, `ar`, `od`, `hp`,
///   `great_hit_window`, `ok_hit_window`, `meh_hit_window`, `n_circles`,
///   `n_sliders`, `n_large_ticks`, `n_spinners`, `aim_difficult_slider_count`,
///   `slider_factor`, `aim_top_weighted_slider_factor`, `speed_top_weighted_slider_factor`,
///   `speed_note_count`, `aim_difficult_strain_count`, `speed_difficult_strain_count`,
///   `nested_score_per_object`, `legacy_score_base_multiplier`, `maximum_legacy_combo_score`
///
/// - **`1` (taiko):** `stamina`, `rhythm`, `color`, `reading`,
///   `mono_stamina_factor`, `mechanical_difficulty`, `consistency_factor`
///
/// - **`2` (catch):** `preempt`, `n_fruits`, `n_droplets`, `n_tiny_droplets`
///
/// - **`3` (mania):** `n_hold_notes`
///
/// Fields `stars`, `max_combo`, `is_convert`, and `mode` are valid for all modes.
#[repr(C)]
pub struct DifficultyAttributes {
    /// Game mode: 0=osu!, 1=taiko, 2=catch, 3=mania
    pub mode: i32,
    /// Star rating (valid for all modes)
    pub stars: f64,
    /// Maximum combo (valid for all modes)
    pub max_combo: u32,
    /// Aim difficulty (osu! only)
    pub aim: f64,
    /// Speed difficulty (osu! only)
    pub speed: f64,
    /// Flashlight difficulty (osu! only)
    pub flashlight: f64,
    /// Stamina difficulty (taiko only)
    pub stamina: f64,
    /// Rhythm difficulty (taiko only)
    pub rhythm: f64,
    /// Color difficulty (taiko only)
    pub color: f64,
    /// Reading difficulty (taiko only)
    pub reading: f64,
    /// Approach Rate (osu! only)
    pub ar: f64,
    /// Overall Difficulty (osu! only)
    pub od: f64,
    /// HP Drain rate (osu! only)
    pub hp: f64,
    /// Great hit window in milliseconds (osu! / taiko)
    pub great_hit_window: f64,
    /// OK hit window in milliseconds (osu! / taiko)
    pub ok_hit_window: f64,
    /// Meh hit window in milliseconds (osu! only)
    pub meh_hit_window: f64,
    /// Number of circles (osu! only)
    pub n_circles: u32,
    /// Number of sliders (osu! only)
    pub n_sliders: u32,
    /// Number of large ticks / whistle hits (osu! only)
    pub n_large_ticks: u32,
    /// Number of spinners (osu! only)
    pub n_spinners: u32,
    /// Number of hit objects (mania only)
    pub n_objects: u32,
    /// Number of difficult aim slider strains (osu! only)
    pub aim_difficult_slider_count: f64,
    /// Slider factor (osu! only)
    pub slider_factor: f64,
    /// Top-weighted aim slider factor (osu! only)
    pub aim_top_weighted_slider_factor: f64,
    /// Top-weighted speed slider factor (osu! only)
    pub speed_top_weighted_slider_factor: f64,
    /// Speed note count (osu! only)
    pub speed_note_count: f64,
    /// Difficult aim strain count (osu! only)
    pub aim_difficult_strain_count: f64,
    /// Difficult speed strain count (osu! only)
    pub speed_difficult_strain_count: f64,
    /// Nested score per object (osu! only)
    pub nested_score_per_object: f64,
    /// Legacy score base multiplier (osu! only)
    pub legacy_score_base_multiplier: f64,
    /// Maximum legacy combo score (osu! only)
    pub maximum_legacy_combo_score: f64,
    /// Mono-stamina factor (taiko only)
    pub mono_stamina_factor: f64,
    /// Mechanical difficulty (taiko only)
    pub mechanical_difficulty: f64,
    /// Consistency factor (taiko only)
    pub consistency_factor: f64,
    /// Preempt value (catch only)
    pub preempt: f64,
    /// Number of fruits (catch only)
    pub n_fruits: u32,
    /// Number of droplets (catch only)
    pub n_droplets: u32,
    /// Number of tiny droplets (catch only)
    pub n_tiny_droplets: u32,
    /// Number of hold notes (mania only)
    pub n_hold_notes: u32,
    /// Whether this is a converted map
    pub is_convert: bool,
}

/// Unified performance attributes for all osu! game modes.
///
/// Contains the total pp and breakdown by category, along with the underlying
/// difficulty attributes. Inspect `difficulty.mode` to determine which fields
/// are valid.
#[repr(C)]
pub struct PerformanceAttributes {
    /// Total performance points
    pub pp: f64,
    /// Performance points from accuracy
    pub pp_acc: f64,
    /// Performance points from aim
    pub pp_aim: f64,
    /// Performance points from speed
    pub pp_speed: f64,
    /// Performance points from flashlight (osu! only)
    pub pp_flashlight: f64,
    /// Performance points from difficulty (taiko / mania)
    pub pp_difficulty: f64,
    /// Maximum combo
    pub max_combo: u32,
    /// Effective miss count (osu! only)
    pub effective_miss_count: f64,
    /// Speed deviation (osu! only)
    pub speed_deviation: f64,
    /// Combo-based estimated miss count (osu! only)
    pub combo_based_estimated_miss_count: f64,
    /// Score-based estimated miss count (osu! only)
    pub score_based_estimated_miss_count: f64,
    /// Estimated slider breaks for aim (osu! only)
    pub aim_estimated_slider_breaks: f64,
    /// Estimated slider breaks for speed (osu! only)
    pub speed_estimated_slider_breaks: f64,
    /// Estimated unstable rate (taiko only)
    pub estimated_unstable_rate: f64,
    /// Underlying difficulty attributes (mode-dependent)
    pub difficulty: DifficultyAttributes,
}

impl From<&OsuDifficultyAttributes> for DifficultyAttributes {
    fn from(attrs: &OsuDifficultyAttributes) -> Self {
        let od = attrs.od();

        let OsuDifficultyAttributes {
            aim,
            aim_difficult_slider_count,
            speed,
            flashlight,
            slider_factor,
            aim_top_weighted_slider_factor,
            speed_top_weighted_slider_factor,
            speed_note_count,
            aim_difficult_strain_count,
            speed_difficult_strain_count,
            nested_score_per_object,
            legacy_score_base_multiplier,
            maximum_legacy_combo_score,
            ar,
            great_hit_window,
            ok_hit_window,
            meh_hit_window,
            hp,
            n_circles,
            n_sliders,
            n_large_ticks,
            n_spinners,
            stars,
            max_combo,
        } = attrs;

        Self {
            mode: 0,
            stars: *stars,
            max_combo: *max_combo,
            aim: *aim,
            speed: *speed,
            flashlight: *flashlight,
            stamina: 0.0,
            rhythm: 0.0,
            color: 0.0,
            reading: 0.0,
            ar: *ar,
            od,
            hp: *hp,
            great_hit_window: *great_hit_window,
            ok_hit_window: *ok_hit_window,
            meh_hit_window: *meh_hit_window,
            n_circles: *n_circles,
            n_sliders: *n_sliders,
            n_large_ticks: *n_large_ticks,
            n_spinners: *n_spinners,
            n_objects: 0,
            aim_difficult_slider_count: *aim_difficult_slider_count,
            slider_factor: *slider_factor,
            aim_top_weighted_slider_factor: *aim_top_weighted_slider_factor,
            speed_top_weighted_slider_factor: *speed_top_weighted_slider_factor,
            speed_note_count: *speed_note_count,
            aim_difficult_strain_count: *aim_difficult_strain_count,
            speed_difficult_strain_count: *speed_difficult_strain_count,
            nested_score_per_object: *nested_score_per_object,
            legacy_score_base_multiplier: *legacy_score_base_multiplier,
            maximum_legacy_combo_score: *maximum_legacy_combo_score,
            mono_stamina_factor: 0.0,
            mechanical_difficulty: 0.0,
            consistency_factor: 0.0,
            preempt: 0.0,
            n_fruits: 0,
            n_droplets: 0,
            n_tiny_droplets: 0,
            n_hold_notes: 0,
            is_convert: false,
        }
    }
}

impl From<&TaikoDifficultyAttributes> for DifficultyAttributes {
    fn from(attrs: &TaikoDifficultyAttributes) -> Self {
        let TaikoDifficultyAttributes {
            stamina,
            rhythm,
            color,
            reading,
            great_hit_window,
            ok_hit_window,
            mono_stamina_factor,
            mechanical_difficulty,
            consistency_factor,
            stars,
            max_combo,
            is_convert,
        } = attrs;

        DifficultyAttributes {
            mode: 1,
            stars: *stars,
            max_combo: *max_combo,
            aim: 0.0,
            speed: 0.0,
            flashlight: 0.0,
            stamina: *stamina,
            rhythm: *rhythm,
            color: *color,
            reading: *reading,
            ar: 0.0,
            od: 0.0,
            hp: 0.0,
            great_hit_window: *great_hit_window,
            ok_hit_window: *ok_hit_window,
            meh_hit_window: 0.0,
            n_circles: 0,
            n_sliders: 0,
            n_large_ticks: 0,
            n_spinners: 0,
            n_objects: 0,
            aim_difficult_slider_count: 0.0,
            slider_factor: 0.0,
            aim_top_weighted_slider_factor: 0.0,
            speed_top_weighted_slider_factor: 0.0,
            speed_note_count: 0.0,
            aim_difficult_strain_count: 0.0,
            speed_difficult_strain_count: 0.0,
            nested_score_per_object: 0.0,
            legacy_score_base_multiplier: 0.0,
            maximum_legacy_combo_score: 0.0,
            mono_stamina_factor: *mono_stamina_factor,
            mechanical_difficulty: *mechanical_difficulty,
            consistency_factor: *consistency_factor,
            preempt: 0.0,
            n_fruits: 0,
            n_droplets: 0,
            n_tiny_droplets: 0,
            n_hold_notes: 0,
            is_convert: *is_convert,
        }
    }
}

impl From<&CatchDifficultyAttributes> for DifficultyAttributes {
    fn from(attrs: &CatchDifficultyAttributes) -> Self {
        let CatchDifficultyAttributes {
            stars,
            preempt,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            is_convert,
        } = attrs;

        let max_combo = attrs.max_combo();

        Self {
            mode: 2,
            stars: *stars,
            max_combo,
            aim: 0.0,
            speed: 0.0,
            flashlight: 0.0,
            stamina: 0.0,
            rhythm: 0.0,
            color: 0.0,
            reading: 0.0,
            ar: 0.0,
            od: 0.0,
            hp: 0.0,
            great_hit_window: 0.0,
            ok_hit_window: 0.0,
            meh_hit_window: 0.0,
            n_circles: 0,
            n_sliders: 0,
            n_large_ticks: 0,
            n_spinners: 0,
            n_objects: 0,
            aim_difficult_slider_count: 0.0,
            slider_factor: 0.0,
            aim_top_weighted_slider_factor: 0.0,
            speed_top_weighted_slider_factor: 0.0,
            speed_note_count: 0.0,
            aim_difficult_strain_count: 0.0,
            speed_difficult_strain_count: 0.0,
            nested_score_per_object: 0.0,
            legacy_score_base_multiplier: 0.0,
            maximum_legacy_combo_score: 0.0,
            mono_stamina_factor: 0.0,
            mechanical_difficulty: 0.0,
            consistency_factor: 0.0,
            preempt: *preempt,
            n_fruits: *n_fruits,
            n_droplets: *n_droplets,
            n_tiny_droplets: *n_tiny_droplets,
            n_hold_notes: 0,
            is_convert: *is_convert,
        }
    }
}

impl From<&ManiaDifficultyAttributes> for DifficultyAttributes {
    fn from(attrs: &ManiaDifficultyAttributes) -> Self {
        let ManiaDifficultyAttributes {
            stars,
            n_objects,
            n_hold_notes,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: 3,
            stars: *stars,
            max_combo: *max_combo,
            aim: 0.0,
            speed: 0.0,
            flashlight: 0.0,
            stamina: 0.0,
            rhythm: 0.0,
            color: 0.0,
            reading: 0.0,
            ar: 0.0,
            od: 0.0,
            hp: 0.0,
            great_hit_window: 0.0,
            ok_hit_window: 0.0,
            meh_hit_window: 0.0,
            n_circles: 0,
            n_sliders: 0,
            n_large_ticks: 0,
            n_spinners: 0,
            n_objects: *n_objects,
            aim_difficult_slider_count: 0.0,
            slider_factor: 0.0,
            aim_top_weighted_slider_factor: 0.0,
            speed_top_weighted_slider_factor: 0.0,
            speed_note_count: 0.0,
            aim_difficult_strain_count: 0.0,
            speed_difficult_strain_count: 0.0,
            nested_score_per_object: 0.0,
            legacy_score_base_multiplier: 0.0,
            maximum_legacy_combo_score: 0.0,
            mono_stamina_factor: 0.0,
            mechanical_difficulty: 0.0,
            consistency_factor: 0.0,
            preempt: 0.0,
            n_fruits: 0,
            n_droplets: 0,
            n_tiny_droplets: 0,
            n_hold_notes: *n_hold_notes,
            is_convert: *is_convert,
        }
    }
}

impl From<&RosuDifficultyAttributes> for DifficultyAttributes {
    fn from(attrs: &RosuDifficultyAttributes) -> Self {
        match attrs {
            RosuDifficultyAttributes::Osu(attrs) => Self::from(attrs),
            RosuDifficultyAttributes::Taiko(attrs) => Self::from(attrs),
            RosuDifficultyAttributes::Catch(attrs) => Self::from(attrs),
            RosuDifficultyAttributes::Mania(attrs) => Self::from(attrs),
        }
    }
}

impl From<&RosuPerformanceAttributes> for PerformanceAttributes {
    fn from(attrs: &RosuPerformanceAttributes) -> Self {
        match attrs {
            RosuPerformanceAttributes::Osu(attrs) => {
                let OsuPerformanceAttributes {
                    difficulty,
                    pp,
                    pp_acc,
                    pp_aim,
                    pp_flashlight,
                    pp_speed,
                    effective_miss_count,
                    speed_deviation,
                    combo_based_estimated_miss_count,
                    score_based_estimated_miss_count,
                    aim_estimated_slider_breaks,
                    speed_estimated_slider_breaks,
                } = attrs;

                PerformanceAttributes {
                    difficulty: difficulty.into(),
                    pp: *pp,
                    pp_acc: *pp_acc,
                    pp_aim: *pp_aim,
                    pp_speed: *pp_speed,
                    pp_flashlight: *pp_flashlight,
                    pp_difficulty: 0.0,
                    max_combo: difficulty.max_combo,
                    effective_miss_count: *effective_miss_count,
                    speed_deviation: speed_deviation.unwrap_or(0.0),
                    combo_based_estimated_miss_count: *combo_based_estimated_miss_count,
                    score_based_estimated_miss_count: score_based_estimated_miss_count
                        .unwrap_or(0.0),
                    aim_estimated_slider_breaks: *aim_estimated_slider_breaks,
                    speed_estimated_slider_breaks: *speed_estimated_slider_breaks,
                    estimated_unstable_rate: 0.0,
                }
            }
            RosuPerformanceAttributes::Taiko(attrs) => {
                let TaikoPerformanceAttributes {
                    difficulty,
                    pp,
                    pp_acc,
                    pp_difficulty,
                    estimated_unstable_rate,
                } = attrs;

                PerformanceAttributes {
                    difficulty: difficulty.into(),
                    pp: *pp,
                    pp_acc: *pp_acc,
                    pp_aim: 0.0,
                    pp_speed: 0.0,
                    pp_flashlight: 0.0,
                    pp_difficulty: *pp_difficulty,
                    max_combo: difficulty.max_combo,
                    effective_miss_count: 0.0,
                    speed_deviation: 0.0,
                    combo_based_estimated_miss_count: 0.0,
                    score_based_estimated_miss_count: 0.0,
                    aim_estimated_slider_breaks: 0.0,
                    speed_estimated_slider_breaks: 0.0,
                    estimated_unstable_rate: estimated_unstable_rate.unwrap_or(0.0),
                }
            }
            RosuPerformanceAttributes::Catch(attrs) => {
                let CatchPerformanceAttributes { difficulty, pp } = attrs;

                PerformanceAttributes {
                    difficulty: difficulty.into(),
                    pp: *pp,
                    pp_acc: 0.0,
                    pp_aim: 0.0,
                    pp_speed: 0.0,
                    pp_flashlight: 0.0,
                    pp_difficulty: 0.0,
                    max_combo: difficulty.max_combo(),
                    effective_miss_count: 0.0,
                    speed_deviation: 0.0,
                    combo_based_estimated_miss_count: 0.0,
                    score_based_estimated_miss_count: 0.0,
                    aim_estimated_slider_breaks: 0.0,
                    speed_estimated_slider_breaks: 0.0,
                    estimated_unstable_rate: 0.0,
                }
            }
            RosuPerformanceAttributes::Mania(attrs) => {
                let ManiaPerformanceAttributes {
                    difficulty,
                    pp,
                    pp_difficulty,
                } = attrs;

                PerformanceAttributes {
                    difficulty: difficulty.into(),
                    pp: *pp,
                    pp_acc: 0.0,
                    pp_aim: 0.0,
                    pp_speed: 0.0,
                    pp_flashlight: 0.0,
                    pp_difficulty: *pp_difficulty,
                    max_combo: difficulty.max_combo,
                    effective_miss_count: 0.0,
                    speed_deviation: 0.0,
                    combo_based_estimated_miss_count: 0.0,
                    score_based_estimated_miss_count: 0.0,
                    aim_estimated_slider_breaks: 0.0,
                    speed_estimated_slider_breaks: 0.0,
                    estimated_unstable_rate: 0.0,
                }
            }
        }
    }
}
