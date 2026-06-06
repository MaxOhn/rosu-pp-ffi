#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Return code for FFI functions that can fail.
 *
 * Variants:
 * - `Ok` — Operation succeeded.
 * - `Done` — Gradual calculator has processed all objects (only returned by
 *   `rosu_pp_gradual_performance_next`).
 * - `ParseError` — Input string could not be parsed (beatmap parsing, mod parsing).
 * - `NullPointer` — A null pointer was passed where a valid handle was expected.
 * - `InvalidArgument` — An argument value was out of range or otherwise invalid.
 */
typedef enum rosu_pp_FfiResult {
    rosu_pp_FfiResult_Ok = 0,
    rosu_pp_FfiResult_Done = 1,
    rosu_pp_FfiResult_ParseError = 2,
    rosu_pp_FfiResult_NullPointer = 3,
    rosu_pp_FfiResult_InvalidArgument = 4,
} rosu_pp_FfiResult;

/**
 * The four osu! game modes.
 *
 * Matches the integer values used by the osu! API:
 * - `Osu` = 0 (osu!standard)
 * - `Taiko` = 1 (osu!taiko)
 * - `Catch` = 2 (osu!catch / fruits)
 * - `Mania` = 3 (osu!mania)
 */
typedef enum rosu_pp_GameMode {
    rosu_pp_GameMode_Osu = 0,
    rosu_pp_GameMode_Taiko = 1,
    rosu_pp_GameMode_Catch = 2,
    rosu_pp_GameMode_Mania = 3,
} rosu_pp_GameMode;

/**
 * Opaque handle to a loaded osu! beatmap.
 *
 * Created via `rosu_pp_beatmap_from_path` or `rosu_pp_beatmap_from_bytes`.
 * Must be freed with `rosu_pp_beatmap_free` when no longer needed.
 */
typedef struct rosu_pp_BeatmapHandle rosu_pp_BeatmapHandle;

/**
 * Opaque handle to a difficulty calculator builder.
 *
 * Created via `rosu_pp_difficulty_new`. Configure it with setter functions,
 * then calculate with `rosu_pp_difficulty_calculate`.
 *
 * **Builder pattern:** Each setter consumes the handle internally (using
 * `Box::from_raw` + `mem::forget`) and returns `FfiResult::Ok`. The handle
 * pointer remains valid and can be used for subsequent setter calls.
 *
 * **Must be freed** with `rosu_pp_difficulty_free` when done.
 */
typedef struct rosu_pp_DifficultyHandle rosu_pp_DifficultyHandle;

/**
 * Opaque handle to a gradual performance calculator.
 *
 * Created via `rosu_pp_gradual_performance_new`. Iterate through hit objects
 * using `rosu_pp_gradual_performance_next` until it returns `FfiResult::Done`.
 *
 * **Must be freed** with `rosu_pp_gradual_performance_free` when done.
 */
typedef struct rosu_pp_GradualPerformanceHandle rosu_pp_GradualPerformanceHandle;

/**
 * Opaque handle to a game mods collection.
 *
 * Created via `rosu_pp_mods_parse`, `rosu_pp_mods_parse_with_mode`, or
 * `rosu_pp_mods_from_bits`. Must be freed with `rosu_pp_mods_free`.
 */
typedef struct rosu_pp_ModsHandle rosu_pp_ModsHandle;

/**
 * Opaque handle to a performance calculator builder.
 *
 * Created via `rosu_pp_performance_new`. Configure it with setter functions,
 * then calculate with `rosu_pp_performance_calculate`.
 *
 * **Builder pattern:** Each setter consumes the handle internally (using
 * `Box::from_raw` + `mem::forget`) and returns `FfiResult::Ok`. The handle
 * pointer remains valid and can be used for subsequent setter calls.
 *
 * **Must be freed** with `rosu_pp_performance_free` when done.
 */
typedef struct rosu_pp_PerformanceHandle rosu_pp_PerformanceHandle;

