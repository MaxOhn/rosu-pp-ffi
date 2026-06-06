//! FFI return code enum for error handling.
//!
//! All fallible FFI functions return `FfiResult`. Check the return value
//! before using output parameters or handles.

use std::fmt;

/// Return code for FFI functions that can fail.
///
/// Variants:
/// - `Ok` — Operation succeeded.
/// - `Done` — Gradual calculator has processed all objects (only returned by
///   `rosu_pp_gradual_performance_next`).
/// - `ParseError` — Input string could not be parsed (beatmap parsing, mod parsing).
/// - `NullPointer` — A null pointer was passed where a valid handle was expected.
/// - `InvalidArgument` — An argument value was out of range or otherwise invalid.
#[repr(C)]
pub enum FfiResult {
    Ok = 0,
    Done = 1,
    ParseError = 2,
    NullPointer = 3,
    InvalidArgument = 4,
}

impl fmt::Display for FfiResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiResult::Ok => f.write_str("Ok"),
            FfiResult::Done => f.write_str("Gradual calculator finished"),
            FfiResult::ParseError => f.write_str("Failed to parse beatmap"),
            FfiResult::NullPointer => f.write_str("Null pointer received"),
            FfiResult::InvalidArgument => f.write_str("Invalid argument"),
        }
    }
}
