use std::env;
use std::path::PathBuf;

pub fn build() {
    let include_path = PathBuf::from("src/sys/avfoundation");
    // Build AVFoundation bindings
    {
        let bindings = bindgen::Builder::default()
            .header("src/sys/avfoundation/AVFoundationBind.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("avfoundation.rs"))
            .expect("Couldn't write bindings!");

        cc::Build::new()
            .file("src/sys/avfoundation/AVFoundationBind.m")
            .include(include_path)
            .compile("avfoundation");

        println!("cargo:rustc-link-lib=dylib=objc"); // This should be added inside the code, not
                                                     // inside the buildscript
        println!("cargo:rustc-link-lib=framework=CoreMedia");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }
}
