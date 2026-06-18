//! Game mod parsing and manipulation.
//!
//! Provides functions to parse mod strings (e.g., `"HDHRDT"`,
//! `"{"acronym":"HDFL","settings":{}}"`) and convert between mod
//! representations (bitflags, strings, etc.).

use std::{ffi, ptr, str::FromStr};

use rosu_mods::{GameModsIntermode, GameModsLegacy, serde::GameModsSeed};
use rosu_pp::GameMods;
use serde::de::DeserializeSeed;

use crate::{
    error::FfiResult,
    handle::{HandleOwned, HandleRef},
    mode::GameMode,
};

handle! {
    /// Opaque handle to a game mods collection.
    ///
    /// Created via `rosu_pp_mods_parse`, `rosu_pp_mods_parse_with_mode`, or
    /// `rosu_pp_mods_from_bits`. Must be freed with `rosu_pp_mods_free`.
    #[cheadergen::config(rename = "rosu_pp_ModsHandle")]
    ModsHandle -> GameMods
}

fn write_mods(mods: Option<GameMods>, out: *mut *mut ModsHandle) -> FfiResult {
    match mods {
        Some(mods) => {
            unsafe { *out = Box::into_raw(Box::new(ModsHandle::from(mods))) };

            FfiResult::Ok
        }
        None => {
            unsafe { *out = ptr::null_mut() };

            FfiResult::ParseError
        }
    }
}

/// Parse a mod string from acronyms (e.g., `"HR"`, `"HDhtFLwG"`).
///
/// Parses a concatenated string of mod acronyms and returns a handle to the
/// resulting mods collection.
///
/// **Parameters:**
/// - `s`: Null-terminated C string containing mod acronyms.
/// - `out`: Pointer to store the resulting `ModsHandle`.
///
/// **Returns:** `FfiResult::Ok` on success, `FfiResult::ParseError` if the
/// string is not valid UTF-8, or `FfiResult::NullPointer` if `s` or `out`
/// is null.
///
/// **Memory:** The caller owns the handle written to `out` and must free it
/// with `rosu_pp_mods_free`.
///
/// # Safety
///
/// `s` must point to a valid null-terminated UTF-8 string, or be null.
/// `out` must point to a valid `*mut *mut ModsHandle` capable of receiving the
/// written pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_from_acronym(
    s: *const ffi::c_char,
    out: *mut *mut ModsHandle,
) -> FfiResult {
    if s.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    write_mods(from_acronym(s), out)
}

// Example input: `"HD"`, `"DTFLEZ"`
//
// Note: Parsing itself *always* succeeds. Unknown acronyms are assigned to the
// "UnknownMod" variant
fn from_acronym(s: *const ffi::c_char) -> Option<GameMods> {
    let c_str = unsafe { ffi::CStr::from_ptr(s) };
    let s = c_str.to_str().ok()?;
    let Ok(mods) = <GameModsIntermode as FromStr>::from_str(s);

    Some(GameMods::from(mods))
}

/// Parse a mod string from JSON with an explicit game mode.
///
/// Parses a JSON-encoded mod specification and returns a handle to the
/// resulting mods collection specific to the given game mode.
///
/// Example input:
// - `""HDHRFL""`
/// - `"72"`
/// - `"[{\"acronym\":\"HD\"},{\"acronym\":\"DT\",\"settings\":{\"speed_change\":1.2}}]"`
///
/// **Parameters:**
/// - `s`: Null-terminated C string containing the JSON-encoded mod specification.
/// - `deny_unknown_fields`: If `true`, parsing fails when unknown mod settings
///   are encountered. If `false`, unknown settings are silently ignored.
/// - `mode`: The game mode to parse mods for (osu!, taiko, catch, or mania).
/// - `out`: Pointer to store the resulting `ModsHandle`.
///
/// **Returns:** `FfiResult::Ok` on success, `FfiResult::ParseError` if the
/// string is not valid JSON or could not be parsed, or `FfiResult::NullPointer`
/// if `s` or `out` is null.
///
/// **Memory:** The caller owns the handle written to `out` and must free it with
/// `rosu_pp_mods_free`.
///
/// # Safety
///
/// `s` must point to a valid null-terminated UTF-8 string, or be null.
/// `out` must point to a valid `*mut *mut ModsHandle` capable of receiving the
/// written pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_from_json_with_mode(
    s: *const ffi::c_char,
    deny_unknown_fields: bool,
    mode: GameMode,
    out: *mut *mut ModsHandle,
) -> FfiResult {
    if s.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let seed = GameModsSeed::Mode {
        mode: mode.into(),
        deny_unknown_fields,
    };

    write_mods(from_json(s, seed), out)
}

