use rosu_pp::any::ScoreState as RosuScoreState;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScoreState {
    pub max_combo: u32,
    pub osu_large_tick_hits: u32,
    pub osu_small_tick_hits: u32,
    pub slider_end_hits: u32,
    pub n_geki: u32,
    pub n_katu: u32,
    pub n300: u32,
    pub n100: u32,
    pub n50: u32,
    pub misses: u32,
    pub legacy_total_score: u32,
    pub legacy_total_score_valid: bool,
}

impl From<&ScoreState> for RosuScoreState {
    fn from(state: &ScoreState) -> Self {
        let ScoreState {
            max_combo,
            osu_large_tick_hits,
            osu_small_tick_hits,
            slider_end_hits,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            misses,
            legacy_total_score,
            legacy_total_score_valid,
        } = state;

        RosuScoreState {
            max_combo: *max_combo,
            osu_large_tick_hits: *osu_large_tick_hits,
            osu_small_tick_hits: *osu_small_tick_hits,
            slider_end_hits: *slider_end_hits,
            n_geki: *n_geki,
            n_katu: *n_katu,
            n300: *n300,
            n100: *n100,
            n50: *n50,
            misses: *misses,
            legacy_total_score: if *legacy_total_score_valid {
                Some(*legacy_total_score)
            } else {
                None
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn rosu_pp_score_state_new() -> ScoreState {
    ScoreState {
        max_combo: 0,
        osu_large_tick_hits: 0,
        osu_small_tick_hits: 0,
        slider_end_hits: 0,
        n_geki: 0,
        n_katu: 0,
        n300: 0,
        n100: 0,
        n50: 0,
        misses: 0,
        legacy_total_score: 0,
        legacy_total_score_valid: false,
    }
}
