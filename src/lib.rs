//! # rosu-pp FFI
//!
//! C FFI bindings for rosu-pp, a high-performance osu! beatmap difficulty and
//! performance calculation library.
//!
//! ## Overview
//!
//! This library provides a C-compatible interface for calculating:
//! - **Star ratings** and difficulty attributes for osu!, taiko, catch, and mania
//! - **Performance points (pp)** for replays and scores
//! - **Gradual/per-frame pp** for real-time score progression tracking
//!
//! ## Memory Management
//!
//! All complex types are exposed as **opaque handles** (`*mut Handle`). The caller is
//! responsible for:
//! 1. Allocating handles via the constructor functions (e.g., `rosu_pp_beatmap_from_path`)
//! 2. Freeing handles via the corresponding `*_free` functions when done
//! 3. **Not** using a handle after it has been freed
//!
//! ### Ownership Transfer
//!
//! Some functions **consume** a handle, meaning the original handle becomes invalid
//! after the call. The caller must NOT use or free the consumed handle:
//!
//! - `rosu_pp_gradual_performance_new` — consumes the difficulty handle. The caller
//!   must NOT call `rosu_pp_difficulty_free` on the difficulty handle passed to this
//!   function, nor use it afterward.
//!
//! ### Handle Reuse After Setters
//!
//! Setter functions (e.g., `rosu_pp_difficulty_mods`, `rosu_pp_performance_accuracy`)
//! **do not consume** the handle. The handle remains valid and can be reused for
//! subsequent setter calls or for calling the calculation function.
//!
//! ## Error Handling
//!
//! Functions that can fail return `FfiResult`. A return value of `NullPointer`
//! indicates a null pointer was passed where a valid handle was expected. Check the
//! return value before using output parameters.
//!
//! ## Thread Safety
//!
//! Handles are **not** thread-safe. Each handle should be used from a single thread.
//! Clone handles (`rosu_pp_difficulty_clone`) for use across threads.
//!
//! ## Usage Pattern
//!
//! ```c
//! // 1. Load a beatmap
//! rosu_pp_BeatmapHandle* map = rosu_pp_beatmap_from_path("path/to/map.osu");
//! if (!map) { /* handle error */ }
//!
//! // 2. Create a difficulty calculator and configure it
//! rosu_pp_DifficultyHandle* diff = rosu_pp_difficulty_new();
//! rosu_pp_difficulty_mods(diff, mods);
//! rosu_pp_difficulty_lazer(diff, true);
//!
//! // 3. Calculate difficulty attributes
//! rosu_pp_DifficultyAttributes attrs;
//! rosu_pp_difficulty_calculate(diff, map, &attrs);
//!
//! // 4. Free resources
//! rosu_pp_difficulty_free(diff);
//! rosu_pp_beatmap_free(map);
//! ```
//!
//! ## Dependencies
//!
//! - rosu-pp — Core pp calculation
//! - rosu-map — Beatmap parsing
//! - rosu-mods — Game mod parsing

#[macro_use]
mod handle;

mod attributes;
mod beatmap;
mod difficulty;
mod error;
mod gradual;
mod mode;
mod mods;
mod performance;
mod score_state;
mod strains;

// Re-export FFI functions and types from all modules for test accessibility.

// Error
pub use error::FfiResult as rosu_pp_FfiResult;

// Mode
pub use mode::GameMode as rosu_pp_GameMode;

// Beatmap
pub use beatmap::{
    rosu_pp_beatmap_ar, rosu_pp_beatmap_bpm, rosu_pp_beatmap_break_count,
    rosu_pp_beatmap_check_suspicion, rosu_pp_beatmap_cs, rosu_pp_beatmap_difficulty_point_count,
    rosu_pp_beatmap_effect_point_count, rosu_pp_beatmap_free, rosu_pp_beatmap_from_bytes,
    rosu_pp_beatmap_from_path, rosu_pp_beatmap_hit_object_count, rosu_pp_beatmap_hit_sound_count,
    rosu_pp_beatmap_hp, rosu_pp_beatmap_is_convert, rosu_pp_beatmap_mode, rosu_pp_beatmap_od,
    rosu_pp_beatmap_slider_multiplier, rosu_pp_beatmap_slider_tick_rate,
    rosu_pp_beatmap_stack_leniency, rosu_pp_beatmap_timing_point_count,
    rosu_pp_beatmap_total_break_time, rosu_pp_beatmap_version,
};

// Mods
pub use mods::ModsHandle as rosu_pp_ModsHandle;
pub use mods::{
    rosu_pp_mods_free, rosu_pp_mods_free_string, rosu_pp_mods_from_bits, rosu_pp_mods_parse,
    rosu_pp_mods_parse_with_mode, rosu_pp_mods_to_bits, rosu_pp_mods_to_string,
};

