#![allow(non_snake_case)] // <-- Kotlin :)

use std::mem;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod window;

/// Converts a Rust string to a Java string
pub fn to_java_string(string: &str) -> *const c_char {
    let cs = CString::new(string.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    // Tell Rust not to clean up the string while we still have a pointer to it.
    // Otherwise, we'll get a segfault.
    mem::forget(cs);
    ptr
}

/// Converts a Java string pointer into a Rust string
pub fn to_rust_string(pointer: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    std::str::from_utf8(slice).unwrap().to_string()
}
