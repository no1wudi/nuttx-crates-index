// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#![no_std]

// Declare the C puts function directly
unsafe extern "C" {
    fn puts(s: *const i8) -> i32;
}

/// Entry point for the core_puts example application
///
/// This function demonstrates using libcore (without std) to call puts from libc
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_core_puts_main() {
    // Create a C-compatible string
    let message = b"Hello World from libcore puts\n\0";

    // Call puts function directly with the message pointer
    unsafe {
        puts(message.as_ptr() as *const i8);
    }
}

#[cfg(target_os = "nuttx")]
mod panic {

    use core::panic::PanicInfo;
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}
