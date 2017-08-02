#![feature(core_intrinsics, compiler_builtins_lib)]
#![no_std]
use core::fmt;
use core::fmt::Write;
use core::intrinsics;

extern crate sel4_start;
extern crate sel4_sys;
extern crate compiler_builtins;

struct SeL4Serial;

impl fmt::Write for SeL4Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            unsafe { sel4_sys::seL4_DebugPutChar(b) }
        }
        Ok(())
    }
}

fn fact(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fact(n-1)
    }
}

macro_rules! print {
    ($($arg:tt)*) => (SeL4Serial.write_fmt(format_args!($($arg)*)));
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

// This is the camkes entry point for this app
#[no_mangle]
pub extern "C" fn run() -> isize {
    println!("fact(5) = {}", fact(5));
    0
}
