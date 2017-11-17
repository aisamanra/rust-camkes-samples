#![feature(core_intrinsics, compiler_builtins_lib)]
#![no_std]
use core::fmt;
use core::fmt::Write;
use core::intrinsics;

extern crate sel4_start;
extern crate sel4_sys;
extern crate compiler_builtins;

pub mod ctypes;

// This imports our camkes.h rust bindings
#[allow(dead_code, non_camel_case_types)]
mod camkes {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

// This is the camkes entry point for this app
#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        camkes::conn_do_the_thing(5);
    }
    0
}
