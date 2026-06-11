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
