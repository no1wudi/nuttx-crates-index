// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use libc::puts;
use std::ffi::CString;

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_libc_puts_main() {
    // Create a C-compatible string with our message
    let message = CString::new("Hello World from libc puts").expect("CString creation failed");

    // Call libc's puts function to print the message
    unsafe {
        puts(message.as_ptr());
    }
}
