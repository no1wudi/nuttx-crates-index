// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

extern crate regex_lite;

use regex_lite::Regex;

/// Demonstrates basic regular expression functionality.
fn test_basic_patterns() {
    println!("Testing basic regex-lite patterns...");

    // Basic email pattern matching
    let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    let test_emails = ["user@example.com", "invalid.email@", "not-an-email"];

    for email in test_emails {
        println!("Is '{}' a valid email? {}", email, email_re.is_match(email));
    }

    // Simple phone number pattern
    let phone_re = Regex::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();
    let test_phones = ["123-456-7890", "1234-567-890", "123-456-789"];

    for phone in test_phones {
        println!(
            "Is '{}' a valid phone number? {}",
            phone,
            phone_re.is_match(phone)
        );
    }

    // Capturing example
    let capture_re = Regex::new(r"(\w+):(\d+)").unwrap();
    let text = "count:42 items:123";

    for cap in capture_re.captures_iter(text) {
        println!("Found key: {}, value: {}", &cap[1], &cap[2]);
    }
}

/// Main entry point for the regex-lite example
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_regex_lite_main() {
    println!("Starting regex-lite examples...");
    test_basic_patterns();
    println!("All regex-lite tests completed successfully!");
}
