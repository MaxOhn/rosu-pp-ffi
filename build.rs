use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cbindgen.toml");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let include_dir = PathBuf::from(&crate_dir).join("include");
    let header_file = PathBuf::from(&out_dir).join("rosu_pp.h");

    fs::create_dir_all(&include_dir).unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(&header_file);

    fs::copy(&header_file, include_dir.join("rosu_pp.h")).unwrap();

    println!("cargo:include={}", include_dir.display());
}
