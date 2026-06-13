//! Shared test utilities for FFI integration tests

#![allow(dead_code, reason = "Not used in all testes")]

use std::{ffi, fs, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
}

impl Mode {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// Absolute paths to test beatmap files
pub fn beatmap_path(mode: Mode) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    match mode {
        Mode::Osu => root.join("resources/2785319.osu"),
        Mode::Taiko => root.join("resources/1028484.osu"),
        Mode::Catch => root.join("resources/2118524.osu"),
        Mode::Mania => root.join("resources/1638954.osu"),
    }
}

pub fn beatmap_bytes(mode: Mode) -> Vec<u8> {
    fs::read(beatmap_path(mode)).expect("failed to read beatmap file")
}

/// Convert a path to a CString for FFI calls
pub fn to_cstring(s: &str) -> ffi::CString {
    ffi::CString::new(s).expect("path contains null byte")
}

/// Mod bitflags (matching rosu-pp tests/common.rs)
pub mod mods {
    pub const NM: u32 = 0;
    pub const NF: u32 = 1 << 0;
    pub const EZ: u32 = 1 << 1;
    pub const TD: u32 = 1 << 2;
    pub const HD: u32 = 1 << 3;
    pub const HR: u32 = 1 << 4;
    pub const DT: u32 = 1 << 6;
    pub const HT: u32 = 1 << 8;
    pub const FL: u32 = 1 << 10;
    pub const SO: u32 = 1 << 12;
}

/// Floating-point comparison helper
pub const FLOAT_EPS: f64 = 1e-5;

#[track_caller]
pub fn assert_float_eq(a: f64, b: f64) {
    let diff = (a - b).abs();
    assert!(
        diff < FLOAT_EPS,
        "{a} != {b} (diff={diff:.2e}, eps={FLOAT_EPS:.2e})"
    );
}

#[track_caller]
pub fn assert_float_eq_f32(a: f32, b: f32) {
    let diff = (a - b).abs();
    assert!(
        diff < FLOAT_EPS as f32,
        "{a} != {b} (diff={diff:.2e}, eps={FLOAT_EPS:.2e})"
    );
}