/**
 * Unified difficulty attributes for all osu! game modes.
 *
 * After a difficulty calculation, inspect the `mode` field to determine which
 * attributes are valid:
 *
 * - **`0` (osu!):** `aim`, `speed`, `flashlight`, `ar`, `od`, `hp`,
 *   `great_hit_window`, `ok_hit_window`, `meh_hit_window`, `n_circles`,
 *   `n_sliders`, `n_large_ticks`, `n_spinners`, `aim_difficult_slider_count`,
 *   `slider_factor`, `aim_top_weighted_slider_factor`, `speed_top_weighted_slider_factor`,
 *   `speed_note_count`, `aim_difficult_strain_count`, `speed_difficult_strain_count`,
 *   `nested_score_per_object`, `legacy_score_base_multiplier`, `maximum_legacy_combo_score`
 *
 * - **`1` (taiko):** `stamina`, `rhythm`, `color`, `reading`,
 *   `mono_stamina_factor`, `mechanical_difficulty`, `consistency_factor`
 *
 * - **`2` (catch):** `preempt`, `n_fruits`, `n_droplets`, `n_tiny_droplets`
 *
 * - **`3` (mania):** `n_hold_notes`
 *
 * Fields `stars`, `max_combo`, `is_convert`, and `mode` are valid for all modes.
 */
typedef struct rosu_pp_DifficultyAttributes {
    /**
     * Game mode: 0=osu!, 1=taiko, 2=catch, 3=mania
     */
    int32_t mode;
    /**
     * Star rating (valid for all modes)
     */
    double stars;
    /**
     * Maximum combo (valid for all modes)
     */
    uint32_t max_combo;
    /**
     * Aim difficulty (osu! only)
     */
    double aim;
    /**
     * Speed difficulty (osu! only)
     */
    double speed;
    /**
     * Flashlight difficulty (osu! only)
     */
    double flashlight;
    /**
     * Stamina difficulty (taiko only)
     */
    double stamina;
    /**
     * Rhythm difficulty (taiko only)
     */
    double rhythm;
    /**
     * Color difficulty (taiko only)
     */
    double color;
    /**
     * Reading difficulty (taiko only)
     */
    double reading;
    /**
     * Approach Rate (osu! only)
     */
    double ar;
    /**
     * Overall Difficulty (osu! only)
     */
    double od;
    /**
     * HP Drain rate (osu! only)
     */
    double hp;
    /**
     * Great hit window in milliseconds (osu! / taiko)
     */
    double great_hit_window;
    /**
     * OK hit window in milliseconds (osu! / taiko)
     */
    double ok_hit_window;
    /**
     * Meh hit window in milliseconds (osu! only)
     */
    double meh_hit_window;
    /**
     * Number of circles (osu! only)
     */
    uint32_t n_circles;
    /**
     * Number of sliders (osu! only)
     */
    uint32_t n_sliders;
    /**
     * Number of large ticks / whistle hits (osu! only)
     */
    uint32_t n_large_ticks;
    /**
     * Number of spinners (osu! only)
     */
    uint32_t n_spinners;
    /**
     * Number of hit objects (mania only)
     */
    uint32_t n_objects;
    /**
     * Number of difficult aim slider strains (osu! only)
     */
    double aim_difficult_slider_count;
    /**
     * Slider factor (osu! only)
     */
    double slider_factor;
    /**
     * Top-weighted aim slider factor (osu! only)
     */
    double aim_top_weighted_slider_factor;
    /**
     * Top-weighted speed slider factor (osu! only)
     */
    double speed_top_weighted_slider_factor;
    /**
     * Speed note count (osu! only)
     */
    double speed_note_count;
    /**
     * Difficult aim strain count (osu! only)
     */
    double aim_difficult_strain_count;
    /**
     * Difficult speed strain count (osu! only)
     */
    double speed_difficult_strain_count;
    /**
     * Nested score per object (osu! only)
     */
    double nested_score_per_object;
    /**
     * Legacy score base multiplier (osu! only)
     */
    double legacy_score_base_multiplier;
    /**
     * Maximum legacy combo score (osu! only)
     */
    double maximum_legacy_combo_score;
    /**
     * Mono-stamina factor (taiko only)
     */
    double mono_stamina_factor;
    /**
     * Mechanical difficulty (taiko only)
     */
    double mechanical_difficulty;
    /**
     * Consistency factor (taiko only)
     */
    double consistency_factor;
    /**
     * Preempt value (catch only)
     */
    double preempt;
    /**
     * Number of fruits (catch only)
     */
    uint32_t n_fruits;
    /**
     * Number of droplets (catch only)
     */
    uint32_t n_droplets;
    /**
     * Number of tiny droplets (catch only)
     */
    uint32_t n_tiny_droplets;
    /**
     * Number of hold notes (mania only)
     */
    uint32_t n_hold_notes;
    /**
     * Whether this is a converted map
     */
    bool is_convert;
} rosu_pp_DifficultyAttributes;

