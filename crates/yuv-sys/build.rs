use cmake;
use std::fs;
use std::{env, path::PathBuf, process::Command};

fn main() {
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libyuv_dir = output_dir.join("libyuv");

    if !libyuv_dir.exists() {
        let status = Command::new("git")
            .current_dir(&output_dir)
            .arg("clone")
            .arg("https://chromium.googlesource.com/libyuv/libyuv")
            .status()
            .unwrap();

        if !status.success() {
            fs::remove_dir_all(&libyuv_dir).unwrap();
            panic!("failed to clone libyuv, is git installed?");
        }
    }

    let dst = cmake::Config::new(&libyuv_dir).build();
    let lib_dir = dst.join("lib");
    let static_lib = lib_dir.join("libyuv.a");

    if !static_lib.exists() {
        panic!("libyuv.a not found");
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=yuv");

    // Generate bindings
    let include_dir = libyuv_dir.join("include");
    let output = bindgen::Builder::default()
        .header(include_dir.join("libyuv.h").to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("yuv.rs");
    output.write_to_file(out_path).unwrap();
}
