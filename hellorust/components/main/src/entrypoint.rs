#![crate_type = "staticlib"]
extern crate sel4_sys;

fn fact(n: u32) -> u32 {
    if n <= 0 {
        1
    } else {
        n * fact(n-1)
    }
}

// This is the camkes entry point for this app
#[no_mangle]
pub extern "C" fn run() -> isize {
    for c in format!("fact(5) = {}\n", fact(5)).bytes() {
        unsafe { sel4_sys::seL4_DebugPutChar(c); }
    }
    0
}
