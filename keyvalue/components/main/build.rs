// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//


extern crate bindgen;

use std::default::Default;
use std::env;
use std::path::{PathBuf};

/**
 * This build.rs script is run before the rust source is compiled and we use it to
 * generate rust bindings of the camkes.h symbols so that we can call them easier.
 *
 * The generated file can be found in (Must compile at least once):
 *   target/{target}/{debug|release}/build/{library or binary name}/out/generated.rs
 *
 * (note: The camkes.h file that gets used can be found at:
 *   build/{arm/imx31|or another target}/keyvalue/include/main_object/generated/camkes.h)
 */
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
