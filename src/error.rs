//! FFI return code enum for error handling.
//!
//! All fallible FFI functions return `FfiResult`. Check the return value
//! before using output parameters or handles.

use std::fmt;

/// Return code for FFI functions that can fail.
///
/// Variants:
/// - `Ok` -- Operation succeeded.
/// - `Done` -- Gradual calculator has processed all objects (only returned by
///   `rosu_pp_gradual_performance_next` and `rosu_pp_gradual_difficulty_next`).
/// - `ParseError` -- Input string could not be parsed (beatmap parsing, mod parsing).
/// - `NullPointer` -- A null pointer was passed where a valid handle was expected.
/// - `InvalidArgument` -- An argument value was out of range or otherwise invalid.
/// - `TooSuspicious` -- The beatmap contains suspicious hit objects that make
///   calculation unreliable (only returned by `checked_*` functions).
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cheadergen::config(rename = "rosu_pp_FfiResult")]
pub enum FfiResult {
    #[cheadergen(rename = "rosu_pp_FfiResult_Ok")]
    Ok = 0,
    #[cheadergen(rename = "rosu_pp_FfiResult_Done")]
    Done = 1,
    #[cheadergen(rename = "rosu_pp_FfiResult_ParseError")]
    ParseError = 2,
    #[cheadergen(rename = "rosu_pp_FfiResult_NullPointer")]
    NullPointer = 3,
    #[cheadergen(rename = "rosu_pp_FfiResult_InvalidArgument")]
    InvalidArgument = 4,
    #[cheadergen(rename = "rosu_pp_FfiResult_TooSuspicious")]
    TooSuspicious = 5,
    #[cheadergen(rename = "rosu_pp_FfiResult_None")]
    None = 6,
}

impl fmt::Display for FfiResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiResult::Ok => f.write_str("Ok"),
            FfiResult::Done => f.write_str("Gradual calculator finished"),
            FfiResult::ParseError => f.write_str("Failed to parse beatmap"),
            FfiResult::NullPointer => f.write_str("Null pointer received"),
            FfiResult::InvalidArgument => f.write_str("Invalid argument"),
            FfiResult::TooSuspicious => f.write_str("Beatmap is too suspicious"),
            FfiResult::None => f.write_str("No value"),
        }
    }
}
