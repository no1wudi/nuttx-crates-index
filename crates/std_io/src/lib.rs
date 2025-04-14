// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::{self, Write};

/// Entry point for the std_io example
/// Demonstrates various IO output methods from Rust's standard library
#[unsafe(no_mangle)]
pub fn rust_crate_test_std_io_main() {
    println!("=== Rust StdIO Example ===");

    // Demonstrate basic IO functionality
    demo_basic_io();

    // Demonstrate io::Error::last_os_error
    demo_last_os_error();

    // Demonstrate custom IO errors
    demo_custom_errors();

    println!("=== StdIO Example Complete ===");
}

/// Demonstrates basic IO operations from std::io
fn demo_basic_io() {
    // Basic println examples
    println!("Hello from Rust std::io!");
    println!("Formatted output: {}, {}, {}", 1, "two", 3.0);

    // Using stdout directly
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "Direct write to stdout using handle").unwrap();

    // Using eprintln for error output
    eprintln!("This is an error message via stderr");

    // Formatted output with different format specifiers
    println!("Decimal: {}, Hex: {:x}, Binary: {:b}", 42, 42, 42);

    // Padding and alignment
    println!("Right-aligned: {:>10}", "text");
    println!("Left-aligned: {:<10}", "text");
    println!("Center-aligned: {:^10}", "text");
    println!("Zero-padded: {:0>5}", "42");
}

/// Demonstrates how to use std::io::Error::last_os_error()
fn demo_last_os_error() {
    println!("\n--- Demonstrating io::Error::last_os_error ---");

    // Try to open a file that doesn't exist
    let file_path = "/this/file/does/not/exist.txt";
    match File::open(file_path) {
        Ok(_) => println!("Successfully opened file (unexpected)"),
        Err(e) => {
            // Print the error we got from the failed operation
            println!("Failed to open file '{}': {}", file_path, e);

            // Get the last OS error and display it
            let os_error = io::Error::last_os_error();
            println!("Last OS error: {:?}", os_error);
            println!("Error kind: {:?}", os_error.kind());
            println!("Error code: {:?}", os_error.raw_os_error());
        }
    }

    println!("--- End of io::Error::last_os_error demo ---\n");
}

/// Demonstrates how to create and work with custom I/O errors
fn demo_custom_errors() {
    println!("\n--- Demonstrating custom io::Error creation ---");

    // Create an error from a string payload
    let string_error = io::Error::new(io::ErrorKind::Other, "custom string error message");
    println!("String error: {}", string_error);
    println!("Error kind: {:?}", string_error.kind());

    // Create an error from another error
    let wrapped_error = io::Error::new(io::ErrorKind::Interrupted, string_error);
    println!("Wrapped error: {}", wrapped_error);
    println!("Wrapped error kind: {:?}", wrapped_error.kind());

    // Create an error without extra payload
    let simple_error = io::Error::from(io::ErrorKind::UnexpectedEof);
    println!("Simple error: {}", simple_error);
    println!("Simple error kind: {:?}", simple_error.kind());

    // Demonstrate error conversion with From trait
    let another_error = io::Error::from(io::ErrorKind::ConnectionRefused);
    println!("From trait error: {}", another_error);

    println!("--- End of custom io::Error demo ---\n");
}
