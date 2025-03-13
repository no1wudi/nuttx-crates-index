// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

/// Example application demonstrating the ryu crate
/// The ryu crate provides fast floating point to string conversion
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_ryu_main() {
    println!("Ryu crate example starting...");

    // Create a buffer and format a floating-point number
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(1.234);
    println!("Formatted output: {}", printed);

    // Verify the output is correct
    assert_eq!(printed, "1.234");

    // Examples with different numbers
    test_format(3.14159);
    test_format(0.000123);
    test_format(42.0);
    test_format(-7.5);

    println!("Ryu crate example completed successfully!");
}

/// Helper function to demonstrate formatting with various numbers
fn test_format(value: f64) {
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(value);
    println!("Value: {} -> Formatted: {}", value, printed);
}
