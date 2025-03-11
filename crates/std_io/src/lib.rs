// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::io::{self, Write};

/// Entry point for the std_io example
/// Demonstrates various IO output methods from Rust's standard library
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_io_main() {
    println!("=== Rust StdIO Example ===");

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

    println!("=== StdIO Example Complete ===");
}
