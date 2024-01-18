use cc;
use std::fs;
use std::process::ExitStatus;
use std::{env, path::PathBuf, process::Command};

const LIBYUV_REPO: &str = "https://chromium.googlesource.com/libyuv/libyuv";
const LIBYUV_COMMIT: &str = "af6ac82";
const FNC_PREFIX: &str = "rs_";

fn run_git_cmd(current_dir: &PathBuf, args: &[&str]) -> ExitStatus {
    Command::new("git")
        .current_dir(current_dir)
        .args(args)
        .status()
        .unwrap()
}

fn rename_symbols(
    fnc_list: &[&str],
    include_files: &[fs::DirEntry],
    source_files: &[fs::DirEntry],
) {
    // Find all occurences of the function in every header and source files
    // and prefix it with FNC_PREFIX
    for line in fnc_list {
        let fnc = line.trim();
        if fnc.is_empty() {
            continue;
        }

        let new_name = format!("{}{}", FNC_PREFIX, fnc);
        for file in include_files {
            let path = file.path();
            let content = fs::read_to_string(&path).unwrap();
            let new_content = content.replace(fnc, &new_name);
            fs::write(&path, new_content).unwrap();
        }

        for file in source_files {
            let path = file.path();
            let content = fs::read_to_string(&path).unwrap();
            let new_content = content.replace(fnc, &new_name);
            fs::write(&path, new_content).unwrap();
        }
    }
}

fn clone_if_needed(output_dir: &PathBuf, libyuv_dir: &PathBuf) -> bool {
    if libyuv_dir.exists() {
        return false; // Already cloned
    }

    let status = run_git_cmd(output_dir, &["clone", LIBYUV_REPO]);
    if !status.success() {
        fs::remove_dir_all(&libyuv_dir).unwrap();
        panic!("failed to clone libyuv, is git installed?");
    }

    let status = run_git_cmd(&libyuv_dir, &["checkout", LIBYUV_COMMIT]);
    if !status.success() {
        fs::remove_dir_all(&libyuv_dir).unwrap();
        panic!("failed to checkout to {}", LIBYUV_COMMIT);
    }

    true
}

fn main() {
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libyuv_dir = output_dir.join("libyuv");
    let include_dir = libyuv_dir.join("include");
    let source_dir = libyuv_dir.join("source");

    let cloned = clone_if_needed(&output_dir, &libyuv_dir);

    let include_files = fs::read_dir(include_dir.join("libyuv"))
        .unwrap()
        .map(Result::unwrap)
        .filter(|f| f.path().extension().unwrap() == "h")
        .collect::<Vec<_>>();

    let source_files = fs::read_dir(source_dir)
        .unwrap()
        .map(Result::unwrap)
        .filter(|f| f.path().extension().unwrap() == "cc")
        .collect::<Vec<_>>();

    let fnc_content = fs::read_to_string("yuv_functions.txt").unwrap();
    let fnc_list = fnc_content.lines().collect::<Vec<_>>();

    if cloned {
        // Rename symbols to avoid conflicts with other libraries
        // that have libyuv statically linked (e.g libwebrtc).
        rename_symbols(&fnc_list, &include_files, &source_files);
    }

    cc::Build::new()
        .warnings(false)
        .include(libyuv_dir.join("include"))
        .files(source_files.iter().map(|f| f.path()))
        .compile("yuv");

    let mut bindgen = bindgen::Builder::default()
        .header(include_dir.join("libyuv.h").to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()));

    for fnc in fnc_list {
        let new_name = format!("{}{}", FNC_PREFIX, fnc);
        bindgen = bindgen.allowlist_function(&new_name);
    }

    let output = bindgen.generate().unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("yuv.rs");
    output.write_to_file(out_path).unwrap();

    println!("cargo:rerun-if-changed=yuv_functions.txt");
}
