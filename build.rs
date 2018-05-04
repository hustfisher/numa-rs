extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // pkg_config::Config::new()
    //     .probe("numa")
    //     .expect("should have found numa");

    println!("cargo:rustc-link-lib=numa");

    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .header("wrapper.h")
        .opaque_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
