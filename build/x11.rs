use std::env;
use std::path::PathBuf;

pub fn build() {
    let include_path = PathBuf::from("src/sys/x11");
    // Build X11 bindings
    {
        let bindings = bindgen::Builder::default()
            .header("src/sys/x11/x11bind.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("x11.rs"))
            .expect("Couldn't write bindings!");

        cc::Build::new()
            .file("src/sys/x11/x11bind.c")
            .file("src/sys/x11/window_utils.c")
            .std("c99")
            .include(include_path)
            .compile("x11");

        println!("cargo:rustc-link-lib=dylib=X11");
    }
}
