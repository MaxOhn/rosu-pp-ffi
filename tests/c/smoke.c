/**
 * smoke.c — compile-only smoke test for rosu_pp.h
 *
 * This file must compile cleanly with:
 *   cc -std=c99  -Wall -Wextra -Wpedantic -fsyntax-only smoke.c -I../../include
 *   cc -std=c11  -Wall -Wextra -Wpedantic -fsyntax-only smoke.c -I../../include
 *   cc -std=c17  -Wall -Wextra -Wpedantic -fsyntax-only smoke.c -I../../include
 *
 * It does NOT link or run — its only job is to confirm that:
 *   - The header parses without errors or implicit-declaration warnings
 *   - Every exported type is reachable and has the expected shape
 *   - Enum values use the expected prefixed names (rosu_pp_*_*)
 *   - Struct fields exist with the expected types
 *   - Every function signature type-checks at a call site
 *
 * Keep this file free of assertions and runtime logic. Add a new section
 * whenever a new type or function group is added to the public API.
 */

#include "rosu_pp.h"

/* ------------------------------------------------------------------ */
/* 1. Enum values                                                       */
/*    Verifies that enum variants use the expected prefixed names      */
/*    (e.g. rosu_pp_GameMode_Osu, rosu_pp_FfiResult_Ok).              */
/* ------------------------------------------------------------------ */

static void check_enums(void) {
    /* GameMode */
    rosu_pp_GameMode gm;
    gm = rosu_pp_GameMode_Osu;
    gm = rosu_pp_GameMode_Taiko;
    gm = rosu_pp_GameMode_Catch;
    gm = rosu_pp_GameMode_Mania;
    (void)gm;

    /* FfiResult */
    rosu_pp_FfiResult r;
    r = rosu_pp_FfiResult_Ok;
    r = rosu_pp_FfiResult_Done;
    r = rosu_pp_FfiResult_ParseError;
    r = rosu_pp_FfiResult_NullPointer;
    r = rosu_pp_FfiResult_InvalidArgument;
    r = rosu_pp_FfiResult_TooSuspicious;
    r = rosu_pp_FfiResult_None;
    (void)r;
}

/* ------------------------------------------------------------------ */
/* 2. Value types (ScoreState)                                          */
/* ------------------------------------------------------------------ */

static void check_value_types(void) {
    /* ScoreState — the only non-opaque struct */
    rosu_pp_ScoreState ss;
    ss.max_combo              = 0;
    ss.osu_large_tick_hits    = 0;
    ss.osu_small_tick_hits    = 0;
    ss.slider_end_hits        = 0;
    ss.n_geki                 = 0;
    ss.n_katu                 = 0;
    ss.n300                   = 0;
    ss.n100                   = 0;
    ss.n50                    = 0;
    ss.misses                 = 0;
    ss.legacy_total_score     = 0;
    ss.legacy_total_score_valid = 0;
    (void)ss;
}

/* ------------------------------------------------------------------ */
/* 3. Concrete structs                                                  */
/*    Verify that output structs have all expected fields with         */
/*    correct types. These structs are now concrete thanks to          */
/*    #[cheadergen::config(export)] on the Rust side.                  */
/* ------------------------------------------------------------------ */

