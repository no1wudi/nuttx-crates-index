// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

//! ASCII utilities demonstration using std::ascii
//! This example demonstrates various features of the ASCII module in Rust's standard library.

/// Demonstrates the AsciiExt trait for ASCII conversion operations
fn ascii_ext_demo() {
    println!("\n== ASCII Conversions ==");

    // Convert to uppercase/lowercase
    let text = "Hello, NuttX!";
    println!("Original: {}", text);
    println!("To uppercase: {}", text.to_ascii_uppercase());
    println!("To lowercase: {}", text.to_ascii_lowercase());

    // Case-insensitive equality
    let a = "Rust";
    let b = "rUsT";
    println!(
        "'{}' eq_ignore_ascii_case '{}': {}",
        a,
        b,
        a.eq_ignore_ascii_case(b)
    );

    // Convert between ASCII and u8
    let ascii_char = 65u8 as char; // ASCII 'A'
    println!("ASCII value 65 as char: {}", ascii_char);
    println!("'A' as ASCII value: {}", 'A' as u8);
}

/// Demonstrates functions to check if characters/strings are ASCII
fn ascii_checks_demo() {
    println!("\n== ASCII Checks ==");

    // Test if a string contains ASCII-only characters
    let ascii_str = "Hello, world!";
    let non_ascii_str = "Hello, 世界!";

    println!("'{}' is_ascii: {}", ascii_str, ascii_str.is_ascii());
    println!("'{}' is_ascii: {}", non_ascii_str, non_ascii_str.is_ascii());

    // Check individual characters
    println!("'A' is_ascii: {}", 'A'.is_ascii());
    println!("'字' is_ascii: {}", '字'.is_ascii());

    // Check ASCII character categories
    let chars = ['a', 'Z', '0', ' ', '\n', '!'];
    for c in chars {
        println!(
            "'{}': alphabetic={}, digit={}, whitespace={}, punctuation={}, control={}",
            c,
            c.is_ascii_alphabetic(),
            c.is_ascii_digit(),
            c.is_ascii_whitespace(),
            c.is_ascii_punctuation(),
            c.is_ascii_control()
        );
    }
}

/// Demonstrates escaping and unescaping ASCII characters
fn escape_demo() {
    println!("\n== ASCII Escape/Unescape ==");

    // Create a string with some control characters
    let original = "Hello\nWorld\tWith\rControl\x1BChars";

    // Escape byte slice
    let escaped = original.as_bytes().escape_ascii().to_string();
    println!("Original: {}", original);
    println!("Escaped: {}", escaped);

    // Working with individual byte escapes
    let special_byte = 0x07u8; // BEL control character
    println!("Byte 0x07 escaped: {}", special_byte.escape_ascii());
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_ascii_main() {
    // Print a header
    println!("=== std::ascii Demo ===");

    // Using AsciiExt trait for conversion
    ascii_ext_demo();

    // Testing ASCII-only characters
    ascii_checks_demo();

    // Escape/unescape ASCII sequences
    escape_demo();

    println!("=== Demo complete ===");
}
