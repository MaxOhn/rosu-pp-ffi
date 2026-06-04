use std::{error, ffi, fmt};

use rosu_mods::{serde::GameModsSeed, GameModsLegacy};
use rosu_pp::GameMods;
use serde::de::{value::StrDeserializer, DeserializeSeed, IntoDeserializer};

use crate::{error::FfiResult, mode::GameMode};

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

#[no_mangle]
pub extern "C" fn rosu_pp_mods_from_bits(bits: u32) -> *mut ModsHandle {
    let mods = GameModsLegacy::from_bits(bits);

    Box::into_raw(Box::new(ModsHandle { mods: mods.into() }))
}

#[no_mangle]
pub extern "C" fn rosu_pp_mods_to_bits(mods: *const ModsHandle) -> u32 {
    let mods = unsafe { &*mods };

    match mods.mods {
        GameMods::Lazer(ref mods) => mods.bits(),
        GameMods::Intermode(ref mods) => mods.bits(),
        GameMods::Legacy(ref mods) => mods.bits(),
    }
}

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

#[no_mangle]
pub extern "C" fn rosu_pp_mods_free(handle: *mut ModsHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}
