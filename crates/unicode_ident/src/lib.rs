// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use unicode_ident::{is_xid_continue, is_xid_start};

#[unsafe(no_mangle)]
pub fn rust_crate_test_unicode_ident_main() {
    // Print header
    println!("Unicode Identifier Demo");
    println!("======================");
    println!("This demo shows the unicode-ident crate's functionality for");
    println!("determining if characters are valid in programming identifiers.");
    println!();

    // Test some standard ASCII characters
    let test_chars = vec![
        ('a', "lowercase a"),
        ('Z', "uppercase Z"),
        ('0', "digit 0"),
        ('_', "underscore"),
        ('$', "dollar sign"),
        ('@', "at symbol"),
        ('π', "pi symbol"),
        ('∑', "summation symbol"),
        ('😊', "smiley face emoji"),
    ];

    println!("Testing characters for identifier validity:");
    println!("| Character | Description      | XID Start | XID Continue |");
    println!("|-----------|------------------|-----------|-------------|");

    for (ch, desc) in test_chars {
        let valid_start = is_xid_start(ch);
        let valid_continue = is_xid_continue(ch);
        println!(
            "| {:<9} | {:<16} | {:<9} | {:<11} |",
            format!("'{}'", ch),
            desc,
            if valid_start { "✓" } else { "✗" },
            if valid_continue { "✓" } else { "✗" }
        );
    }

    println!();
    println!("Note: XID Start refers to characters that can appear at the");
    println!("beginning of an identifier, while XID Continue refers to");
    println!("characters that can appear after the first character.");

    // Show a practical example
    println!();
    println!("Practical example - validating a custom identifier:");

    let test_id = "my_var_π_123";
    println!("Testing identifier: '{}'", test_id);

    if let Some(first_char) = test_id.chars().next() {
        if !is_xid_start(first_char) {
            println!(
                "Invalid identifier: first character '{}' cannot start an identifier",
                first_char
            );
        } else {
            let mut valid = true;
            for (i, ch) in test_id.chars().enumerate().skip(1) {
                if !is_xid_continue(ch) {
                    println!(
                        "Invalid identifier: character '{}' at position {} is not allowed",
                        ch, i
                    );
                    valid = false;
                    break;
                }
            }
            if valid {
                println!(
                    "'{}' is a valid identifier according to Unicode rules",
                    test_id
                );
            }
        }
    }
}
