use std::{env::temp_dir, fs::write, path::PathBuf};

use bindgen;
use guess_host_triple::guess_host_triple;
use libpng_src::build_artifact;

const MANUAL_BEGINNING: &str = "#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use libc::{time_t, tm, FILE};

#[allow(unused_imports)]
// Used for linking only
use libz_sys;

";

fn main() {
    let target = guess_host_triple().expect("Cannot detect this hosts target");

    let working_dir = temp_dir().join("bindgen-helper");

    let artifact_info = build_artifact(target, &working_dir).unwrap();
    let png_h_path = artifact_info.include_dir.join("png.h");

    let dest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("src")
        .join("lib.rs");

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
