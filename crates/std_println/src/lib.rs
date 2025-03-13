// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

/// Test function that demonstrates various uses of the println! macro in Rust
///
/// This function shows different formatting options available with println!, including:
/// * Basic string printing
/// * Number formatting
/// * Multiple parameter printing
/// * Named parameter usage
///
/// # Safety
///
/// This function is marked as `no_mangle` and is intended to be called from C code.
#[unsafe(no_mangle)]
pub fn rust_crate_test_std_println_main() {
    println!("Hello from Rust!");
    println!("Demonstrating different println formats:");
    println!("Numbers: {}", 42);
    println!("Multiple values: {} and {}", "first", "second");
    println!("Named parameters: {value}", value = "test");

    println!("Float formatting: {:.2}", 3.1415926);
    println!("Debug formatting: {:?}", vec![1, 2, 3]);
    println!("Padding and alignment: |{:>10}|{:<10}|", "right", "left");
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 42, 255, 64);
    println!("With width and fill: {:0>5}", 123);

    println!("Scientific notation: {:e}", 1000000.0);
    println!("Alternative hex: {:#x}", 255);
    println!("Sign display: {:+}", 42);
    println!("Nested formatting: {x:0>width$}", x = 1, width = 5);
    println!(
        "Error display: {:?}",
        Result::<i32, &str>::Err("error message")
    );
    println!("Tuple formatting: {:?}", (10, "hello", true));
    println!("Precision control: {:.1} vs {:.5}", 3.141592, 3.141592);
    println!("Mixed alignment: {:^10}", "center");

    println!("All std_println tests completed successfully!");
}
