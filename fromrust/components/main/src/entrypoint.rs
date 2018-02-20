#![crate_type = "staticlib"]

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
