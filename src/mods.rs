//! Game mod parsing and manipulation.
//!
//! Provides functions to parse mod strings (e.g., `"HDHRDT"`, `"PFDT+HR"`)
//! and convert between mod representations (bitflags, strings, etc.).

use std::{error, ffi, fmt};

use rosu_mods::{serde::GameModsSeed, GameModsLegacy};
use rosu_pp::GameMods;
use serde::de::{value::StrDeserializer, DeserializeSeed, IntoDeserializer};

use crate::{error::FfiResult, mode::GameMode};

/// Opaque handle to a game mods collection.
///
/// Created via `rosu_pp_mods_parse`, `rosu_pp_mods_parse_with_mode`, or
/// `rosu_pp_mods_from_bits`. Must be freed with `rosu_pp_mods_free`.
#[repr(C)]
pub struct ModsHandle {
    pub(crate) mods: GameMods,
}

fn parse_mods(s: *const ffi::c_char, seed: GameModsSeed, out: *mut ModsHandle) -> FfiResult {
    #[derive(Debug)]
    struct SerdeError;

    impl serde::de::Error for SerdeError {
        fn custom<T: fmt::Display>(_: T) -> Self {
            Self
        }
    }

    impl error::Error for SerdeError {}

    impl fmt::Display for SerdeError {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }

    if s.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let c_str = unsafe { ffi::CStr::from_ptr(s) };

    let Ok(s) = c_str.to_str() else {
        return FfiResult::ParseError;
    };

    let deserializer: StrDeserializer<'_, SerdeError> = s.into_deserializer();

    let Ok(mods) = seed.deserialize(deserializer) else {
        return FfiResult::ParseError;
    };

    unsafe { *out = ModsHandle { mods: mods.into() } };

    FfiResult::Ok
}

/// Parse a mod string with an explicit game mode.
///
/// Parses mod acronyms (e.g., `"HDHR"`, `"DT", "FLHR"`) and returns a handle
/// to the resulting mods collection specific to the given game mode.
///
/// **Parameters:**
/// - `s`: Null-terminated C string containing the mod acronyms.
/// - `deny_unknown_fields`: If `true`, parsing fails when unknown mod acronyms
///   are encountered. If `false`, unknown mods are silently ignored.
/// - `mode`: The game mode to parse mods for (osu!, taiko, catch, or mania).
/// - `out`: Pointer to store the resulting `ModsHandle`.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::ParseError` if the
/// string could not be parsed, or `FfiResult::NullPointer` if `s` or `out` is null.
///
/// **Memory:** The caller owns the handle written to `out` and must free it with
/// `rosu_pp_mods_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_mods_parse_with_mode(
    s: *const ffi::c_char,
    deny_unknown_fields: bool,
    mode: GameMode,
    out: *mut ModsHandle,
) -> FfiResult {
    let seed = GameModsSeed::Mode {
        mode: mode.into(),
        deny_unknown_fields,
    };

    parse_mods(s, seed, out)
}

/// Parse a mod string with automatic mode detection.
///
/// Parses mod acronyms and infers the game mode from the mod combinations.
/// For example, `"FL"` (Flashlight) implies osu! mode since Flashlight is
/// osu!-specific.
///
/// **Parameters:**
/// - `s`: Null-terminated C string containing the mod acronyms.
/// - `deny_unknown_fields`: If `true`, parsing fails when unknown mod acronyms
///   are encountered. If `false`, unknown mods are silently ignored.
/// - `out`: Pointer to store the resulting `ModsHandle`.
///
/// **Returns:** `FfiResult::Ok` on success, or `FfiResult::ParseError` if the
/// string could not be parsed, or `FfiResult::NullPointer` if `s` or `out` is null.
///
/// **Memory:** The caller owns the handle written to `out` and must free it with
/// `rosu_pp_mods_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_mods_parse(
    s: *const ffi::c_char,
    deny_unknown_fields: bool,
    out: *mut ModsHandle,
) -> FfiResult {
    let seed = GameModsSeed::SameModeForEachMod {
        deny_unknown_fields,
    };

    parse_mods(s, seed, out)
}

/// Create a mods handle from legacy bitflags.
///
/// Converts a u32 bitflag representation (as used by the osu! legacy API) into
/// a full mods handle. Unknown bits are silently dropped.
///
/// **Parameters:**
/// - `bits`: Legacy bitflag value representing the mods.
///
/// **Returns:** A non-null `ModsHandle` pointer.
///
/// **Memory:** The caller owns the returned handle and must free it with
/// `rosu_pp_mods_free`.
#[no_mangle]
pub extern "C" fn rosu_pp_mods_from_bits(bits: u32) -> *mut ModsHandle {
    let mods = GameModsLegacy::from_bits(bits);

    Box::into_raw(Box::new(ModsHandle { mods: mods.into() }))
}

/// Convert a mods handle to legacy bitflags.
///
/// **Parameters:**
/// - `mods`: A valid `ModsHandle` pointer (must not be null).
///
/// **Returns:** A u32 bitflag value representing the mods, or 0 if `mods` is null.
#[no_mangle]
pub extern "C" fn rosu_pp_mods_to_bits(mods: *const ModsHandle) -> u32 {
    let mods = unsafe { &*mods };

    match mods.mods {
        GameMods::Lazer(ref mods) => mods.bits(),
        GameMods::Intermode(ref mods) => mods.bits(),
        GameMods::Legacy(ref mods) => mods.bits(),
    }
}

/// Convert a mods handle to a string representation.
///
/// Returns the mod acronyms as a string (e.g., `"HDHRDT"`).
///
/// **Parameters:**
/// - `mods`: A valid `ModsHandle` pointer (must not be null).
///
/// **Returns:** A null-terminated C string on success, or `NULL` if `mods` is null.
///
/// **Memory:** The caller **owns** the returned string and must free it using
/// `rosu_pp_mods_free_string`. Do NOT use standard `free()` on this pointer.
#[no_mangle]
pub extern "C" fn rosu_pp_mods_to_string(mods: *const ModsHandle) -> *mut ffi::c_char {
    let mods = unsafe { &*mods };

    let s = match mods.mods {
        GameMods::Lazer(ref mods) => mods.to_string(),
        GameMods::Intermode(ref mods) => mods.to_string(),
        GameMods::Legacy(ref mods) => mods.to_string(),
    };

    ffi::CString::new(s).unwrap().into_raw()
}

/// Free a string returned by `rosu_pp_mods_to_string`.
///
/// **Parameters:**
/// - `s`: A string returned by `rosu_pp_mods_to_string`. May be null (null is a no-op).
///
/// **Note:** This is the ONLY correct way to free strings from `mods_to_string`.
/// Do NOT use standard C `free()` on this pointer.
#[no_mangle]
pub extern "C" fn rosu_pp_mods_free_string(s: *mut ffi::c_char) {
    if !s.is_null() {
        unsafe { drop(ffi::CString::from_raw(s)) };
    }
}

/// Free a mods handle and release its memory.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_mods_parse`,
///   `rosu_pp_mods_parse_with_mode`, or `rosu_pp_mods_from_bits`.
///   May be null (null is a no-op).
#[no_mangle]
pub extern "C" fn rosu_pp_mods_free(handle: *mut ModsHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
