use std::ffi;

use rosu_map::section::general::GameMode as RosuMapGameMode;
use rosu_mods::GameMode as RosuModsGameMode;

use crate::error::FfiResult;

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

#[no_mangle]
pub extern "C" fn rosu_pp_mode_to_str(mode: GameMode) -> *const ffi::c_char {
    let s = match mode {
        GameMode::Osu => "osu",
        GameMode::Taiko => "taiko",
        GameMode::Catch => "catch",
        GameMode::Mania => "mania",
    };

    s.as_ptr() as *const ffi::c_char
}

#[no_mangle]
pub extern "C" fn rosu_pp_mode_from_str(s: *const ffi::c_char, out: *mut GameMode) -> FfiResult {
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