static void check_concrete_structs(void) {
    /* AdjustedBeatmapAttributes */
    {
        rosu_pp_AdjustedBeatmapAttributes adj;
        adj.ar  = 0.0;
        adj.cs  = 0.0f;
        adj.hp  = 0.0f;
        adj.od  = 0.0;
        (void)adj;
    }

    /* HitWindows */
    {
        rosu_pp_HitWindows hw;
        hw.ar              = 0.0;
        hw.od_perfect      = 0.0;
        hw.od_great        = 0.0;
        hw.od_good         = 0.0;
        hw.od_ok           = 0.0;
        hw.od_meh          = 0.0;
        (void)hw;
    }

    /* StrainsData */
    {
        rosu_pp_StrainsData sd;
        sd.mode                    = 0;
        sd.section_len             = 0.0;
        sd.len                     = 0;
        sd.aim                     = NULL;
        sd.aim_no_sliders          = NULL;
        sd.speed                   = NULL;
        sd.flashlight              = NULL;
        sd.stamina                 = NULL;
        sd.rhythm                  = NULL;
        sd.color                   = NULL;
        sd.reading                 = NULL;
        sd.single_color_stamina    = NULL;
        sd.movement                = NULL;
        sd.strains                 = NULL;
        (void)sd;
    }

    /* DifficultyAttributes */
    {
        rosu_pp_DifficultyAttributes da;
        da.mode                        = 0;
        da.stars                       = 0.0;
        da.max_combo                   = 0;
        da.aim                         = 0.0;
        da.speed                       = 0.0;
        da.flashlight                  = 0.0;
        da.stamina                     = 0.0;
        da.rhythm                      = 0.0;
        da.color                       = 0.0;
        da.reading                     = 0.0;
        da.ar                          = 0.0;
        da.od                          = 0.0;
        da.hp                          = 0.0;
        da.great_hit_window            = 0.0;
        da.ok_hit_window               = 0.0;
        da.meh_hit_window              = 0.0;
        da.n_circles                   = 0;
        da.n_sliders                   = 0;
        da.n_large_ticks               = 0;
        da.n_spinners                  = 0;
        da.n_objects                   = 0;
        da.aim_difficult_slider_count  = 0.0;
        da.slider_factor               = 0.0;
        da.aim_top_weighted_slider_factor = 0.0;
        da.speed_top_weighted_slider_factor = 0.0;
        da.speed_note_count            = 0.0;
        da.aim_difficult_strain_count  = 0.0;
        da.speed_difficult_strain_count = 0.0;
        da.nested_score_per_object     = 0.0;
        da.legacy_score_base_multiplier = 0.0;
        da.maximum_legacy_combo_score  = 0.0;
        da.mono_stamina_factor         = 0.0;
        da.mechanical_difficulty       = 0.0;
        da.consistency_factor          = 0.0;
        da.preempt                     = 0.0;
        da.n_fruits                    = 0;
        da.n_droplets                  = 0;
        da.n_tiny_droplets             = 0;
        da.n_hold_notes                = 0;
        da.is_convert                  = 0;
        (void)da;
    }

    /* PerformanceAttributes */
    {
        rosu_pp_PerformanceAttributes pa;
        pa.pp                          = 0.0;
        pa.pp_acc                      = 0.0;
        pa.pp_aim                      = 0.0;
        pa.pp_speed                    = 0.0;
        pa.pp_flashlight               = 0.0;
        pa.pp_difficulty               = 0.0;
        pa.max_combo                   = 0;
        pa.effective_miss_count        = 0.0;
        pa.speed_deviation             = 0.0;
        pa.combo_based_estimated_miss_count = 0.0;
        pa.score_based_estimated_miss_count = 0.0;
        pa.aim_estimated_slider_breaks = 0.0;
        pa.speed_estimated_slider_breaks = 0.0;
        pa.estimated_unstable_rate     = 0.0;
        pa.difficulty.mode             = 0;
        pa.difficulty.stars            = 0.0;
        pa.difficulty.max_combo        = 0;
        (void)pa;
    }
}

/* ------------------------------------------------------------------ */
/* 4. Opaque types                                                      */
/*    We verify that pointers to opaque handle types work in           */
/*    function calls.                                                  */
/* ------------------------------------------------------------------ */

