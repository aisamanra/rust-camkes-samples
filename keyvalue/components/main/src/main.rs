// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//
#![feature(alloc, core_intrinsics, compiler_builtins_lib)]
#![no_std]
use core::fmt;
use core::fmt::Write;
use core::intrinsics;
use alloc::vec::Vec;

extern crate alloc;
extern crate sel4_start;
extern crate sel4_sys;
extern crate compiler_builtins;

pub mod ctypes;

struct SeL4Serial;

impl fmt::Write for SeL4Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            unsafe { sel4_sys::seL4_DebugPutChar(b) }
        }
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => (SeL4Serial.write_fmt(format_args!($($arg)*)));
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

// This imports our camkes.h rust bindings
#[allow(dead_code, non_camel_case_types)]
mod camkes {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

#[no_mangle]
pub extern "C" fn run() -> isize {
    println!("Hello, world!!");

    // Check that when we ask for an invalid key it isn't there.
    assert_eq!{unsafe {camkes::camkes_btreemap_contains_key(43)}, false};
    unsafe { camkes::camkes_btreemap_insert(43, 54) };
    // Check that when we ask for a valid key, it is there.
    assert_eq!{unsafe {camkes::camkes_btreemap_contains_key(43)}, true};

    // Insert some key values.
    unsafe { camkes::camkes_btreemap_insert(44, 55) };
    unsafe { camkes::camkes_btreemap_insert(45, 56) };
    unsafe { camkes::camkes_btreemap_insert(46, 57) };
    unsafe { camkes::camkes_btreemap_insert(47, 58) };

    // Insert a value for a key that we have already used and check we get the old value back.
    let res = unsafe { camkes::camkes_btreemap_insert(43, 59) };
    assert_eq!(res.option_type as usize,
               camkes::Option_C_Some as usize);
    assert_eq!(res.val, 54);

    // Wait for the other app (secondary) to finish inserting keys and values
    unsafe { camkes::barrier_event_wait() };

    // Get a list of keys.
    let res = unsafe { camkes::camkes_btreemap_keys() };

    // Check that the keys are what we expect.
    println!("Keys len: {}", res.len);
    let vec = unsafe {
        Vec::from_raw_parts(camkes::btreemap_buffer as *mut usize,
                            res.len as usize,
                            res.len as usize)
    };
    println!("Keys: {:?}", vec);
    let keys = [1, 2, 3, 4, 5, 43, 44, 45, 46, 47];
    assert_eq!(vec.as_slice(), keys);
    // std::mem::forget(vec);

    // Get a list of values and check that the values are also correct.
    let res = unsafe { camkes::camkes_btreemap_values() };
    let vec = unsafe {
        Vec::from_raw_parts(camkes::btreemap_buffer as *mut usize,
                            res.len as usize,
                            res.len as usize)
    };
    println!("values: {:?}", vec);
    let values = [2, 3, 4, 5, 6, 59, 55, 56, 57, 58];
    assert_eq!(vec.as_slice(), values);
    // std::mem::forget(vec);

    // Test passed finish executing
    println!("main component done");
    0
}
