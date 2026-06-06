//! Score state struct for performance calculations.
//!
//! Represents the hit results and score composition for a single play, used
//! in performance and gradual performance calculations.

use rosu_pp::any::ScoreState as RosuScoreState;
use rosu_map::section::general::GameMode as RosuMapGameMode;

/// Hit result counts and score composition for a single play.
///
/// Initialize with `rosu_pp_score_state_new` and fill in the fields
/// corresponding to the game mode and hit results achieved.
///
/// **osu! fields:** `max_combo`, `osu_large_tick_hits`, `osu_small_tick_hits`,
/// `slider_end_hits`, `n300`, `n100`, `n50`, `misses`, `legacy_total_score`,
/// `legacy_total_score_valid`
///
/// **Taiko fields:** `max_combo`, `n300`, `n100`, `n50`, `misses`
///
/// **Catch fields:** `max_combo`, `n300`, `n100`, `n50`, `n_katu`, `misses`
///
/// **Mania fields:** `max_combo`, `n_geki`, `n_katu`, `n300`, `n100`, `n50`,
/// `misses`, `legacy_total_score`, `legacy_total_score_valid`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScoreState {
    /// Maximum combo achieved in the play
    pub max_combo: u32,
    /// Number of large tick (whistle) hits on sliders (osu! only)
    pub osu_large_tick_hits: u32,
    /// Number of small tick (clap) hits on sliders (osu! only)
    pub osu_small_tick_hits: u32,
    /// Number of slider end hits (osu! only)
    pub slider_end_hits: u32,
    /// Number of geki (320) hits (mania only)
    pub n_geki: u32,
    /// Number of katu (200) hits / tiny droplet misses (mania / catch)
    pub n_katu: u32,
    /// Number of 300-score hit results
    pub n300: u32,
    /// Number of 100-score hit results
    pub n100: u32,
    /// Number of 50-score hit results
    pub n50: u32,
    /// Number of misses
    pub misses: u32,
    /// Legacy total score value (osu! only)
    pub legacy_total_score: u32,
    /// Whether `legacy_total_score` is valid (osu! only)
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

/// Create a new zero-initialized ScoreState.
///
/// **Returns:** A `ScoreState` struct with all fields set to zero/false.
///
/// Initialize the returned struct with the appropriate hit result counts
/// for the play being evaluated.
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

/// Calculate the total number of hits from a score state for a given game mode.
///
/// Adds up n300, n100, n50 (if not taiko), n_katu (if not osu/taiko),
/// and n_geki (if not osu/taiko/catch) to get the total hit count.
///
/// **Parameters:**
/// - `state`: A reference to a `ScoreState` struct.
/// - `mode`: The game mode (0=osu!, 1=taiko, 2=catch, 3=mania).
///
/// **Returns:** The total number of hits, or 0 if `state` is null.
#[no_mangle]
pub extern "C" fn rosu_pp_score_state_total_hits(state: *const ScoreState, mode: i32) -> u32 {
    if state.is_null() {
        return 0;
    }

    let s = unsafe { &*state };
    let mode = match mode {
        0 => RosuMapGameMode::Osu,
        1 => RosuMapGameMode::Taiko,
        2 => RosuMapGameMode::Catch,
        3 => RosuMapGameMode::Mania,
        _ => return 0,
    };

    let mut amount = s.n300 + s.n100 + s.misses;

    if mode != RosuMapGameMode::Taiko {
        amount += s.n50;

        if mode != RosuMapGameMode::Osu {
            amount += s.n_katu;
            amount += u32::from(mode != RosuMapGameMode::Catch) * s.n_geki;
        }
    }

    amount
}
