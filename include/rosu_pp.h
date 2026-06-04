#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum rosu_pp_FfiResult {
    rosu_pp_FfiResult_Ok = 0,
    rosu_pp_FfiResult_Done = 1,
    rosu_pp_FfiResult_ParseError = 2,
    rosu_pp_FfiResult_NullPointer = 3,
    rosu_pp_FfiResult_InvalidArgument = 4,
} rosu_pp_FfiResult;

typedef enum rosu_pp_GameMode {
    rosu_pp_GameMode_Osu = 0,
    rosu_pp_GameMode_Taiko = 1,
    rosu_pp_GameMode_Catch = 2,
    rosu_pp_GameMode_Mania = 3,
} rosu_pp_GameMode;

typedef struct rosu_pp_BeatmapHandle {
    rosu_pp_Beatmap beatmap;
} rosu_pp_BeatmapHandle;

typedef struct rosu_pp_DifficultyHandle {
    rosu_pp_Difficulty difficulty;
} rosu_pp_DifficultyHandle;

typedef struct rosu_pp_ModsHandle {
    rosu_pp_GameMods mods;
} rosu_pp_ModsHandle;

typedef struct rosu_pp_GradualPerformanceHandle {
    rosu_pp_RosuGradualPerformance gradual;
} rosu_pp_GradualPerformanceHandle;

typedef struct rosu_pp_ScoreState {
    uint32_t max_combo;
    uint32_t osu_large_tick_hits;
    uint32_t osu_small_tick_hits;
    uint32_t slider_end_hits;
    uint32_t n_geki;
    uint32_t n_katu;
    uint32_t n300;
    uint32_t n100;
    uint32_t n50;
    uint32_t misses;
    uint32_t legacy_total_score;
    bool legacy_total_score_valid;
} rosu_pp_ScoreState;

typedef struct rosu_pp_DifficultyAttributes {
    int32_t mode;
    double stars;
    uint32_t max_combo;
    double aim;
    double speed;
    double flashlight;
    double stamina;
    double rhythm;
    double color;
    double reading;
    double ar;
    double od;
    double hp;
    double great_hit_window;
    double ok_hit_window;
    double meh_hit_window;
    uint32_t n_circles;
    uint32_t n_sliders;
    uint32_t n_large_ticks;
    uint32_t n_spinners;
    uint32_t n_objects;
    double aim_difficult_slider_count;
    double slider_factor;
    double aim_top_weighted_slider_factor;
    double speed_top_weighted_slider_factor;
    double speed_note_count;
    double aim_difficult_strain_count;
    double speed_difficult_strain_count;
    double nested_score_per_object;
    double legacy_score_base_multiplier;
    double maximum_legacy_combo_score;
    double mono_stamina_factor;
    double mechanical_difficulty;
    double consistency_factor;
    double preempt;
    uint32_t n_fruits;
    uint32_t n_droplets;
    uint32_t n_tiny_droplets;
    uint32_t n_hold_notes;
    bool is_convert;
} rosu_pp_DifficultyAttributes;

typedef struct rosu_pp_PerformanceAttributes {
    double pp;
    double pp_acc;
    double pp_aim;
    double pp_speed;
    double pp_flashlight;
    double pp_difficulty;
    uint32_t max_combo;
    double effective_miss_count;
    double speed_deviation;
    double combo_based_estimated_miss_count;
    double score_based_estimated_miss_count;
    double aim_estimated_slider_breaks;
    double speed_estimated_slider_breaks;
    double estimated_unstable_rate;
    struct rosu_pp_DifficultyAttributes difficulty;
} rosu_pp_PerformanceAttributes;

typedef struct rosu_pp_PerformanceHandle {
    rosu_pp_Performance performance;
} rosu_pp_PerformanceHandle;

struct rosu_pp_BeatmapHandle *rosu_pp_beatmap_from_path(const char *path);

struct rosu_pp_BeatmapHandle *rosu_pp_beatmap_from_bytes(const uint8_t *bytes, size_t len);

void rosu_pp_beatmap_free(struct rosu_pp_BeatmapHandle *handle);

struct rosu_pp_DifficultyHandle *rosu_pp_difficulty_new(void);

struct rosu_pp_DifficultyHandle *rosu_pp_difficulty_clone(const struct rosu_pp_DifficultyHandle *handle);

enum rosu_pp_FfiResult rosu_pp_difficulty_mods(struct rosu_pp_DifficultyHandle *handle,
                                               const struct rosu_pp_ModsHandle *mods);

void rosu_pp_difficulty_free(struct rosu_pp_DifficultyHandle *handle);

struct rosu_pp_GradualPerformanceHandle *rosu_pp_gradual_performance_new(struct rosu_pp_DifficultyHandle *difficulty,
                                                                         const struct rosu_pp_BeatmapHandle *map);

enum rosu_pp_FfiResult rosu_pp_gradual_performance_next(struct rosu_pp_GradualPerformanceHandle *handle,
                                                        const struct rosu_pp_ScoreState *state,
                                                        struct rosu_pp_PerformanceAttributes *out);

void rosu_pp_gradual_performance_free(struct rosu_pp_GradualPerformanceHandle *handle);

const char *rosu_pp_mode_to_str(enum rosu_pp_GameMode mode);

enum rosu_pp_FfiResult rosu_pp_mode_from_str(const char *s, enum rosu_pp_GameMode *out);

enum rosu_pp_FfiResult rosu_pp_mods_parse_with_mode(const char *s,
                                                    bool deny_unknown_fields,
                                                    enum rosu_pp_GameMode mode,
                                                    struct rosu_pp_ModsHandle *out);

enum rosu_pp_FfiResult rosu_pp_mods_parse(const char *s,
                                          bool deny_unknown_fields,
                                          struct rosu_pp_ModsHandle *out);

struct rosu_pp_ModsHandle *rosu_pp_mods_from_bits(uint32_t bits);

uint32_t rosu_pp_mods_to_bits(const struct rosu_pp_ModsHandle *mods);

char *rosu_pp_mods_to_string(const struct rosu_pp_ModsHandle *mods);

void rosu_pp_mods_free(struct rosu_pp_ModsHandle *handle);

struct rosu_pp_PerformanceHandle *rosu_pp_performance_new(const struct rosu_pp_BeatmapHandle *map);

enum rosu_pp_FfiResult rosu_pp_performance_mods(struct rosu_pp_PerformanceHandle *handle,
                                                const struct rosu_pp_ModsHandle *mods);

enum rosu_pp_FfiResult rosu_pp_performance_state(struct rosu_pp_PerformanceHandle *handle,
                                                 const struct rosu_pp_ScoreState *state);

enum rosu_pp_FfiResult rosu_pp_performance_calculate(struct rosu_pp_PerformanceHandle *handle,
                                                     struct rosu_pp_PerformanceAttributes *out);

void rosu_pp_performance_free(struct rosu_pp_PerformanceHandle *handle);

struct rosu_pp_ScoreState rosu_pp_score_state_new(void);