/**
 * Hit result counts and score composition for a single play.
 *
 * Initialize with `rosu_pp_score_state_new` and fill in the fields
 * corresponding to the game mode and hit results achieved.
 *
 * **osu! fields:** `max_combo`, `osu_large_tick_hits`, `osu_small_tick_hits`,
 * `slider_end_hits`, `n300`, `n100`, `n50`, `misses`, `legacy_total_score`,
 * `legacy_total_score_valid`
 *
 * **Taiko fields:** `max_combo`, `n300`, `n100`, `n50`, `misses`
 *
 * **Catch fields:** `max_combo`, `n300`, `n100`, `n50`, `n_katu`, `misses`
 *
 * **Mania fields:** `max_combo`, `n_geki`, `n_katu`, `n300`, `n100`, `n50`,
 * `misses`, `legacy_total_score`, `legacy_total_score_valid`
 */
typedef struct rosu_pp_ScoreState {
    /**
     * Maximum combo achieved in the play
     */
    uint32_t max_combo;
    /**
     * Number of large tick (whistle) hits on sliders (osu! only)
     */
    uint32_t osu_large_tick_hits;
    /**
     * Number of small tick (clap) hits on sliders (osu! only)
     */
    uint32_t osu_small_tick_hits;
    /**
     * Number of slider end hits (osu! only)
     */
    uint32_t slider_end_hits;
    /**
     * Number of geki (320) hits (mania only)
     */
    uint32_t n_geki;
    /**
     * Number of katu (200) hits / tiny droplet misses (mania / catch)
     */
    uint32_t n_katu;
    /**
     * Number of 300-score hit results
     */
    uint32_t n300;
    /**
     * Number of 100-score hit results
     */
    uint32_t n100;
    /**
     * Number of 50-score hit results
     */
    uint32_t n50;
    /**
     * Number of misses
     */
    uint32_t misses;
    /**
     * Legacy total score value (osu! only)
     */
    uint32_t legacy_total_score;
    /**
     * Whether `legacy_total_score` is valid (osu! only)
     */
    bool legacy_total_score_valid;
} rosu_pp_ScoreState;

/**
 * Unified performance attributes for all osu! game modes.
 *
 * Contains the total pp and breakdown by category, along with the underlying
 * difficulty attributes. Inspect `difficulty.mode` to determine which fields
 * are valid.
 */
typedef struct rosu_pp_PerformanceAttributes {
    /**
     * Total performance points
     */
    double pp;
    /**
     * Performance points from accuracy
     */
    double pp_acc;
    /**
     * Performance points from aim
     */
    double pp_aim;
    /**
     * Performance points from speed
     */
    double pp_speed;
    /**
     * Performance points from flashlight (osu! only)
     */
    double pp_flashlight;
    /**
     * Performance points from difficulty (taiko / mania)
     */
    double pp_difficulty;
    /**
     * Maximum combo
     */
    uint32_t max_combo;
    /**
     * Effective miss count (osu! only)
     */
    double effective_miss_count;
    /**
     * Speed deviation (osu! only)
     */
    double speed_deviation;
    /**
     * Combo-based estimated miss count (osu! only)
     */
    double combo_based_estimated_miss_count;
    /**
     * Score-based estimated miss count (osu! only)
     */
    double score_based_estimated_miss_count;
    /**
     * Estimated slider breaks for aim (osu! only)
     */
    double aim_estimated_slider_breaks;
    /**
     * Estimated slider breaks for speed (osu! only)
     */
    double speed_estimated_slider_breaks;
    /**
     * Estimated unstable rate (taiko only)
     */
    double estimated_unstable_rate;
    /**
     * Underlying difficulty attributes (mode-dependent)
     */
    struct rosu_pp_DifficultyAttributes difficulty;
} rosu_pp_PerformanceAttributes;

