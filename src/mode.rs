//! Game mode enum and conversion utilities.
//!
//! Provides the `GameMode` enum (osu!, taiko, catch, mania) and functions to
//! convert between string representations and the enum.

use std::ffi;

use rosu_map::section::general::GameMode as RosuMapGameMode;
use rosu_mods::GameMode as RosuModsGameMode;

use crate::error::FfiResult;

/// The four osu! game modes.
///
/// Matches the integer values used by the osu! API:
/// - `Osu` = 0 (osu!standard)
/// - `Taiko` = 1 (osu!taiko)
/// - `Catch` = 2 (osu!catch / fruits)
/// - `Mania` = 3 (osu!mania)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

macro_rules! from {
    ( $ty:ident ) => {
        impl From<$ty> for GameMode {
            fn from(mode: $ty) -> Self {
                match mode {
                    $ty::Osu => GameMode::Osu,
                    $ty::Taiko => GameMode::Taiko,
                    $ty::Catch => GameMode::Catch,
                    $ty::Mania => GameMode::Mania,
                }
            }
        }

        impl From<GameMode> for $ty {
            fn from(mode: GameMode) -> Self {
                match mode {
                    GameMode::Osu => $ty::Osu,
                    GameMode::Taiko => $ty::Taiko,
                    GameMode::Catch => $ty::Catch,
                    GameMode::Mania => $ty::Mania,
                }
            }
        }
    };
}

from!(RosuMapGameMode);
from!(RosuModsGameMode);

/// Convert a game mode to its string representation.
///
/// **Parameters:**
/// - `mode`: A `GameMode` value.
///
/// **Returns:** A pointer to a static null-terminated string:
/// - `Osu` -> `"osu"`
/// - `Taiko` -> `"taiko"`
/// - `Catch` -> `"catch"`
/// - `Mania` -> `"mania"`
///
/// **Memory:** The returned pointer points to static data and does NOT need
/// to be freed.
///
/// # Safety
///
/// This function is safe to call from any context. It takes no raw pointer
/// arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mode_to_str(mode: GameMode) -> *const ffi::c_char {
    let s = match mode {
        GameMode::Osu => c"osu",
        GameMode::Taiko => c"taiko",
        GameMode::Catch => c"catch",
        GameMode::Mania => c"mania",
    };

    s.as_ptr()
}

/// Convert a string to a game mode.
///
/// Accepts multiple string aliases for each mode:
/// - **osu!:** `"osu"`, `"std"`, `"0"`
/// - **taiko:** `"taiko"`, `"tko"`, `"1"`
/// - **catch:** `"catch"`, `"ctb"`, `"fruits"`, `"2"`
/// - **mania:** `"mania"`, `"mna"`, `"3"`
///
/// **Parameters:**
/// - `s`: Null-terminated C string containing the mode name (may be null).
/// - `out`: Pointer to store the resulting `GameMode` (may be null).
///
/// **Returns:** `FfiResult::Ok` on success, `FfiResult::InvalidArgument` if the
/// string doesn't match any known mode, or `FfiResult::NullPointer` if `s` or
/// `out` is null.
///
/// # Safety
///
/// `s` must point to a valid null-terminated UTF-8 string, or be null.
/// `out` must point to a valid `GameMode`, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mode_from_str(
    s: *const ffi::c_char,
    out: *mut GameMode,
) -> FfiResult {
    if s.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let c_str = unsafe { ffi::CStr::from_ptr(s) };

    let Ok(s) = c_str.to_str() else {
        return FfiResult::ParseError;
    };

    let mode = match s {
        "osu" | "std" | "0" => GameMode::Osu,
        "taiko" | "tko" | "1" => GameMode::Taiko,
        "catch" | "ctb" | "fruits" | "2" => GameMode::Catch,
        "mania" | "mna" | "3" => GameMode::Mania,
        _ => return FfiResult::InvalidArgument,
    };

    unsafe { *out = mode };

    FfiResult::Ok
}
