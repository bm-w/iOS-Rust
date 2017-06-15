//
// lib.rs
// iOS+Rust
//
// Copyright (c) 2017 Bastiaan Marinus van de Weerd. All rights reserved.
//


use std::os::raw::c_char;
use std::ffi::CStr;


#[no_mangle]
pub extern fn test(string: *const c_char) {
	println!("Hello, {}!", unsafe { CStr::from_ptr(string).to_string_lossy().into_owned() });
}
