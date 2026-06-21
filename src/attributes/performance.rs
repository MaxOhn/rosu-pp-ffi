use rosu_pp::{
    any::PerformanceAttributes as RosuPerformanceAttributes,
    catch::{CatchDifficultyAttributes, CatchPerformanceAttributes},
    mania::{ManiaDifficultyAttributes, ManiaPerformanceAttributes},
    osu::{OsuDifficultyAttributes, OsuPerformanceAttributes},
    taiko::{TaikoDifficultyAttributes, TaikoPerformanceAttributes},
};

use crate::attributes::difficulty::DifficultyAttributes;

/// Unified performance attributes for all osu! game modes.
///
/// Contains the total pp and breakdown by category, along with the underlying
/// difficulty attributes. Inspect `difficulty.mode` to determine which fields
/// are valid.
#[repr(C)]
#[cheadergen::config(export)]
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

impl From<&PerformanceAttributes> for RosuPerformanceAttributes {
    fn from(attrs: &PerformanceAttributes) -> Self {
        let PerformanceAttributes {
            pp,
            pp_acc,
            pp_aim,
            pp_speed,
            pp_flashlight,
            pp_difficulty,
            effective_miss_count,
            speed_deviation,
            combo_based_estimated_miss_count,
            score_based_estimated_miss_count,
            aim_estimated_slider_breaks,
            speed_estimated_slider_breaks,
            estimated_unstable_rate,
            difficulty:
                DifficultyAttributes {
                    mode,
                    stars,
                    max_combo,
                    aim,
                    speed,
                    flashlight,
                    stamina,
                    rhythm,
                    color,
                    reading,
                    ar,
                    hp,
                    great_hit_window,
                    ok_hit_window,
                    meh_hit_window,
                    n_circles,
                    n_sliders,
                    n_large_ticks,
                    n_spinners,
                    n_objects,
                    aim_difficult_slider_count,
                    slider_factor,
                    aim_top_weighted_slider_factor,
                    speed_top_weighted_slider_factor,
                    speed_note_count,
                    aim_difficult_strain_count,
                    speed_difficult_strain_count,
                    nested_score_per_object,
                    legacy_score_base_multiplier,
                    maximum_legacy_combo_score,
                    mono_stamina_factor,
                    mechanical_difficulty,
                    consistency_factor,
                    preempt,
                    n_fruits,
                    n_droplets,
                    n_tiny_droplets,
                    n_hold_notes,
                    is_convert,
                },
        } = attrs;

        match mode {
            0 => RosuPerformanceAttributes::Osu(OsuPerformanceAttributes {
                difficulty: OsuDifficultyAttributes {
                    stars: *stars,
                    max_combo: *max_combo,
                    aim: *aim,
                    speed: *speed,
                    flashlight: *flashlight,
                    ar: *ar,
                    hp: *hp,
                    great_hit_window: *great_hit_window,
                    ok_hit_window: *ok_hit_window,
                    meh_hit_window: *meh_hit_window,
                    n_circles: *n_circles,
                    n_sliders: *n_sliders,
                    n_large_ticks: *n_large_ticks,
                    n_spinners: *n_spinners,
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
                },
                pp: *pp,
                pp_acc: *pp_acc,
                pp_aim: *pp_aim,
                pp_flashlight: *pp_flashlight,
                pp_speed: *pp_speed,
                effective_miss_count: *effective_miss_count,
                speed_deviation: Some(*speed_deviation),
                combo_based_estimated_miss_count: *combo_based_estimated_miss_count,
                score_based_estimated_miss_count: Some(*score_based_estimated_miss_count),
                aim_estimated_slider_breaks: *aim_estimated_slider_breaks,
                speed_estimated_slider_breaks: *speed_estimated_slider_breaks,
            }),
            1 => RosuPerformanceAttributes::Taiko(TaikoPerformanceAttributes {
                difficulty: TaikoDifficultyAttributes {
                    stars: *stars,
                    max_combo: *max_combo,
                    stamina: *stamina,
                    rhythm: *rhythm,
                    color: *color,
                    reading: *reading,
                    great_hit_window: *great_hit_window,
                    ok_hit_window: *ok_hit_window,
                    mono_stamina_factor: *mono_stamina_factor,
                    mechanical_difficulty: *mechanical_difficulty,
                    consistency_factor: *consistency_factor,
                    is_convert: *is_convert,
                },
                pp: *pp,
                pp_acc: *pp_acc,
                pp_difficulty: *pp_difficulty,
                estimated_unstable_rate: Some(*estimated_unstable_rate),
            }),
            2 => RosuPerformanceAttributes::Catch(CatchPerformanceAttributes {
                difficulty: CatchDifficultyAttributes {
                    stars: *stars,
                    preempt: *preempt,
                    n_fruits: *n_fruits,
                    n_droplets: *n_droplets,
                    n_tiny_droplets: *n_tiny_droplets,
                    is_convert: *is_convert,
                },
                pp: *pp,
            }),
            3 => RosuPerformanceAttributes::Mania(ManiaPerformanceAttributes {
                difficulty: ManiaDifficultyAttributes {
                    stars: *stars,
                    n_objects: *n_objects,
                    n_hold_notes: *n_hold_notes,
                    max_combo: *max_combo,
                    is_convert: *is_convert,
                },
                pp: *pp,
                pp_difficulty: *pp_difficulty,
            }),
            _ => unreachable!("invalid mode {mode}"),
        }
    }
}