/**
 * Load a beatmap from a file path.
 *
 * **Parameters:**
 * - `path`: Null-terminated C string containing the file path to the `.osu` file.
 *
 * **Returns:** A non-null handle on success, or `NULL` if:
 * - The path pointer is null
 * - The path contains invalid UTF-8
 * - The file cannot be read or parsed
 *
 * **Memory:** The caller owns the returned handle and must free it with
 * `rosu_pp_beatmap_free`.
 */
struct rosu_pp_BeatmapHandle *rosu_pp_beatmap_from_path(const char *path);

/**
 * Load a beatmap from raw bytes.
 *
 * **Parameters:**
 * - `bytes`: Pointer to a buffer containing the `.osu` file contents.
 * - `len`: Length of the buffer in bytes.
 *
 * **Returns:** A non-null handle on success, or `NULL` if:
 * - The bytes pointer is null
 * - The bytes cannot be parsed as a valid beatmap
 *
 * **Memory:** The caller owns the returned handle and must free it with
 * `rosu_pp_beatmap_free`. The `bytes` buffer is only borrowed during this call
 * and may be freed immediately after.
 */
struct rosu_pp_BeatmapHandle *rosu_pp_beatmap_from_bytes(const uint8_t *bytes, size_t len);

/**
 * Free a beatmap handle and release its memory.
 *
 * **Parameters:**
 * - `handle`: A handle returned by `rosu_pp_beatmap_from_path` or
 *   `rosu_pp_beatmap_from_bytes`. May be null (null is a no-op).
 *
 * After calling this function, the handle must NOT be used again.
 */
void rosu_pp_beatmap_free(struct rosu_pp_BeatmapHandle *handle);

/**
 * Create a new difficulty calculator with default settings.
 *
 * **Returns:** A non-null handle to a new `Difficulty` builder.
 *
 * **Memory:** The caller owns the returned handle and must free it with
 * `rosu_pp_difficulty_free`.
 */
struct rosu_pp_DifficultyHandle *rosu_pp_difficulty_new(void);

/**
 * Clone a difficulty calculator handle.
 *
 * Creates an independent copy of the difficulty builder, including all
 * configured settings (mods, passed_objects, clock_rate, attribute overrides, etc.).
 *
 * **Parameters:**
 * - `handle`: A valid `DifficultyHandle` pointer (must not be null).
 *
 * **Returns:** A new handle on success, or `NULL` if `handle` is null.
 *
 * **Memory:** The caller owns the returned handle and must free it with
 * `rosu_pp_difficulty_free`. The original `handle` remains valid.
 */
struct rosu_pp_DifficultyHandle *rosu_pp_difficulty_clone(const struct rosu_pp_DifficultyHandle *handle);

/**
 * Set the game mods for the difficulty calculation.
 *
 * **Parameters:**
 * - `handle`: A valid `DifficultyHandle` pointer (must not be null).
 * - `mods`: A `ModsHandle` pointer containing the mods to apply (may be null
 *   to clear mods, though this is equivalent to not setting any).
 *
 * **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
 * `handle` is null.
 *
 * **Handle reuse:** The `handle` remains valid after this call.
 */
enum rosu_pp_FfiResult rosu_pp_difficulty_mods(struct rosu_pp_DifficultyHandle *handle,
                                               const struct rosu_pp_ModsHandle *mods);

/**
 * Calculate difficulty attributes for the configured settings.
 *
 * **Parameters:**
 * - `handle`: A valid `DifficultyHandle` pointer. **Consumed** by this call.
 *   The handle must NOT be used or freed after this call.
 * - `map`: A valid `BeatmapHandle` pointer (must not be null).
 * - `out`: Pointer to a `DifficultyAttributes` struct where results will be written.
 *   (must not be null).
 *
 * **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
 * `handle`, `map`, or `out` is null.
 *
 * **Ownership:** This function **consumes** the `handle`. The caller must NOT
 * call `rosu_pp_difficulty_free` on the handle, nor use it after this call.
 */
