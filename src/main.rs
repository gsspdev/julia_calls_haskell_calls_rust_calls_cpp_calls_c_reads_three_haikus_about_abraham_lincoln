// #![allow(unsafe_code)]
// extern crate libc;

extern crate libc;
// use libc;

// Declare the C++ functin
extern "C" {
    fn callCFunction();
}

#[no_mangle]
pub extern "C" fn callRustFunction() {
    unsafe {
        callCFunction();
    }
}

fn main() {
    callRustFunction();
}
