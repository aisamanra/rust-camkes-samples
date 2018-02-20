extern crate bindgen;

use std::default::Default;
use std::env;
use std::path::PathBuf;

fn main() {

    // Setup build and stage paths from global env variables provided by kbuild
    let build_dir = PathBuf::from(&env::var("BUILD_DIR").expect("BUILD_DIR env var"));
    let stage_dir = PathBuf::from(&env::var("STAGE_DIR").expect("STAGE_DIR env var"));
    let out_dir = PathBuf::from(&env::var("OUT_DIR").expect("OUT_DIR env var"));

    bindgen::Builder::default()
        .header(format!("{}", build_dir.join("include/main_object/generated/camkes.h").display()))
        .clang_args(&[
            format!("-I{}", build_dir.join("include/main_object/generated").display()),
            format!("-I{}", stage_dir.join("include").display()),
        ])
        .emit_builtins()
        .blacklist_type("seL4_CapRights.*")
        .generate()
        .unwrap()
        .write_to_file(
            out_dir.join("generated.rs").to_str().unwrap()
        ).unwrap();
}