// Difficulty
pub use difficulty::inspect::{
    rosu_pp_difficulty_inspect_new, rosu_pp_inspect_difficulty_ar,
    rosu_pp_inspect_difficulty_clock_rate, rosu_pp_inspect_difficulty_cs,
    rosu_pp_inspect_difficulty_free, rosu_pp_inspect_difficulty_hardrock_offsets,
    rosu_pp_inspect_difficulty_hp, rosu_pp_inspect_difficulty_lazer,
    rosu_pp_inspect_difficulty_mods, rosu_pp_inspect_difficulty_od,
    rosu_pp_inspect_difficulty_passed_objects,
};
pub use difficulty::{
    rosu_pp_difficulty_ar, rosu_pp_difficulty_calculate, rosu_pp_difficulty_checked_calculate,
    rosu_pp_difficulty_clock_rate, rosu_pp_difficulty_clone, rosu_pp_difficulty_cs,
    rosu_pp_difficulty_free, rosu_pp_difficulty_hardrock_offsets, rosu_pp_difficulty_hp,
    rosu_pp_difficulty_lazer, rosu_pp_difficulty_mods, rosu_pp_difficulty_new,
    rosu_pp_difficulty_od, rosu_pp_difficulty_passed_objects, rosu_pp_difficulty_strains,
};

// Gradual
pub use gradual::difficulty::{
    rosu_pp_gradual_difficulty_free, rosu_pp_gradual_difficulty_new,
    rosu_pp_gradual_difficulty_next,
};
pub use gradual::performance::{
    rosu_pp_gradual_performance_free, rosu_pp_gradual_performance_new,
    rosu_pp_gradual_performance_next,
};

// Performance
pub use performance::{
    rosu_pp_performance_accuracy, rosu_pp_performance_ar, rosu_pp_performance_calculate,
    rosu_pp_performance_checked_calculate, rosu_pp_performance_clock_rate,
    rosu_pp_performance_combo, rosu_pp_performance_cs, rosu_pp_performance_free,
    rosu_pp_performance_hardrock_offsets, rosu_pp_performance_hitresult_priority,
    rosu_pp_performance_hp, rosu_pp_performance_large_tick_hits, rosu_pp_performance_lazer,
    rosu_pp_performance_legacy_total_score, rosu_pp_performance_misses, rosu_pp_performance_mods,
    rosu_pp_performance_n100, rosu_pp_performance_n300, rosu_pp_performance_n50,
    rosu_pp_performance_n_geki, rosu_pp_performance_n_katu, rosu_pp_performance_new,
    rosu_pp_performance_od, rosu_pp_performance_passed_objects,
    rosu_pp_performance_slider_end_hits, rosu_pp_performance_small_tick_hits,
    rosu_pp_performance_state,
};

// Score state
pub use score_state::{rosu_pp_score_state_new, rosu_pp_score_state_total_hits};

// Strains
pub use strains::rosu_pp_strains_free;

// Attributes
pub use attributes::difficulty::DifficultyAttributes as rosu_pp_DifficultyAttributes;
pub use attributes::performance::PerformanceAttributes as rosu_pp_PerformanceAttributes;
pub use score_state::ScoreState as rosu_pp_ScoreState;
pub use strains::StrainsData as rosu_pp_StrainsData;

// Beatmap attributes (from attributes::beatmap submodule)
pub use attributes::beatmap::{
    adjusted::rosu_pp_beatmap_attrs_apply_clock_rate,
    adjusted::AdjustedBeatmapAttributes,
    builder::{
        rosu_pp_beatmap_attrs_builder_ar, rosu_pp_beatmap_attrs_builder_build,
        rosu_pp_beatmap_attrs_builder_clock_rate, rosu_pp_beatmap_attrs_builder_cs,
        rosu_pp_beatmap_attrs_builder_difficulty, rosu_pp_beatmap_attrs_builder_free,
        rosu_pp_beatmap_attrs_builder_hp, rosu_pp_beatmap_attrs_builder_map,
        rosu_pp_beatmap_attrs_builder_mode, rosu_pp_beatmap_attrs_builder_mods,
        rosu_pp_beatmap_attrs_builder_new, rosu_pp_beatmap_attrs_builder_od,
    },
    hit_windows::HitWindows,
    rosu_pp_beatmap_attrs_ar, rosu_pp_beatmap_attrs_clock_rate, rosu_pp_beatmap_attrs_cs,
    rosu_pp_beatmap_attrs_free, rosu_pp_beatmap_attrs_hit_windows, rosu_pp_beatmap_attrs_hp,
    rosu_pp_beatmap_attrs_od,
};
