use std::fmt;

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