static void check_opaque_types(void) {
    /* Verify that pointer-to-opaque handle types can be declared and used */
    rosu_pp_BeatmapHandle *beatmap = NULL;
    rosu_pp_DifficultyHandle *diff = NULL;
    rosu_pp_ModsHandle *mods = NULL;
    rosu_pp_PerformanceHandle *perf = NULL;
    rosu_pp_GradualDifficultyHandle *gdiff = NULL;
    rosu_pp_GradualPerformanceHandle *gperf = NULL;
    rosu_pp_InspectDifficultyHandle *inspect = NULL;
    rosu_pp_BeatmapAttributesHandle *battrib = NULL;
    rosu_pp_BeatmapAttributesBuilderHandle *battrib_builder = NULL;

    (void)beatmap; (void)diff; (void)mods; (void)perf;
    (void)gdiff; (void)gperf; (void)inspect; (void)battrib;
    (void)battrib_builder;
}

/* ------------------------------------------------------------------ */
/* 5. Function signatures                                               */
/*    We declare function pointer variables matching every public       */
/*    function's signature. A type mismatch (wrong return type,        */
/*    wrong parameter type) will be caught here.                        */
/* ------------------------------------------------------------------ */

static void check_function_signatures(void) {
    /* Beatmap */
    rosu_pp_FfiResult (*p_from_path)(const char *, rosu_pp_BeatmapHandle **)
        = rosu_pp_beatmap_from_path;
    rosu_pp_FfiResult (*p_from_bytes)(const unsigned char *, size_t, rosu_pp_BeatmapHandle **)
        = rosu_pp_beatmap_from_bytes;
    void (*p_beatmap_free)(rosu_pp_BeatmapHandle *)
        = rosu_pp_beatmap_free;
    int   (*p_version)(const rosu_pp_BeatmapHandle *)  = rosu_pp_beatmap_version;
    rosu_pp_GameMode (*p_bmode)  (const rosu_pp_BeatmapHandle *)  = rosu_pp_beatmap_mode;
    float (*p_ar)     (const rosu_pp_BeatmapHandle *)  = rosu_pp_beatmap_ar;
    float (*p_cs)     (const rosu_pp_BeatmapHandle *)  = rosu_pp_beatmap_cs;
    float (*p_hp)     (const rosu_pp_BeatmapHandle *)  = rosu_pp_beatmap_hp;
    float (*p_od)     (const rosu_pp_BeatmapHandle *)  = rosu_pp_beatmap_od;
    double (*p_bpm)   (const rosu_pp_BeatmapHandle *)  = rosu_pp_beatmap_bpm;
    rosu_pp_FfiResult (*p_suspicious)(const rosu_pp_BeatmapHandle *)
        = rosu_pp_beatmap_check_suspicion;

    /* Mode */
    const char *(*p_to_str)(rosu_pp_GameMode)               = rosu_pp_mode_to_str;
    rosu_pp_FfiResult (*p_from_str)(const char *, rosu_pp_GameMode *) = rosu_pp_mode_from_str;

    /* Mods */
    rosu_pp_FfiResult (*p_mods_acr)(const char *, rosu_pp_ModsHandle **)
        = rosu_pp_mods_from_acronym;
    rosu_pp_FfiResult (*p_mods_json)(const char *, bool, rosu_pp_ModsHandle **)
        = rosu_pp_mods_from_json;
    rosu_pp_FfiResult (*p_mods_json_mode)(const char *, bool, rosu_pp_GameMode,
                                          rosu_pp_ModsHandle **)
        = rosu_pp_mods_from_json_with_mode;
    rosu_pp_ModsHandle *(*p_mods_bits)(unsigned int) = rosu_pp_mods_from_bits;
    unsigned int (*p_mods_to_bits)(const rosu_pp_ModsHandle *) = rosu_pp_mods_to_bits;
    char *(*p_mods_str)(const rosu_pp_ModsHandle *)            = rosu_pp_mods_to_string;
    void (*p_mods_free_str)(char *)                            = rosu_pp_mods_free_string;
    void (*p_mods_free)(rosu_pp_ModsHandle *)                  = rosu_pp_mods_free;

    /* Difficulty */
    rosu_pp_DifficultyHandle *(*p_diff_new)(void)                = rosu_pp_difficulty_new;
    rosu_pp_DifficultyHandle *(*p_diff_clone)(const rosu_pp_DifficultyHandle *)
        = rosu_pp_difficulty_clone;
    rosu_pp_FfiResult (*p_diff_mods)(rosu_pp_DifficultyHandle *, const rosu_pp_ModsHandle *)
        = rosu_pp_difficulty_mods;
    rosu_pp_FfiResult (*p_diff_lazer)(rosu_pp_DifficultyHandle *, bool)
        = rosu_pp_difficulty_lazer;
    rosu_pp_FfiResult (*p_diff_calc)(rosu_pp_DifficultyHandle *,
                                     const rosu_pp_BeatmapHandle *,
                                     rosu_pp_DifficultyAttributes *)
        = rosu_pp_difficulty_calculate;
    rosu_pp_FfiResult (*p_diff_checked)(rosu_pp_DifficultyHandle *,
                                        const rosu_pp_BeatmapHandle *,
                                        rosu_pp_DifficultyAttributes *)
        = rosu_pp_difficulty_checked_calculate;
    rosu_pp_StrainsData *(*p_diff_strains)(rosu_pp_DifficultyHandle *,
                                           const rosu_pp_BeatmapHandle *)
        = rosu_pp_difficulty_strains;
    void (*p_diff_free)(rosu_pp_DifficultyHandle *) = rosu_pp_difficulty_free;

    /* DifficultyInspect */
    rosu_pp_InspectDifficultyHandle *(*p_inspect)(rosu_pp_DifficultyHandle *)
        = rosu_pp_difficulty_inspect_new;
    void (*p_inspect_free)(rosu_pp_InspectDifficultyHandle *)
        = rosu_pp_inspect_difficulty_free;

    /* Performance */
    rosu_pp_PerformanceHandle *(*p_perf_new)(const rosu_pp_BeatmapHandle *)
        = rosu_pp_performance_new;
    rosu_pp_FfiResult (*p_perf_mods)(rosu_pp_PerformanceHandle *, const rosu_pp_ModsHandle *)
        = rosu_pp_performance_mods;
    rosu_pp_FfiResult (*p_perf_calc)(rosu_pp_PerformanceHandle *,
                                     rosu_pp_PerformanceAttributes *)
        = rosu_pp_performance_calculate;
    void (*p_perf_free)(rosu_pp_PerformanceHandle *) = rosu_pp_performance_free;

    /* ScoreState */
    rosu_pp_ScoreState (*p_ss_new)(void)      = rosu_pp_score_state_new;
    unsigned int (*p_ss_hits)(const rosu_pp_ScoreState *, rosu_pp_GameMode)
        = rosu_pp_score_state_total_hits;

    /* Gradual difficulty */
    rosu_pp_GradualDifficultyHandle *(*p_gdiff_new)(rosu_pp_DifficultyHandle *,
                                                    const rosu_pp_BeatmapHandle *)
        = rosu_pp_gradual_difficulty_new;
    rosu_pp_FfiResult (*p_gdiff_next)(rosu_pp_GradualDifficultyHandle *,
                                      rosu_pp_DifficultyAttributes *)
        = rosu_pp_gradual_difficulty_next;
    void (*p_gdiff_free)(rosu_pp_GradualDifficultyHandle *)
        = rosu_pp_gradual_difficulty_free;

    /* Gradual performance */
    rosu_pp_GradualPerformanceHandle *(*p_gperf_new)(rosu_pp_DifficultyHandle *,
                                                     const rosu_pp_BeatmapHandle *)
        = rosu_pp_gradual_performance_new;
    rosu_pp_FfiResult (*p_gperf_next)(rosu_pp_GradualPerformanceHandle *,
                                      const rosu_pp_ScoreState *,
                                      rosu_pp_PerformanceAttributes *)
        = rosu_pp_gradual_performance_next;
    void (*p_gperf_free)(rosu_pp_GradualPerformanceHandle *)
        = rosu_pp_gradual_performance_free;

    /* Strains */
    void (*p_strains_free)(rosu_pp_StrainsData *) = rosu_pp_strains_free;

    /* BeatmapAttributes builder */
    rosu_pp_BeatmapAttributesBuilderHandle *(*p_bab_new)(void)
        = rosu_pp_beatmap_attrs_builder_new;
    rosu_pp_FfiResult (*p_bab_map)(rosu_pp_BeatmapAttributesBuilderHandle *,
                                   const rosu_pp_BeatmapHandle *)
        = rosu_pp_beatmap_attrs_builder_map;
    rosu_pp_BeatmapAttributesHandle *(*p_bab_build)(rosu_pp_BeatmapAttributesBuilderHandle *)
        = rosu_pp_beatmap_attrs_builder_build;
    void (*p_bab_free)(rosu_pp_BeatmapAttributesBuilderHandle *)
        = rosu_pp_beatmap_attrs_builder_free;

    /* BeatmapAttributes getters */
    float (*p_ba_ar)(const rosu_pp_BeatmapAttributesHandle *) = rosu_pp_beatmap_attrs_ar;
    float (*p_ba_od)(const rosu_pp_BeatmapAttributesHandle *) = rosu_pp_beatmap_attrs_od;
    float (*p_ba_cs)(const rosu_pp_BeatmapAttributesHandle *) = rosu_pp_beatmap_attrs_cs;
    float (*p_ba_hp)(const rosu_pp_BeatmapAttributesHandle *) = rosu_pp_beatmap_attrs_hp;
    double (*p_ba_cr)(const rosu_pp_BeatmapAttributesHandle *)
        = rosu_pp_beatmap_attrs_clock_rate;
    rosu_pp_FfiResult (*p_ba_hw)(const rosu_pp_BeatmapAttributesHandle *,
                                 rosu_pp_HitWindows *)
        = rosu_pp_beatmap_attrs_hit_windows;
    rosu_pp_FfiResult (*p_ba_acr)(const rosu_pp_BeatmapAttributesHandle *,
                                  rosu_pp_AdjustedBeatmapAttributes *)
        = rosu_pp_beatmap_attrs_apply_clock_rate;
    void (*p_ba_free)(rosu_pp_BeatmapAttributesHandle *) = rosu_pp_beatmap_attrs_free;

    /* Suppress unused-variable warnings */
    (void)p_from_path; (void)p_from_bytes; (void)p_beatmap_free;
    (void)p_version; (void)p_bmode; (void)p_ar; (void)p_cs;
    (void)p_hp; (void)p_od; (void)p_bpm; (void)p_suspicious;
    (void)p_to_str; (void)p_from_str;
    (void)p_mods_acr; (void)p_mods_json; (void)p_mods_json_mode;
    (void)p_mods_bits; (void)p_mods_to_bits; (void)p_mods_str;
    (void)p_mods_free_str; (void)p_mods_free;
    (void)p_diff_new; (void)p_diff_clone; (void)p_diff_mods;
    (void)p_diff_lazer; (void)p_diff_calc; (void)p_diff_checked;
    (void)p_diff_strains; (void)p_diff_free;
    (void)p_inspect; (void)p_inspect_free;
    (void)p_perf_new; (void)p_perf_mods; (void)p_perf_calc; (void)p_perf_free;
    (void)p_ss_new; (void)p_ss_hits;
    (void)p_gdiff_new; (void)p_gdiff_next; (void)p_gdiff_free;
    (void)p_gperf_new; (void)p_gperf_next; (void)p_gperf_free;
    (void)p_strains_free;
    (void)p_bab_new; (void)p_bab_map; (void)p_bab_build; (void)p_bab_free;
    (void)p_ba_ar; (void)p_ba_od; (void)p_ba_cs; (void)p_ba_hp;
    (void)p_ba_cr; (void)p_ba_hw; (void)p_ba_acr; (void)p_ba_free;
}
