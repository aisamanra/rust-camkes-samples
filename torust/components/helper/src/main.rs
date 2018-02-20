#![crate_type = "staticlib"]

extern crate sel4_sys;

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

// This is the camkes entry point for this app
#[no_mangle]
pub extern "C" fn hconn_do_the_thing(n: u32) {
    println!("the thing is done with {}", n);
}
