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

#[repr(C)]
pub struct DifficultyAttributes {
    pub mode: i32,
    pub stars: f64,
    pub max_combo: u32,
    pub aim: f64,
    pub speed: f64,
    pub flashlight: f64,
    pub stamina: f64,
    pub rhythm: f64,
    pub color: f64,
    pub reading: f64,
    pub ar: f64,
    pub od: f64,
    pub hp: f64,
    pub great_hit_window: f64,
    pub ok_hit_window: f64,
    pub meh_hit_window: f64,
    pub n_circles: u32,
    pub n_sliders: u32,
    pub n_large_ticks: u32,
    pub n_spinners: u32,
    pub n_objects: u32,
    pub aim_difficult_slider_count: f64,
    pub slider_factor: f64,
    pub aim_top_weighted_slider_factor: f64,
    pub speed_top_weighted_slider_factor: f64,
    pub speed_note_count: f64,
    pub aim_difficult_strain_count: f64,
    pub speed_difficult_strain_count: f64,
    pub nested_score_per_object: f64,
    pub legacy_score_base_multiplier: f64,
    pub maximum_legacy_combo_score: f64,
    pub mono_stamina_factor: f64,
    pub mechanical_difficulty: f64,
    pub consistency_factor: f64,
    pub preempt: f64,
    pub n_fruits: u32,
    pub n_droplets: u32,
    pub n_tiny_droplets: u32,
    pub n_hold_notes: u32,
    pub is_convert: bool,
}

#[repr(C)]
pub struct PerformanceAttributes {
    pub pp: f64,
    pub pp_acc: f64,
    pub pp_aim: f64,
    pub pp_speed: f64,
    pub pp_flashlight: f64,
    pub pp_difficulty: f64,
    pub max_combo: u32,
    pub effective_miss_count: f64,
    pub speed_deviation: f64,
    pub combo_based_estimated_miss_count: f64,
    pub score_based_estimated_miss_count: f64,
    pub aim_estimated_slider_breaks: f64,
    pub speed_estimated_slider_breaks: f64,
    pub estimated_unstable_rate: f64,
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
