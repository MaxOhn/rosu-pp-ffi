# rosu-pp-ffi

C FFI bindings for [rosu-pp](https://github.com/MaxOhn/rosu-pp), a performance and difficulty calculator for all osu! gamemods.

## Features

Calculates for all four modes:
- **Star ratings** and difficulty attributes
- **Performance points (pp)** and attributes for scores
- **Beatmap attributes** with custom overrides (AR, OD, CS, HP, clock rate, mods)
- **Gradual/per-object attributes** for real-time difficulty & score progression tracking
- **Strain data** for plotting difficulty over time

## C API

All functions are prefixed `rosu_pp_*` and follow a consistent pattern:

1. **Constructors** (e.g. `rosu_pp_beatmap_from_path`) return opaque handles — caller owns them
2. **Setters** (e.g. `rosu_pp_difficulty_mods`) mutate handles in-place — handle remains valid
3. **Calculators** (e.g. `rosu_pp_difficulty_calculate`) write results into output structs
4. **Free functions** (`*_free`) release memory — null is a no-op
5. **Fallible functions** return `rosu_pp_FfiResult` — check before using output parameters

### Ownership Transfer

Some functions **consume** a handle, invalidating the input:
- `rosu_pp_gradual_performance_new` — consumes the difficulty handle
- `rosu_pp_gradual_difficulty_new` — consumes the difficulty handle
- `rosu_pp_difficulty_inspect_new` — consumes the difficulty handle

The consumed handle must NOT be freed or used afterward.

### Memory Management

```c
// 1. Load a beatmap
rosu_pp_BeatmapHandle* map;
rosu_pp_FfiResult res = rosu_pp_beatmap_from_path("map.osu", &map);
if (res != Ok) { /* handle error */ }

// 2. Configure difficulty calculator
rosu_pp_DifficultyHandle* diff = rosu_pp_difficulty_new();
rosu_pp_difficulty_mods(diff, mods);

// 3. Calculate
rosu_pp_DifficultyAttributes attrs;
rosu_pp_difficulty_calculate(diff, map, &attrs);

// 4. Free (order doesn't matter)
rosu_pp_difficulty_free(diff);
rosu_pp_beatmap_free(map);
```

### Strings

Strings returned by `*_to_string` are owned by the caller. Free them with the matching `*_free_string` (NOT `free()`):

```c
char* s = rosu_pp_mods_to_string(mods);
// ... use s ...
rosu_pp_mods_free_string(s);
```