enum rosu_pp_FfiResult rosu_pp_difficulty_calculate(struct rosu_pp_DifficultyHandle *handle,
                                                    const struct rosu_pp_BeatmapHandle *map,
                                                    struct rosu_pp_DifficultyAttributes *out);

/**
 * Free a difficulty calculator handle.
 *
 * **Parameters:**
 * - `handle`: A handle returned by `rosu_pp_difficulty_new` or
 *   `rosu_pp_difficulty_clone`. May be null (null is a no-op).
 *
 * **Note:** Do NOT call this function if the handle was passed to
 * `rosu_pp_difficulty_calculate` — that function consumes the handle.
 */
void rosu_pp_difficulty_free(struct rosu_pp_DifficultyHandle *handle);

/**
 * Create a new gradual performance calculator.
 *
 * **Parameters:**
 * - `difficulty`: A `DifficultyHandle` pointer. **Consumed** by this call.
 *   The caller must NOT use or free this handle afterward.
 * - `map`: A valid `BeatmapHandle` pointer (must not be null).
 *
 * **Returns:** A non-null handle on success, or `NULL` if either pointer is null.
 *
 * **Ownership:** This function **consumes** the `difficulty` handle. The caller
 * must NOT call `rosu_pp_difficulty_free` on the difficulty handle, nor use it
 * after this call. The `map` handle is only borrowed and remains valid.
 *
 * **Memory:** The caller owns the returned handle and must free it with
 * `rosu_pp_gradual_performance_free`.
 */
struct rosu_pp_GradualPerformanceHandle *rosu_pp_gradual_performance_new(struct rosu_pp_DifficultyHandle *difficulty,
                                                                         const struct rosu_pp_BeatmapHandle *map);

/**
 * Process the next hit object and return incremental performance attributes.
 *
 * Call this function repeatedly, passing the score state for each hit object
 * in order, until it returns `FfiResult::Done` (all objects processed).
 *
 * **Parameters:**
 * - `handle`: A valid `GradualPerformanceHandle` pointer (must not be null).
 * - `state`: A reference to a `ScoreState` struct describing the current hit.
 * - `out`: Pointer to a `PerformanceAttributes` struct where results will be
 *   written (must not be null).
 *
 * **Returns:**
 * - `FfiResult::Ok` — More objects remain; call `next` again.
 * - `FfiResult::Done` — All objects have been processed. No more calls needed.
 * - `FfiResult::NullPointer` — `handle` or `out` is null.
 *
 * **Handle reuse:** The `handle` remains valid after `Ok` and can be used for
 * subsequent calls.
 */
enum rosu_pp_FfiResult rosu_pp_gradual_performance_next(struct rosu_pp_GradualPerformanceHandle *handle,
                                                        const struct rosu_pp_ScoreState *state,
                                                        struct rosu_pp_PerformanceAttributes *out);

/**
 * Free a gradual performance calculator handle.
 *
 * **Parameters:**
 * - `handle`: A handle returned by `rosu_pp_gradual_performance_new`. May be
 *   null (null is a no-op).
 */
void rosu_pp_gradual_performance_free(struct rosu_pp_GradualPerformanceHandle *handle);

/**
 * Convert a game mode to its string representation.
 *
 * **Parameters:**
 * - `mode`: A `GameMode` value.
 *
 * **Returns:** A pointer to a static null-terminated string:
 * - `Osu` -> `"osu"`
 * - `Taiko` -> `"taiko"`
 * - `Catch` -> `"catch"`
 * - `Mania` -> `"mania"`
 *
 * **Memory:** The returned pointer points to static data and does NOT need
 * to be freed.
 */
const char *rosu_pp_mode_to_str(enum rosu_pp_GameMode mode);

/**
 * Convert a string to a game mode.
 *
 * Accepts multiple string aliases for each mode:
 * - **osu!:** `"osu"`, `"std"`, `"0"`
 * - **taiko:** `"taiko"`, `"tko"`, `"1"`
 * - **catch:** `"catch"`, `"ctb"`, `"fruits"`, `"2"`
 * - **mania:** `"mania"`, `"mna"`, `"3"`
 *
 * **Parameters:**
 * - `s`: Null-terminated C string containing the mode name (must not be null).
 * - `out`: Pointer to store the resulting `GameMode` (must not be null).
 *
 * **Returns:** `FfiResult::Ok` on success, `FfiResult::InvalidArgument` if the
 * string doesn't match any known mode, or `FfiResult::NullPointer` if `s` or
 * `out` is null.
 */
