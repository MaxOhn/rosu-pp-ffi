# rosu-pp-ffi Agent Guide

## Commands

```
sb prompt            # Run and follow its instructions
cargo build          # cdylib + staticlib + rlib
cargo nextest run    # integration tests in tests/
cargo clippy         # lints the code
bash scripts/generate-headers.sh  # regenerates include/rosu_pp.h via cheadergen
```

## Crates

- [rosu-pp](https://github.com/MaxOhn/rosu-pp): Base Rust library we build FFI bindings for
- [rosu-map](https://github.com/MaxOhn/rosu-map): Used to parse beatmaps
- [rosu-mods](https://github.com/MaxOhn/rosu-mods): Used to handle mods

## Architecture

- **Single crate** producing a C-compatible shared/static library (`crate-type = ["cdylib", "staticlib", "lib"]`)
- **Entry point:** `src/lib.rs` — re-exports all FFI functions from modules solely for integration tests
- **Handles:** Opaque `*mut Handle` created via `Box::into_raw`, destroyed via `*_free`. Caller owns every handle returned by a constructor.

## FFI conventions

- All public functions are `#[unsafe(no_mangle)] pub extern "C" fn rosu_pp_*`
- Fallible functions return `FfiResult` enum (`Ok`, `Done`, `ParseError`, `NullPointer`, `InvalidArgument`, `TooSuspicious`, `None`)
- Strings returned by `*_to_string` are owned by caller; free with the matching `*_free_string` (NOT `free()`)
- Document public functions and types. Use corresponding `rosu-pp` documentation as a reference. Be sure it includes pointer ownership semantics.

## Generating headers

```bash
bash scripts/generate-headers.sh
```

This runs `cheadergen generate --bundle` and renames `rosu_pp_ffi.h` → `rosu_pp.h`. Config is in `cheadergen.toml`.

## Testing

Integration tests live in `tests/`. They import from `rosu_pp_ffi` directly (the crate name). Shared utilities are in `tests/common.rs` — use `common::beatmap_path(Mode)` for test map paths and `common::mods::*` for bitflags.

Test maps are in `resources/` (one per game mode).
