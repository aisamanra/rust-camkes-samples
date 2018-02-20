#![crate_type = "staticlib"]

extern crate sel4_sys;

// This is the camkes entry point for this app
#[no_mangle]
pub extern "C" fn hconn_do_the_thing(n: u32) {
    for b in format!("the thing is done with {}\n", n).bytes() {
        unsafe { sel4_sys::seL4_DebugPutChar(b); }
    }
}