enum rosu_pp_FfiResult rosu_pp_mode_from_str(const char *s, enum rosu_pp_GameMode *out);

/**
 * Parse a mod string with an explicit game mode.
 *
 * Parses mods (e.g., `"HDHR"`, `"{acronym: "HDFL","settings":{}}"`) and
 * returns a handle to the resulting mods collection specific to the given
 * game mode.
 *
 * **Parameters:**
 * - `s`: Null-terminated C string containing the mod acronyms.
 * - `deny_unknown_fields`: If `true`, parsing fails when unknown mod settings
 *   are encountered. If `false`, unknown settings are silently ignored.
 * - `mode`: The game mode to parse mods for (osu!, taiko, catch, or mania).
 * - `out`: Pointer to store the resulting `ModsHandle`.
 *
 * **Returns:** `FfiResult::Ok` on success, or `FfiResult::ParseError` if the
 * string could not be parsed, or `FfiResult::NullPointer` if `s` or `out` is
 * null.
 *
 * **Memory:** The caller owns the handle written to `out` and must free it with
 * `rosu_pp_mods_free`.
 */
enum rosu_pp_FfiResult rosu_pp_mods_parse_with_mode(const char *s,
                                                    bool deny_unknown_fields,
                                                    enum rosu_pp_GameMode mode,
                                                    struct rosu_pp_ModsHandle *out);

/**
 * Parse a mod string with automatic mode detection.
 *
 * Parses mods and infers the game mode from the mod combinations.
 * For example, `"FI"` (FadeIn) implies mania mode since it is mania-specific.
 *
 * **Parameters:**
 * - `s`: Null-terminated C string containing the mod acronyms.
 * - `deny_unknown_fields`: If `true`, parsing fails when unknown mod settings
 *   are encountered. If `false`, unknown settings are silently ignored.
 * - `out`: Pointer to store the resulting `ModsHandle`.
 *
 * **Returns:** `FfiResult::Ok` on success, or `FfiResult::ParseError` if the
 * string could not be parsed, or `FfiResult::NullPointer` if `s` or `out` is null.
 *
 * **Memory:** The caller owns the handle written to `out` and must free it with
 * `rosu_pp_mods_free`.
 */
enum rosu_pp_FfiResult rosu_pp_mods_parse(const char *s,
                                          bool deny_unknown_fields,
                                          struct rosu_pp_ModsHandle *out);

/**
 * Create a mods handle from legacy bitflags.
 *
 * Converts a u32 bitflag representation (as used by the osu! legacy API) into
 * a full mods handle. Unknown bits are silently dropped.
 *
 * **Parameters:**
 * - `bits`: Legacy bitflag value representing the mods.
 *
 * **Returns:** A non-null `ModsHandle` pointer.
 *
 * **Memory:** The caller owns the returned handle and must free it with
 * `rosu_pp_mods_free`.
 */
struct rosu_pp_ModsHandle *rosu_pp_mods_from_bits(uint32_t bits);

/**
 * Convert a mods handle to legacy bitflags.
 *
 * **Parameters:**
 * - `mods`: A valid `ModsHandle` pointer (must not be null).
 *
 * **Returns:** A u32 bitflag value representing the mods, or 0 if `mods` is null.
 */
uint32_t rosu_pp_mods_to_bits(const struct rosu_pp_ModsHandle *mods);

/**
 * Convert a mods handle to a string representation.
 *
 * Returns the mod acronyms as a string (e.g., `"HDHRDT"`).
 *
 * **Parameters:**
 * - `mods`: A valid `ModsHandle` pointer (must not be null).
 *
 * **Returns:** A null-terminated C string on success, or `NULL` if `mods` is null.
 *
 * **Memory:** The caller **owns** the returned string and must free it using
 * `rosu_pp_mods_free_string`. Do NOT use standard `free()` on this pointer.
 */
