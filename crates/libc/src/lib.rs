// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

extern crate libc;

use core::ffi::c_int;
use libc::{O_CREAT, O_WRONLY, close, free, malloc, open, strcpy, strlen, time, time_t, write};
use libc::{c_char, getpid, printf};
use std::ffi::CString;

/// Example functionality demonstrating the use of the libc crate.
/// This shows how to call various libc functions from Rust.
#[unsafe(no_mangle)]
pub fn rust_crate_test_libc_main() {
    println!("Starting libc crate example...");

    test_get_process_id();
    test_printf_function();
    test_memory_operations();
    test_string_operations();
    test_file_operations();
    test_time_functions();

    println!("Libc crate example completed!");
}

/// Test function that retrieves and displays the current process ID
fn test_get_process_id() {
    // Get process ID using libc's getpid
    let pid = unsafe { getpid() };
    println!("Current process ID: {}", pid);
}

/// Test function demonstrating the use of printf from libc
fn test_printf_function() {
    // Demonstrate using printf from libc
    let message = CString::new("Hello from libc crate via printf: %d\n").unwrap();
    unsafe {
        printf(message.as_ptr(), 42 as c_int);
    }
}

/// Test function demonstrating memory allocation and deallocation
fn test_memory_operations() {
    println!("Testing memory operations...");

    // Allocate memory using malloc
    let size = 100;
    let ptr = unsafe { malloc(size) };

    // Assert that memory allocation succeeds
    assert!(!ptr.is_null(), "Memory allocation failed!");

    println!("Successfully allocated {} bytes at {:p}", size, ptr);

    // Free the allocated memory
    unsafe { free(ptr) };
    println!("Memory freed");
}

/// Test function demonstrating string operations
fn test_string_operations() {
    println!("Testing string operations...");

    let test_str = CString::new("Hello, libc string functions!").unwrap();

    // Get string length
    let length = unsafe { strlen(test_str.as_ptr()) };
    println!("String length: {}", length);

    // Demonstrate strcpy
    unsafe {
        // Allocate destination buffer
        let dest = malloc(length + 1) as *mut c_char;
        assert!(!dest.is_null(), "Failed to allocate memory for string copy");

        // Copy the string
        strcpy(dest, test_str.as_ptr());

        // Create a Rust string from the C string to print it
        let copied_str = std::ffi::CStr::from_ptr(dest).to_string_lossy();
        println!("Copied string: {}", copied_str);

        // Verify string was copied correctly
        assert_eq!(length, strlen(dest), "String length mismatch after copy");

        // Clean up
        free(dest as *mut libc::c_void);
    }
}

/// Test function demonstrating file operations
fn test_file_operations() {
    println!("Testing file operations...");

    let filename = CString::new("/tmp/libc_test.txt").unwrap();
    let message = CString::new("Written using libc file operations\n").unwrap();

    unsafe {
        // Open file for writing
        let fd = open(filename.as_ptr(), O_WRONLY | O_CREAT, 0o644);
        assert!(fd >= 0, "Failed to open file for writing");

        // Write to file
        let bytes_written = write(
            fd,
            message.as_ptr() as *const libc::c_void,
            message.as_bytes().len(),
        );
        assert!(bytes_written >= 0, "Failed to write to file");
        assert_eq!(
            bytes_written as usize,
            message.as_bytes().len(),
            "Incomplete write to file"
        );

        println!("Wrote {} bytes to file", bytes_written);

        // Close the file
        let close_result = close(fd);
        assert_eq!(close_result, 0, "Failed to close file");

        println!("File closed");
    }
}

/// Test function demonstrating time functions
fn test_time_functions() {
    println!("Testing time functions...");

    unsafe {
        let mut current_time: time_t = 0;

        // Get current time
        let time_result = time(&mut current_time);
        assert!(time_result != -1, "Failed to get current time");

        println!("Current time (seconds since epoch): {}", current_time);

        // Note: ctime usage removed
        println!("Time functions tested successfully");
    }
}
