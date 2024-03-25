use std::{env::{temp_dir, var}, fs::{create_dir, write}, path::PathBuf};

use bindgen;
use libpng_src::build_artifact;

const MANUAL_BEGINNING: &str = 
"#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use libc::{FILE, time_t, tm};

";

fn main() {
    let working_dir = temp_dir().join("bindgen-helper");

    let artifact_info = build_artifact("aarch64-apple-darwin", &working_dir).unwrap();
    let png_h_path = artifact_info.include_dir.join("png.h");

    let dest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("src")
        .join("raw_ffi.rs");

    let bindings = bindgen::builder()
        .header(png_h_path.to_string_lossy())
        .allowlist_file(png_h_path.to_string_lossy())
        .blocklist_item("FILE")
        .blocklist_item("time_t")
        .blocklist_item("tm")
        .blocklist_item("fpos_t")
        .blocklist_item("__sFILE")
        .blocklist_item("__sFILEX")
        .blocklist_item("__sbuf")
        .blocklist_item("__darwin_time_t")
        .blocklist_item("__darwin_off_t")
        .blocklist_item("__int64_t")
        .generate()
        .unwrap()
        .to_string();

    write(dest_path, format!("{MANUAL_BEGINNING}{bindings}")).unwrap();
}