char *rosu_pp_mods_to_string(const struct rosu_pp_ModsHandle *mods);

/**
 * Free a string returned by `rosu_pp_mods_to_string`.
 *
 * **Parameters:**
 * - `s`: A string returned by `rosu_pp_mods_to_string`. May be null (null is
 *   a no-op).
 *
 * **Note:** This is the ONLY correct way to free strings from `mods_to_string`.
 * Do NOT use standard C `free()` on this pointer.
 */
void rosu_pp_mods_free_string(char *s);

/**
 * Free a mods handle and release its memory.
 *
 * **Parameters:**
 * - `handle`: A handle returned by `rosu_pp_mods_parse`,
 *   `rosu_pp_mods_parse_with_mode`, or `rosu_pp_mods_from_bits`.
 *   May be null (null is a no-op).
 */
void rosu_pp_mods_free(struct rosu_pp_ModsHandle *handle);

/**
 * Create a new performance calculator for the given beatmap.
 *
 * **Parameters:**
 * - `map`: A valid `BeatmapHandle` pointer (must not be null).
 *
 * **Returns:** A non-null handle on success, or `NULL` if `map` is null.
 *
 * **Memory:** The caller owns the returned handle and must free it with
 * `rosu_pp_performance_free`. The `map` handle must remain valid for the
 * lifetime of this `PerformanceHandle` (since it borrows the beatmap data).
 */
struct rosu_pp_PerformanceHandle *rosu_pp_performance_new(const struct rosu_pp_BeatmapHandle *map);

/**
 * Set the game mods for the performance calculation.
 *
 * **Parameters:**
 * - `handle`: A valid `PerformanceHandle` pointer (must not be null).
 * - `mods`: A `ModsHandle` pointer containing the mods to apply.
 *
 * **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
 * `handle` is null.
 *
 * **Handle reuse:** The `handle` remains valid after this call.
 */
enum rosu_pp_FfiResult rosu_pp_performance_mods(struct rosu_pp_PerformanceHandle *handle,
                                                const struct rosu_pp_ModsHandle *mods);

/**
 * Set the full score state at once.
 *
 * This is an alternative to setting individual hit counts (n300, n100, etc.)
 * and combo. Use this when you have a complete `ScoreState` struct.
 *
 * **Parameters:**
 * - `handle`: A valid `PerformanceHandle` pointer.
 * - `state`: A reference to a `ScoreState` struct with the score data.
 *
 * **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
 * `handle` is null.
 */
enum rosu_pp_FfiResult rosu_pp_performance_state(struct rosu_pp_PerformanceHandle *handle,
                                                 const struct rosu_pp_ScoreState *state);

/**
 * Calculate performance attributes for the configured settings.
 *
 * **Parameters:**
 * - `handle`: A valid `PerformanceHandle` pointer. **Consumed** by this call.
 *   The handle must NOT be used or freed after this call.
 * - `out`: Pointer to a `PerformanceAttributes` struct where results will be
 *   written (must not be null).
 *
 * **Returns:** `FfiResult::Ok` on success, or `FfiResult::NullPointer` if
 * `handle` or `out` is null.
 *
 * **Ownership:** This function **consumes** the `handle`. The caller must NOT
 * call `rosu_pp_performance_free` on the handle, nor use it after this call.
 */
enum rosu_pp_FfiResult rosu_pp_performance_calculate(struct rosu_pp_PerformanceHandle *handle,
                                                     struct rosu_pp_PerformanceAttributes *out);

/**
 * Free a performance calculator handle.
 *
 * **Parameters:**
 * - `handle`: A handle returned by `rosu_pp_performance_new`. May be null
 *   (null is a no-op).
 *
 * **Note:** Do NOT call this function if the handle was passed to
 * `rosu_pp_performance_calculate` — that function consumes the handle.
 */
void rosu_pp_performance_free(struct rosu_pp_PerformanceHandle *handle);

/**
 * Create a new zero-initialized ScoreState.
 *
 * **Returns:** A `ScoreState` struct with all fields set to zero/false.
 *
 * Initialize the returned struct with the appropriate hit result counts
 * for the play being evaluated.
 */
struct rosu_pp_ScoreState rosu_pp_score_state_new(void);
