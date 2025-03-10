// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::println;

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_char_main() {
    println!("=== Demonstrating std::char functionality ===");

    // Demonstrating char::is_alphabetic
    let c = 'a';
    println!("Is '{}' alphabetic? {}", c, c.is_alphabetic());

    // Demonstrating char::is_numeric
    let d = '5';
    println!("Is '{}' numeric? {}", d, d.is_numeric());

    // Demonstrating char::to_uppercase and char::to_lowercase
    let upper = 'A';
    let lower = 'b';
    println!("'{}' to lowercase: {}", upper, upper.to_lowercase());
    println!("'{}' to uppercase: {}", lower, lower.to_uppercase());

    // Demonstrating char::from_u32
    if let Some(c) = std::char::from_u32(65) {
        println!("Character from Unicode point 65: {}", c);
    } else {
        println!("Invalid Unicode code point");
    }

    // Demonstrating char::is_whitespace
    let spaces = [' ', '\t', '\n'];
    for space in spaces {
        println!("Is '{:?}' whitespace? {}", space, space.is_whitespace());
    }

    println!("=== std::char demonstration completed ===");
}