/// Parse a mod string from JSON with automatic mode detection.
///
/// Parses a JSON-encoded mod specification and infers the game mode from the
/// mod combinations. For example, `"FI"` (FadeIn) implies mania mode since it
/// is mania-specific.
///
/// Example input:
// - `""HDHRFL""`
/// - `"72"`
/// - `"[{\"acronym\":\"HD\"},{\"acronym\":\"DT\",\"settings\":{\"speed_change\":1.2}}]"`
///
/// **Parameters:**
/// - `s`: Null-terminated C string containing the JSON-encoded mod specification.
/// - `deny_unknown_fields`: If `true`, parsing fails when unknown mod settings
///   are encountered. If `false`, unknown settings are silently ignored.
/// - `out`: Pointer to store the resulting `ModsHandle`.
///
/// **Returns:** `FfiResult::Ok` on success, `FfiResult::ParseError` if the
/// string is not valid JSON or could not be parsed, or `FfiResult::NullPointer`
/// if `s` or `out` is null.
///
/// **Memory:** The caller owns the handle written to `out` and must free it with
/// `rosu_pp_mods_free`.
///
/// # Safety
///
/// `s` must point to a valid null-terminated UTF-8 string, or be null.
/// `out` must point to a valid `*mut *mut ModsHandle` capable of receiving the
/// written pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_from_json(
    s: *const ffi::c_char,
    deny_unknown_fields: bool,
    out: *mut *mut ModsHandle,
) -> FfiResult {
    if s.is_null() || out.is_null() {
        return FfiResult::NullPointer;
    }

    let seed = GameModsSeed::SameModeForEachMod {
        deny_unknown_fields,
    };

    write_mods(from_json(s, seed), out)
}

// Example input:
// - `""HDHRFL""`
// - `"72"`
// - `"{"acronym":"EZ"}"`
// - `"{"acronym":"HD",1024,{"acronym":"DT","settings":{"speed_change":1.2}}}"`
fn from_json(s: *const ffi::c_char, seed: GameModsSeed) -> Option<GameMods> {
    let c_str = unsafe { ffi::CStr::from_ptr(s) };
    let s = c_str.to_str().ok()?;
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let mods = seed.deserialize(&mut deserializer).ok()?;

    Some(GameMods::from(mods))
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
///
/// # Safety
///
/// This function is safe to call from any context. It takes no raw pointer
/// arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_from_bits(bits: u32) -> *mut ModsHandle {
    let mods = GameModsLegacy::from_bits(bits);

    Box::into_raw(Box::new(ModsHandle::from(GameMods::from(mods))))
}

/// Convert a mods handle to legacy bitflags.
///
/// **Parameters:**
/// - `mods`: A valid `ModsHandle` pointer (may be null).
///
/// **Returns:** A u32 bitflag value representing the mods, or 0 if `mods` is null.
///
/// # Safety
///
/// `mods` must be a valid pointer to a `ModsHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_to_bits(mods: *const ModsHandle) -> u32 {
    match mods.checked_by_ref() {
        Some(GameMods::Lazer(mods)) => mods.bits(),
        Some(GameMods::Intermode(mods)) => mods.bits(),
        Some(GameMods::Legacy(mods)) => mods.bits(),
        None => 0,
    }
}

/// Convert a mods handle to a string representation.
///
/// Returns the mod acronyms as a string (e.g., `"HDHRDT"`).
///
/// **Parameters:**
/// - `mods`: A valid `ModsHandle` pointer (may be null).
///
/// **Returns:** A null-terminated C string on success, or `NULL` if `mods` is null.
///
/// **Memory:** The caller **owns** the returned string and must free it using
/// `rosu_pp_mods_free_string`. Do NOT use standard `free()` on this pointer.
///
/// # Safety
///
/// `mods` must be a valid pointer to a `ModsHandle`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_to_string(mods: *const ModsHandle) -> *mut ffi::c_char {
    if mods.is_null() {
        return ptr::null_mut();
    }

    let s = match mods.by_ref() {
        GameMods::Lazer(mods) => mods.to_string(),
        GameMods::Intermode(mods) => mods.to_string(),
        GameMods::Legacy(mods) => mods.to_string(),
    };

    ffi::CString::new(s).unwrap().into_raw()
}

/// Free a string returned by `rosu_pp_mods_to_string`.
///
/// **Parameters:**
/// - `s`: A string returned by `rosu_pp_mods_to_string`. May be null (null is
///   a no-op).
///
/// **Note:** This is the ONLY correct way to free strings from `mods_to_string`.
/// Do NOT use standard C `free()` on this pointer.
///
/// # Safety
///
/// `s` must be a null pointer, or a string previously returned by
/// `rosu_pp_mods_to_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_free_string(s: *mut ffi::c_char) {
    if !s.is_null() {
        unsafe { drop(ffi::CString::from_raw(s)) };
    }
}

/// Free a mods handle and release its memory.
///
/// **Parameters:**
/// - `handle`: A handle returned by `rosu_pp_mods_from_acronym`,
///   `rosu_pp_mods_from_json`, `rosu_pp_mods_from_json_with_mode`, or
///   `rosu_pp_mods_from_bits`. May be null (null is a no-op).
///
/// # Safety
///
/// `handle` must be a null pointer, or a valid handle previously returned by
/// a mods constructor function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rosu_pp_mods_free(handle: *mut ModsHandle) {
    handle.drop_handle();
}
